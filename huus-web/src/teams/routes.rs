use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use pg::UniqueId;
use pg::time::DateTime;
use serde::{Deserialize, Serialize};

use crate::AppState;
use crate::error::Result;
use crate::teams::Teams;
use crate::teams::data::Team;
use crate::users::utils::AuthedUser;
use crate::users::utils::RightsRoot;
use crate::utils::ConnOwned;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CreateTeamReq {
	name: String,
}

async fn create(
	_user: AuthedUser<RightsRoot>,
	State(teams): State<Teams>,
	conn: ConnOwned,
	Json(req): Json<CreateTeamReq>,
) -> Result<Json<Team>> {
	let teams = teams.with_conn(conn.conn());

	let team = Team {
		id: UniqueId::new(),
		name: req.name,
		created_on: DateTime::now(),
	};

	teams.insert(&team).await?;

	Ok(Json(team))
}

pub fn routes() -> Router<AppState> {
	Router::new().route("/", post(create))
}
