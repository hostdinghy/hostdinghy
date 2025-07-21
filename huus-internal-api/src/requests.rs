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
#[serde(rename_all = "UPPERCASE")]
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
