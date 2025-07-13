use axum::extract::State;
use axum::routing::get;
use axum::{Json, Router};
use internal_api::client::ApiClient;
use internal_api::requests::ApiToken;
use pg::UniqueId;
use pg::time::DateTime;
use serde::{Deserialize, Serialize};

use crate::AppState;
use crate::error::Result;
use crate::servers::Servers;
use crate::users::utils::RightsAny;
use crate::utils::ConnOwned;
use crate::{servers::data::Server, users::utils::AuthedUser};

async fn all(
	user: AuthedUser<RightsAny>,
	State(servers): State<Servers>,
	conn: ConnOwned,
) -> Result<Json<Vec<Server>>> {
	let servers = servers.with_conn(conn.conn());

	if user.user.rights.root {
		servers.all().await.map(Json)
	} else {
		servers.all_by_team(&user.user.team_id).await.map(Json)
	}
	.map_err(Into::into)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CreateServerReq {
	name: String,
	addr: String,
	api_token: ApiToken,
	cert: String,
}

async fn create(
	user: AuthedUser<RightsAny>,
	State(servers): State<Servers>,
	State(client): State<ApiClient>,
	conn: ConnOwned,
	Json(req): Json<CreateServerReq>,
) -> Result<Json<Server>> {
	let servers = servers.with_conn(conn.conn());
	let client = client.connect(&req.addr, &req.cert, req.api_token.clone())?;

	// check if the information of the server works
	let _version = client.version().await?;
	// seems to work else version would have failed now we can insert the server

	let server = Server {
		id: UniqueId::new(),
		team_id: user.user.team_id,
		name: req.name,
		addr: req.addr,
		api_token: req.api_token,
		tls_cert: req.cert,
		created_on: DateTime::now(),
	};

	servers.insert(&server).await?;

	Ok(Json(server))
}

pub fn routes() -> Router<AppState> {
	Router::new().route("/", get(all).post(create))
}
