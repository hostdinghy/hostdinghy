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
	fn path(huus_dir: impl AsRef<Path>) -> PathBuf {
		huus_dir.as_ref().join("traefik/config.toml")
	}

	pub async fn read(huus_dir: impl AsRef<Path>) -> Result<Self, CliError> {
		read_toml(Self::path(huus_dir)).await.with_message(
			"Failed to read Traefik config from $HUUS_DIR/traefik/config.toml",
		)
	}
}
