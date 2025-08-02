use std::path::Path;

use api::app_id::AppId;
use clap::Parser;
use tokio::{fs, io::AsyncWriteExt as _};
use tracing::{error, info};

use crate::utils::{
	cli::{CliError, WithMessage as _},
	compose, hostdinghy_dir,
};

#[derive(Debug, Parser)]
pub struct Registry {
	#[clap(subcommand)]
	cmd: SubCommand,
}

#[derive(Debug, Parser)]
enum SubCommand {
	AddUser(AddUser),
	RemoveUser(RemoveUser),
	ListUsers,
	Restart,
}

pub async fn registry(registry: Registry) {
	let res = inner_registry(registry).await;

	if let Err(e) = res {
		error!("Setup failed: {}", e);
	}
}

pub async fn inner_registry(registry: Registry) -> Result<(), CliError> {
	match registry.cmd {
		SubCommand::AddUser(au) => {
			let username = add_user(au).await?;

			info!("User \"{username}\" added to registry successfully.");
		}
		SubCommand::RemoveUser(ru) => {
			remove_user(ru).await?;

			info!("User removed from registry successfully.");
		}
		SubCommand::ListUsers => {
			let users = list_users().await?;

			if users.is_empty() {
				info!("No users found in the registry.");
			} else {
				info!("Users in the registry:");
				for user in users {
					info!("- {}", user);
				}
			}
		}
		SubCommand::Restart => {
			restart_registry(hostdinghy_dir()?).await?;

			info!("Registry restarted successfully.");
		}
	}

	Ok(())
}

async fn restart_registry(
	hostdinghy_dir: impl AsRef<Path>,
) -> Result<(), CliError> {
	compose::restart(hostdinghy_dir.as_ref().join("registry/compose.yml"))
		.await?;

	Ok(())
}

#[derive(Debug, Parser)]
pub struct AddUser {
	app_id: Option<AppId>,
	username: String,
	password: String,
}

async fn add_user(add_user: AddUser) -> Result<String, CliError> {
	let prefix = add_user
		.app_id
		.map(|a| a.to_string())
		.unwrap_or_else(|| "internal".into());
	let username = format!("{prefix}${}", add_user.username);

	// check that the username does not exist and is valid
	if username.contains(":") {
		// todo better errors
		return Err(CliError::any(
			"username contains a colon symbol",
			username,
		));
	}

	let users = list_users().await?;
	if users.contains(&username) {
		return Err(CliError::any("username already exists", username));
	}

	let password = bcrypt::hash(&add_user.password, bcrypt::DEFAULT_COST)
		.with_message("Failed to hash password")?;

	let hostdinghy_dir = hostdinghy_dir()?;

	let password_file = hostdinghy_dir.join("registry/registry.password");
	fs::OpenOptions::new()
		.append(true)
		.open(&password_file)
		.await
		.with_message(
			"Failed to open $HOSTDINGHY_DIR/registry/registry.password",
		)?
		.write_all(format!("{username}:{password}\n").as_bytes())
		.await
		.with_message(
			"Failed to write to $HOSTDINGHY_DIR/registry/registry.password",
		)?;

	restart_registry(&hostdinghy_dir).await?;

	Ok(username)
}

#[derive(Debug, Parser)]
pub struct RemoveUser {
	username: String,
}

async fn remove_user(remove_user: RemoveUser) -> Result<(), CliError> {
	let hostdinghy_dir = hostdinghy_dir()?;
	let password_file = hostdinghy_dir.join("registry/registry.password");
	let content = fs::read_to_string(&password_file).await.with_message(
		"Failed to read $HOSTDINGHY_DIR/registry/registry.password",
	)?;

	let new_content: String = content
		.lines()
		.filter(|line| !line.starts_with(&remove_user.username))
		.collect::<Vec<_>>()
		.join("\n");

	fs::write(&password_file, new_content).await.with_message(
		"Failed to write to $HOSTDINGHY_DIR/registry/registry.password",
	)?;

	restart_registry(&hostdinghy_dir).await?;

	Ok(())
}

async fn list_users() -> Result<Vec<String>, CliError> {
	let hostdinghy_dir = hostdinghy_dir()?;
	let password_file = hostdinghy_dir.join("registry/registry.password");
	let content = fs::read_to_string(&password_file).await.with_message(
		"Failed to read $HOSTDINGHY_DIR/registry/registry.password",
	)?;

	let users: Vec<String> = content
		.lines()
		// skip comments
		.filter(|line| !line.starts_with('#'))
		.filter_map(|line| line.split_once(':'))
		.map(|(username, _)| username.to_string())
		.collect();

	Ok(users)
}
