use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraefikRoute {
	pub name: String,
	pub rule: String,
	// todo type correctly
	pub status: String,
}
