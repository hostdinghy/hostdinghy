use std::{
	collections::{HashMap, HashSet},
	path::PathBuf,
	sync::{Arc, Mutex},
};

use api::{
	Error,
	apps::AppId,
	registry::{CreateUserReq, CreateUserRes, RegistryUsersRes},
};
use axum::{
	Json, Router,
	extract::{Path, State},
	routing::{delete, get, post},
};
use hyper::{HeaderMap, header::AUTHORIZATION};
use serde::{Deserialize, Serialize};
use subtle::ConstantTimeEq;
use tracing::{error, info, warn};

use crate::{
	config::Config,
	registry::{
		AddUser, RemoveUser, WebhookToken, add_user, list_users, remove_user,
	},
	server::{Authenticated, router::AppState},
	utils::{compose, hostdinghy_dir, is_file},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Events {
	events: Vec<Event>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EventAction {
	Push,
	#[serde(untagged)]
	Other(String),
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

/// Redeploys apps after a push, in the background.
///
/// The registry gives up on a webhook after about a second and retries it,
/// a `compose up` takes longer. So the webhook only triggers a redeploy and
/// answers right away. There is only one redeploy per app at a time, a push
/// during a redeploy runs it once more afterwards.
#[derive(Debug, Clone, Default)]
pub struct Redeploys {
	/// the apps being redeployed, true if another run is pending
	running: Arc<Mutex<HashMap<AppId, bool>>>,
}

impl Redeploys {
	pub fn trigger(&self, app: AppId, compose_file: PathBuf) {
		{
			let mut running = self.running.lock().unwrap();
			if let Some(pending) = running.get_mut(&app) {
				*pending = true;
				return;
			}
			running.insert(app.clone(), false);
		}

		let this = self.clone();
		tokio::spawn(async move {
			loop {
				info!("Redeploying app {app} after a push");
				match compose::up(&compose_file, None).await {
					Ok(()) => info!("Redeployed app {app}"),
					Err(e) => error!("Failed to redeploy app {app}: {e}"),
				}

				let mut running = this.running.lock().unwrap();
				if running.get(&app) == Some(&true) {
					running.insert(app.clone(), false);
					continue;
				}
				running.remove(&app);
				break;
			}
		});
	}
}

async fn webhook(
	State(cfg): State<Arc<Config>>,
	State(redeploys): State<Redeploys>,
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

		redeploys.trigger(app, compose_file);
	}

	Ok(())
}

async fn all_users(
	_auth: Authenticated,
) -> Result<Json<RegistryUsersRes>, Error> {
	list_users()
		.await
		.map(RegistryUsersRes)
		.map(Json)
		.map_err(Into::into)
}

async fn create_user(
	_auth: Authenticated,
	Json(req): Json<CreateUserReq>,
) -> Result<Json<CreateUserRes>, Error> {
	let mut au = AddUser {
		username: req.username.into(),
		password: None,
	};
	add_user(&mut au).await?;

	Ok(Json(CreateUserRes {
		username: au.username,
		password: au.password.unwrap(),
	}))
}

async fn delete_user(
	_auth: Authenticated,
	Path(username): Path<String>,
) -> Result<Json<()>, Error> {
	remove_user(RemoveUser { username })
		.await
		.map(Json)
		.map_err(Into::into)
}

pub fn routes() -> Router<AppState> {
	Router::new()
		.route("/webhook", post(webhook))
		.route("/users", get(all_users).post(create_user))
		.route("/users/{username}", delete(delete_user))
}
