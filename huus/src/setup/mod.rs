mod registry;
mod traefik;

use std::path::PathBuf;

use clap::Parser;
use tokio::{
	fs::{self, OpenOptions},
	io::AsyncWriteExt,
};
use tracing::info;

use crate::utils::{
	cli::{CliError, WithMessage as _},
	cmd::cmd,
	huus_dir,
};

#[derive(Debug, Parser)]
pub struct Setup {
	#[clap(subcommand)]
	cmd: SubCommand,
}

#[derive(Debug, Parser)]
enum SubCommand {
	Docker,
	Dir {
		#[clap(default_value = "/huus")]
		dir: String,
	},
	Traefik(traefik::Traefik),
	Registry(registry::Registry),
}

pub async fn setup(setup: Setup) {
	let res = inner_setup(setup).await;

	if let Err(e) = res {
		tracing::error!("Setup failed: {}", e);
	}
}

pub async fn inner_setup(setup: Setup) -> Result<(), CliError> {
	match setup.cmd {
		SubCommand::Docker => {
			setup_docker().await?;

			info!("Docker setup completed successfully.");
		}
		SubCommand::Dir { dir } => {
			let new_dir = setup_dir(dir).await?;

			info!(
				"Directory \"{}\" setup completed successfully.",
				new_dir.display()
			);
		}
		SubCommand::Traefik(traefik) => {
			traefik::setup(traefik).await?;

			info!("Traefik setup completed successfully.");
		}
		SubCommand::Registry(registry) => {
			registry::setup(registry).await?;

			info!("Registry setup completed successfully.");
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

sudo apt-get install docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin -y

# sudo usermod -aG docker username
"#;

async fn setup_docker() -> Result<(), CliError> {
	cmd(&["bash", "-c", SETUP_DOCKER]).as_root().run().await?;

	Ok(())
}

async fn setup_dir(dir: String) -> Result<PathBuf, CliError> {
	// check if HUUS_DIR env variable is already set
	match huus_dir() {
		Ok(dir) => {
			return Err(CliError::HuusDirAlreadySet(dir.display().to_string()));
		}
		Err(CliError::HuusDirNotPresent) => {}
		Err(e) => return Err(e),
	};

	if dir.contains("\"") {
		return Err(CliError::any("HUUS_DIR cannot contain double quotes", ""));
	}

	// lets first check if the dir exists or can be created
	// maybe we need to canonicalize first
	fs::create_dir_all(&dir)
		.await
		.with_message("Failed to create directory")?;
	let abs_dir = fs::canonicalize(dir)
		.await
		.with_message("Failed to canonicalize directory")?;

	{
		let mut file = OpenOptions::new()
			.append(true)
			.open("/etc/environment")
			.await
			.with_message("Failed to open /etc/environment")?;
		file.write_all(
			format!("HUUS_DIR=\"{}\"\n", abs_dir.display()).as_bytes(),
		)
		.await
		.with_message("Failed to write to /etc/environment")?;
	}

	Ok(abs_dir)
}
