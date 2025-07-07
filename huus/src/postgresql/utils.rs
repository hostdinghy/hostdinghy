use crate::utils::{
	cli::CliError,
	cmd::{CmdError, cmd},
};

pub async fn create_superuser(
	name: &str,
	password: &str,
) -> Result<(), CliError> {
	let sql = format!(
		"CREATE USER {name} WITH LOGIN SUPERUSER CREATEDB CREATEROLE \
		INHERIT NOREPLICATION CONNECTION LIMIT -1 PASSWORD '{password}';"
	);

	cli_execute_sql(&sql).await.map(|_| ())
}

async fn cli_execute_sql(sql: &str) -> Result<String, CliError> {
	cmd(&["sudo", "-u", "postgres", "psql", "-c", sql])
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
