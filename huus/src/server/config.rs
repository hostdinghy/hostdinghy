use std::{io::ErrorKind, path::Path};

use api::{
	error::{Error, WithMessage as _},
	requests::ApiToken,
};
use chuchi_crypto::token::Token;
use serde::{Deserialize, Serialize};
use tokio::fs;

pub type SecretToken = Token<32>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
	pub domain: String,
	/// Only use this token in combination with something else
	/// plus a hash
	#[serde(default = "SecretToken::new")]
	pub secret: SecretToken,
	pub api_token: Option<ApiToken>,
}

impl Default for Config {
	fn default() -> Self {
		Self {
			domain: String::new(),
			secret: SecretToken::new(),
			api_token: None,
		}
	}
}

impl Config {
	pub async fn read(huus_dir: impl AsRef<Path>) -> Result<Self, Error> {
		let path = huus_dir.as_ref().join("config.toml");
		match fs::read_to_string(path).await {
			Ok(s) => toml::from_str(&s)
				.with_message("could not parse $HUUS_DIR/config.toml"),
			// if the file does not exist, return a default config
			Err(e) if e.kind() == ErrorKind::NotFound => Ok(Default::default()),
			Err(e) => {
				Err(Error::any("could not read $HUUS_DIR/config.toml", e))
			}
		}
	}

	pub async fn write(&self, huus_dir: impl AsRef<Path>) -> Result<(), Error> {
		let path = huus_dir.as_ref().join("config.toml");
		let s = toml::to_string(self)
			.with_message("could not serialize config to TOML")?;
		fs::write(path, s)
			.await
			.with_message("could not write $HUUS_DIR/config.toml")
	}
}
