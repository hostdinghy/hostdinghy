use api::{
	error::{Error, WithMessage},
	requests::{AppInfoRes, AppService, ServiceState},
};
use axum::{
	Json, Router,
	extract::{Path, State},
	routing::get,
};

use crate::{
	apps::utils::{
		cont_sum_state_enum_to_service_state, container_names_to_service_name,
		traefik_route_to_service_route,
	},
	docker::Docker,
	server::{Authenticated, router::AppState},
	traefik::client::Traefik,
};

async fn app_info(
	_auth: Authenticated,
	State(docker): State<Docker>,
	State(traefik): State<Traefik>,
	Path(id): Path<String>,
) -> Result<Json<AppInfoRes>, Error> {
	// search for all containers and find the ones that are tagged
	// with the given composer id
	let services = docker
		.containers_by_composer_project(&id)
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

pub fn routes() -> Router<AppState> {
	Router::new().route("/{id}", get(app_info))
}
