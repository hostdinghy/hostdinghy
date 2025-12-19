use internal_api::{
	app_id::AppId,
	apps::{AppInfoRes, ComposeCommand, GetComposeRes, SaveComposeReq},
	client::{self as int, Result},
	registry::CreateUserRes,
	requests::{InfoRes, PingRes},
};

use crate::{
	internal::{
		ApiServerAppsClientTrait, ApiServerClientTrait,
		ApiServerRegistryClientTrait,
	},
	servers::data::Server,
};

pub struct ApiServerClient {
	inner: int::ApiServerClient,
}

impl ApiServerClient {
	pub fn new(client: &int::ApiClient, server: &Server) -> Result<Self> {
		client
			.connect(&server.domain, &server.tls_cert, server.api_token.clone())
			.map(|inner| Self { inner })
	}
}

#[async_trait::async_trait]
impl ApiServerClientTrait for ApiServerClient {
	async fn ping(&self) -> Result<PingRes> {
		self.inner.ping().await
	}

	async fn info(&self) -> Result<InfoRes> {
		self.inner.info().await
	}

	fn apps(&self) -> &dyn ApiServerAppsClientTrait {
		self
	}

	fn registry(&self) -> &dyn ApiServerRegistryClientTrait {
		self
	}
}

#[async_trait::async_trait]
impl ApiServerAppsClientTrait for ApiServerClient {
	async fn app_info(&self, id: &AppId) -> Result<AppInfoRes> {
		self.inner.apps().app_info(id).await
	}

	async fn get_compose(&self, id: &AppId) -> Result<GetComposeRes> {
		self.inner.apps().get_compose(id).await
	}

	async fn set_compose(
		&self,
		id: &AppId,
		req: &SaveComposeReq,
	) -> Result<()> {
		self.inner.apps().set_compose(id, req).await
	}

	async fn compose_command(
		&self,
		id: &AppId,
		cmd: &ComposeCommand,
	) -> Result<()> {
		self.inner.apps().compose_command(id, cmd).await
	}

	async fn compose_service_command(
		&self,
		id: &AppId,
		service: &str,
		cmd: &ComposeCommand,
	) -> Result<()> {
		self.inner
			.apps()
			.compose_service_command(id, service, cmd)
			.await
	}

	async fn app_logs(&self, id: &AppId, lines: Option<u32>) -> Result<String> {
		self.inner.apps().app_logs(id, lines).await
	}
}

#[async_trait::async_trait]
impl ApiServerRegistryClientTrait for ApiServerClient {
	async fn users(&self) -> Result<Vec<String>> {
		self.inner.registry().users().await
	}

	async fn create_user(&self, username: &str) -> Result<CreateUserRes> {
		self.inner.registry().create_user(username).await
	}

	async fn delete_user(&self, username: &str) -> Result<()> {
		self.inner.registry().delete_user(username).await
	}
}
