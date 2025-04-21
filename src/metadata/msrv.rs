use std::{error::Error, ffi::OsString, io::Error as IoError, process::Command, str::Utf8Error};

use hierrorchy::{error_leaf, error_node};

// Tries to detect MSRV, first from the `Cargo.toml`, else from the used rust compiler.
pub fn msrv() -> Result<String, MsrvError> {
    let cargo_manifest_msrv = std::env::var("CARGO_PKG_RUST_VERSION")
        .expect("CARGO_PKG_RUST_VERSION should be populated by cargo");
    if cargo_manifest_msrv.is_empty() {
        let rustc = std::env::var_os("RUSTC").unwrap_or_else(|| OsString::from("rustc"));
        let mut cmd =
            if let Some(wrapper) = std::env::var_os("RUSTC_WRAPPER").filter(|w| !w.is_empty()) {
                let mut cmd = Command::new(wrapper);
                cmd.arg(rustc);
                cmd
            } else {
                Command::new(rustc)
            };
        cmd.arg("-V");
        let result = cmd.output()?;
        if result.status.success() {
            let text = std::str::from_utf8(&result.stdout)?.to_owned();
            Ok(text
                .split(' ')
                .nth(1)
                .expect("Rustc -V output should contain its version")
                .to_owned())
        } else {
            Err(CommandError::new(std::str::from_utf8(&result.stderr)?).into())
        }
    } else {
        Ok(cargo_manifest_msrv)
    }
}

error_node! {
    pub type MsrvError<CommandError, Utf8Error, IoError> = "failed to detect msrv"
}

#[error_leaf(format!("command execution failed: {}", self.error))]
pub struct CommandError {
    error: String,
}

impl CommandError {
    pub fn new(error: &str) -> Self {
        CommandError {
            error: error.to_owned(),
        }
    }
}
