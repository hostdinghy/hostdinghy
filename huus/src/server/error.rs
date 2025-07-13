use api::error::Error;

use crate::utils::cli::CliError;

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
