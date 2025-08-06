use axum::extract::State;
use axum::routing::get;
use axum::{Json, Router};
use internal_api::requests::{ApiToken, InfoRes};
use pg::UniqueId;
use pg::time::DateTime;
use serde::{Deserialize, Serialize};
use tracing::error;

use crate::AppState;
use crate::error::Result;
use crate::internal::ApiClient;
use crate::servers::Servers;
use crate::users::utils::RightsAny;
use crate::utils::ConnOwned;
use crate::{servers::data, users::utils::AuthedUser};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Server {
	pub id: UniqueId,
	pub team_id: UniqueId,
	pub name: String,
	pub domain: String,
	pub registry_domain: Option<String>,
	/// if this is empty the server could not be reached
	pub version: Option<String>,
	pub created_on: DateTime,
}

impl Server {
	pub fn populate_from_info(&mut self, info: InfoRes) {
		self.registry_domain = Some(info.registry_domain);
		self.version = Some(info.version.to_string());
	}
}

impl From<data::Server> for Server {
	fn from(server: data::Server) -> Self {
		Self {
			id: server.id,
			team_id: server.team_id,
			name: server.name,
			domain: server.domain,
			registry_domain: None,
			version: None,
			created_on: server.created_on,
		}
	}
}

async fn all(
	user: AuthedUser<RightsAny>,
	State(servers): State<Servers>,
	State(api): State<ApiClient>,
	conn: ConnOwned,
) -> Result<Json<Vec<Server>>> {
	let servers = servers.with_conn(conn.conn());

	let servers = servers.all(&user.team_for_filter()).await?;
	let mut n_servers = Vec::with_capacity(servers.len());

	for server in servers {
		let client = api.connect(&server)?;
		let mut server = Server::from(server);

		match client.info().await {
			Ok(i) => server.populate_from_info(i),
			Err(e) => {
				error!("Failed to get info for server {}: {e}", server.id)
			}
		}

		n_servers.push(server);
	}

	Ok(Json(n_servers))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateServerReq {
	name: String,
	domain: String,
	api_token: ApiToken,
	tls_cert: String,
}

async fn create(
	user: AuthedUser<RightsAny>,
	State(servers): State<Servers>,
	State(client): State<ApiClient>,
	conn: ConnOwned,
	Json(req): Json<CreateServerReq>,
) -> Result<Json<Server>> {
	let servers = servers.with_conn(conn.conn());

	// let's create a server and then check if the server can be connected to
	let server = data::Server {
		id: UniqueId::new(),
		team_id: user.user.team_id,
		name: req.name,
		domain: req.domain,
		api_token: req.api_token,
		tls_cert: req.tls_cert,
		created_on: DateTime::now(),
	};
	let client = client.connect(&server)?;

	// check if the information of the server works
	let info = client.info().await?;
	servers.insert(&server).await?;

	let mut server = Server::from(server);
	server.populate_from_info(info);

	Ok(Json(server))
}

pub fn routes() -> Router<AppState> {
	Router::new().route("/", get(all).post(create))
}
