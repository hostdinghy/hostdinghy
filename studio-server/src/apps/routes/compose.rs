use axum::extract::{Path, State};
use axum::routing::get;
use axum::{Json, Router};
use internal_api::app_id::AppId;
use internal_api::apps::{ComposeCommand, SaveComposeReq};
use internal_api::error::Error as ApiError;

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
	let servers = servers.with_conn(conn.conn());

	let AppWithServer { api, .. } =
		app_with_server(&id, &user, &apps, &servers, &api_client).await?;

	api.app_set_compose(&id, &req).await?;

	let gcompose = api.app_get_compose(&id).await?;

	api.app_compose_command(&id, &ComposeCommand::Up).await?;

	Ok(Json(gcompose.compose))
}

pub fn routes() -> Router<AppState> {
	Router::new().route("/{id}/compose", get(get_compose).post(set_compose))
}
