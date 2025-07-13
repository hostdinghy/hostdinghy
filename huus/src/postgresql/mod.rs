pub mod client;
pub mod utils;

use clap::Parser;
use tracing::info;

use crate::utils::cli::CliError;
use client::Client;

#[derive(Debug, Parser)]
pub struct Postgresql {
	#[clap(subcommand)]
	cmd: SubCommand,
}

#[derive(Debug, Parser)]
enum SubCommand {
	CreateUser(CreateUser),
	CreateDatabase(CreateDatabase),
	DropUser(DropUser),
	DropDatabase(DropDatabase),
	ListDatabases,
	ListUsers,
}

pub async fn postgresql(postgresql: Postgresql) {
	let res = inner_postgresql(postgresql).await;

	if let Err(e) = res {
		tracing::error!("PostgreSQL command failed: {e}");
	}
}

pub async fn inner_postgresql(postgresql: Postgresql) -> Result<(), CliError> {
	match postgresql.cmd {
		SubCommand::CreateUser(cu) => {
			create_user(cu).await?;
			info!("User created successfully.");
		}
		SubCommand::CreateDatabase(cd) => {
			create_database(cd).await?;
			info!("Database created successfully.");
		}
		SubCommand::DropUser(du) => {
			drop_user(du).await?;
			info!("User dropped successfully.");
		}
		SubCommand::DropDatabase(dd) => {
			drop_database(dd).await?;
			info!("Database dropped successfully.");
		}
		SubCommand::ListDatabases => {
			let databases = list_databases().await?;
			if databases.is_empty() {
				info!("No databases found.");
			} else {
				info!("Databases:");
				for database in databases {
					info!("- {}", database);
				}
			}
		}
		SubCommand::ListUsers => {
			let users = list_users().await?;
			if users.is_empty() {
				info!("No users found.");
			} else {
				info!("Users:");
				for user in users {
					info!("- {}", user);
				}
			}
		}
	}

	Ok(())
}

#[derive(Debug, Parser)]
pub struct CreateUser {
	username: String,
	password: String,
	#[clap(long, help = "Create user with superuser privileges")]
	superuser: bool,
}

async fn create_user(create_user: CreateUser) -> Result<(), CliError> {
	let client = Client::new().await?;

	if create_user.superuser {
		client
			.create_superuser(&create_user.username, &create_user.password)
			.await?;
	} else {
		client
			.create_user(&create_user.username, &create_user.password)
			.await?;
	}

	Ok(())
}

#[derive(Debug, Parser)]
pub struct CreateDatabase {
	database_name: String,
	owner: String,
}

async fn create_database(
	create_database: CreateDatabase,
) -> Result<(), CliError> {
	let client = Client::new().await?;

	client
		.create_database(&create_database.database_name, &create_database.owner)
		.await?;

	Ok(())
}

#[derive(Debug, Parser)]
pub struct DropUser {
	username: String,
}

async fn drop_user(drop_user: DropUser) -> Result<(), CliError> {
	let client = Client::new().await?;

	client.drop_user(&drop_user.username).await?;

	Ok(())
}

#[derive(Debug, Parser)]
pub struct DropDatabase {
	database_name: String,
}

async fn drop_database(drop_database: DropDatabase) -> Result<(), CliError> {
	let client = Client::new().await?;

	client.drop_database(&drop_database.database_name).await?;

	Ok(())
}

async fn list_databases() -> Result<Vec<String>, CliError> {
	let client = Client::new().await?;

	client.list_databases().await
}

async fn list_users() -> Result<Vec<String>, CliError> {
	let client = Client::new().await?;

	client.list_users().await
}
