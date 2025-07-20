mod mock;
mod real;

use std::sync::Arc;

use internal_api::{
	client::{self as int, Result},
	requests::{PingRes, VersionRes},
};

use crate::servers::data::Server;

#[derive(Debug, Clone)]
pub struct ApiClient {
	real: Option<int::ApiClient>,
}

impl ApiClient {
	pub fn new(mock: bool) -> Self {
		Self {
			real: mock.then(int::ApiClient::new),
		}
	}

	pub fn connect(&self, server: &Server) -> Result<ApiServerClient> {
		if let Some(real) = &self.real {
			Ok(Arc::new(Box::new(real::ApiServerClient::new(
				real, server,
			)?)))
		} else {
			Ok(Arc::new(Box::new(mock::ApiServerClient::new(server))))
		}
	}
}

pub type ApiServerClient = Arc<Box<dyn ApiServerClientTrait + Send + Sync>>;

#[async_trait::async_trait]
pub trait ApiServerClientTrait {
	async fn ping(&self) -> Result<PingRes>;

	async fn version(&self) -> Result<VersionRes>;
}
