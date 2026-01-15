mod mock;
mod real;

use std::{
	collections::HashMap,
	fmt,
	sync::{Arc, RwLock},
};

use bytes::Bytes;
use futures::stream::BoxStream;
use internal_api::{
	apps::{AppId, AppInfoRes, ComposeCommand, GetComposeRes, SaveComposeReq},
	client::{self as int, Result},
	postgres::{CreateDatabaseRes, DatabaseName, NewPasswordRes},
	registry::{CreateUserRes, RegistryUsername},
	requests::{InfoRes, PingRes},
};
use pg::{UniqueId, db::ConnOwned};

use crate::{AppState, servers::data::Server};

// todo should this be wrappen in a arc?
// a clone here contains at least two arcs
#[derive(Clone)]
pub struct ApiClient {
	inner: Inner,
	servers: Arc<RwLock<HashMap<UniqueId, ApiServerClient>>>,
}

#[derive(Debug, Clone)]
enum Inner {
	Real(int::ApiClient),
	Mock(mock::ApiClient),
}

impl ApiClient {
	pub fn new(mock: bool) -> Self {
		Self {
			inner: if mock {
				Inner::Mock(mock::ApiClient::new())
			} else {
				Inner::Real(int::ApiClient::new())
			},
			servers: Arc::new(RwLock::new(HashMap::new())),
		}
	}

	pub fn is_mock(&self) -> bool {
		matches!(self.inner, Inner::Mock(_))
	}

	pub async fn populate_mock_data(
		&self,
		conn: &mut ConnOwned,
		state: &AppState,
	) {
		if let Inner::Mock(mock) = &self.inner {
			mock.populate_mock_data(conn, state).await
		}
	}

	/// This will resuse connections if one exists
	pub fn connect(&self, server: &Server) -> Result<ApiServerClient> {
		// let's check if we already have a connection for this server
		{
			let servers = self.servers.read().unwrap();
			if let Some(client) = servers.get(&server.id) {
				return Ok(client.clone());
			}
		}

		let mut servers = self.servers.write().unwrap();
		// since we unlocked let's check again if we already have a connection
		if let Some(client) = servers.get(&server.id) {
			return Ok(client.clone());
		}

		let api_server_client: ApiServerClient = match &self.inner {
			Inner::Real(real) => {
				Arc::new(Box::new(real::ApiServerClient::new(real, server)?))
			}
			Inner::Mock(mock) => {
				Arc::new(Box::new(mock::ApiServerClient::new(mock, server)?))
			}
		};

		servers.insert(server.id, api_server_client.clone());

		Ok(api_server_client)
	}
}

impl fmt::Debug for ApiClient {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match &self.inner {
			Inner::Real(_) => write!(f, "ApiClient(Real)"),
			Inner::Mock(_) => write!(f, "ApiClient(Mock)"),
		}
	}
}

pub type ApiServerClient = Arc<Box<dyn ApiServerClientTrait + Send + Sync>>;

#[async_trait::async_trait]
pub trait ApiServerClientTrait {
	#[allow(dead_code)]
	async fn ping(&self) -> Result<PingRes>;

	async fn info(&self) -> Result<InfoRes>;

	fn apps(&self) -> &dyn ApiServerAppsClientTrait;

	fn registry(&self) -> &dyn ApiServerRegistryClientTrait;

	fn postgres(&self) -> &dyn ApiServerPostgresClientTrait;
}

#[async_trait::async_trait]
pub trait ApiServerAppsClientTrait {
	async fn app_info(&self, id: &AppId) -> Result<AppInfoRes>;

	async fn get_compose(&self, id: &AppId) -> Result<GetComposeRes>;

	async fn set_compose(&self, id: &AppId, req: &SaveComposeReq)
	-> Result<()>;

	async fn compose_command(
		&self,
		id: &AppId,
		cmd: &ComposeCommand,
	) -> Result<()>;

	async fn compose_service_command(
		&self,
		id: &AppId,
		service: &str,
		cmd: &ComposeCommand,
	) -> Result<()>;

	/// How many lines to return, if None all lines are returned
	async fn app_logs(&self, id: &AppId, lines: Option<u32>) -> Result<String>;
}

#[async_trait::async_trait]
pub trait ApiServerRegistryClientTrait {
	async fn users(&self) -> Result<Vec<String>>;

	async fn create_user(
		&self,
		username: &RegistryUsername,
	) -> Result<CreateUserRes>;

	async fn delete_user(&self, username: &RegistryUsername) -> Result<()>;
}

#[async_trait::async_trait]
pub trait ApiServerPostgresClientTrait {
	async fn databases(&self) -> Result<Vec<String>>;

	async fn create_database(
		&self,
		name: &DatabaseName,
	) -> Result<CreateDatabaseRes>;

	async fn new_password(&self, name: &DatabaseName)
	-> Result<NewPasswordRes>;

	async fn restore_database(
		&self,
		name: &DatabaseName,
		bytes: BoxStream<'static, Result<Bytes>>,
	) -> Result<()>;

	async fn dump_database(
		&self,
		name: &DatabaseName,
	) -> Result<BoxStream<'static, Result<Bytes>>>;
}
