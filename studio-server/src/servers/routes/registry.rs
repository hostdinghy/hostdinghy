use axum::extract::{Path, State};
use axum::routing::{delete, get};
use axum::{Json, Router};
use internal_api::registry::{CreateUserReq, CreateUserRes};
use pg::UniqueId;

use crate::AppState;
use crate::error::Result;
use crate::internal::ApiClient;
use crate::servers::Servers;
use crate::servers::routes::utils::{LoadServer, load_server};
use crate::users::utils::AuthedUser;
use crate::users::utils::RightsAny;
use crate::utils::ConnOwned;

/// Returns all users of that server not only for this app
pub async fn all_users(
	user: AuthedUser<RightsAny>,
	State(servers): State<Servers>,
	State(api_client): State<ApiClient>,
	conn: ConnOwned,
	Path(id): Path<UniqueId>,
) -> Result<Json<Vec<String>>> {
	let servers = servers.with_conn(conn.conn());

	let LoadServer { api, .. } =
		load_server(&id, &user, &servers, &api_client).await?;

	api.registry_users().await.map(Json).map_err(Into::into)
}

pub async fn create_user(
	user: AuthedUser<RightsAny>,
	State(servers): State<Servers>,
	State(api_client): State<ApiClient>,
	conn: ConnOwned,
	Path(id): Path<UniqueId>,
	Json(req): Json<CreateUserReq>,
) -> Result<Json<CreateUserRes>> {
	let servers = servers.with_conn(conn.conn());

	let LoadServer { api, .. } =
		load_server(&id, &user, &servers, &api_client).await?;

	api.registry_create_user(&req.username)
		.await
		.map(Json)
		.map_err(Into::into)
}

pub async fn delete_user(
	user: AuthedUser<RightsAny>,
	State(servers): State<Servers>,
	State(api_client): State<ApiClient>,
	conn: ConnOwned,
	Path((id, username)): Path<(UniqueId, String)>,
) -> Result<Json<()>> {
	let servers = servers.with_conn(conn.conn());

	let LoadServer { api, .. } =
		load_server(&id, &user, &servers, &api_client).await?;

	api.registry_delete_user(&username)
		.await
		.map(Json)
		.map_err(Into::into)
}

pub fn routes() -> Router<AppState> {
	Router::new()
		.route("/{id}/registry/users", get(all_users).post(create_user))
		.route("/{id}/registry/users/{username}", delete(delete_user))
}
