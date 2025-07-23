use std::{path::Path, sync::Arc};

use api::error::{Error, WithMessage as _};
use hyper::Method;
use reqwest::RequestBuilder;
use serde::de::DeserializeOwned;

use crate::{
	traefik::{TraefikConfig, api::TraefikRoute},
	utils::cli::CliError,
};

#[derive(Debug, Clone)]
pub struct Traefik {
	pub inner: reqwest::Client,
	pub cfg: Arc<TraefikConfig>,
}

impl Traefik {
	pub async fn new(huus_dir: impl AsRef<Path>) -> Result<Self, CliError> {
		Ok(Self {
			inner: reqwest::Client::new(),
			cfg: Arc::new(TraefikConfig::read(huus_dir).await?),
		})
	}

	fn request(&self, method: Method, uri: &str) -> RequestBuilder {
		self.inner
			.request(method, format!("http://127.0.0.1:8080{}", uri))
			.basic_auth("huus", Some(self.cfg.api_token.to_string()))
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
	async fn send<Res>(&self, req: RequestBuilder) -> Result<Res, Error>
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
			let text = response
				.text()
				.await
				.with_message("failed to read error response")?;

			Err(Error::any("traefik request failed returned", text))
		}
	}

	// /// for example: crelte-tut-2-craft@docker
	// pub async fn service_by_name(
	// 	&self,
	// 	name: &str,
	// ) -> Result<TraefikService, Error> {
	// 	self.send(self.get(&format!("/api/services/{}", name)))
	// }

	pub async fn routers_by_service(
		&self,
		service_name: &str,
	) -> Result<Vec<TraefikRoute>, Error> {
		self.send(
			self.get(&format!("/api/http/routers?serviceName={service_name}",)),
		)
		.await
	}
}
