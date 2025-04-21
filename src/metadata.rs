pub mod dogana_metadata;
mod msrv;
use std::sync::LazyLock;

use crate::metadata::dogana_metadata::ImageVariant;
use cargo_metadata::{MetadataCommand, Package};
use dogana_metadata::DoganaMetadata;
use msrv::msrv;

static PACKAGE_NAME: LazyLock<String> = LazyLock::new(|| {
    std::env::var("CARGO_PKG_NAME").expect("CARGO_PKG_NAME should be populated by cargo")
});

static PACKAGE_VERSION: LazyLock<String> = LazyLock::new(|| {
    std::env::var("CARGO_PKG_VERSION").expect("CARGO_PKG_VERSION should be populated by cargo")
});

static MSRV: LazyLock<String> = LazyLock::new(|| match msrv() {
    Ok(m) => m,
    Err(e) => panic!("failed to retrieve MSRV: {}", e),
});

static PACKAGE_METADATA: LazyLock<Package> = LazyLock::new(|| {
    let mut cmd = MetadataCommand::new();
    cmd.no_deps();
    let result = cmd.exec();
    match result {
        Ok(metadata) => metadata
            .workspace_packages()
            .iter()
            .find(|it| it.name == package_name())
            .expect("The current package must exist in cargo metadata")
            .to_owned()
            .to_owned(),
        Err(e) => panic!("{}", e),
    }
});

static PACKAGE_BINS: LazyLock<Vec<&'static str>> = LazyLock::new(|| {
    PACKAGE_METADATA
        .targets
        .iter()
        .filter(|it| it.is_bin())
        .map(|it| it.name.as_str())
        .collect()
});

static DOGANA_METADATA: LazyLock<Option<DoganaMetadata>> =
    LazyLock::new(
        || match serde_json::from_value(PACKAGE_METADATA.metadata.clone()) {
            Ok(m) => m,
            Err(e) => panic!("failed to parse Dogana metadata: {}", e),
        },
    );

pub fn package_name() -> &'static str {
    PACKAGE_NAME.as_str()
}

pub fn package_version() -> &'static str {
    PACKAGE_VERSION.as_str()
}

pub fn package_msrv() -> &'static str {
    MSRV.as_str()
}

pub fn package_bins() -> &'static [&'static str] {
    PACKAGE_BINS.as_slice()
}

pub fn required_system_packages(variant: ImageVariant) -> Vec<String> {
    DOGANA_METADATA
        .as_ref()
        .and_then(|it| it.dogana.as_ref())
        .and_then(|it| it.get(&variant))
        .and_then(|it| it.required_packages.clone())
        .unwrap_or_default()
}
