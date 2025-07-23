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
