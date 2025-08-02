pub mod api;
pub mod client;
pub mod utils;

use std::path::{Path, PathBuf};

use chuchi_crypto::token::Token;
use serde::{Deserialize, Serialize};

use crate::utils::{
	cli::{CliError, WithMessage as _},
	read_toml,
};

pub type ApiToken = Token<32>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct TraefikConfig {
	pub letsencrypt_email: String,
	pub dashboard_domain: String,
	pub api_token: ApiToken,
}

impl TraefikConfig {
	fn path(hostdinghy_dir: impl AsRef<Path>) -> PathBuf {
		hostdinghy_dir.as_ref().join("traefik/config.toml")
	}

	pub async fn read(
		hostdinghy_dir: impl AsRef<Path>,
	) -> Result<Self, CliError> {
		read_toml(Self::path(hostdinghy_dir)).await.with_message(
			"Failed to read Traefik config from $HOSTDINGHY_DIR/traefik/config.toml",
		)
	}
}
