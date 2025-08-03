use bollard::secret::NetworkCreateRequest;
use clap::Parser;
use tokio::fs;

use crate::{
	config::Config,
	docker::Docker,
	utils::{
		cli::{CliError, WithMessage as _},
		compose,
	},
};

use super::hostdinghy_dir;

const COMPOSE_YML: &str = r#"
services:
  traefik:
    image: "traefik:v3"
    ports:
      - "80:80"
      - "443:443"
      - "8080:8080"
    networks:
     - traefik
    extra_hosts:
      - "host.docker.internal:host-gateway"
    volumes:
      - "/var/run/docker.sock:/var/run/docker.sock:ro"
      - "./traefik.yml:/etc/traefik/traefik.yml:ro"
      - "./dynamic.yml:/etc/traefik/dynamic.yml:ro"
      - "./letsencrypt:/letsencrypt"
networks:
  traefik:
    external: true
"#;

const TRAEFIK_YML: &str = r#"
global:
  checkNewVersion: true
  sendAnonymousUsage: true

log:
  level: DEBUG

entryPoints:
  web:
    address: :80
    http:
      redirections:
        entrypoint:
          to: websecure
          scheme: https
          priority: 10

  websecure:
    address: :443

  weblocal:
    address: "127.0.0.1:8080"

api:
  dashboard: true

certificatesResolvers:
  letsencrypt:
    acme:
      email: {letsencrypt_email}
      storage: /letsencrypt/acme.json
      httpChallenge:
        # used during the challenge
        entryPoint: web

providers:
  # Enable Docker configuration backend
  docker:
    exposedByDefault: false
  file:
    filename: /etc/traefik/dynamic.yml
"#;

const TRAEFIK_DYNAMIC_YML: &str = r#"
http:
  routers:
    apilocal:
      rule: "PathPrefix(`/api`)"
      entryPoints:
        - weblocal
      service: api@internal
      middlewares:
        - authlocal

    dashboard:
      rule: "Host(`{dashboard_domain}`)"
      entryPoints:
        - websecure
      service: api@internal
      tls:
        certResolver: letsencrypt
      middlewares:
        - auth

  middlewares:
    auth:
      basicAuth:
        users:
          - "test:$2a$12$b5Od6Dmn1cWAw25kIvrcYuTY67RbF81Dpz5njSBZCtu.aHX/zSeUa"

    authlocal:
      basicAuth:
        users:
          - "hostdinghy:{api_token}"
"#;

#[derive(Debug, Parser)]
pub struct Traefik {}

pub async fn setup(_traefik: Traefik) -> Result<(), CliError> {
	let hostdinghy_dir = hostdinghy_dir()?;
	let cfg = Config::read(&hostdinghy_dir)
		.await
		.with_message("Failed to read config")?;

	let traefik_dir = hostdinghy_dir.join("traefik");

	fs::create_dir_all(&traefik_dir)
		.await
		.with_message("Failed to create $HOSTDINGHY_DIR/traefik")?;

	let compose_file = traefik_dir.join("compose.yml");
	fs::write(&compose_file, COMPOSE_YML)
		.await
		.with_message("Failed to write $HOSTDINGHY_DIR/traefik/compose.yml")?;

	let traefik_yml = traefik_dir.join("traefik.yml");
	fs::write(
		traefik_yml,
		TRAEFIK_YML
			.replace("{letsencrypt_email}", &cfg.traefik.letsencrypt_email),
	)
	.await
	.with_message("Failed to write $HOSTDINGHY_DIR/traefik/traefik.yml")?;

	let dynamic_yml = traefik_dir.join("dynamic.yml");
	fs::write(
		dynamic_yml,
		TRAEFIK_DYNAMIC_YML
			.replace("{dashboard_domain}", &cfg.traefik.dashboard_domain)
			.replace(
				"{api_token}",
				&bcrypt::hash(&cfg.traefik.api_token, 10).unwrap(),
			),
	)
	.await
	.with_message("Failed to write $HOSTDINGHY_DIR/traefik/dynamic.yml")?;

	{
		let docker = Docker::new()?;

		docker
			.create_network(NetworkCreateRequest {
				name: "traefik".into(),
				..Default::default()
			})
			.await?;
	}

	compose::up(compose_file).await?;

	Ok(())
}
