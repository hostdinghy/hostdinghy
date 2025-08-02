use thiserror::Error;

use crate::utils::cmd::CmdError;

#[derive(Debug, Error)]
pub enum CliError {
	#[error("Failed to run command: {command}, message: {message}")]
	Command { command: String, message: String },
	#[error("HOSTDINGHY_DIR environment variable is not set")]
	HostdinghyDirNotPresent,
	#[error(
		"HOSTDINGHY_DIR environment variable is already set to {0}, cannot set it again"
	)]
	HostdinghyDirAlreadySet(String),
	#[error("This command must be run as root")]
	NotRoot,
	#[error("{message} - {error}")]
	Any {
		message: String,
		error: Box<dyn std::error::Error + Send + Sync>,
	},
}

impl CliError {
	pub fn cmd(command: impl ToString, message: impl ToString) -> Self {
		Self::Command {
			command: command.to_string(),
			message: message.to_string(),
		}
	}

	pub fn any<E: Into<Box<dyn std::error::Error + Send + Sync + 'static>>>(
		message: impl ToString,
		e: E,
	) -> Self {
		Self::Any {
			message: message.to_string(),
			error: e.into(),
		}
	}
}

impl From<CmdError> for CliError {
	fn from(e: CmdError) -> Self {
		match e {
			CmdError::Command { command, message } => {
				CliError::cmd(command, message)
			}
		}
	}
}

pub trait WithMessage {
	type Ret;

	fn with_message(self, message: impl ToString) -> Self::Ret;
}

impl<T, E> WithMessage for Result<T, E>
where
	E: std::error::Error + Send + Sync + 'static,
{
	type Ret = Result<T, CliError>;

	fn with_message(self, message: impl ToString) -> Self::Ret {
		self.map_err(|e| CliError::any(message, e))
	}
}
