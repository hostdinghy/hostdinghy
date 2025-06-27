use thiserror::Error;

#[derive(Debug, Error)]
pub enum SetupError {
	#[error("Failed to run command: {command}, message: {message}")]
	Command { command: String, message: String },
}

impl SetupError {
	pub fn cmd(command: impl ToString, message: impl ToString) -> Self {
		Self::Command {
			command: command.to_string(),
			message: message.to_string(),
		}
	}
}
