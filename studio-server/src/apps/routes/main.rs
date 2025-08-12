use std::collections::HashMap;

use axum::extract::{Path, State};
use axum::routing::get;
use axum::{Json, Router};
use internal_api::app_id::AppId;
use internal_api::apps::{AppService, ServiceState};
use internal_api::error::Error as ApiError;
use pg::UniqueId;
use pg::time::DateTime;
use serde::{Deserialize, Serialize};

use crate::AppState;
use crate::apps::routes::utils::{AppWithServer, app_with_server};
use crate::apps::{Apps, data};
use crate::error::Result;
use crate::internal::ApiClient;
use crate::servers::Servers;
use crate::users::utils::AuthedUser;
use crate::users::utils::RightsAny;
use crate::utils::ConnOwned;

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

pub async fn all(
	user: AuthedUser<RightsAny>,
	State(servers): State<Servers>,
	State(apps): State<Apps>,
	State(api_client): State<ApiClient>,
	conn: ConnOwned,
) -> Result<Json<Vec<AppSummary>>> {
	let servers = servers.with_conn(conn.conn());
	let apps = apps.with_conn(conn.conn());

	let servers = servers.all(&user.team_for_filter()).await?;
	let servers = servers
		.into_iter()
		.map(|s| (s.id, s))
		.collect::<HashMap<_, _>>();

	let apps = apps.all(&user.team_for_filter()).await?;

	let mut apps = apps
		.into_iter()
		.map(|app| AppSummary {
			id: app.id,
			name: app.name,
			team_id: app.team_id,
			server_id: app.server_id,
			created_on: app.created_on,
			services_states: vec![],
		})
		.collect::<Vec<_>>();

	// todo parallize this
	for app in &mut apps {
		let Some(server) = servers.get(&app.server_id) else {
			continue;
		};

		let api = api_client.connect(server)?;

		let app_info = match api.app_info(&app.id).await {
			Ok(a) => a,
			Err(ApiError::AppNotFound) => continue,
			Err(e) => return Err(e.into()),
		};

		app.services_states =
			app_info.services.into_iter().map(|s| s.state).collect();
	}

	Ok(Json(apps))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct App {
	pub id: AppId,
	pub name: String,
	pub team_id: UniqueId,
	pub server_id: UniqueId,
	pub created_on: DateTime,
	pub services: Vec<AppService>,
}

pub async fn by_id(
	user: AuthedUser<RightsAny>,
	State(apps): State<Apps>,
	State(servers): State<Servers>,
	State(api_client): State<ApiClient>,
	Path(id): Path<AppId>,
	conn: ConnOwned,
) -> Result<Json<App>> {
	let apps = apps.with_conn(conn.conn());
	let servers = servers.with_conn(conn.conn());

	let AppWithServer { app, api, .. } =
		app_with_server(&id, &user, &apps, &servers, &api_client).await?;

	let services = match api.app_info(&app.id).await {
		Ok(a) => a.services,
		Err(ApiError::AppNotFound) => vec![],
		Err(e) => return Err(e.into()),
	};

	Ok(Json(App {
		id: app.id,
		name: app.name,
		team_id: app.team_id,
		server_id: app.server_id,
		created_on: app.created_on,
		services: services.into_iter().map(Into::into).collect(),
	}))
}

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
) -> Result<Json<data::App>> {
	let apps = apps.with_conn(conn.conn());

	// Create a new app
	let app = data::App {
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
