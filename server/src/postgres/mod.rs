pub mod client;
pub mod utils;

use chuchi_crypto::token::Token;
use clap::Parser;
use tokio::{fs::File, io};
use tracing::info;

use crate::utils::cli::{CliError, WithMessage};
pub use client::Client;

#[derive(Debug, Parser)]
pub struct Postgres {
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
	DumpDatabase(DumpDatabase),
}

pub async fn postgres(postgres: Postgres) {
	let res = inner_postgres(postgres).await;

	if let Err(e) = res {
		tracing::error!("PostgreSQL command failed: {e}");
	}
}

pub async fn inner_postgres(postgres: Postgres) -> Result<(), CliError> {
	match postgres.cmd {
		SubCommand::CreateUser(mut cu) => {
			let tell_pw = cu.password.is_none();
			create_user(&mut cu).await?;
			info!("User created successfully.");
			if tell_pw {
				info!("Username: {}", cu.username);
				info!("Password: {}", cu.password.as_ref().unwrap());
			}
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
		SubCommand::DumpDatabase(dd) => {
			dump_database(dd).await?;
			info!("Database dumped successfully.");
		}
	}

	Ok(())
}

#[derive(Debug, Parser)]
pub struct CreateUser {
	username: String,
	/// If not password is provided a password will be generated
	password: Option<String>,
	#[clap(long, help = "Create user with superuser privileges")]
	superuser: bool,
}

async fn create_user(create_user: &mut CreateUser) -> Result<(), CliError> {
	let client = Client::new().await?;

	let password = create_user
		.password
		.get_or_insert_with(|| Token::<32>::new().to_string());

	if create_user.superuser {
		client
			.create_superuser(&create_user.username, &password)
			.await?;
	} else {
		client.create_user(&create_user.username, &password).await?;
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

#[derive(Debug, Parser)]
pub struct DumpDatabase {
	database_name: String,
	output_file: String,
}

async fn dump_database(dump_database: DumpDatabase) -> Result<(), CliError> {
	let mut file = File::create(&dump_database.output_file)
		.await
		.with_message("failed to create output file")?;

	let mut child = utils::dump_database(&dump_database.database_name).await?;

	io::copy(&mut child, &mut file)
		.await
		.with_message("failed to write database dump to file")?;

	Ok(())
}
