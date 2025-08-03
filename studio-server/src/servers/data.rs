use std::sync::Arc;

use axum::extract::FromRef;
use internal_api::requests::ApiToken;
use pg::{Result, UniqueId, db::Conn, time::DateTime};
use serde::{Deserialize, Serialize};

use crate::AppState;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Server {
	pub id: UniqueId,
	pub team_id: UniqueId,
	pub name: String,
	pub domain: String,
	pub api_token: ApiToken,
	pub tls_cert: String,
	pub created_on: DateTime,
}

pub type Servers = Arc<Box<dyn ServersBuilderTrait + Send + Sync>>;
pub type ServersWithConn<'a> = Box<dyn ServersTrait + Send + Sync + 'a>;

impl FromRef<AppState> for Servers {
	fn from_ref(state: &AppState) -> Self {
		state.servers.clone()
	}
}

pub trait ServersBuilderTrait {
	fn with_conn<'a>(&'a self, conn: Conn<'a>) -> ServersWithConn<'a>;
}

#[async_trait::async_trait]
pub trait ServersTrait {
	async fn all(&self, team_id: &Option<UniqueId>) -> Result<Vec<Server>>;

	async fn by_id(
		&self,
		id: &UniqueId,
		team_id: &Option<UniqueId>,
	) -> Result<Option<Server>>;

	async fn insert(&self, server: &Server) -> Result<()>;
}
