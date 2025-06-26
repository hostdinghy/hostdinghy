#[derive(Debug, Clone)]
pub struct App {
	// an ascii string without any spaces unique for every app
	// it is also the folder where everything is stored
	// in $HUUS_DIR
	pub id: String,
	pub name: String,
	pub services: AppServices,
}

#[derive(Debug, Clone)]
pub struct AppServices {
	inner: Vec<AppService>,
}

/// An docker app service
#[derive(Debug, Clone)]
pub struct AppService {
	pub container_id: String,
	pub image: String,
	pub created: String,
	pub status: String,
}
