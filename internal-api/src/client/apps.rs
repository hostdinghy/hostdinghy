use crate::{
	app_id::AppId,
	apps::{AppInfoRes, ComposeCommand, GetComposeRes, SaveComposeReq},
	client::{ApiServerClient, Result},
	error::WithMessage,
};

#[derive(Debug, Clone)]
pub struct ApiServerAppsClient<'a> {
	inner: &'a ApiServerClient,
}

impl<'a> ApiServerAppsClient<'a> {
	pub(crate) fn new(inner: &'a ApiServerClient) -> Self {
		Self { inner }
	}

	pub async fn app_info(&self, id: &AppId) -> Result<AppInfoRes> {
		self.inner
			.send_json(self.inner.get(&format!("/apps/{id}")))
			.await
	}

	pub async fn get_compose(&self, id: &AppId) -> Result<GetComposeRes> {
		self.inner
			.send_json(self.inner.get(&format!("/apps/{id}/compose")))
			.await
	}

	pub async fn set_compose(
		&self,
		id: &AppId,
		req: &SaveComposeReq,
	) -> Result<()> {
		self.inner
			.send(self.inner.post(&format!("/apps/{id}/compose")).json(req))
			.await
			.map(|_| ())
	}

	pub async fn compose_command(
		&self,
		id: &AppId,
		cmd: &ComposeCommand,
	) -> Result<()> {
		self.inner
			.send(self.inner.post(&format!("/apps/{id}/action/{cmd}")))
			.await
			.map(|_| ())
	}

	pub async fn compose_service_command(
		&self,
		id: &AppId,
		service: &str,
		cmd: &ComposeCommand,
	) -> Result<()> {
		self.inner
			.send(
				self.inner.post(&format!(
					"/apps/{id}/service/{service}/action/{cmd}"
				)),
			)
			.await
			.map(|_| ())
	}

	pub async fn app_logs(
		&self,
		id: &AppId,
		lines: Option<u32>,
	) -> Result<String> {
		let mut uri = format!("/apps/{id}/logs");
		if let Some(lines) = lines {
			// todo could this be a query type?
			uri.push_str(&format!("?lines={lines}"));
		}

		let response = self.inner.send(self.inner.get(&uri)).await?;

		response
			.text()
			.await
			.with_message("failed to parse logs response")
	}
}
