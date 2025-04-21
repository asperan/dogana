use hierrorchy::error_leaf;
use std::path::Path;
use std::sync::{Arc, LazyLock};
use which::which;

const SUPPORTED_MANAGERS: [&str; 2] = ["podman", "docker"];

pub static CONTAINER_MANAGER: LazyLock<Arc<Path>> = LazyLock::new(|| match container_manager() {
    Ok(c) => c,
    Err(e) => panic!("failed to evaluate container manager: {}", e),
});

fn container_manager() -> Result<Arc<Path>, SupportedContainerManagerNotFound> {
    for ele in SUPPORTED_MANAGERS {
        if let Ok(container_manager) = which(ele) {
            return Ok(Path::new(&container_manager).into());
        }
    }
    Err(SupportedContainerManagerNotFound::new())
}

#[error_leaf(format!("No supported container manager found in path. Install one of [{}].", SUPPORTED_MANAGERS.join(", ")))]
#[derive(Clone, Copy)]
pub struct SupportedContainerManagerNotFound {}

impl SupportedContainerManagerNotFound {
    pub fn new() -> Self {
        SupportedContainerManagerNotFound {}
    }
}
