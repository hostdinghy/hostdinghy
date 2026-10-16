use crate::utils::{
	cli::CliError,
	cmd::{ChildReadableStdout, ChildWritableStdin, CmdError, cmd},
};

pub async fn cli_execute_sql(sql: &str) -> Result<String, CliError> {
	cmd(&["sudo", "-u", "postgres", "psql", "-c", sql])
		.as_root()
		.run()
		.await
		.map_err(|e| match e {
			CmdError::Command { message, .. } => CliError::Command {
				command: format!("failed to execute sql: {sql}"),
				message,
			},
		})
}

pub async fn start_postgres() -> Result<(), CliError> {
	cmd(&["systemctl", "start", "postgresql"])
		.run()
		.await
		.map(|_| ())
		.map_err(Into::into)
}

pub async fn stop_postgres() -> Result<(), CliError> {
	cmd(&["systemctl", "stop", "postgresql"])
		.run()
		.await
		.map(|_| ())
		.map_err(Into::into)
}

pub async fn dump_database(
	name: &str,
) -> Result<ChildReadableStdout, CliError> {
	// -Fc = custom format
	cmd(&["sudo", "-u", "postgres", "pg_dump", "-Fc", "-d", name])
		.as_root()
		.spawn_readable_stdout()
		.await
		.map_err(Into::into)
}

pub async fn restore_database(
	name: &str,
) -> Result<ChildWritableStdin, CliError> {
	cmd(&[
		"sudo",
		"-u",
		"postgres",
		"pg_restore",
		"--clean",
		"--if-exists",
		"--no-owner",
		"--no-privileges",
		"--role",
		name,
		"-d",
		name,
	])
	.as_root()
	.spawn_writable_stdin()
	.await
	.map_err(Into::into)
}

/// Restores a plain sql file as the database's role, stops at the first error
pub async fn restore_database_sql(
	name: &str,
) -> Result<ChildWritableStdin, CliError> {
	let set_role = format!("SET ROLE \"{name}\"");
	cmd(&[
		"sudo",
		"-u",
		"postgres",
		"psql",
		"--quiet",
		"--no-psqlrc",
		"--single-transaction",
		"--set",
		"ON_ERROR_STOP=1",
		"--output",
		"/dev/null",
		"-d",
		name,
		"-c",
		&set_role,
		"-f",
		"-",
	])
	.as_root()
	.spawn_writable_stdin()
	.await
	.map_err(Into::into)
}
