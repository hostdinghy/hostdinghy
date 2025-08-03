use axum::Json;
use axum::extract::{Path, State};
use internal_api::app_id::AppId;
use internal_api::apps::{ComposeCommand, SaveComposeReq};
use internal_api::error::Error as ApiError;

use crate::apps::Apps;
use crate::error::Error;
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
	let app = apps
		.by_id(&id, &user.team_for_filter())
		.await?
		.ok_or(Error::NotFound)?;

	let servers = servers.with_conn(conn.conn());
	let server = servers
		.by_id(&app.server_id, &user.team_for_filter())
		.await?
		.ok_or(Error::Internal("Server was not found".into()))?;

	let api = api_client
		.connect(&server)
		.map_err(|e| Error::InternalApiServer(e.to_string()))?;

	let compose = match api.app_get_compose(&id).await {
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
	let app = apps
		.by_id(&id, &user.team_for_filter())
		.await?
		.ok_or(Error::NotFound)?;

	let servers = servers.with_conn(conn.conn());
	let server = servers
		.by_id(&app.server_id, &user.team_for_filter())
		.await?
		.ok_or(Error::Internal("Server was not found".into()))?;

	let api = api_client
		.connect(&server)
		.map_err(|e| Error::InternalApiServer(e.to_string()))?;

	api.app_set_compose(&id, &req).await?;

	let gcompose = api.app_get_compose(&id).await?;

	api.app_compose_command(&id, &ComposeCommand::Up).await?;

	Ok(Json(gcompose.compose))
}
