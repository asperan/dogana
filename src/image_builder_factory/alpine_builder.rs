use crate::{
    image_builder::ImageBuilder,
    image_name::ImageName,
    metadata::{dogana_metadata::ImageVariant, package_msrv},
};

pub struct AlpineImageBuilder;

impl ImageBuilder for AlpineImageBuilder {
    fn variant(&self) -> ImageVariant {
        ImageVariant::Alpine
    }
    fn build_stage_base_image(&self) -> ImageName {
        ImageName(format!(
            "docker.io/library/rust:{}-alpine3.21",
            package_msrv()
        ))
    }
    fn run_stage_base_image(&self) -> ImageName {
        ImageName("docker.io/library/alpine:3.21".to_string())
    }
}
