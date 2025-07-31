mod mock;
mod real;

use std::sync::Arc;

use internal_api::{
	app_id::AppId,
	apps::{AppInfoRes, GetComposeRes, SaveComposeReq},
	client::{self as int, Result},
	requests::{PingRes, VersionRes},
};
use pg::db::ConnOwned;

use crate::{AppState, servers::data::Server};

#[derive(Debug, Clone)]
pub enum ApiClient {
	Real(int::ApiClient),
	Mock(mock::ApiClient),
}

impl ApiClient {
	pub fn new(mock: bool) -> Self {
		if mock {
			Self::Mock(mock::ApiClient::new())
		} else {
			Self::Real(int::ApiClient::new())
		}
	}

	pub async fn populate_mock_data(
		&self,
		conn: &mut ConnOwned,
		state: &AppState,
	) {
		if let Self::Mock(mock) = self {
			mock.populate_mock_data(conn, state).await
		}
	}

	pub fn connect(&self, server: &Server) -> Result<ApiServerClient> {
		Ok(match self {
			Self::Real(real) => {
				Arc::new(Box::new(real::ApiServerClient::new(real, server)?))
			}
			Self::Mock(mock) => {
				Arc::new(Box::new(mock::ApiServerClient::new(mock, server)?))
			}
		})
	}
}

pub type ApiServerClient = Arc<Box<dyn ApiServerClientTrait + Send + Sync>>;

#[async_trait::async_trait]
pub trait ApiServerClientTrait {
	#[allow(dead_code)]
	async fn ping(&self) -> Result<PingRes>;

	async fn version(&self) -> Result<VersionRes>;

	async fn app_info(&self, id: &AppId) -> Result<AppInfoRes>;

	async fn app_get_compose(&self, id: &AppId) -> Result<GetComposeRes>;

	async fn app_set_compose(
		&self,
		id: &AppId,
		req: &SaveComposeReq,
	) -> Result<()>;

	/// How many lines to return, if None all lines are returned
	async fn app_logs(&self, id: &AppId, lines: Option<u32>) -> Result<String>;
}
