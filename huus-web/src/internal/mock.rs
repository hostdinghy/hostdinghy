use crate::{internal::ApiServerClientTrait, servers::data::Server};

use internal_api::{
	client::Result,
	requests::{PingRes, VersionRes},
};
use pg::time::DateTime;

pub struct ApiServerClient {}

impl ApiServerClient {
	pub fn new(_server: &Server) -> Self {
		Self {}
	}
}

#[async_trait::async_trait]
impl ApiServerClientTrait for ApiServerClient {
	async fn ping(&self) -> Result<PingRes> {
		Ok(PingRes {
			time: DateTime::now(),
		})
	}

	async fn version(&self) -> Result<VersionRes> {
		Ok(VersionRes {
			version: "0.0.0-debug.0".parse().unwrap(),
			commit: None,
			build_date: None,
		})
	}
}
