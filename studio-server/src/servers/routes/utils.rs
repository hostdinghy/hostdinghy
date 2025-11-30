use internal_api::app_id::AppId;
use pg::UniqueId;

use crate::{
	error::{Error, Result},
	internal::{ApiClient, ApiServerClient},
	servers::data::{Server, ServersWithConn},
	users::utils::AuthedUser,
};

pub async fn load_server<R>(
	id: &UniqueId,
	user: &AuthedUser<R>,
	servers: &ServersWithConn<'_>,
	api_client: &ApiClient,
) -> Result<LoadServer> {
	let server = servers
		.by_id(&id, &user.team_for_filter())
		.await?
		.ok_or(Error::Internal("Server was not found".into()))?;

	let api = api_client
		.connect(&server)
		.map_err(|e| Error::InternalApiServer(e.to_string()))?;

	Ok(LoadServer { server, api })
}

pub struct LoadServer {
	#[allow(dead_code)]
	pub server: Server,
	pub api: ApiServerClient,
}
