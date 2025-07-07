use crate::utils::{
	cli::CliError,
	cmd::{CmdError, cmd},
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

pub async fn start_postgresql() -> Result<(), CliError> {
	cmd(&["systemctl", "start", "postgresql"])
		.run()
		.await
		.map(|_| ())
		.map_err(Into::into)
}

pub async fn stop_postgresql() -> Result<(), CliError> {
	cmd(&["systemctl", "stop", "postgresql"])
		.run()
		.await
		.map(|_| ())
		.map_err(Into::into)
}
