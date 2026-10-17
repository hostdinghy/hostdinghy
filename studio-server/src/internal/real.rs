use bytes::Bytes;
use futures::stream::BoxStream;
use internal_api::{
	apps::{
		AppId, AppInfoRes, AppLogsReq, ComposeCommand, GetComposeRes,
		SaveComposeReq,
	},
	client::{self as int, FsDownload, Result},
	fs::{
		FsExtractReq, FsInfoRes, FsMoveReq, FsPath, FsPermissionsReq,
		FsWriteReq, FsZipReq,
	},
	postgres::{CreateDatabaseRes, DatabaseName, NewPasswordRes},
	registry::{CreateUserRes, RegistryUsername},
	requests::{InfoRes, PingRes},
};

use crate::{
	internal::{
		ApiServerAppsClientTrait, ApiServerClientTrait, ApiServerFsClientTrait,
		ApiServerPostgresClientTrait, ApiServerRegistryClientTrait,
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

	fn postgres(&self) -> &dyn ApiServerPostgresClientTrait {
		self
	}

	fn fs(&self) -> &dyn ApiServerFsClientTrait {
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
		service: Option<&str>,
		cmd: &ComposeCommand,
	) -> Result<()> {
		self.inner.apps().compose_command(id, service, cmd).await
	}

	async fn compose_command_stream(
		&self,
		id: &AppId,
		service: Option<&str>,
		cmd: &ComposeCommand,
	) -> Result<BoxStream<'static, Result<Bytes>>> {
		self.inner
			.apps()
			.compose_command_stream(id, service, cmd)
			.await
	}

	async fn app_logs(
		&self,
		id: &AppId,
		req: &AppLogsReq,
	) -> Result<BoxStream<'static, Result<Bytes>>> {
		self.inner.apps().app_logs(id, req).await
	}
}

#[async_trait::async_trait]
impl ApiServerRegistryClientTrait for ApiServerClient {
	async fn users(&self) -> Result<Vec<String>> {
		self.inner.registry().users().await
	}

	async fn create_user(
		&self,
		username: &RegistryUsername,
	) -> Result<CreateUserRes> {
		self.inner.registry().create_user(username).await
	}

	async fn delete_user(&self, username: &RegistryUsername) -> Result<()> {
		self.inner.registry().delete_user(username).await
	}
}

#[async_trait::async_trait]
impl ApiServerPostgresClientTrait for ApiServerClient {
	async fn databases(&self) -> Result<Vec<String>> {
		self.inner.postgres().databases().await
	}

	async fn create_database(
		&self,
		name: &DatabaseName,
	) -> Result<CreateDatabaseRes> {
		self.inner.postgres().create_database(name).await
	}

	async fn new_password(
		&self,
		name: &DatabaseName,
	) -> Result<NewPasswordRes> {
		self.inner.postgres().new_password(name).await
	}

	async fn restore_database(
		&self,
		name: &DatabaseName,
		bytes: BoxStream<'static, Result<Bytes>>,
	) -> Result<()> {
		self.inner.postgres().restore_database(name, bytes).await
	}

	async fn dump_database(
		&self,
		name: &DatabaseName,
	) -> Result<BoxStream<'static, Result<Bytes>>> {
		self.inner.postgres().dump_database(name).await
	}
}

#[async_trait::async_trait]
impl ApiServerFsClientTrait for ApiServerClient {
	async fn info(&self, id: &AppId, path: &FsPath) -> Result<FsInfoRes> {
		self.inner.fs().info(id, path).await
	}

	async fn read(
		&self,
		id: &AppId,
		path: &FsPath,
	) -> Result<BoxStream<'static, Result<Bytes>>> {
		self.inner.fs().read(id, path).await
	}

	async fn download(&self, id: &AppId, path: &FsPath) -> Result<FsDownload> {
		self.inner.fs().download(id, path).await
	}

	async fn write(
		&self,
		id: &AppId,
		req: &FsWriteReq,
		stream: BoxStream<'static, Result<Bytes>>,
	) -> Result<()> {
		self.inner.fs().write(id, req, stream).await
	}

	async fn mkdir(&self, id: &AppId, path: &FsPath) -> Result<()> {
		self.inner.fs().mkdir(id, path).await
	}

	async fn delete(&self, id: &AppId, path: &FsPath) -> Result<()> {
		self.inner.fs().delete(id, path).await
	}

	async fn rename(&self, id: &AppId, req: &FsMoveReq) -> Result<()> {
		self.inner.fs().rename(id, req).await
	}

	async fn copy(&self, id: &AppId, req: &FsMoveReq) -> Result<()> {
		self.inner.fs().copy(id, req).await
	}

	async fn permissions(
		&self,
		id: &AppId,
		req: &FsPermissionsReq,
	) -> Result<()> {
		self.inner.fs().permissions(id, req).await
	}

	async fn zip(&self, id: &AppId, req: &FsZipReq) -> Result<()> {
		self.inner.fs().zip(id, req).await
	}

	async fn extract(&self, id: &AppId, req: &FsExtractReq) -> Result<()> {
		self.inner.fs().extract(id, req).await
	}
}
