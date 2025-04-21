use crate::{
    image_builder::ImageBuilder,
    image_name::ImageName,
    metadata::{dogana_metadata::ImageVariant, package_msrv},
};

pub struct DebianImageBuilder;

impl ImageBuilder for DebianImageBuilder {
    fn run_stage_base_image(&self) -> ImageName {
        ImageName("docker.io/library/debian:12.9-slim".to_string())
    }
    fn build_stage_base_image(&self) -> ImageName {
        ImageName(format!("docker.io/library/rust:{}-slim", package_msrv()))
    }
    fn variant(&self) -> ImageVariant {
        ImageVariant::Debian
    }
}
