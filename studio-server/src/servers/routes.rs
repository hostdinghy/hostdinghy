use std::sync::Arc;

use axum::extract::State;
use axum::http::HeaderMap;
use axum::http::header::AUTHORIZATION;
use axum::routing::get;
use axum::{Json, Router};
use internal_api::requests::ApiToken;
use pg::UniqueId;
use pg::time::DateTime;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::error::{Error, Result};
use crate::internal::ApiClient;
use crate::servers::Servers;
use crate::users::utils::RightsAny;
use crate::utils::ConnOwned;
use crate::{AppState, Config};
use crate::{servers::data::Server, users::utils::AuthedUser};

async fn all(
	user: AuthedUser<RightsAny>,
	State(servers): State<Servers>,
	conn: ConnOwned,
) -> Result<Json<Vec<Server>>> {
	let servers = servers.with_conn(conn.conn());

	servers
		.all(&user.team_for_filter())
		.await
		.map(Json)
		.map_err(Into::into)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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

	// let's create a server and then check if the server can be connected to
	let server = Server {
		id: UniqueId::new(),
		team_id: user.user.team_id,
		name: req.name,
		addr: req.addr,
		api_token: req.api_token,
		tls_cert: req.cert,
		created_on: DateTime::now(),
	};
	let client = client.connect(&server)?;

	// check if the information of the server works
	let _version = client.version().await?;
	// seems to work else version would have failed now we can insert the server

	servers.insert(&server).await?;

	Ok(Json(server))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Events {
	events: Vec<Event>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EventAction {
	Push,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
	id: String,
	action: EventAction,
	target: PushTarget,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PushTarget {
	digest: String,
	repository: String,
	url: String,
	#[serde(default)]
	tag: String,
}

async fn registry_webhook(
	State(servers): State<Servers>,
	State(client): State<ApiClient>,
	State(cfg): State<Arc<Config>>,
	headers: HeaderMap,
	conn: ConnOwned,
	Json(req): Json<Events>,
) -> Result<()> {
	let bearer = headers
		.get(AUTHORIZATION)
		.ok_or(Error::MissingSessionToken)?
		.to_str()
		.map_err(|_| Error::MissingSessionToken)?
		.strip_prefix("Bearer ")
		.ok_or(Error::MissingSessionToken)?;

	if !cfg.registry_webhook_tokens.contains(bearer) {
		return Err(Error::InvalidSessionToken);
	}

	// https://distribution.github.io/distribution/about/notifications/
	info!("Received registry webhook: {:?}", req.events);

	Ok(())
}

pub fn routes() -> Router<AppState> {
	Router::new()
		.route("/", get(all).post(create))
		.route("/registry/webhook", get(registry_webhook))
}
