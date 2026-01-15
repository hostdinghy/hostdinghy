use axum::extract::{Path, State};
use axum::routing::{delete, get};
use axum::{Json, Router};
use internal_api::postgres::{CreateDatabaseReq, CreateDatabaseRes};
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
pub async fn all_databases(
	user: AuthedUser<RightsAny>,
	State(servers): State<Servers>,
	State(api_client): State<ApiClient>,
	conn: ConnOwned,
	Path(id): Path<UniqueId>,
) -> Result<Json<Vec<String>>> {
	let servers = servers.with_conn(conn.conn());

	let LoadServer { api, .. } =
		load_server(&id, &user, &servers, &api_client).await?;

	api.postgres()
		.databases()
		.await
		.map(Json)
		.map_err(Into::into)
}

pub async fn create_database(
	user: AuthedUser<RightsAny>,
	State(servers): State<Servers>,
	State(api_client): State<ApiClient>,
	conn: ConnOwned,
	Path(id): Path<UniqueId>,
	Json(req): Json<CreateDatabaseReq>,
) -> Result<Json<CreateDatabaseRes>> {
	let servers = servers.with_conn(conn.conn());

	let LoadServer { api, .. } =
		load_server(&id, &user, &servers, &api_client).await?;

	api.postgres()
		.create_database(&req.name)
		.await
		.map(Json)
		.map_err(Into::into)
}

// pub async fn delete_user(
// 	user: AuthedUser<RightsAny>,
// 	State(servers): State<Servers>,
// 	State(api_client): State<ApiClient>,
// 	conn: ConnOwned,
// 	Path((id, username)): Path<(UniqueId, RegistryUsername)>,
// ) -> Result<Json<()>> {
// 	let servers = servers.with_conn(conn.conn());

// 	let LoadServer { api, .. } =
// 		load_server(&id, &user, &servers, &api_client).await?;

// 	api.registry()
// 		.delete_user(&username)
// 		.await
// 		.map(Json)
// 		.map_err(Into::into)
// }

pub fn routes() -> Router<AppState> {
	Router::new().route(
		"/{id}/postgres/databases",
		get(all_databases).post(create_database),
	)
	// .route("/{id}/registry/users/{username}", delete(delete_user))
}
