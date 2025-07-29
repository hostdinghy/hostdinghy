use api::error::Error;

use crate::utils::{cli::CliError, cmd::CmdError};

impl From<CliError> for Error {
	fn from(e: CliError) -> Self {
		match e {
			CliError::Command { command, message } => {
				Error::Command { command, message }
			}
			CliError::HuusDirNotPresent => Error::HuusDirNotPresent,
			CliError::Any { message, error } => Error::Any {
				message,
				error: error.to_string(),
			},
			e => Error::Any {
				message: "Other CliError".into(),
				error: e.to_string(),
			},
		}
	}
}

impl From<CmdError> for Error {
	fn from(e: CmdError) -> Self {
		match e {
			CmdError::Command { command, message } => {
				Error::Command { command, message }
			}
		}
	}
}
