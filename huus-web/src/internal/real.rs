use internal_api::{
	client::{self as int, Result},
	requests::{PingRes, VersionRes},
};

use crate::{internal::ApiServerClientTrait, servers::data::Server};

pub struct ApiServerClient {
	inner: int::ApiServerClient,
}

impl ApiServerClient {
	pub fn new(client: &int::ApiClient, server: &Server) -> Result<Self> {
		client
			.connect(&server.addr, &server.tls_cert, server.api_token.clone())
			.map(|inner| Self { inner })
	}
}

#[async_trait::async_trait]
impl ApiServerClientTrait for ApiServerClient {
	async fn ping(&self) -> Result<PingRes> {
		self.inner.ping().await
	}

	async fn version(&self) -> Result<VersionRes> {
		self.inner.version().await
	}
}
