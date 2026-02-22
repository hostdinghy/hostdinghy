pub mod database;
#[cfg(test)]
pub mod mock;

use std::sync::Arc;

use axum::extract::FromRef;
use internal_api::apps::AppId;
use pg::{Result, UniqueId, db::Conn, time::DateTime};
use serde::{Deserialize, Serialize};

use crate::AppState;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct App {
	pub id: AppId,
	pub name: String,
	pub team_id: UniqueId,
	pub server_id: UniqueId,
	pub created_on: DateTime,
}

pub type Apps = Arc<dyn AppsBuilderTrait + Send + Sync>;
pub type AppsWithConn<'a> = Box<dyn AppsTrait + Send + Sync + 'a>;

impl FromRef<AppState> for Apps {
	fn from_ref(state: &AppState) -> Self {
		state.apps.clone()
	}
}

pub trait AppsBuilderTrait {
	fn with_conn<'a>(&'a self, conn: Conn<'a>) -> AppsWithConn<'a>;
}

#[async_trait::async_trait]
pub trait AppsTrait {
	async fn all(&self, team_id: &Option<UniqueId>) -> Result<Vec<App>>;

	async fn by_id(
		&self,
		id: &AppId,
		team_id: &Option<UniqueId>,
	) -> Result<Option<App>>;

	async fn insert(&self, app: &App) -> Result<()>;
}
