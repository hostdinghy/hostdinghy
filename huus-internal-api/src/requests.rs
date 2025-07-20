use chuchi_crypto::token::Token;
use chuchi_postgres::time::DateTime;
use semver::Version;
use serde::{Deserialize, Serialize};

pub type ApiToken = Token<42>;

/// A simple request to check if the server is running.
///
/// URL: `/ping`
/// Method: `GET`
/// Authentication: No
pub struct PingReq;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PingRes {
	pub time: DateTime,
}

/// A request to check the version of the server.
///
/// URL: `/version`
/// Method: `GET`
/// Authentication: Yes
pub struct VersionReq;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionRes {
	pub version: Version,
	// on prod this should never be None
	pub commit: Option<String>,
	// on prod this should never be None
	pub build_date: Option<DateTime>,
}

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
	pub traefik: AppInfoTraefik,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppService {
	pub name: String,
	pub state: ServiceState,
	pub started_at: DateTime,
	pub finished_at: Option<DateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ServiceState {
	Starting,
	Running,
	/// if the healthcheck failed
	Unhealthy,
	Stopped,
	Error,
	Unknown,
}

// this should correspond to one traefik service
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppInfoTraefik {
	routers: Vec<AppTraefikRouter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppTraefikRouter {
	pub rule: String,
	// a list of all domains aka Host(`example.com`) rules
	// this needs to be a parsed manually, also all operands
	// need to be considered
	pub domains: Vec<String>,
}
