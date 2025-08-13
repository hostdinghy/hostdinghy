use std::sync::Arc;

use api::{
	error::Error,
	requests::{InfoRes, PingRes},
};
use axum::{
	Json, Router,
	extract::{FromRef, State},
	routing::get,
};
use chuchi_postgres::time::DateTime;
use tower_http::trace::TraceLayer;

use crate::{
	apps,
	docker::Docker,
	registry,
	server::{Config, utils::Authenticated},
	traefik::client::Traefik,
};

#[derive(Clone)]
pub struct AppState {
	pub docker: Docker,
	pub traefik: Traefik,
	pub cfg: Arc<Config>,
}

impl FromRef<AppState> for Docker {
	fn from_ref(state: &AppState) -> Self {
		state.docker.clone()
	}
}

impl FromRef<AppState> for Traefik {
	fn from_ref(state: &AppState) -> Self {
		state.traefik.clone()
	}
}

impl FromRef<AppState> for Arc<Config> {
	fn from_ref(state: &AppState) -> Self {
		state.cfg.clone()
	}
}

pub async fn app(cfg: Config) -> Result<Router<()>, Error> {
	let state = AppState {
		docker: Docker::new()?,
		traefik: Traefik::new(cfg.traefik.clone()),
		cfg: Arc::new(cfg),
	};

	let router = Router::new()
		.route("/ping", get(ping_req))
		.route("/info", get(info_req))
		.nest("/apps", apps::routes::routes())
		.nest("/registry", registry::routes::routes())
		.layer(TraceLayer::new_for_http())
		.with_state(state);

	Ok(router)
}

async fn ping_req() -> Json<PingRes> {
	Json(PingRes {
		time: DateTime::now(),
	})
}

async fn info_req(
	_auth: Authenticated,
	State(cfg): State<Arc<Config>>,
) -> Json<InfoRes> {
	Json(InfoRes {
		registry_domain: cfg.registry.domain.clone(),
		version: env!("CARGO_PKG_VERSION").parse().unwrap(),
		commit: option_env!("GIT_COMMIT_HASH").map(|s| s.to_string()),
		build_date: option_env!("BUILD_DATE")
			.and_then(|s| DateTime::parse_from_iso8601(s).ok()),
	})
}
