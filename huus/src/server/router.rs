use std::sync::Arc;

use api::{PingRes, VersionRes, error::Error};
use axum::{Json, Router, extract::FromRef, routing::get};
use chuchi_postgres::time::DateTime;

use crate::server::{Config, utils::Authenticated};

#[derive(Clone)]
pub struct AppState {
	pub cfg: Arc<Config>,
}

impl FromRef<AppState> for Arc<Config> {
	fn from_ref(state: &AppState) -> Self {
		state.cfg.clone()
	}
}

pub async fn app(cfg: Config) -> Result<Router<()>, Error> {
	let state = AppState { cfg: Arc::new(cfg) };

	let router = Router::new()
		.route("/ping", get(ping_req))
		.route("/version", get(version_req))
		.with_state(state);

	Ok(router)
}

async fn ping_req() -> Json<PingRes> {
	Json(PingRes {
		time: DateTime::now(),
	})
}

async fn version_req(_auth: Authenticated) -> Json<VersionRes> {
	Json(VersionRes {
		version: env!("CARGO_PKG_VERSION").parse().unwrap(),
		commit: option_env!("GIT_COMMIT_HASH").map(|s| s.to_string()),
		build_date: option_env!("BUILD_DATE")
			.and_then(|s| DateTime::parse_from_iso8601(s).ok()),
	})
}
