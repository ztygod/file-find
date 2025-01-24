use std::process;

use chrono::format::Item;
#[cfg(unix)]
use nix::sys::signal::{raise, signal, SigHandler, Signal};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExitCode {
    Success,
    HasResult(bool),
    GeneralError,
    KilledBySigint,
}

impl From<ExitCode> for i32 {
    fn from(value: ExitCode) -> Self {
        match value {
            ExitCode::Success => 0,
            ExitCode::HasResult(has_results) => !has_results as i32,
            ExitCode::GeneralError => 1,
            ExitCode::KilledBySigint => 130,
        }
    }
}

impl ExitCode {
    fn is_error(self) -> bool {
        i32::from(self) != 0
    }

    pub fn exit(self) -> ! {
        #[cfg(unix)]
        if self == ExitCode::KilledBySigint {
            unsafe {
                if signal(Signal::SIGINT, SigHandler::SigDfl).is_ok() {
                    let _ = raise(Signal::SIGINT);
                }
            }
        }
        process::exit(self.into())
    }

    pub fn merge_exitcodes(results: impl IntoIterator<Item = ExitCode>) -> ExitCode {
        if results.into_iter().any(ExitCode::is_error) {
            return ExitCode::GeneralError;
        }
        ExitCode::Success
    }
}
