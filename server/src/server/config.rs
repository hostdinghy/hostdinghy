use api::requests::ApiToken;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ServerConfig {
	pub api_token: ApiToken,
}

impl ServerConfig {
	pub fn new_from_user() -> Self {
		Self {
			api_token: ApiToken::new(),
		}
	}
}
