use internal_api::app_id::AppId;

use crate::{
	apps::data::{self, AppsWithConn},
	error::{Error, Result},
	internal::{ApiClient, ApiServerClient},
	servers::data::{Server, ServersWithConn},
	users::utils::AuthedUser,
};

pub async fn app_with_server<R>(
	id: &AppId,
	user: &AuthedUser<R>,
	apps: &AppsWithConn<'_>,
	servers: &ServersWithConn<'_>,
	api_client: &ApiClient,
) -> Result<AppWithServer> {
	let app = apps
		.by_id(&id, &user.team_for_filter())
		.await?
		.ok_or(Error::NotFound)?;

	let server = servers
		.by_id(&app.server_id, &user.team_for_filter())
		.await?
		.ok_or(Error::Internal("Server was not found".into()))?;

	let api = api_client
		.connect(&server)
		.map_err(|e| Error::InternalApiServer(e.to_string()))?;

	Ok(AppWithServer { app, server, api })
}

pub struct AppWithServer {
	pub app: data::App,
	#[allow(dead_code)]
	pub server: Server,
	pub api: ApiServerClient,
}
