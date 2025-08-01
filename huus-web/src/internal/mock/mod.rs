mod storage;

use std::sync::{Arc, Mutex};

use crate::{
	AppState,
	internal::{
		ApiServerClientTrait,
		mock::storage::{ServerMock, ServersMock},
	},
	servers::data::Server,
};

use internal_api::{
	app_id::AppId,
	apps::{AppInfoRes, ComposeCommand, GetComposeRes, SaveComposeReq},
	client::Result,
	error::Error,
	requests::{PingRes, VersionRes},
};
use pg::{UniqueId, db::ConnOwned, time::DateTime};

#[derive(Debug, Clone)]
pub struct ApiClient {
	inner: Arc<Mutex<ServersMock>>,
}

impl ApiClient {
	pub fn new() -> Self {
		Self {
			inner: Arc::new(Mutex::new(ServersMock::new())),
		}
	}

	fn get_server(&self, id: &UniqueId) -> Option<Arc<Mutex<ServerMock>>> {
		let inner = self.inner.lock().unwrap();
		inner.get(id)
	}

	// not sure this is a good idear, but it works
	pub async fn populate_mock_data(&self, conn: &ConnOwned, state: &AppState) {
		let servers = state.servers.with_conn(conn.conn());
		let servers = servers.all(&None).await.unwrap();

		let apps = state.apps.with_conn(conn.conn());
		let apps = apps.all(&None).await.unwrap();

		let mut inner = self.inner.lock().unwrap();
		inner.populate_mock_data(servers, apps);
	}
}

pub struct ApiServerClient {
	server: Arc<Mutex<ServerMock>>,
}

impl ApiServerClient {
	pub fn new(client: &ApiClient, server: &Server) -> Result<Self> {
		let server = client.get_server(&server.id).ok_or(Error::any(
			"server not found",
			format!("the server {} could not be found", server.id),
		))?;

		Ok(Self { server })
	}
}

#[async_trait::async_trait]
impl ApiServerClientTrait for ApiServerClient {
	async fn ping(&self) -> Result<PingRes> {
		Ok(PingRes {
			time: DateTime::now(),
		})
	}

	async fn version(&self) -> Result<VersionRes> {
		Ok(VersionRes {
			version: "0.0.0-debug.0".parse().unwrap(),
			commit: None,
			build_date: None,
		})
	}

	async fn app_info(&self, id: &AppId) -> Result<AppInfoRes> {
		let server = self.server.lock().unwrap();
		server.app_info(id)
	}

	async fn app_get_compose(&self, id: &AppId) -> Result<GetComposeRes> {
		let server = self.server.lock().unwrap();
		server.app_get_compose(id)
	}

	async fn app_set_compose(
		&self,
		id: &AppId,
		req: &SaveComposeReq,
	) -> Result<()> {
		let mut server = self.server.lock().unwrap();
		server.app_set_compose(id, req)
	}

	async fn app_compose_command(
		&self,
		id: &AppId,
		cmd: &ComposeCommand,
	) -> Result<()> {
		let mut server = self.server.lock().unwrap();
		server.app_compose_command(id, cmd)
	}

	async fn app_logs(&self, id: &AppId, lines: Option<u32>) -> Result<String> {
		let server = self.server.lock().unwrap();
		server.app_logs(id, lines)
	}
}
