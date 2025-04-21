use alpine_builder::AlpineImageBuilder;
use debian_builder::DebianImageBuilder;

use crate::image_builder::ImageBuilder;

mod alpine_builder;
mod debian_builder;

pub struct ImageBuilderFactory;

impl ImageBuilderFactory {
    pub fn debian_builder() -> impl ImageBuilder {
        DebianImageBuilder {}
    }

    pub fn alpine_builder() -> impl ImageBuilder {
        AlpineImageBuilder {}
    }
}
