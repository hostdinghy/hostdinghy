mod postgres;
mod registry;
mod server;
mod studio;
mod traefik;

use std::{env, path::PathBuf};

use clap::Parser;
use dialoguer::{Input, theme::ColorfulTheme};
use tokio::{
	fs::{self, OpenOptions},
	io::AsyncWriteExt,
};
use tracing::info;

use crate::{
	config::Config,
	server::maybe_create_cert,
	utils::{
		cli::{CliError, WithMessage as _},
		cmd::cmd,
		hostdinghy_dir, verify_root,
	},
};

#[derive(Debug, Parser)]
pub struct Setup {
	#[clap(subcommand)]
	cmd: SubCommand,
}

#[derive(Debug, Parser)]
enum SubCommand {
	Config,
	Docker,
	Traefik(traefik::Traefik),
	Registry(registry::Registry),
	Postgres(postgres::Postgres),
	Server,
	Studio(studio::Studio),
}

pub async fn setup(setup: Setup) {
	let res = inner_setup(setup).await;

	if let Err(e) = res {
		tracing::error!("Setup failed: {}", e);
	}
}

pub async fn inner_setup(setup: Setup) -> Result<(), CliError> {
	match setup.cmd {
		SubCommand::Config => {
			verify_root().await?;
			let hostdinghy_dir = setup_dir().await?;
			setup_config(hostdinghy_dir).await?;

			info!(
				"Config setup completed successfully at $HOSTDINGHY_DIR/config.toml"
			);
		}
		SubCommand::Docker => {
			verify_root().await?;
			setup_docker().await?;

			info!("Docker setup completed successfully.");
		}
		SubCommand::Traefik(traefik) => {
			verify_root().await?;
			traefik::setup(traefik).await?;

			info!("Traefik setup completed successfully.");
		}
		SubCommand::Registry(registry) => {
			verify_root().await?;
			registry::setup(registry).await?;

			info!("Registry setup completed successfully.");
		}
		SubCommand::Postgres(postgres) => {
			verify_root().await?;
			postgres::setup(postgres).await?;

			info!("PostgreSQL setup completed successfully.");
		}
		SubCommand::Server => {
			verify_root().await?;
			server::setup().await?;
		}
		SubCommand::Studio(su) => {
			verify_root().await?;
			studio::setup(su).await?;

			info!("Studio setup completed successfully.");
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

sudo systemctl enable docker.service
sudo systemctl enable containerd.service
"#;

async fn setup_docker() -> Result<(), CliError> {
	cmd(&["bash", "-c", SETUP_DOCKER]).as_root().run().await?;

	Ok(())
}

async fn setup_dir() -> Result<PathBuf, CliError> {
	// check if HOSTDINGHY_DIR env variable is already set
	match hostdinghy_dir() {
		Ok(dir) => return Ok(dir),
		Err(CliError::HostdinghyDirNotPresent) => {}
		Err(e) => return Err(e),
	}

	let dir: String = Input::with_theme(&ColorfulTheme::default())
		.with_prompt(
			"At what directory should hostdinghy and \
			all projects be stored?\nNote this cannot be changed \
			after the setup is complete.",
		)
		.validate_with(|dir: &String| {
			if dir.contains("\"") {
				Err("Directory cannot contain double quotes")
			} else {
				Ok(())
			}
		})
		.with_initial_text("/hostdinghy")
		.interact_text()
		.unwrap();

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
			format!("HOSTDINGHY_DIR=\"{}\"\n", abs_dir.display()).as_bytes(),
		)
		.await
		.with_message("Failed to write to /etc/environment")?;
	}

	// Safe this function will not be running while somebody else
	// tries to read env variables
	unsafe {
		env::set_var("HOSTDINGHY_DIR", &abs_dir);
	}

	Ok(abs_dir)
}

async fn setup_config(hostdinghy_dir: PathBuf) -> Result<(), CliError> {
	let cfg = match Config::try_read(&hostdinghy_dir)
		.await
		.with_message("failed to read config")?
	{
		Some(c) => {
			info!("Config already exists at $HOSTDINGHY_DIR/config.toml");
			c
		}
		None => {
			let cfg = Config::new_from_user();
			cfg.write(&hostdinghy_dir)
				.await
				.with_message("Failed to write config")?;

			cfg
		}
	};

	maybe_create_cert(&cfg, &hostdinghy_dir)
		.await
		.with_message("Failed to create self-signed certificate")?;

	Ok(())
}
