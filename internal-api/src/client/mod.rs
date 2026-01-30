mod apps;
mod postgres;
mod registry;

use http::Method;
use reqwest::{Certificate, RequestBuilder, Response};
use serde::de::DeserializeOwned;

pub use apps::ApiServerAppsClient;

use crate::{
	client::{
		postgres::ApiServerPostgresClient, registry::ApiServerRegistryClient,
	},
	error::{Error, WithMessage},
	requests::{ApiToken, InfoRes, PingRes},
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

	pub(crate) fn get(&self, uri: &str) -> RequestBuilder {
		self.request(Method::GET, uri)
	}

	#[allow(dead_code)]
	pub(crate) fn post(&self, uri: &str) -> RequestBuilder {
		self.request(Method::POST, uri)
	}

	#[allow(dead_code)]
	pub(crate) fn put(&self, uri: &str) -> RequestBuilder {
		self.request(Method::PUT, uri)
	}

	#[allow(dead_code)]
	pub(crate) fn delete(&self, uri: &str) -> RequestBuilder {
		self.request(Method::DELETE, uri)
	}

	// todo should probably improve the errors
	pub(crate) async fn send(&self, req: RequestBuilder) -> Result<Response> {
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
	pub(crate) async fn send_json<Res>(
		&self,
		req: RequestBuilder,
	) -> Result<Res>
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

	pub async fn info(&self) -> Result<InfoRes> {
		self.send_json(self.get("/info")).await
	}

	pub fn apps(&self) -> ApiServerAppsClient<'_> {
		ApiServerAppsClient::new(&self)
	}

	pub fn registry(&self) -> ApiServerRegistryClient<'_> {
		ApiServerRegistryClient::new(&self)
	}

	pub fn postgres(&self) -> ApiServerPostgresClient<'_> {
		ApiServerPostgresClient::new(&self)
	}
}
