use std::path::Path;

use tokio::process::Command;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum CmdError {
	#[error("Failed to run command: {command}, message: {message}")]
	Command { command: String, message: String },
}

impl CmdError {
	pub fn cmd(command: impl ToString, message: impl ToString) -> Self {
		Self::Command {
			command: command.to_string(),
			message: message.to_string(),
		}
	}
}

pub fn cmd(args: &[&str]) -> CmdBuilder {
	assert!(!args.is_empty(), "args should not be empty");
	let mut c = Command::new(args[0]);
	c.args(&args[1..]);

	CmdBuilder {
		display: args.join(" "),
		inner: c,
	}
}

#[derive(Debug)]
pub struct CmdBuilder {
	display: String,
	inner: Command,
}

impl CmdBuilder {
	#[allow(dead_code)]
	pub fn current_dir(mut self, path: impl AsRef<Path>) -> Self {
		self.inner.current_dir(path);
		self
	}

	pub fn as_root(mut self) -> Self {
		#[cfg(unix)]
		self.inner.uid(0);
		self
	}

	pub async fn run(mut self) -> Result<String, CmdError> {
		let output = self
			.inner
			.output()
			.await
			.map_err(|e| CmdError::cmd(&self.display, e))?;

		if !output.status.success() {
			return Err(CmdError::cmd(
				self.display,
				String::from_utf8_lossy(&output.stderr),
			));
		}

		Ok(String::from_utf8_lossy(&output.stdout).to_string())
	}
}
