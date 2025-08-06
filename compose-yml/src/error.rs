use serde::{Deserialize, Serialize};

#[derive(Debug, thiserror::Error, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "detail", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ComposeError {
	#[error("Compose file is not valid: {0}")]
	Parsing(String),
	#[error(
		"Traefik router name should be {expected}, but found {unexpected:?}"
	)]
	UnexpectedTraefikRouterName {
		unexpected: Vec<String>,
		expected: String,
	},
	#[error("Image {image} is not valid, expected {expected}")]
	InvalidImage { image: String, expected: String },
}

impl From<serde_yaml::Error> for ComposeError {
	fn from(e: serde_yaml::Error) -> Self {
		Self::Parsing(e.to_string())
	}
}
