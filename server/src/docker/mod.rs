use bollard::{
	query_parameters::ListContainersOptionsBuilder,
	secret::{ContainerSummary, NetworkCreateRequest, NetworkCreateResponse},
};

use crate::utils::cli::{CliError, WithMessage as _};

#[derive(Debug, Clone)]
pub struct Docker {
	inner: bollard::Docker,
}

impl Docker {
	pub fn new() -> Result<Self, CliError> {
		Ok(Self {
			inner: bollard::Docker::connect_with_local_defaults()
				.with_message("Failed to connect to Docker")?,
		})
	}

	pub async fn create_network(
		&self,
		req: NetworkCreateRequest,
	) -> Result<NetworkCreateResponse, CliError> {
		let name = req.name.clone();
		self.inner
			.create_network(req)
			.await
			.with_message(format!("Failed to create Docker \"{name}\" network"))
	}

	pub async fn containers_by_composer_project(
		&self,
		id: &str,
	) -> Result<Vec<ContainerSummary>, CliError> {
		self.inner
			.list_containers(Some(
				ListContainersOptionsBuilder::new()
					.all(true)
					.filters(
						&[(
							"label",
							vec![format!("com.docker.compose.project={id}")],
						)]
						.into(),
					)
					.build(),
			))
			.await
			.with_message(format!(
				"Failed to list Docker services for composer ID: {id}"
			))
	}
}
