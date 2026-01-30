mod storage;

use std::sync::{Arc, Mutex};

use crate::{
	AppState,
	internal::{
		ApiServerAppsClientTrait, ApiServerClientTrait,
		ApiServerPostgresClientTrait, ApiServerRegistryClientTrait,
		mock::storage::{ServerMock, ServersMock},
	},
	servers::data::Server,
};

use bytes::{Bytes, BytesMut};
use futures::{
	StreamExt,
	stream::{self, BoxStream},
};
use internal_api::{
	apps::{AppId, AppInfoRes, ComposeCommand, GetComposeRes, SaveComposeReq},
	client::Result,
	error::Error,
	postgres::{CreateDatabaseRes, DatabaseName, NewPasswordRes},
	registry::{CreateUserRes, RegistryUsername},
	requests::{InfoRes, PingRes},
};
use pg::{UniqueId, db::ConnOwned, time::DateTime};
use rand::Rng;

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
		if let Some(server) = client.get_server(&server.id) {
			return Ok(Self {
				server: server.clone(),
			});
		}

		// now let's have a percentage chance to fail
		let mut rng = rand::rng();
		if rng.random_bool(0.5) {
			Err(Error::any(
				"server not found",
				format!("the server {} could not be found", server.id),
			))
		} else {
			Ok(Self {
				server: Arc::new(Mutex::new(ServerMock::new(server.clone()))),
			})
		}
	}
}

#[async_trait::async_trait]
impl ApiServerClientTrait for ApiServerClient {
	async fn ping(&self) -> Result<PingRes> {
		Ok(PingRes {
			time: DateTime::now(),
		})
	}

	async fn info(&self) -> Result<InfoRes> {
		let server = self.server.lock().unwrap();

		Ok(InfoRes {
			registry_domain: server.registry_domain.clone(),
			version: server.version.clone(),
			commit: None,
			build_date: None,
		})
	}

	fn apps(&self) -> &dyn ApiServerAppsClientTrait {
		self
	}

	fn registry(&self) -> &dyn ApiServerRegistryClientTrait {
		self
	}

	fn postgres(&self) -> &dyn ApiServerPostgresClientTrait {
		self
	}
}

#[async_trait::async_trait]
impl ApiServerAppsClientTrait for ApiServerClient {
	async fn app_info(&self, id: &AppId) -> Result<AppInfoRes> {
		let server = self.server.lock().unwrap();
		server.app_info(id)
	}

	async fn get_compose(&self, id: &AppId) -> Result<GetComposeRes> {
		let server = self.server.lock().unwrap();
		server.app_get_compose(id)
	}

	async fn set_compose(
		&self,
		id: &AppId,
		req: &SaveComposeReq,
	) -> Result<()> {
		let mut server = self.server.lock().unwrap();
		server.app_set_compose(id, req)
	}

	async fn compose_command(
		&self,
		id: &AppId,
		cmd: &ComposeCommand,
	) -> Result<()> {
		let mut server = self.server.lock().unwrap();
		server.app_compose_command(id, cmd)
	}

	async fn compose_service_command(
		&self,
		id: &AppId,
		_service: &str,
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

#[async_trait::async_trait]
impl ApiServerRegistryClientTrait for ApiServerClient {
	async fn users(&self) -> Result<Vec<String>> {
		let server = self.server.lock().unwrap();
		server.registry_users()
	}

	async fn create_user(
		&self,
		username: &RegistryUsername,
	) -> Result<CreateUserRes> {
		let mut server = self.server.lock().unwrap();
		server.registry_create_user(username.as_ref())
	}

	async fn delete_user(&self, username: &RegistryUsername) -> Result<()> {
		let mut server = self.server.lock().unwrap();
		server.registry_delete_user(username.as_ref())
	}
}

#[async_trait::async_trait]
impl ApiServerPostgresClientTrait for ApiServerClient {
	async fn databases(&self) -> Result<Vec<String>> {
		let server = self.server.lock().unwrap();
		server.postgres_databases()
	}

	async fn create_database(
		&self,
		name: &DatabaseName,
	) -> Result<CreateDatabaseRes> {
		let mut server = self.server.lock().unwrap();
		server.postgres_create_database(name.as_ref())
	}

	async fn new_password(
		&self,
		name: &DatabaseName,
	) -> Result<NewPasswordRes> {
		let mut server = self.server.lock().unwrap();
		server.postgres_new_password(name.as_ref())
	}

	async fn restore_database(
		&self,
		name: &DatabaseName,
		mut bytes_stream: BoxStream<'static, Result<Bytes>>,
	) -> Result<()> {
		let mut bytes = BytesMut::new();
		while let Some(b) = bytes_stream.next().await {
			bytes.extend_from_slice(b?.as_ref());
		}

		let mut server = self.server.lock().unwrap();
		server.postgres_restore_database(name.as_ref(), bytes.into())
	}

	async fn dump_database(
		&self,
		name: &DatabaseName,
	) -> Result<BoxStream<'static, Result<Bytes>>> {
		let server = self.server.lock().unwrap();
		let bytes = server.postgres_dump_database(name.as_ref())?;

		Ok(stream::once(async move { Ok(bytes) }).boxed())
	}
}
