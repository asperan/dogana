pub mod accept_exit_code;
pub mod builder;
pub mod test_options;

use std::{
    env::temp_dir,
    error::Error,
    fs,
    io::Error as IoError,
    num::TryFromIntError,
    path::PathBuf,
    process::{Command, Output},
    str::Utf8Error,
    sync::Arc,
};

use builder::DoganaTestBuilder;
use hierrorchy::error_node;
use test_options::DoganaTestOptions;

use crate::{container_manager::CONTAINER_MANAGER, image_name::ImageName, metadata::package_name};

const INIT_PHASE_DELIMITER: &str = "===== INIT PHASE TERMINATED =====";

/// The type returned by test runs.
pub type DoganaTestResult = Result<(), TestExecutionError>;

/// A Dogana test instance.
#[derive(Debug)]
pub struct DoganaTest {
    test_name: String,
    base_image: Arc<ImageName>,
    init_commands: Vec<String>,
    run_commands: Vec<String>,
    expected_output: String,
    test_options: DoganaTestOptions,
}

impl DoganaTest {
    /// Create a new [DoganaTestBuilder].
    pub fn builder() -> DoganaTestBuilder {
        DoganaTestBuilder::new()
    }

    /// Create a new test instance. Prefer using the builder as it has already the logic for
    /// handling optional fields.
    pub fn new(
        test_name: String,
        base_image: &Arc<ImageName>,
        init_commands: Vec<String>,
        run_commands: Vec<String>,
        expected_output: String,
        test_options: DoganaTestOptions,
    ) -> DoganaTest {
        DoganaTest {
            test_name,
            base_image: base_image.clone(),
            init_commands,
            run_commands,
            expected_output,
            test_options,
        }
    }

    pub fn run(&self) -> DoganaTestResult {
        let result = self.prepare_test_container()?.output()?;
        let (output, err_output, exit_code) = self.extract_output(result)?;
        self.test_options.accepted_exit_codes.accept(
            exit_code,
            &format!("stdout:\n{output}\n\nstderr:\n{err_output}"),
        );
        let run_output = output
            .lines()
            .skip_while(|it| it != &INIT_PHASE_DELIMITER)
            .skip(1)
            .map(|it| it.to_owned())
            .reduce(|acc, it| acc + "\n" + &it)
            .unwrap_or_else(String::new);
        assert_eq!(run_output, self.expected_output);
        Ok(())
    }

    fn prepare_test_script(&self) -> Result<PathBuf, TestScriptPreparationError> {
        let test_script_content = format!(
            "{}\necho '{}'\n{}",
            &self.init_commands.join("\n"),
            INIT_PHASE_DELIMITER,
            &self.run_commands.join("\n")
        );
        let test_script_path = temp_dir().join(format!(
            "test-script_{}_{}",
            package_name(),
            &self.test_name
        ));
        fs::write(&test_script_path, test_script_content)?;
        Ok(test_script_path)
    }

    fn prepare_test_container(&self) -> Result<Command, TestContainerPreparationError> {
        let container_name = format!("{}_dogana-test_{}", package_name(), &self.test_name);
        let test_script_path = self.prepare_test_script()?;
        let mut cmd = std::process::Command::new(&*CONTAINER_MANAGER.clone());
        cmd.arg("run");
        if !self.test_options.keep_old_containers {
            cmd.arg("--rm");
        }
        cmd.args([
            "-v",
            &format!(
                "{}:/test_script",
                &test_script_path
                    .to_str()
                    .expect("test script path is correct")
            ),
            "--name",
            &container_name,
            &self.base_image,
            "/usr/bin/env",
            &self.test_options.shell.to_string(),
            "/test_script",
        ]);
        Ok(cmd)
    }

    fn extract_output(
        &self,
        cmd_output: Output,
    ) -> Result<(String, String, u8), OutputExtractionError> {
        let output = std::str::from_utf8(&cmd_output.stdout)?.to_owned();
        let err_output = std::str::from_utf8(&cmd_output.stderr)?.to_owned();
        let exit_status = cmd_output
            .status
            .code()
            .expect("The container should return an exit code")
            .try_into()?;
        Ok((output, err_output, exit_status))
    }
}

error_node! {
    pub type OutputExtractionError<Utf8Error, TryFromIntError> = "failed to extract output"
}

error_node! {
    pub type TestScriptPreparationError<IoError> = "failed to prepare test script"
}

error_node! {
    pub type TestContainerPreparationError<TestScriptPreparationError> = "failed to prepare test container"
}

error_node! {
    pub type TestExecutionError<TestContainerPreparationError, OutputExtractionError, IoError> = "failed to execute test"
}
