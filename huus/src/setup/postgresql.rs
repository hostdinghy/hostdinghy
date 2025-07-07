use clap::Parser;
use std::path::Path;
use std::{borrow::Cow, time::Duration};
use tokio::{
	fs::{self, OpenOptions},
	io::AsyncWriteExt,
	time::sleep,
};

use crate::postgresql::utils::{cli_execute_sql, start_postgresql};
use crate::{
	postgresql::utils::stop_postgresql,
	utils::{
		cli::{CliError, WithMessage as _},
		cmd::cmd,
	},
};

use super::huus_dir;

#[derive(Debug, Parser)]
pub struct Postgresql {
	// domain: String,
}

pub async fn setup(_registry: Postgresql) -> Result<(), CliError> {
	let huus_dir = huus_dir()?;
	let postgresql_dir = huus_dir.join("postgresql");
	let data_dir = postgresql_dir.join("data");

	// todo update to async
	if data_dir.is_dir() {
		return Err(CliError::any(
			"$HUUS_DIR/postgresql/data already exists",
			"",
		));
	}

	cmd(&["apt", "install", "postgresql", "-y"])
		.as_root()
		.run()
		.await?;

	sleep(Duration::from_secs(5)).await;

	// change the data folder
	stop_postgresql().await?;

	sleep(Duration::from_secs(5)).await;

	fs::create_dir_all(&postgresql_dir)
		.await
		.with_message("Failed to create $HUUS_DIR/postgresql directory")?;

	// detect the latest postgresql version
	let version = latest_postgresql_version().await?;
	let var_path = Path::new("/var/lib/postgresql").join(&version.to_string());
	let etc_path = Path::new("/etc/postgresql").join(&version.to_string());

	// move previous data_directory to the new location
	let old_data_path = var_path.join("main");
	cmd(&[
		"mv",
		&old_data_path.to_string_lossy(),
		&data_dir.to_string_lossy(),
	])
	.run()
	.await?;

	// chown to postgres:postgres
	cmd(&[
		"chown",
		"-R",
		"postgres:postgres",
		&data_dir.to_string_lossy(),
	])
	.run()
	.await?;

	// change the data_directory in postgresql.conf
	let conf_path = etc_path.join("main/postgresql.conf");

	let content = fs::read_to_string(&conf_path).await.with_message(
		format!("Failed to read {} configuration file", conf_path.display()),
	)?;

	let new_content: String = content
		.lines()
		.map(|line| {
			if line.trim_start().starts_with("data_directory")
				|| line.trim_start().starts_with("#data_directory")
			{
				Cow::Owned(format!("data_directory = '{}'", data_dir.display()))
			} else if line.trim_start().starts_with("listen_addresses")
				|| line.trim_start().starts_with("#listen_addresses")
			{
				Cow::Borrowed("listen_addresses = '*'")
			} else {
				Cow::Borrowed(line)
			}
		})
		.collect::<Vec<_>>()
		.join("\n");

	fs::write(&conf_path, new_content)
		.await
		.with_message("Failed to write PostgreSQL configuration file")?;

	// modify pg_hba.conf to allow docker containers to connect
	let pg_hba_path = etc_path.join("main/pg_hba.conf");

	OpenOptions::new()
		.append(true)
		.open(&pg_hba_path)
		.await
		.with_message("Failed to open pg_hba.conf file")?
		.write_all(b"host  all  all  172.16.0.0/12  scram-sha-256\n")
		.await
		.with_message("Failed to append to pg_hba.conf file")?;

	start_postgresql().await?;

	sleep(Duration::from_secs(5)).await;

	// create a root user with we then can use to execute a query
	cli_execute_sql(
		"DO $$ BEGIN IF NOT EXISTS \
		(SELECT FROM pg_catalog.pg_user WHERE usename = 'root') \
		THEN CREATE USER root WITH SUPERUSER; END IF; END $$;",
	)
	.await?;

	Ok(())
}

async fn latest_postgresql_version() -> Result<u32, CliError> {
	// list directories in /var/lib/postgresql/ to find installed versions
	let mut entries = fs::read_dir("/var/lib/postgresql/")
		.await
		.with_message("Failed to read /var/lib/postgresql/ directory")?;

	let mut version = 0;
	while let Some(entry) = entries
		.next_entry()
		.await
		.with_message("Failed to read PostgreSQL directory entry")?
	{
		if entry
			.file_type()
			.await
			.with_message("Failed to get PostgreSQL directory file type")?
			.is_dir()
		{
			if let Some(parsed_version) = entry
				.file_name()
				.to_str()
				.and_then(|n| n.parse::<u32>().ok())
			{
				version = version.max(parsed_version);
			};
		}
	}

	assert!(
		version != 0,
		"No PostgreSQL versions found in /var/lib/postgresql/"
	);

	Ok(version)
}
