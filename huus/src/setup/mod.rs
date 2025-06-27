mod cmd;
mod error;

use clap::Parser;
use tracing::info;

use crate::setup::{cmd::cmd, error::SetupError};

#[derive(Debug, Parser)]
pub struct Setup {
	#[clap(subcommand)]
	cmd: SubCommand,
}

#[derive(Debug, Parser)]
enum SubCommand {
	Docker,
	Traefik,
}

pub async fn setup(setup: Setup) {
	let res = inner_setup(setup).await;

	if let Err(e) = res {
		tracing::error!("Setup failed: {}", e);
	}
}

pub async fn inner_setup(setup: Setup) -> Result<(), SetupError> {
	match setup.cmd {
		SubCommand::Docker => {
			setup_docker().await?;

			info!("Docker setup completed successfully.");
		}
		SubCommand::Traefik => {
			setup_traefik().await?;

			info!("Traefik setup completed successfully.");
		}
	}

	Ok(())
}

const SETUP_DOCKER: &str = r#"
# Add Docker's official GPG key:
sudo apt-get update
sudo apt-get install ca-certificates curl
sudo install -m 0755 -d /etc/apt/keyrings
sudo curl -fsSL https://download.docker.com/linux/debian/gpg -o /etc/apt/keyrings/docker.asc
sudo chmod a+r /etc/apt/keyrings/docker.asc

# Add the repository to Apt sources:
echo \
  "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.asc] https://download.docker.com/linux/debian \
  $(. /etc/os-release && echo "$VERSION_CODENAME") stable" | \
  sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
sudo apt-get update
"#;

async fn setup_docker() -> Result<(), SetupError> {
	// cmd(&["apt", "update"]).run().await?;
	// cmd(&["apt", "install", "-y", "ca-certificates", "curl"])
	// 	.run()
	// 	.await?;
	// cmd(&["install", "-m", "0755", "-d", "/etc/apt/keyrings"])
	// 	.run()
	// 	.await?;
	// cmd(&[
	// 	"curl",
	// 	"-fsSL",
	// 	"https://download.docker.com/linux/debian/gpg",
	// 	"-o",
	// 	"/etc/apt/keyrings/docker.asc",
	// ])
	// .run()
	// .await?;
	// cmd(&["chmod", "a+r", "/etc/apt/keyrings/docker.asc"])
	// 	.run()
	// 	.await?;

	cmd(&["bash", "-c", SETUP_DOCKER]).run().await?;

	Ok(())
}

async fn setup_traefik() -> Result<(), SetupError> {
	todo!()
}
