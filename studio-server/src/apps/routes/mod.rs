pub mod compose;
pub mod get;
pub mod set;

use axum::Router;
use axum::routing::get;
use internal_api::app_id::AppId;
use internal_api::apps::ServiceState;
use pg::UniqueId;
use pg::time::DateTime;
use serde::{Deserialize, Serialize};

use crate::AppState;
use crate::apps::routes::compose::{get_compose, set_compose};
use crate::apps::routes::get::{all, by_id};
use crate::apps::routes::set::create;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSummary {
	pub id: AppId,
	pub name: String,
	pub team_id: UniqueId,
	pub server_id: UniqueId,
	pub created_on: DateTime,
	pub services_states: Vec<ServiceState>,
}

pub fn routes() -> Router<AppState> {
	Router::new()
		.route("/", get(all).post(create))
		.route("/{id}", get(by_id))
		.route("/{id}/compose", get(get_compose).post(set_compose))
}
