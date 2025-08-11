use std::path::Path;

use crate::utils::cmd::{CmdError, cmd};

pub async fn up(file: impl AsRef<Path>) -> Result<(), CmdError> {
	cmd(&[
		"docker",
		"compose",
		"-f",
		&file.as_ref().to_string_lossy(),
		"up",
		"-d",
		"--pull",
		"always",
		"--remove-orphans",
	])
	.run()
	.await
	.map(|_| ())
}

pub async fn start(file: impl AsRef<Path>) -> Result<(), CmdError> {
	cmd(&[
		"docker",
		"compose",
		"-f",
		&file.as_ref().to_string_lossy(),
		"start",
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
		&file.as_ref().to_string_lossy(),
		"restart",
	])
	.run()
	.await
	.map(|_| ())
}

pub async fn stop(file: impl AsRef<Path>) -> Result<(), CmdError> {
	cmd(&[
		"docker",
		"compose",
		"-f",
		&file.as_ref().to_string_lossy(),
		"stop",
	])
	.run()
	.await
	.map(|_| ())
}

pub async fn logs(
	file: impl AsRef<Path>,
	lines: Option<u32>,
) -> Result<String, CmdError> {
	let file = file.as_ref().to_string_lossy();
	let mut args = vec!["docker", "compose", "-f", &file, "logs"];

	let lines_string;
	if let Some(l) = lines {
		args.push("-n");
		lines_string = l.to_string();
		args.push(&lines_string);
	}

	cmd(&args).run().await
}

pub async fn exec(
	file: impl AsRef<Path>,
	service: &str,
	command: &[&str],
) -> Result<String, CmdError> {
	let file_str = file.as_ref().to_string_lossy();
	let mut args = vec!["docker", "compose", "-f", &file_str, "exec", service];
	args.extend(command);

	cmd(&args).run().await
}
