pub mod error;

pub use error::ComposeError;

use std::{
	collections::{HashMap, HashSet},
	convert::Infallible,
	str::FromStr,
	sync::LazyLock,
};

use regex::Regex;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Compose {
	pub services: HashMap<String, ComposeService>,
}

impl Compose {
	pub fn validate_for(
		&self,
		registry: &str,
		app_id: &str,
	) -> Result<(), ComposeError> {
		for (name, service) in &self.services {
			// validate the the image is correct
			let image = service.parse_image();
			image.validate_for(registry, app_id, name)?;

			// make sure all traefik router names are correct
			let valid_route = format!("{app_id}-{name}");
			let mut parsed_names = service.traefik_router_names();
			// remove the correct one so we have only unexpected ones left
			parsed_names.remove(&valid_route);
			if !parsed_names.is_empty() {
				return Err(ComposeError::UnexpectedTraefikRouterName {
					unexpected: parsed_names.into_iter().collect(),
					expected: valid_route,
				}
				.into());
			}
		}

		Ok(())
	}
}

impl FromStr for Compose {
	type Err = ComposeError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		serde_yaml::from_str(s).map_err(Into::into)
	}
}

#[derive(Debug, Clone, Deserialize)]
pub struct ComposeService {
	pub image: String,
	pub container_name: Option<String>,
	#[serde(default)]
	pub labels: Vec<String>,
}

static TRAEFIK_ROUTER_NAMES: LazyLock<Regex> = LazyLock::new(|| {
	Regex::new(r"traefik\.http\.(?:routers|services)\.([^.]+)\.").unwrap()
});

impl ComposeService {
	pub fn parse_image(&self) -> ComposeImage {
		self.image.parse().unwrap()
	}

	pub fn traefik_router_names(&self) -> HashSet<String> {
		self.labels
			.iter()
			.filter_map(|label| {
				TRAEFIK_ROUTER_NAMES
					.captures(label)
					.map(|c| c[1].to_string())
			})
			.collect()
	}
}

const VALID_IMAGE_REGEX: LazyLock<Regex> =
	LazyLock::new(|| Regex::new(r"^(.*?\..*?)/([^/]+)/([^/]+)$").unwrap());

#[derive(Debug, Clone)]
pub enum ComposeImage {
	// ex: registry.example.com/appid/service:tag
	Valid {
		/// `registry/app_id/service
		image: String,
		registry: String,
		app_id: String,
		service: String,
		tag: Option<String>,
	},
	Unknown {
		image: String,
		tag: Option<String>,
	},
}

impl ComposeImage {
	pub fn image(&self) -> &str {
		match self {
			Self::Valid { image, .. } => image,
			Self::Unknown { image, .. } => image,
		}
	}

	pub fn tag(&self) -> Option<&str> {
		match self {
			Self::Valid { tag, .. } => tag.as_deref(),
			Self::Unknown { tag, .. } => tag.as_deref(),
		}
	}

	pub fn validate_for(
		&self,
		registry: &str,
		app_id: &str,
		service: &str,
	) -> Result<(), ComposeError> {
		match self {
			ComposeImage::Valid {
				image,
				registry: reg,
				app_id: id,
				service: name,
				..
			} if reg == registry => {
				if id != app_id || name != service {
					return Err(ComposeError::InvalidImage {
						image: image.clone(),
						expected: format!("{registry}/{app_id}/{service}"),
					}
					.into());
				}
			}
			ComposeImage::Unknown { image, .. }
				if image.starts_with(registry) =>
			{
				return Err(ComposeError::InvalidImage {
					image: image.clone(),
					expected: format!("{registry}/{app_id}/{service}"),
				});
			}
			_ => {}
		}

		Ok(())
	}
}

impl FromStr for ComposeImage {
	type Err = Infallible;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		// lets first split by last colon
		let (image, tag) = s
			.rsplit_once(':')
			.map_or((s, None), |(i, t)| (i, Some(t.to_string())));

		if let Some(caps) = VALID_IMAGE_REGEX.captures(image) {
			Ok(Self::Valid {
				image: image.to_string(),
				registry: caps[1].to_string(),
				app_id: caps[2].to_string(),
				service: caps[3].to_string(),
				tag,
			})
		} else {
			Ok(Self::Unknown {
				image: image.to_string(),
				tag,
			})
		}
	}
}
