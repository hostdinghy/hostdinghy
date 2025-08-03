use std::sync::Arc;

use api::error::Error;
use axum::{Json, Router, extract::State, routing::get};
use hyper::{HeaderMap, header::AUTHORIZATION};
use serde::{Deserialize, Serialize};
use subtle::ConstantTimeEq;
use tracing::info;

use crate::{config::Config, server::router::AppState};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Events {
	events: Vec<Event>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EventAction {
	Push,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
	id: String,
	action: EventAction,
	target: PushTarget,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PushTarget {
	digest: String,
	repository: String,
	url: String,
	#[serde(default)]
	tag: String,
}

async fn webhook(
	State(cfg): State<Arc<Config>>,
	headers: HeaderMap,
	Json(req): Json<Events>,
) -> Result<(), Error> {
	let token = headers
		.get(AUTHORIZATION)
		.and_then(|v| v.to_str().ok())
		.and_then(|s| s.strip_prefix("Bearer "))
		.ok_or(Error::MissingApiToken)?;

	// this is probably not necessary but why not
	let choice = cfg.registry.webhook_token.as_ref().ct_eq(token.as_ref());
	if !bool::from(choice) {
		return Err(Error::InvalidApiToken);
	}

	// https://distribution.github.io/distribution/about/notifications/
	info!("Received registry webhook: {:?}", req.events);

	Ok(())
}

pub fn routes() -> Router<AppState> {
	Router::new().route("/webhook", get(webhook))
}
