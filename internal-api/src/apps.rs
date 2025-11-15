use serde::{Deserialize, Serialize};
use serde_plain::derive_display_from_serialize;

/// A request to get information about the application.
///
/// URL: `/apps/:id`
/// Method: `GET`
/// Authentication: Yes
pub struct AppInfoReq;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppInfoRes {
	// todo add postgresql
	pub services: Vec<AppService>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppService {
	pub name: String,
	pub container_name: String,
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

/// Get the compose.yml content.
///
/// URL: `/apps/:id/compose`
/// Method: `GET`
/// Authentication: Yes
///
/// Returns 404 if no compose file was found
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetComposeReq;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetComposeRes {
	pub compose: String,
	/// Returns true if a database was created once
	///
	/// Todo this is deprecated
	pub database: bool,
}

/// Save compose.yml
///
/// URL: `/apps/:id/compose`
/// Method: `POST`
/// Authentication: Yes
///
/// If `database` is true the {DB_PASSWORD} will be replaced
/// with the database password
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveComposeReq {
	pub compose: String,
	/// Setting this to false will not delete the database
	/// it is only possible to delete the database
	/// when deleteing the app
	///
	/// Todo this is deprecated
	pub create_database: bool,
}

/// A request to execute a composer command.
///
/// URL: `/apps/:id/action/:command`
/// Method: `POST`
/// Authentication: Yes
///
/// Or
/// URL: `/apps/:id/service/:service/action/:command`
/// Method: `POST`
/// Authentication: Yes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ComposeCommand {
	/// only call this if you just called stop before
	/// this will not rebuild the containers if for example
	/// the compose file changed
	Start,
	Up,
	Restart,
	Stop,
}

derive_display_from_serialize!(ComposeCommand);

/// A request to get the logs of a compose.
///
/// URL: `/apps/:id/logs?lines=<lines>`
/// Method: `GET`
/// Return Body: `text/plain`
/// Authentication: Yes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppLogsReq;

// /// A request to delete an application.
// ///
// /// This will remove the application and all of its data.
// ///
// /// URL: `/apps/:id`
// /// Method: `DELETE`
// /// Authentication: Yes
// #[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct DeleteAppReq;
