use axum::extract::{Path, State};
use axum::routing::get;
use axum::{Json, Router};
use pg::UniqueId;
use pg::time::DateTime;
use serde::{Deserialize, Serialize};

use crate::AppState;
use crate::apps::Apps;
use crate::error::Result;
use crate::users::utils::RightsAny;
use crate::utils::ConnOwned;
use crate::{apps::data::App, users::utils::AuthedUser};

async fn all(
	user: AuthedUser<RightsAny>,
	State(apps): State<Apps>,
	conn: ConnOwned,
) -> Result<Json<Vec<App>>> {
	let apps = apps.with_conn(conn.conn());

	if user.user.rights.root {
		apps.all().await.map(Json)
	} else {
		apps.all_by_team(&user.user.team_id).await.map(Json)
	}
	.map_err(Into::into)
}

async fn by_id(
	_user: AuthedUser<RightsAny>,
	State(apps): State<Apps>,
	Path(id): Path<String>,
	conn: ConnOwned,
) -> Result<Json<Option<App>>> {
	let apps = apps.with_conn(conn.conn());
	apps.by_id(&id).await.map(Json).map_err(Into::into)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateAppReq {
	id: String,
	name: String,
	server_id: UniqueId,
}

async fn create(
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

pub fn routes() -> Router<AppState> {
	Router::new()
		.route("/", get(all).post(create))
		.route("/{id}", get(by_id))
}
