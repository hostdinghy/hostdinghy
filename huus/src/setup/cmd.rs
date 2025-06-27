use std::path::Path;

use tokio::process::Command;

use crate::setup::error::SetupError;

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
	pub fn current_dir(mut self, path: impl AsRef<Path>) -> Self {
		self.inner.current_dir(path);
		self
	}

	pub async fn run(mut self) -> Result<String, SetupError> {
		let output = self
			.inner
			.output()
			.await
			.map_err(|e| SetupError::cmd(&self.display, e))?;

		if !output.status.success() {
			return Err(SetupError::cmd(
				self.display,
				String::from_utf8_lossy(&output.stderr),
			));
		}

		Ok(String::from_utf8_lossy(&output.stdout).to_string())
	}
}
