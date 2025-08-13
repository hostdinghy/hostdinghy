use std::path::Path;

use base64::{Engine, prelude::BASE64_URL_SAFE_NO_PAD};
use tokio::fs;
use tracing::info;

use crate::{
	config::Config,
	server::{maybe_create_cert, read_cert},
	utils::{
		cli::{CliError, WithMessage as _},
		cmd::cmd,
		hostdinghy_dir,
	},
};

const SYSTEMD_CONFIG: &str = r#"
[Unit]
Description=Hostdinghy Server
After=network-online.target
Wants=network-online.target

[Service]
Type=simple
ExecStart=/usr/local/bin/hostdinghy serve
Restart=always

[Install]
WantedBy=multi-user.target
"#;

pub async fn setup() -> Result<(), CliError> {
	let hostdinghy_dir = hostdinghy_dir()?;
	let cfg = Config::read(&hostdinghy_dir)
		.await
		.with_message("Failed to read config")?;

	maybe_create_cert(&cfg, &hostdinghy_dir)
		.await
		.with_message("Failed to create self-signed certificate")?;

	let systemd_dir = Path::new("/etc/systemd/system");
	let service_file = systemd_dir.join("hostdinghy.service");

	fs::write(&service_file, SYSTEMD_CONFIG)
		.await
		.with_message(
			"Failed to write /etc/systemd/system/hostdinghy.service",
		)?;

	cmd(&["systemctl", "daemon-reload"])
		.run()
		.await
		.with_message("Failed to reload systemd daemon")?;

	cmd(&["systemctl", "enable", "hostdinghy"])
		.run()
		.await
		.with_message("Failed to enable hostdinghy service")?;

	cmd(&["systemctl", "start", "hostdinghy"])
		.run()
		.await
		.with_message("Failed to start hostdinghy service")?;

	let cert = read_cert(hostdinghy_dir)
		.await
		.with_message("Failed to read self-signed certificate")?;
	let cert = BASE64_URL_SAFE_NO_PAD.encode(cert);

	info!(
		"Server setup completed successfully with domain: {}",
		cfg.domain
	);

	eprintln!(
		"With the following information you can add the server \
					to the hostdinghy ui:\n\n{}\n{cert}",
		cfg.server.api_token,
	);

	Ok(())
}
