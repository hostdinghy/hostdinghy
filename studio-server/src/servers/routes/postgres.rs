use axum::body::Body;
use axum::extract::{Path, State};
use axum::routing::{get, post, put};
use axum::{Json, Router};
use futures::StreamExt;
use internal_api::error::WithMessage;
use internal_api::postgres::{
	CreateDatabaseReq, CreateDatabaseRes, DatabaseName, NewPasswordRes,
};
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

pub async fn new_password(
	user: AuthedUser<RightsAny>,
	State(servers): State<Servers>,
	State(api_client): State<ApiClient>,
	conn: ConnOwned,
	Path((id, name)): Path<(UniqueId, DatabaseName)>,
) -> Result<Json<NewPasswordRes>> {
	let servers = servers.with_conn(conn.conn());

	let LoadServer { api, .. } =
		load_server(&id, &user, &servers, &api_client).await?;

	api.postgres()
		.new_password(&name)
		.await
		.map(Json)
		.map_err(Into::into)
}

pub async fn restore_database(
	user: AuthedUser<RightsAny>,
	State(servers): State<Servers>,
	State(api_client): State<ApiClient>,
	conn: ConnOwned,
	Path((id, name)): Path<(UniqueId, DatabaseName)>,
	body: Body,
) -> Result<()> {
	let servers = servers.with_conn(conn.conn());

	let LoadServer { api, .. } =
		load_server(&id, &user, &servers, &api_client).await?;

	// todo this operation needs a security check again

	// todo does the body need to be limited in size?

	api.postgres()
		.restore_database(
			&name,
			body.into_data_stream()
				.map(|r| r.with_message("failed to read restore database"))
				.boxed(),
		)
		.await
		.map_err(Into::into)
}

pub async fn dump_database(
	user: AuthedUser<RightsAny>,
	State(servers): State<Servers>,
	State(api_client): State<ApiClient>,
	conn: ConnOwned,
	Path((id, name)): Path<(UniqueId, DatabaseName)>,
) -> Result<Body> {
	let servers = servers.with_conn(conn.conn());

	let LoadServer { api, .. } =
		load_server(&id, &user, &servers, &api_client).await?;

	let stream = api
		.postgres()
		.dump_database(&name)
		.await?
		.map(|r| r.with_message("failed to dump database"));

	Ok(Body::from_stream(stream))
}

pub fn routes() -> Router<AppState> {
	Router::new()
		.route(
			"/{id}/postgres/databases",
			get(all_databases).post(create_database),
		)
		.route(
			"/{id}/postgres/databases/{name}/password",
			post(new_password),
		)
		.route(
			"/{id}/postgres/databases/{name}/restore",
			put(restore_database),
		)
		.route("/{id}/postgres/databases/{name}/dump", get(dump_database))
}
