use http::Method;
use reqwest::{Certificate, RequestBuilder};
use serde::de::DeserializeOwned;

use crate::{
	error::{Error, WithMessage},
	requests::{ApiToken, AppInfoRes, PingRes, VersionRes},
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
		addr: impl Into<String>,
		cert: &str,
		token: ApiToken,
	) -> Result<ApiServerClient> {
		let addr = addr.into();
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
	async fn send<Res>(&self, req: RequestBuilder) -> Result<Res>
	where
		Res: DeserializeOwned,
	{
		let response =
			req.send().await.with_message("failed to send request")?;

		if response.status().is_success() {
			response
				.json()
				.await
				.with_message("failed to parse response")
		} else {
			Err(response
				.json()
				.await
				.with_message("failed to parse error response")?)
		}
	}

	pub async fn ping(&self) -> Result<PingRes> {
		self.send(self.get("/ping")).await
	}

	pub async fn version(&self) -> Result<VersionRes> {
		self.send(self.get("/version")).await
	}

	pub async fn app_info(&self, id: &str) -> Result<AppInfoRes> {
		self.send(self.get(&format!("/apps/{id}"))).await
	}
}
