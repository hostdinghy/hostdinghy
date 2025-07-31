use internal_api::{
	app_id::AppId,
	apps::{AppInfoRes, GetComposeRes, SaveComposeReq},
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

	async fn app_info(&self, id: &AppId) -> Result<AppInfoRes> {
		self.inner.app_info(id).await
	}

	async fn app_get_compose(&self, id: &AppId) -> Result<GetComposeRes> {
		self.inner.app_get_compose(id).await
	}

	async fn app_set_compose(
		&self,
		id: &AppId,
		req: &SaveComposeReq,
	) -> Result<()> {
		self.inner.app_set_compose(id, req).await
	}

	async fn app_logs(&self, id: &AppId, lines: Option<u32>) -> Result<String> {
		self.inner.app_logs(id, lines).await
	}
}
