use chuchi_crypto::token::Token;
use clap::Parser;
use serde::{Deserialize, Serialize};
use tokio::fs;

use crate::utils::{
	cli::{CliError, WithMessage as _},
	compose, write_toml,
};

use super::hostdinghy_dir;

pub type WebhookToken = Token<32>;

const COMPOSE_YML: &str = r#"
services:
  registry:
    image: "registry:2"
    networks:
     - traefik
    environment:
      - SERVICE_FQDN_REGISTRY="https://{registry_domain}"
    volumes:
      - "./registry.password:/auth/registry.password:ro"
      - "./config.yml:/etc/docker/registry/config.yml:ro"
      - "./data:/var/lib/registry"
    labels:
      - "traefik.enable=true"
      - "traefik.http.routers.registry.rule=Host(`{registry_domain}`)"
      - "traefik.http.routers.registry.entrypoints=websecure"
      - "traefik.http.routers.registry.tls.certresolver=letsencrypt"
      - "traefik.http.services.registry.loadbalancer.server.port=5000"
networks:
  traefik:
    external: true
"#;

const CONFIG_YML: &str = r#"
version: 0.1
log:
  fields:
    service: registry
storage:
  cache:
    blobdescriptor: inmemory
  filesystem:
    rootdirectory: /var/lib/registry
auth:
  htpasswd:
    realm: Registry
    path: /auth/registry.password
http:
  addr: :5000
  headers:
    X-Content-Type-Options: [nosniff]
health:
  storagedriver:
    enabled: true
    interval: 10s
    threshold: 3
notifications:
      endpoints:
        - name: webhook
          url: https://{studio_domain}/api/servers/registry/webhook
          headers:
            Authorization: [Bearer {webhook_token}]
          events:
            - name: push
              action: push
"#;

#[derive(Debug, Parser)]
pub struct Registry {
	domain: String,
	// should not contain https or anything else
	studio_domain: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegistryConfig {
	pub domain: String,
	pub studio_domain: String,
	pub webhook_token: WebhookToken,
}

pub async fn setup(registry: Registry) -> Result<(), CliError> {
	let hostdinghy_dir = hostdinghy_dir()?;
	let registry_dir = hostdinghy_dir.join("registry");

	fs::create_dir_all(&registry_dir)
		.await
		.with_message("Failed to create $HOSTDINGHY_DIR/registry")?;

	let cfg = RegistryConfig {
		domain: registry.domain,
		studio_domain: registry.studio_domain,
		webhook_token: WebhookToken::new(),
	};

	write_toml(&cfg, registry_dir.join("config.toml"))
		.await
		.with_message("Failed to write $HOSTDINGHY_DIR/registry/config.toml")?;

	let compose_file = registry_dir.join("compose.yml");
	fs::write(
		&compose_file,
		COMPOSE_YML
			.replace("{registry_domain}", &cfg.domain)
			.replace("{studio_domain}", &cfg.studio_domain)
			.replace("{webhook_token}", &cfg.webhook_token.to_string()),
	)
	.await
	.with_message("Failed to write $HOSTDINGHY_DIR/registry/compose.yml")?;

	let config_yml = registry_dir.join("config.yml");
	fs::write(config_yml, CONFIG_YML)
		.await
		.with_message("Failed to write $HOSTDINGHY_DIR/registry/config.yml")?;

	let password_file = registry_dir.join("registry.password");
	fs::write(password_file, "").await.with_message(
		"Failed to write $HOSTDINGHY_DIR/registry/registry.password",
	)?;

	compose::up(compose_file).await?;

	Ok(())
}
