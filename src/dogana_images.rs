//! This module is the catalog of Dogana-built images.
//! Each if the static [LazyLock]s below is an image variant. See below for the metadata key to use
//! for configuring the images.

use std::sync::{Arc, LazyLock};

use crate::{
    image_builder::ImageBuilder, image_builder_factory::ImageBuilderFactory, image_name::ImageName,
};

/// The debian image, with metadata key `debian`.
pub static DEBIAN_IMAGE: LazyLock<Arc<ImageName>> =
    LazyLock::new(|| match ImageBuilderFactory::debian_builder().build() {
        Ok(image) => image.into(),
        Err(e) => panic!("failed to build image: {}", e),
    });

/// The alpine image, with metadata key `alpine`.
pub static ALPINE_IMAGE: LazyLock<Arc<ImageName>> =
    LazyLock::new(|| match ImageBuilderFactory::alpine_builder().build() {
        Ok(image) => image.into(),
        Err(e) => panic!("failed to build image: {}", e),
    });
