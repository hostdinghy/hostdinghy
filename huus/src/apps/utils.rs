use api::{
	error::Error,
	requests::{ServiceRoute, ServiceState},
};
use bollard::secret::ContainerSummaryStateEnum;

use crate::traefik::api::TraefikRoute;

pub fn cont_sum_state_enum_to_service_state(
	en: ContainerSummaryStateEnum,
) -> ServiceState {
	use ContainerSummaryStateEnum::*;
	use ServiceState::*;

	match en {
		EMPTY => Empty,
		CREATED => Created,
		RUNNING => Running,
		PAUSED => Paused,
		RESTARTING => Restarting,
		EXITED => Exited,
		REMOVING => Removing,
		DEAD => Dead,
	}
}

pub fn container_names_to_service_name(
	names: &Option<Vec<String>>,
) -> Option<String> {
	// get the first name that starts with a slash
	names
		.as_ref()?
		.iter()
		.filter_map(|n| n.strip_prefix("/"))
		.next()
		.map(Into::into)
}

pub fn traefik_route_to_service_route(
	route: TraefikRoute,
) -> Result<ServiceRoute, Error> {
	Ok(ServiceRoute {
		rule: route.rule,
		// todo parse hosts
		domains: vec![],
	})
}
