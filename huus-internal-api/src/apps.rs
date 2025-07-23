use serde::{Deserialize, Serialize};

/// A request to get information about the application.
///
/// URL: `/apps/:id`
/// Method: `GET`
/// Authentication: Yes
pub struct AppInfoReq;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppInfoRes {
	pub services: Vec<AppService>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppService {
	pub name: String,
	pub state: ServiceState,
	// human readable state
	pub state_hr: String,
	pub routes: Vec<ServiceRoute>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ServiceState {
	Empty,
	Created,
	/// if the healthcheck failed
	Unhealthy,
	Running,
	Paused,
	Restarting,
	Exited,
	Removing,
	Dead,
	Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceRoute {
	pub rule: String,
	// a list of all domains aka Host(`example.com`) rules
	// this needs to be a parsed manually, also all operands
	// need to be considered
	pub domains: Vec<String>,
}

/// A request to create a new application.
///
/// URL: `/apps/:id`
/// Method: `POST`
/// Authentication: Yes
pub struct CreateAppReq;
