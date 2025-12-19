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

pub async fn get_fs_res(
	user: AuthedUser<RightsAny>,
	State(apps): State<Apps>,
	State(servers): State<Servers>,
	State(api_client): State<ApiClient>,
	Path((id, path)): Path<(AppId, String)>,
	conn: ConnOwned,
) -> Result<Json<String>> {
	let segments: Vec<_> = path.split('/').collect();
	Ok(Json(segments.join(",")))
}

pub async fn replace_fs_res(
	user: AuthedUser<RightsAny>,
	State(apps): State<Apps>,
	State(servers): State<Servers>,
	State(api_client): State<ApiClient>,
	Path((id, path)): Path<(AppId, String)>,
	conn: ConnOwned,
) -> Result<Json<String>> {
	let segments: Vec<_> = path.split('/').collect();
	Ok(Json(segments.join(",")))
}

pub fn routes() -> Router<AppState> {
	Router::new().route("/{id}/fs/{*path}", get(get_fs_res).put(replace_fs_res))
}
