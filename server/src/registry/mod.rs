pub mod routes;

use std::path::Path;

use chuchi_crypto::token::Token;
use clap::Parser;
use dialoguer::{Input, theme::ColorfulTheme};
use serde::{Deserialize, Serialize};
use tokio::{fs, io::AsyncWriteExt as _};
use tracing::{error, info};

use crate::utils::{
	cli::{CliError, WithMessage as _},
	compose, hostdinghy_dir, verify_root,
};

pub type WebhookToken = Token<32>;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct RegistryConfig {
	pub domain: String,
	pub webhook_token: WebhookToken,
}

impl RegistryConfig {
	pub fn new_from_user() -> Self {
		let domain: String = Input::with_theme(&ColorfulTheme::default())
			.with_prompt("Enter the domain for the registry")
			.interact_text()
			.unwrap();

		Self {
			domain,
			webhook_token: WebhookToken::new(),
		}
	}
}

pub async fn registry(registry: Registry) {
	let res = inner_registry(registry).await;

	if let Err(e) = res {
		error!("Setup failed: {}", e);
	}
}

pub async fn inner_registry(registry: Registry) -> Result<(), CliError> {
	verify_root().await?;

	match registry.cmd {
		SubCommand::AddUser(mut au) => {
			let tell_pw = au.password.is_none();
			add_user(&mut au).await?;

			info!("User added to registry successfully.");
			if tell_pw {
				info!("Username: {}", au.username);
				info!("Password: {}", au.password.as_ref().unwrap());
			}
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
	compose::restart(
		hostdinghy_dir.as_ref().join("registry/compose.yml"),
		None,
	)
	.await?;

	Ok(())
}

#[derive(Debug, Clone, Parser)]
pub struct AddUser {
	pub username: String,
	/// If no password is provided a random one will be generated
	pub password: Option<String>,
}

pub async fn add_user(add_user: &mut AddUser) -> Result<(), CliError> {
	// check that the username does not exist and is valid
	if add_user.username.contains(":") {
		// todo better errors
		return Err(CliError::any(
			"username contains a colon symbol",
			add_user.username.clone(),
		));
	}

	let users = list_users().await?;
	if users.contains(&add_user.username) {
		return Err(CliError::any(
			"username already exists",
			add_user.username.clone(),
		));
	}

	let password = add_user.password.get_or_insert_with(|| {
		// generate a random password
		Token::<32>::new().to_string()
	});

	let password = bcrypt::hash(password, bcrypt::DEFAULT_COST)
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
		.write_all(format!("{}:{password}\n", add_user.username).as_bytes())
		.await
		.with_message(
			"Failed to write to $HOSTDINGHY_DIR/registry/registry.password",
		)?;

	restart_registry(&hostdinghy_dir).await?;

	Ok(())
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
