use std::{io::ErrorKind, sync::Arc};

use api::{
	app_id::AppId,
	apps::{
		AppInfoRes, AppService, ComposeCommand, GetComposeRes, SaveComposeReq,
		ServiceState,
	},
	error::{Error, WithMessage},
};
use axum::{
	Json, Router,
	extract::{Path, Query, State},
	routing::{get, post},
};
use chuchi_crypto::hash::Hasher;
use serde::{Deserialize, Serialize};
use tokio::fs;

use crate::{
	apps::utils::{
		cont_sum_state_enum_to_service_state, container_names_to_service_name,
		traefik_route_to_service_route,
	},
	docker::Docker,
	postgresql::Client,
	server::{Authenticated, Config, config::SecretToken, router::AppState},
	traefik::client::Traefik,
	utils::{cmd::CmdError, compose, huus_dir, is_dir, is_file},
};

async fn app_info(
	_auth: Authenticated,
	State(docker): State<Docker>,
	State(traefik): State<Traefik>,
	Path(id): Path<AppId>,
) -> Result<Json<AppInfoRes>, Error> {
	// let's first check if the id exists
	// we do this by checking if the folder exists
	let path = huus_dir()?.join(id.as_ref());
	if !is_dir(&path).await {
		return Err(Error::AppNotFound);
	}

	// search for all containers and find the ones that are tagged
	// with the given composer id
	let services = docker
		.containers_by_composer_project(id.as_ref())
		.await
		.with_message("Failed to list Docker services")?;

	let mut services = services
		.into_iter()
		.map(|s| AppService {
			name: container_names_to_service_name(&s.names).unwrap_or_default(),
			state: s
				.state
				.map(cont_sum_state_enum_to_service_state)
				.unwrap_or(ServiceState::Unknown),
			state_hr: s.status.unwrap_or_default(),
			routes: vec![],
		})
		.collect::<Vec<_>>();

	/*
	 * So for each compose.yaml file there is a rule how to name routers
	 * each router needs to start with <compose-project-name>-<service-name>
		* and the traefik service needs to be called <compose-project-name>-<service-name>
	 */

	// todo parallelize
	for service in &mut services {
		let service_name = format!("{id}-{}", &service.name);
		let routers = traefik.routers_by_service(&service_name).await?;

		service.routes = routers
			.into_iter()
			.map(traefik_route_to_service_route)
			.collect::<Result<_, _>>()?;
	}

	Ok(Json(AppInfoRes { services }))
}

async fn get_compose(
	_auth: Authenticated,
	Path(id): Path<AppId>,
) -> Result<Json<GetComposeRes>, Error> {
	// we need to check if the folder exists
	// and if a compose.yml file exists
	//
	// also we need to check if a db exists with the AppId name

	// let's first check if the folder exists
	let app_dir = huus_dir()?.join(id.as_ref());
	if !is_dir(&app_dir).await {
		return Err(Error::AppNotFound);
	}

	// now lets check if we have a compose file
	let compose_path = app_dir.join("compose.yml");
	if is_file(&compose_path).await {
		return Err(Error::AppNotFound);
	}

	let compose =
		fs::read_to_string(&compose_path)
			.await
			.with_message(format!(
				"Failed to read compose {} file",
				compose_path.display()
			))?;

	// lets check the db
	let client = Client::new().await?;
	let database_exists = client.database_exists(id.as_ref()).await?;

	Ok(Json(GetComposeRes {
		compose,
		database: database_exists,
	}))
}

fn generate_db_password(id: &AppId, secret: &SecretToken) -> String {
	let mut hasher = Hasher::new();
	hasher.update(id.as_ref());
	hasher.update(secret.as_ref());
	hasher.finalize().to_string()
}

async fn save_compose(
	_auth: Authenticated,
	State(config): State<Arc<Config>>,
	Path(id): Path<AppId>,
	Json(req): Json<SaveComposeReq>,
) -> Result<(), Error> {
	let mut db_password: Option<String> = None;

	if req.create_database {
		let password = generate_db_password(&id, &config.secret);

		let client = Client::new().await?;
		// lets first check if the database exists
		let database_exists = client.database_exists(id.as_ref()).await?;
		if !database_exists {
			// if the database does not exist we assume the user was not
			// created password is also not set
			client.create_user(id.as_ref(), &password).await?;
			client.create_database(id.as_ref(), id.as_ref()).await?;
		}

		db_password = Some(password);
	}

	let app_dir = huus_dir()?.join(id.as_ref());
	match fs::create_dir(&app_dir).await {
		Ok(()) => {}
		Err(e) if e.kind() == ErrorKind::AlreadyExists => {}
		Err(e) => {
			return Err(Error::any("Failed to create app directory", e));
		}
	}

	let compose = match db_password {
		Some(password) => req.compose.replace("{DB_PASSWORD}", &password),
		None => req.compose,
	};

	// let's write the file
	let compose_path = app_dir.join("compose.yml");
	fs::write(&compose_path, compose)
		.await
		.with_message(format!(
			"Failed to write compose file to {}",
			compose_path.display()
		))?;

	Ok(())
}

async fn compose_action(
	_auth: Authenticated,
	Path((id, command)): Path<(AppId, ComposeCommand)>,
) -> Result<(), Error> {
	let app_dir = huus_dir()?.join(id.as_ref());
	if !is_dir(&app_dir).await {
		return Err(Error::AppNotFound);
	}

	let compose_path = app_dir.join("compose.yml");
	if !is_file(&compose_path).await {
		return Err(Error::AppNotFound);
	}

	match command {
		ComposeCommand::Start => compose::start(&app_dir).await,
		ComposeCommand::Up => compose::up(&app_dir).await,
		ComposeCommand::Restart => compose::restart(&app_dir).await,
		ComposeCommand::Stop => compose::stop(&app_dir).await,
	}?;

	Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogsQueryReq {
	lines: Option<u32>,
}

async fn logs(
	_auth: Authenticated,
	Path(id): Path<AppId>,
	Query(req): Query<LogsQueryReq>,
) -> Result<String, Error> {
	let app_dir = huus_dir()?.join(id.as_ref());
	if !is_dir(&app_dir).await {
		return Err(Error::AppNotFound);
	}

	let compose_path = app_dir.join("compose.yml");
	if !is_file(&compose_path).await {
		return Err(Error::AppNotFound);
	}

	compose::logs(&compose_path, req.lines)
		.await
		.map_err(Into::into)
}

pub fn routes() -> Router<AppState> {
	Router::new()
		.route("/{id}", get(app_info))
		.route("/{id}/compose", get(get_compose).post(save_compose))
		.route("/{id}/action/{cmd}", post(compose_action))
		.route("/{id}/logs", get(logs))
}
