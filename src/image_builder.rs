use std::{path::Path, sync::Arc};

use hierrorchy::error_leaf;
use indoc::formatdoc;
use std::io::Error as IoError;

use crate::{
    container_manager::CONTAINER_MANAGER,
    image_name::ImageName,
    metadata::{
        dogana_metadata::ImageVariant, package_bins, package_msrv, package_name, package_version,
        required_system_packages,
    },
};

const BUILD_STAGE: &str = "builder";
const RUN_STAGE: &str = "runner";
const BASE_BUILD_DIR: &str = "/project";

pub trait ImageBuilder {
    fn variant(&self) -> ImageVariant;

    fn build_stage_base_image(&self) -> ImageName;

    fn run_stage_base_image(&self) -> ImageName;

    fn build(&self) -> Result<ImageName, Box<dyn std::error::Error>> {
        let image_name = output_image_name(self.variant());
        let dockerfile_path = self.temp_dockerfile()?;
        let result = std::process::Command::new(&*image_builder_executable())
            .args([
                "build",
                "-t",
                &image_name,
                "-f",
                dockerfile_path
                    .to_str()
                    .expect("The temp dockerfile path should be a valid UTF-8 string"),
                ".",
            ])
            .output()
            .expect("Building the image does not fail");
        if !result.status.success() {
            return Err(ImageBuildError::new(
                std::str::from_utf8(&result.stderr).expect("result stderr is a valid string"),
            )
            .into());
        }
        Ok(image_name)
    }

    fn temp_dockerfile(&self) -> Result<Box<Path>, IoError> {
        let tmp_dockerfile_path = std::env::temp_dir().join(format!(
            "Dockerfile.{}-integration-tests.{}",
            package_name(),
            self.variant(),
        ));
        let dockerfile_content = formatdoc! { "
        FROM {} AS {BUILD_STAGE}
        WORKDIR {BASE_BUILD_DIR}
        COPY ./ ./
        RUN cargo build

        FROM {} AS {RUN_STAGE}
        {}
        {}
        ",
        self.build_stage_base_image(),
        self.run_stage_base_image(),
        install_system_packages_instruction(self.variant()),
        copy_bins_instruction(),
        };
        std::fs::write(&tmp_dockerfile_path, dockerfile_content)?;
        Ok(tmp_dockerfile_path.as_path().into())
    }
}

#[error_leaf(format!("failed to build image: {}", self.stderr_content))]
struct ImageBuildError {
    stderr_content: String,
}

impl ImageBuildError {
    pub fn new(message: &str) -> Self {
        ImageBuildError {
            stderr_content: message.to_owned(),
        }
    }
}

fn image_builder_executable() -> Arc<Path> {
    CONTAINER_MANAGER.clone()
}

fn output_image_name(variant: ImageVariant) -> ImageName {
    ImageName(format!(
        "{}-integration-tests-base-{}:{}-rust{}",
        package_name(),
        variant,
        package_version(),
        package_msrv()
    ))
}

fn install_system_packages_instruction(variant: ImageVariant) -> String {
    let sys_packages = required_system_packages(variant);
    if sys_packages.is_empty() {
        String::new()
    } else {
        format!(
            "RUN {}",
            variant.install_sys_package_instruction(&sys_packages)
        )
    }
}

fn copy_bins_instruction() -> String {
    let generated_bins = package_bins();
    if generated_bins.is_empty() {
        String::new()
    } else {
        let bin_paths = generated_bins
            .iter()
            .map(|it| format!("{}/{}/{}", BASE_BUILD_DIR, "target/debug", it))
            .reduce(|acc, elem| acc + " " + &elem)
            .expect("iterator of generated bins cannot be empty");
        format!("COPY --from={} {} /usr/local/bin/", BUILD_STAGE, bin_paths)
    }
}
