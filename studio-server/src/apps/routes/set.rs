use axum::Json;
use axum::extract::State;
use internal_api::app_id::AppId;
use pg::UniqueId;
use pg::time::DateTime;
use serde::{Deserialize, Serialize};

use crate::apps::Apps;
use crate::apps::data::App;
use crate::error::Result;
use crate::users::utils::AuthedUser;
use crate::users::utils::RightsAny;
use crate::utils::ConnOwned;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAppReq {
	id: AppId,
	name: String,
	server_id: UniqueId,
}

pub async fn create(
	user: AuthedUser<RightsAny>,
	State(apps): State<Apps>,
	conn: ConnOwned,
	Json(req): Json<CreateAppReq>,
) -> Result<Json<App>> {
	let apps = apps.with_conn(conn.conn());

	// Create a new app
	let app = App {
		id: req.id,
		name: req.name,
		team_id: user.user.team_id,
		server_id: req.server_id,
		created_on: DateTime::now(),
	};

	apps.insert(&app).await?;

	Ok(Json(app))
}
