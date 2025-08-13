use std::time::Duration;

use base64::{Engine, prelude::BASE64_URL_SAFE_NO_PAD};
use chuchi_crypto::token::Token;
use clap::Parser;
use serde::{Deserialize, Serialize};
use tokio::{fs, time::sleep};

use crate::{
	config::Config,
	postgresql::Client,
	server::read_cert,
	utils::{
		cli::{CliError, WithMessage},
		compose, hostdinghy_dir, is_file, write_toml,
	},
};

const COMPOSE_YML: &str = r#"
services:
  studio:
    image: registry.s2.goodserver.ch/hostdinghy/studio:latest
    networks:
      - traefik
    restart: unless-stopped
    extra_hosts:
      - "host.docker.internal:host-gateway"
    volumes:
      - "./config.toml:/data/config.toml"
    labels:
      - "traefik.enable=true"
      - "traefik.http.routers.hostdinghy-studio.rule=Host(`{domain}`)"
      - "traefik.http.routers.hostdinghy-studio.entrypoints=websecure"
      - "traefik.http.routers.hostdinghy-studio.tls.certresolver=letsencrypt"
      - "traefik.http.services.hostdinghy-studio.loadbalancer.server.port=3030"
networks:
  traefik:
    external: true
"#;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct StudioConfig {
	database: DatabaseConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct DatabaseConfig {
	host: String,
	name: String,
	user: String,
	password: String,
}

#[derive(Debug, Parser)]
pub struct Studio {
	/// First user
	username: String,
	password: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
	team_id: String,
}

pub async fn setup(args: Studio) -> Result<(), CliError> {
	let hostdinghy_dir = hostdinghy_dir()?;
	let cfg = Config::read(&hostdinghy_dir)
		.await
		.with_message("Failed to read config")?;

	let studio_dir = hostdinghy_dir.join("hostdinghy");
	fs::create_dir_all(&studio_dir)
		.await
		.with_message("Failed to create $HOSTDINGHY_DIR/hostdinghy")?;

	let compose_file = studio_dir.join("compose.yml");
	if is_file(&compose_file).await {
		return Err(CliError::any("Hostdinghy studio is already set up", ""));
	}

	fs::write(&compose_file, COMPOSE_YML.replace("{domain}", &cfg.domain))
		.await
		.with_message(
			"Failed to write $HOSTDINGHY_DIR/hostdinghy/compose.yml",
		)?;

	let pg_client = Client::new().await?;
	let pw = Token::<32>::new().to_string();
	pg_client.create_user("hostdinghy", &pw).await?;
	pg_client
		.create_database("hostdinghy", "hostdinghy")
		.await?;

	// let's add the config.toml file
	let config = StudioConfig {
		database: DatabaseConfig {
			host: "host.docker.internal".into(),
			name: "hostdinghy".into(),
			user: "hostdinghy".into(),
			password: pw.clone(),
		},
	};

	let config_file = studio_dir.join("config.toml");
	write_toml(&config, &config_file).await.with_message(
		"Failed to write $HOSTDINGHY_DIR/hostdinghy/config.toml",
	)?;

	compose::up(&compose_file).await?;

	// let's wait until the container is started
	sleep(Duration::from_secs(5)).await;

	let str = compose::exec(
		&compose_file,
		"studio",
		&[
			"./studio-server",
			"create-user",
			&args.username,
			&args.password,
			"--json",
		],
	)
	.await?;
	let user: User = serde_json::from_str(str.trim())
		.with_message("Failed to parse user from studio-server")?;

	let cert = read_cert(hostdinghy_dir)
		.await
		.with_message("Failed to read self-signed certificate")?;
	let cert = BASE64_URL_SAFE_NO_PAD.encode(cert);

	compose::exec(
		&compose_file,
		"studio",
		&[
			"./studio-server",
			"create-server",
			"main",
			&user.team_id,
			&cfg.domain,
			&cfg.server.api_token.to_string(),
			&cert,
		],
	)
	.await
	.with_message("Failed to create server in studio")?;

	Ok(())
}
