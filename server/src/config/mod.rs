use std::{io::ErrorKind, path::Path};

use api::error::{Error, WithMessage as _};
use chuchi_crypto::token::Token;
use dialoguer::{Input, theme::ColorfulTheme};
use serde::{Deserialize, Serialize};
use tokio::fs;

use crate::{
	registry::RegistryConfig, server::config::ServerConfig,
	traefik::TraefikConfig,
};

pub type SecretToken = Token<32>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
	pub domain: String,
	/// Only use this token in combination with something else
	/// and then hash it
	#[serde(default = "SecretToken::new")]
	pub secret: SecretToken,
	pub server: ServerConfig,
	pub traefik: TraefikConfig,
	pub registry: RegistryConfig,
}

impl Config {
	pub async fn try_read(
		hostdinghy_dir: impl AsRef<Path>,
	) -> Result<Option<Self>, Error> {
		let path = hostdinghy_dir.as_ref().join("config.toml");
		match fs::read_to_string(path).await {
			Ok(s) => toml::from_str(&s)
				.with_message("could not parse $HOSTDINGHY_DIR/config.toml"),
			// if the file does not exist, return a default config
			Err(e) if e.kind() == ErrorKind::NotFound => Ok(None),
			Err(e) => {
				Err(Error::any("could not read $HOSTDINGHY_DIR/config.toml", e))
			}
		}
	}

	pub async fn read(hostdinghy_dir: impl AsRef<Path>) -> Result<Self, Error> {
		Self::try_read(hostdinghy_dir).await?.ok_or_else(|| {
			Error::any(
				"could not find $HOSTDINGHY_DIR/config.toml \
				run `hostdinghy setup config` first",
				"",
			)
		})
	}

	pub async fn write(
		&self,
		hostdinghy_dir: impl AsRef<Path>,
	) -> Result<(), Error> {
		let path = hostdinghy_dir.as_ref().join("config.toml");
		let s = toml::to_string(self)
			.with_message("could not serialize config to TOML")?;
		fs::write(path, s)
			.await
			.with_message("could not write $HOSTDINGHY_DIR/config.toml")
	}

	pub fn new_from_user() -> Self {
		let domain: String = Input::with_theme(&ColorfulTheme::default())
			.with_prompt(
				"Enter the domain for this server\n\
				This is the domain which resolves to this server. No website \
				needs to be hosted on this domain.\n\
				The self signed certificate for the internal API will use this domain.",
			)
			.validate_with(|domain: &String| {
				if domain.starts_with("http") {
					Err("The domain should not start with http or https")
				} else {
					Ok(())
				}
			})
			.interact_text()
			.unwrap();

		Self {
			domain,
			secret: SecretToken::new(),
			server: ServerConfig::new_from_user(),
			traefik: TraefikConfig::new_from_user(),
			registry: RegistryConfig::new_from_user(),
		}
	}
}
