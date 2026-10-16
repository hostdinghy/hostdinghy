use serde::{Deserialize, Serialize};
use serde_plain::derive_display_from_serialize;

pub use crate::app_id::AppId;

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

/// The response of a compose command is a stream of events, one json object
/// per line (`application/x-ndjson`). The command continues if the
/// connection is closed. The last event is either `done` or `error`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum CommandEvent {
	/// A line printed by the command, stdout and stderr mixed
	Output { line: String },
	/// The command exited successfully
	Done,
	/// The command failed
	Error { message: String },
}

/// A request to get the logs of a compose.
///
/// URL: `/apps/:id/logs?lines=<lines>&follow=<bool>`
/// Method: `GET`
/// Return Body: `application/x-ndjson`, one [`LogLine`] per line
/// Authentication: Yes
///
/// Returns the last `lines` lines of each container (all if not set) and
/// with follow the new ones as they come, until the connection closes.
/// The lines of different containers are not in order, sort them by time.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppLogsReq {
	pub lines: Option<u32>,
	#[serde(default)]
	pub follow: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogLine {
	/// e.g. `web-1`
	pub container: String,
	pub service: String,
	/// RFC 3339 in UTC with nanoseconds, so the strings sort by time
	pub time: String,
	/// may contain ansi escape codes
	pub line: String,
}

impl LogLine {
	/// Parses a line of `docker compose logs --no-color --timestamps`:
	/// `web-1  | 2025-01-01T00:00:00.123Z the line`
	pub fn parse_compose(raw: &str) -> Option<Self> {
		let (container, rest) = raw.split_once(" | ")?;
		let container = container.trim().to_string();
		let (time, line) = rest.split_once(' ').unwrap_or((rest, ""));
		let time = pad_nanos(time)?;

		let service = match container.rsplit_once('-') {
			Some((service, idx)) if idx.parse::<u32>().is_ok() => service,
			_ => &container,
		}
		.to_string();

		Some(Self {
			container,
			service,
			time,
			line: line.to_string(),
		})
	}
}

/// docker drops trailing zeros of the fraction, pad it to 9 digits
fn pad_nanos(time: &str) -> Option<String> {
	let time = time.strip_suffix('Z')?;
	let (secs, frac) = time.split_once('.').unwrap_or((time, ""));
	if secs.len() != 19 || frac.len() > 9 {
		return None;
	}

	Some(format!("{secs}.{frac:0<9}Z"))
}

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
