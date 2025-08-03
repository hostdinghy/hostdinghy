use http::Method;
use reqwest::{Certificate, RequestBuilder, Response};
use serde::de::DeserializeOwned;

use crate::{
	app_id::AppId,
	apps::{AppInfoRes, ComposeCommand, GetComposeRes, SaveComposeReq},
	error::{Error, WithMessage},
	requests::{ApiToken, PingRes, VersionRes},
};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub struct ApiClient {
	// todo at some point we might want to build a ApiServerClient Pool
	// to reuse connections
	// inner: reqwest::Client,
}

impl ApiClient {
	pub fn new() -> Self {
		Self {
			// inner: reqwest::Client::new(),
		}
	}

	pub fn connect(
		&self,
		domain: impl Into<String>,
		cert: &str,
		token: ApiToken,
	) -> Result<ApiServerClient> {
		let addr = format!("https://{}:4242", domain.into());
		let cert = Certificate::from_pem(cert.as_bytes())
			.map_err(|_| Error::InvalidCertificate)?;

		let inner = reqwest::Client::builder()
			.add_root_certificate(cert)
			.build()
			.with_message("Failed to build reqwest client")?;

		Ok(ApiServerClient { inner, addr, token })
	}
}

#[derive(Debug, Clone)]
pub struct ApiServerClient {
	inner: reqwest::Client,
	addr: String,
	token: ApiToken,
}

impl ApiServerClient {
	fn request(&self, method: Method, uri: &str) -> RequestBuilder {
		self.inner
			.request(method, format!("{}{}", self.addr, uri))
			.bearer_auth(&self.token)
	}

	fn get(&self, uri: &str) -> RequestBuilder {
		self.request(Method::GET, uri)
	}

	#[allow(dead_code)]
	fn post(&self, uri: &str) -> RequestBuilder {
		self.request(Method::POST, uri)
	}

	#[allow(dead_code)]
	fn put(&self, uri: &str) -> RequestBuilder {
		self.request(Method::PUT, uri)
	}

	#[allow(dead_code)]
	fn delete(&self, uri: &str) -> RequestBuilder {
		self.request(Method::DELETE, uri)
	}

	// todo should probably improve the errors
	async fn send(&self, req: RequestBuilder) -> Result<Response> {
		let response =
			req.send().await.with_message("failed to send request")?;

		if response.status().is_success() {
			Ok(response)
		} else {
			Err(response
				.json()
				.await
				.with_message("failed to parse error response")?)
		}
	}

	// todo should probably improve the errors
	async fn send_json<Res>(&self, req: RequestBuilder) -> Result<Res>
	where
		Res: DeserializeOwned,
	{
		let response = self.send(req).await?;

		response
			.json()
			.await
			.with_message("failed to parse response")
	}

	pub async fn ping(&self) -> Result<PingRes> {
		self.send_json(self.get("/ping")).await
	}

	pub async fn version(&self) -> Result<VersionRes> {
		self.send_json(self.get("/version")).await
	}

	pub async fn app_info(&self, id: &AppId) -> Result<AppInfoRes> {
		self.send_json(self.get(&format!("/apps/{id}"))).await
	}

	pub async fn app_get_compose(&self, id: &AppId) -> Result<GetComposeRes> {
		self.send_json(self.get(&format!("/apps/{id}/compose")))
			.await
	}

	pub async fn app_set_compose(
		&self,
		id: &AppId,
		req: &SaveComposeReq,
	) -> Result<()> {
		self.send(self.post(&format!("/apps/{id}/compose")).json(req))
			.await
			.map(|_| ())
	}

	pub async fn app_compose_command(
		&self,
		id: &AppId,
		cmd: &ComposeCommand,
	) -> Result<()> {
		self.send(self.post(&format!("/apps/{id}/action/{cmd}")))
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

		let response = self.send(self.get(&uri)).await?;

		response
			.text()
			.await
			.with_message("failed to parse logs response")
	}
}
