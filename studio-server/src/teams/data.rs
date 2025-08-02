use std::sync::Arc;

use axum::extract::FromRef;
use pg::{Result, UniqueId, db::Conn, time::DateTime};
use serde::{Deserialize, Serialize};

use crate::AppState;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Team {
	pub id: UniqueId,
	pub name: String,
	pub created_on: DateTime,
}

pub type Teams = Arc<Box<dyn TeamsBuilderTrait + Send + Sync>>;
pub type TeamsWithConn<'a> = Box<dyn TeamsTrait + Send + Sync + 'a>;

impl FromRef<AppState> for Teams {
	fn from_ref(state: &AppState) -> Self {
		state.teams.clone()
	}
}

pub trait TeamsBuilderTrait {
	fn with_conn<'a>(&'a self, conn: Conn<'a>) -> TeamsWithConn<'a>;
}

#[async_trait::async_trait]
pub trait TeamsTrait {
	async fn by_id(&self, id: &UniqueId) -> Result<Option<Team>>;

	async fn insert(&self, team: &Team) -> Result<()>;
}
