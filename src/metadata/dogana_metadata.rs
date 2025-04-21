use std::{collections::HashMap, fmt::Display};

use serde::Deserialize;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ImageVariant {
    Alpine,
    Debian,
}

impl ImageVariant {
    pub fn install_sys_package_instruction(&self, packages: &[String]) -> String {
        let pkgs = packages.join(" ");
        format!(
            "{} {}",
            match self {
                Self::Alpine => "apk add --no-cache",
                Self::Debian => "apt-get update && apt-get install -y --no-install-recommends",
            },
            pkgs
        )
    }
}

impl Display for ImageVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ImageVariant::Alpine => "alpine",
                ImageVariant::Debian => "debian",
            }
        )
    }
}

#[derive(Deserialize)]
pub struct DoganaMetadata {
    pub dogana: Option<HashMap<ImageVariant, VariantMetadata>>,
}

#[derive(Deserialize)]
pub struct VariantMetadata {
    pub required_packages: Option<Vec<String>>,
}
