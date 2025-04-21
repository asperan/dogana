/// The policy for accepting an exit code of a test.
#[derive(Debug, Clone, Copy)]
pub enum AcceptExitCode {
    /// Accept all exit codes, i.e. ignore all errors.
    All,
    /// Accept only success exit codes (i.e. only `0`).
    Success,
    /// Accept only error exit codes (i.e. from `1` to `255`).
    Error,
    /// Accept only a specific exit code.
    Specific(u8),
}

impl AcceptExitCode {
    pub(crate) fn accept(&self, exit_code: u8, deny_message: &str) {
        match self {
            AcceptExitCode::Success => assert_eq!(0u8, exit_code, "{}", deny_message),
            AcceptExitCode::Specific(c) => assert_eq!(c.to_owned(), exit_code, "{}", deny_message),
            AcceptExitCode::Error => assert_ne!(0u8, exit_code, "{}", deny_message),
            _ => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success_accept_zero() {
        AcceptExitCode::Success.accept(0, "this message should not be displayed");
    }

    #[test]
    #[should_panic]
    fn success_deny_non_zero() {
        AcceptExitCode::Success.accept(1, "Success does not accept non-zero exit codes");
    }

    #[test]
    fn error_accept_non_zero() {
        AcceptExitCode::Error.accept(1, "this message should not be displayed");
    }

    #[test]
    #[should_panic]
    fn error_deny_zero() {
        AcceptExitCode::Error.accept(0, "Error does not accept 0 exit code");
    }

    #[test]
    fn specific_accept_inner() {
        let ec = 8;
        AcceptExitCode::Specific(ec).accept(ec, "this message should no be displayed");
    }

    #[test]
    #[should_panic]
    fn specific_deny_non_inner() {
        let ec = 8;
        AcceptExitCode::Specific(ec).accept(
            ec + 1,
            "Specific does not accept other exit codes other than the inner one",
        );
    }

    #[test]
    fn all_accept_any() {
        AcceptExitCode::All.accept(0, "All accept any exit code");
        AcceptExitCode::All.accept(1, "All accept any exit code");
        AcceptExitCode::All.accept(8, "All accept any exit code");
    }
}
