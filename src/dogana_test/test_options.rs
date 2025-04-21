use std::fmt::Display;

use super::accept_exit_code::AcceptExitCode;

/// The options for a Dogana Test.
///
/// The default DoganaTestOptions has the following values:
/// - `shell` = `Shell::Sh`
/// - `keep_old_containers` = `false`
/// - `accepted_exit_codes` = `AcceptExitCode::Success`
#[derive(Debug, Clone)]
pub struct DoganaTestOptions {
    /// The shell used to run the test script. It must be available in the container `PATH`, thus
    /// you may need to add it to the `required_packages` section in the image variant metadata.
    pub shell: Shell,
    /// Whether to keep the containers.
    /// **This option is most useful when debugging a test. It is not recommended to enable this
    /// option for normal usage.**
    pub keep_old_containers: bool,
    /// See [AcceptExitCode] for more details.
    pub accepted_exit_codes: AcceptExitCode,
}

impl Default for DoganaTestOptions {
    fn default() -> Self {
        DoganaTestOptions {
            shell: Shell::default(),
            keep_old_containers: false,
            accepted_exit_codes: AcceptExitCode::Success,
        }
    }
}

/// The supported/available shell for running the test script.
/// Defaults to `sh` as it is always available.
/// Any other supported shell may be installed through the `required_packages` field in the image
/// variant metadata.
#[derive(Debug, Clone, Copy, Default)]
pub enum Shell {
    #[default]
    Sh,
    Bash,
}

impl Display for Shell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Sh => "sh",
                Self::Bash => "bash",
            }
        )
    }
}
