use axum::body::Body;
use axum::extract::{Path, State};
use axum::http::header::CONTENT_TYPE;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use bytes::Bytes;
use futures::stream::BoxStream;
use internal_api::apps::{
	AppId, ComposeCommand as ApiComposeCommand, SaveComposeReq,
};
use internal_api::client::Result as ApiResult;
use internal_api::error::Error as ApiError;
use serde::{Deserialize, Serialize};

use crate::AppState;
use crate::apps::Apps;
use crate::apps::routes::utils::{AppWithServer, app_with_server};
use crate::error::Result;
use crate::internal::ApiClient;
use crate::servers::Servers;
use crate::users::utils::AuthedUser;
use crate::users::utils::RightsAny;
use crate::utils::ConnOwned;

pub async fn get_compose(
	user: AuthedUser<RightsAny>,
	State(apps): State<Apps>,
	State(servers): State<Servers>,
	State(api_client): State<ApiClient>,
	Path(id): Path<AppId>,
	conn: ConnOwned,
) -> Result<Json<String>> {
	let apps = apps.with_conn(conn.conn());
	let servers = servers.with_conn(conn.conn());

	let AppWithServer { api, .. } =
		app_with_server(&id, &user, &apps, &servers, &api_client).await?;

	let compose = match api.apps().get_compose(&id).await {
		Ok(a) => a.compose,
		Err(ApiError::AppNotFound) => "".into(),
		Err(e) => return Err(e.into()),
	};

	Ok(Json(compose))
}

pub async fn set_compose(
	user: AuthedUser<RightsAny>,
	State(apps): State<Apps>,
	State(servers): State<Servers>,
	State(api_client): State<ApiClient>,
	Path(id): Path<AppId>,
	conn: ConnOwned,
	Json(req): Json<SaveComposeReq>,
) -> Result<Json<String>> {
	let apps = apps.with_conn(conn.conn());
	let servers = servers.with_conn(conn.conn());

	let AppWithServer { api, .. } =
		app_with_server(&id, &user, &apps, &servers, &api_client).await?;

	api.apps().set_compose(&id, &req).await?;

	let gcompose = api.apps().get_compose(&id).await?;

	// the webui runs up afterwards to show its output
	Ok(Json(gcompose.compose))
}

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

impl From<ComposeCommand> for ApiComposeCommand {
	fn from(cmd: ComposeCommand) -> Self {
		match cmd {
			ComposeCommand::Start => Self::Start,
			ComposeCommand::Up => Self::Up,
			ComposeCommand::Restart => Self::Restart,
			ComposeCommand::Stop => Self::Stop,
		}
	}
}

/// Streams the events of the command as json lines
/// (see [`internal_api::apps::CommandEvent`])
pub async fn compose_command(
	user: AuthedUser<RightsAny>,
	State(apps): State<Apps>,
	State(servers): State<Servers>,
	State(api_client): State<ApiClient>,
	Path((id, cmd)): Path<(AppId, ComposeCommand)>,
	conn: ConnOwned,
) -> Result<Response> {
	let apps = apps.with_conn(conn.conn());
	let servers = servers.with_conn(conn.conn());

	let AppWithServer { api, .. } =
		app_with_server(&id, &user, &apps, &servers, &api_client).await?;

	let stream = api
		.apps()
		.compose_command_stream(&id, None, &cmd.into())
		.await?;

	Ok(ndjson(stream))
}

pub async fn compose_service_command(
	user: AuthedUser<RightsAny>,
	State(apps): State<Apps>,
	State(servers): State<Servers>,
	State(api_client): State<ApiClient>,
	Path((id, service, cmd)): Path<(AppId, String, ComposeCommand)>,
	conn: ConnOwned,
) -> Result<Response> {
	let apps = apps.with_conn(conn.conn());
	let servers = servers.with_conn(conn.conn());

	let AppWithServer { api, .. } =
		app_with_server(&id, &user, &apps, &servers, &api_client).await?;

	let stream = api
		.apps()
		.compose_command_stream(&id, Some(&service), &cmd.into())
		.await?;

	Ok(ndjson(stream))
}

fn ndjson(stream: BoxStream<'static, ApiResult<Bytes>>) -> Response {
	(
		[(CONTENT_TYPE, "application/x-ndjson")],
		Body::from_stream(stream),
	)
		.into_response()
}

pub fn routes() -> Router<AppState> {
	Router::new()
		.route("/{id}/compose", get(get_compose).post(set_compose))
		.route("/{id}/compose/{cmd}", post(compose_command))
		.route(
			"/{id}/compose/service/{service}/{cmd}",
			post(compose_service_command),
		)
}
