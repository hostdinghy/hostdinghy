use std::{collections::HashSet, sync::Arc};

use api::{app_id::AppId, error::Error};
use axum::{Router, extract::State, routing::post};
use hyper::{HeaderMap, header::AUTHORIZATION};
use serde::{Deserialize, Serialize};
use subtle::ConstantTimeEq;
use tracing::{error, info, warn};

use crate::{
	config::Config,
	registry::WebhookToken,
	server::router::AppState,
	utils::{compose, hostdinghy_dir, is_file},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Events {
	events: Vec<Event>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EventAction {
	/// It seems that even if specified we get a pull as well
	Pull,
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
	body: String,
) -> Result<(), Error> {
	let token: WebhookToken = headers
		.get(AUTHORIZATION)
		.and_then(|v| v.to_str().ok())
		.and_then(|s| s.strip_prefix("Bearer "))
		.and_then(|s| s.parse().ok())
		.ok_or(Error::MissingApiToken)?;

	// this is probably not necessary but why not
	let choice = cfg.registry.webhook_token.as_ref().ct_eq(token.as_ref());
	if !bool::from(choice) {
		return Err(Error::InvalidApiToken);
	}

	let events: Events = match serde_json::from_str(&body) {
		Ok(events) => events,
		Err(e) => {
			warn!(
				"Failed to parse registry webhook body:\n{body}\n with error {e}"
			);
			return Ok(());
		}
	};

	// https://distribution.github.io/distribution/about/notifications/
	if events
		.events
		.iter()
		.any(|e| matches!(e.action, EventAction::Push))
	{
		// only log if we have a push event
		info!("Received registry webhook:\n{:?}", events.events);
	}

	let apps_involved: HashSet<AppId> = events
		.events
		.into_iter()
		// we only care about push events
		.filter(|e| matches!(e.action, EventAction::Push))
		// we ignore tag that are empty because this means it is a layer and not
		// the completed image
		.filter(|e| !e.target.tag.is_empty())
		.filter_map(|e| {
			// lets try to parse target repository
			let repo = e.target.repository;

			let Some((app_id, _service)) =
				repo.split_once('/').and_then(|(app_id, service)| {
					app_id.parse::<AppId>().ok().map(|a| (a, service))
				})
			else {
				warn!("unknown repository {repo}");
				return None;
			};

			Some(app_id)
		})
		.collect();

	info!("Apps involved in the webhook: {:?}", apps_involved);

	let hostdinghy_dir = hostdinghy_dir()?;
	for app in apps_involved {
		let app_dir = hostdinghy_dir.join(app.to_string());
		let compose_file = app_dir.join("compose.yml");
		if !is_file(&compose_file).await {
			warn!("No compose file found for app {app}");
			continue;
		}

		if let Err(e) = compose::up(compose_file).await {
			error!("Failed to start app {app} after pull {e}");
			continue;
		}
	}

	Ok(())
}

pub fn routes() -> Router<AppState> {
	Router::new().route("/webhook", post(webhook))
}
