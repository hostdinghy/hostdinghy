use std::collections::HashMap;

use axum::Json;
use axum::extract::{Path, State};
use internal_api::app_id::AppId;
use internal_api::apps::AppService;
use internal_api::error::Error as ApiError;
use pg::UniqueId;
use pg::time::DateTime;
use serde::{Deserialize, Serialize};

use crate::apps::Apps;
use crate::apps::routes::AppSummary;
use crate::error::Error;
use crate::error::Result;
use crate::internal::ApiClient;
use crate::servers::Servers;
use crate::users::utils::AuthedUser;
use crate::users::utils::RightsAny;
use crate::utils::ConnOwned;

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
) -> Result<Json<Option<App>>> {
	let apps = apps.with_conn(conn.conn());
	let Some(app) = apps.by_id(&id, &user.team_for_filter()).await? else {
		return Ok(Json(None));
	};

	let server = servers
		.with_conn(conn.conn())
		.by_id(&app.server_id, &user.team_for_filter())
		.await?
		.ok_or(Error::Internal("Server was not found".into()))?;

	let api = api_client
		.connect(&server)
		.map_err(|e| Error::InternalApiServer(e.to_string()))?;

	let services = match api.app_info(&app.id).await {
		Ok(a) => a.services,
		Err(ApiError::AppNotFound) => vec![],
		Err(e) => return Err(e.into()),
	};

	Ok(Json(Some(App {
		id: app.id,
		name: app.name,
		team_id: app.team_id,
		server_id: app.server_id,
		created_on: app.created_on,
		services: services.into_iter().map(Into::into).collect(),
	})))
}
