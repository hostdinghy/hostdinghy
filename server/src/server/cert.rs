use std::path::{Path, PathBuf};

use api::error::{Error, WithMessage as _};
use rcgen::generate_simple_self_signed;
use tokio::fs;

use crate::config::Config;

pub fn key_path(hostdinghy_dir: impl AsRef<Path>) -> PathBuf {
	hostdinghy_dir.as_ref().join("key.pem")
}

pub fn cert_path(hostdinghy_dir: impl AsRef<Path>) -> PathBuf {
	hostdinghy_dir.as_ref().join("cert.pem")
}

pub async fn maybe_create_cert(
	cfg: &Config,
	hostdinghy_dir: impl AsRef<Path>,
) -> Result<(), Error> {
	let cert_path = cert_path(&hostdinghy_dir);
	if fs::metadata(cert_path).await.map_or(false, |m| m.is_file()) {
		return Ok(());
	}

	create_cert(&cfg.domain, hostdinghy_dir).await?;

	Ok(())
}

async fn create_cert(
	domain: &str,
	hostdinghy_dir: impl AsRef<Path>,
) -> Result<(), Error> {
	let cert = generate_simple_self_signed(vec![domain.to_string()])
		.with_message("failed to generate self signed cert")?;

	let key_path = key_path(&hostdinghy_dir);
	let cert_path = cert_path(&hostdinghy_dir);

	fs::write(&key_path, cert.signing_key.serialize_pem())
		.await
		.with_message("failed to write $HOSTDINGHY_DIR/key.pem")?;
	fs::write(&cert_path, cert.cert.pem())
		.await
		.with_message("failed to write $HOSTDINGHY_DIR/cert.pem")?;

	Ok(())
}

pub async fn read_cert(
	hostdinghy_dir: impl AsRef<Path>,
) -> Result<String, Error> {
	let cert_path = cert_path(&hostdinghy_dir);
	let cert = fs::read_to_string(cert_path)
		.await
		.with_message("failed to read $HOSTDINGHY_DIR/cert.pem")?;
	Ok(cert)
}
