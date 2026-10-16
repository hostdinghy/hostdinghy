use std::path::Path;

use api::apps::ComposeCommand;

use crate::utils::cmd::{CmdBuilder, CmdError, cmd};

/// A compose command with plain output, to stream it with
/// [`CmdBuilder::spawn_events`]
pub fn command(
	file: impl AsRef<Path>,
	command: &ComposeCommand,
	service: Option<&str>,
) -> CmdBuilder {
	let file = file.as_ref().to_string_lossy();
	let mut args = vec![
		"docker",
		"compose",
		"--ansi",
		"never",
		"--progress",
		"plain",
		"-f",
		&file,
	];
	args.extend_from_slice(match command {
		ComposeCommand::Start => &["start"],
		ComposeCommand::Up => {
			&["up", "-d", "--pull", "always", "--remove-orphans"]
		}
		ComposeCommand::Restart => &["restart"],
		ComposeCommand::Stop => &["stop"],
	});

	cmd(&args).arg_opt(service)
}

pub async fn up(
	file: impl AsRef<Path>,
	service: Option<&str>,
) -> Result<(), CmdError> {
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
	.arg_opt(service)
	.run()
	.await
	.map(|_| ())
}

pub async fn restart(
	file: impl AsRef<Path>,
	service: Option<&str>,
) -> Result<(), CmdError> {
	cmd(&[
		"docker",
		"compose",
		"-f",
		&file.as_ref().to_string_lossy(),
		"restart",
	])
	.arg_opt(service)
	.run()
	.await
	.map(|_| ())
}

/// `docker compose logs` with a line per entry, see
/// [`api::apps::LogLine::parse_compose`]
pub fn logs(
	file: impl AsRef<Path>,
	lines: Option<u32>,
	follow: bool,
) -> CmdBuilder {
	let file = file.as_ref().to_string_lossy();
	let lines = lines.map_or("all".to_string(), |l| l.to_string());
	let mut args = vec![
		"docker",
		"compose",
		"--ansi",
		"never",
		"-f",
		&file,
		"logs",
		"--no-color",
		"--timestamps",
		"--tail",
		&lines,
	];
	if follow {
		args.push("--follow");
	}

	cmd(&args)
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
