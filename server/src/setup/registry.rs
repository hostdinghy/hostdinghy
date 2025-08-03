use clap::Parser;
use tokio::fs;

use crate::{
	config::Config,
	utils::{
		cli::{CliError, WithMessage as _},
		compose,
	},
};

use super::hostdinghy_dir;

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
      - "../cert.pem:/usr/local/share/ca-certificates/server.pem:ro"
    command: >
      sh -c "update-ca-certificates && /entrypoint.sh /etc/docker/registry/config.yml"
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
          url: https://{server_domain}:4242/registry/webhook
          headers:
            Authorization: [Bearer {webhook_token}]
          events:
            - name: push
              action: push
"#;

#[derive(Debug, Parser)]
pub struct Registry {}

pub async fn setup(_registry: Registry) -> Result<(), CliError> {
	let hostdinghy_dir = hostdinghy_dir()?;
	let cfg = Config::read(&hostdinghy_dir)
		.await
		.with_message("Failed to read config")?;

	let registry_dir = hostdinghy_dir.join("registry");
	fs::create_dir_all(&registry_dir)
		.await
		.with_message("Failed to create $HOSTDINGHY_DIR/registry")?;

	let compose_file = registry_dir.join("compose.yml");
	fs::write(
		&compose_file,
		COMPOSE_YML.replace("{registry_domain}", &cfg.registry.domain),
	)
	.await
	.with_message("Failed to write $HOSTDINGHY_DIR/registry/compose.yml")?;

	let config_yml = registry_dir.join("config.yml");
	fs::write(
		config_yml,
		CONFIG_YML.replace("{server_domain}", &cfg.domain).replace(
			"{webhook_token}",
			&cfg.registry.webhook_token.to_string(),
		),
	)
	.await
	.with_message("Failed to write $HOSTDINGHY_DIR/registry/config.yml")?;

	let password_file = registry_dir.join("registry.password");
	fs::write(password_file, "").await.with_message(
		"Failed to write $HOSTDINGHY_DIR/registry/registry.password",
	)?;

	compose::up(compose_file).await?;

	Ok(())
}
