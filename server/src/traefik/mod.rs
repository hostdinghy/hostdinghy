pub mod api;
pub mod client;
pub mod utils;

use chuchi_crypto::token::Token;
use dialoguer::{Input, theme::ColorfulTheme};
use serde::{Deserialize, Serialize};

pub type ApiToken = Token<32>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct TraefikConfig {
	pub letsencrypt_email: String,
	pub dashboard_domain: String,
	pub api_token: ApiToken,
}

impl TraefikConfig {
	pub fn new_from_user() -> Self {
		let email: String = Input::with_theme(&ColorfulTheme::default())
			.with_prompt("Enter the email to use for Let's Encrypt")
			.interact_text()
			.unwrap();

		let domain: String = Input::with_theme(&ColorfulTheme::default())
			.with_prompt("Enter the domain for the Traefik dashboard")
			.interact_text()
			.unwrap();

		Self {
			letsencrypt_email: email,
			dashboard_domain: domain,
			api_token: ApiToken::new(),
		}
	}
}
