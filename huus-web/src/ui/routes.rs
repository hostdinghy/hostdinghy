use std::path::Path;
use std::sync::Arc;

use aho_corasick::AhoCorasick;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::response::{Html, IntoResponse as _, Response};
use axum::routing::get;
use axum::{Router, extract::State};
use tokio::fs;
use tower::ServiceExt as _;
use tower_http::services::ServeDir;

use crate::ui::DistDir;
use crate::{AppState, Config};

async fn index(
	State(cfg): State<Arc<Config>>,
	State(dist_dir): State<DistDir>,
	req: Request<Body>,
) -> Response {
	let resp = ServeDir::new(dist_dir.0.clone())
		.oneshot(req)
		.await
		.unwrap();
	if resp.status() != StatusCode::NOT_FOUND {
		return resp.map(Body::new);
	}

	let path = Path::new(&dist_dir.0).join("index.html");

	let Ok(html) = fs::read_to_string(path).await else {
		return StatusCode::NOT_FOUND.into_response();
	};

	// replace multiple values
	let ac = AhoCorasick::new(&[
		"window.ENVIRONMENT = 'debug';",
		"window.VERSION = 'debug';",
		"window.API_ADDR = 'http://localhost:3000/';",
	])
	.unwrap();

	let html = ac.replace_all(
		&html,
		&[
			&*format!("window.ENVIRONMENT = '{}';", cfg.environment),
			&*format!("window.VERSION = '{}';", env!("CARGO_PKG_VERSION")),
			"window.API_ADDR = '/';",
		],
	);

	Html(html).into_response()
}

pub fn routes() -> Router<AppState> {
	Router::new()
		.route("/", get(index))
		.route("/{*rest}", get(index))
}
