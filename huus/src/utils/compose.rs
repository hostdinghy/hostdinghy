use std::path::Path;

use crate::utils::cmd::{CmdError, cmd};

pub async fn up(file: impl AsRef<Path>) -> Result<(), CmdError> {
	cmd(&[
		"docker",
		"compose",
		"-f",
		&file.as_ref().display().to_string(),
		"up",
		"-d",
		"--pull",
		"always",
	])
	.run()
	.await
	.map(|_| ())
}

pub async fn restart(file: impl AsRef<Path>) -> Result<(), CmdError> {
	cmd(&[
		"docker",
		"compose",
		"-f",
		&file.as_ref().display().to_string(),
		"restart",
	])
	.run()
	.await
	.map(|_| ())
}
