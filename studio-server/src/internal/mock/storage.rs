use std::{
	collections::HashMap,
	sync::{Arc, Mutex},
};

use internal_api::{
	app_id::AppId,
	apps::{
		AppInfoRes, AppService, ComposeCommand, GetComposeRes, SaveComposeReq,
		ServiceRoute, ServiceState,
	},
	client::Result,
	error::Error,
};
use pg::UniqueId;
use rand::Rng;

use crate::{apps::data::App, servers::data::Server};

#[derive(Debug)]
pub struct ServersMock {
	// we have an Arc Mutex inside as well so we can share only servers
	// with the client without needing to go over Servers all the time
	// not sure this is better or worse?
	servers: HashMap<UniqueId, Arc<Mutex<ServerMock>>>,
}

impl ServersMock {
	pub fn new() -> Self {
		Self {
			servers: HashMap::new(),
		}
	}

	pub fn get(&self, id: &UniqueId) -> Option<Arc<Mutex<ServerMock>>> {
		self.servers.get(id).cloned()
	}

	pub fn populate_mock_data(&mut self, servers: Vec<Server>, apps: Vec<App>) {
		let mut s_mocks = HashMap::new();

		// lets first insert all servers
		for server in servers {
			s_mocks.insert(server.id, ServerMock::new(server));
		}

		// now lets insert all apps for each server
		for app in apps {
			let server_mock = s_mocks
				.get_mut(&app.server_id)
				.expect("server mock not found for app");

			server_mock.insert_app(app);
		}

		// finally we can insert the server mocks into the servers map
		self.servers = s_mocks
			.into_iter()
			.map(|(id, mock)| (id, Arc::new(Mutex::new(mock))))
			.collect();
	}
}

#[derive(Debug)]
pub struct ServerMock {
	id: UniqueId,
	apps: HashMap<AppId, AppMock>,
}

impl ServerMock {
	fn new(server: Server) -> Self {
		Self {
			id: server.id,
			apps: HashMap::new(),
		}
	}

	fn insert_app(&mut self, app: App) {
		self.apps.insert(app.id.clone(), AppMock::new(app.id));
	}

	pub fn app_info(&self, id: &AppId) -> Result<AppInfoRes> {
		let app = self.apps.get(id).ok_or(Error::AppNotFound)?;
		app.app_info()
	}

	pub fn app_get_compose(&self, id: &AppId) -> Result<GetComposeRes> {
		let app = self.apps.get(id).ok_or(Error::AppNotFound)?;
		app.app_get_compose()
	}

	pub fn app_set_compose(
		&mut self,
		id: &AppId,
		req: &SaveComposeReq,
	) -> Result<()> {
		let app = self
			.apps
			.entry(id.clone())
			.or_insert_with(|| AppMock::new(id.clone()));
		app.app_set_compose(req)
	}

	pub fn app_compose_command(
		&mut self,
		id: &AppId,
		cmd: &ComposeCommand,
	) -> Result<()> {
		let app = self.apps.get_mut(id).ok_or(Error::AppNotFound)?;
		app.app_compose_command(cmd)
	}

	pub fn app_logs(&self, id: &AppId, lines: Option<u32>) -> Result<String> {
		let app = self.apps.get(id).ok_or(Error::AppNotFound)?;
		app.app_logs(lines)
	}
}

const MOCK_COMPOSE: &str = include_str!("./mock_compose.yml");
const MOCK_LOGS: &str = include_str!("./mock_logs.txt");

#[derive(Debug, Clone)]
pub struct AppMock {
	id: AppId,
	compose: Option<String>,
	started: Option<bool>,
	database: bool,
}

impl AppMock {
	fn new(id: AppId) -> Self {
		let mut rng = rand::rng();

		Self {
			id,
			compose: rng.random_bool(0.5).then(|| MOCK_COMPOSE.to_string()),
			started: None,
			database: rng.random_bool(0.5),
		}
	}

	pub fn app_info(&self) -> Result<AppInfoRes> {
		let _compose = self.compose.as_ref().ok_or(Error::AppNotFound)?;
		let mut rng = rand::rng();

		let mut services = vec![];

		if rng.random_bool(0.5) {
			let state = random_service_state(self.started);
			services.push(AppService {
				name: "craft".to_string(),
				container_name: format!("{}-craft-1", self.id),
				state_hr: service_state_to_str(&state).to_string(),
				state,
				routes: vec![ServiceRoute {
					rule: "Host(`craft.example.com`)".to_string(),
					domains: vec!["craft.example.com".to_string()],
				}],
			});
		}

		if rng.random_bool(0.5) {
			let state = random_service_state(self.started);
			services.push(AppService {
				name: "svelte".to_string(),
				container_name: format!("{}-svelte-1", self.id),
				state_hr: service_state_to_str(&state).to_string(),
				state,
				routes: vec![ServiceRoute {
					rule: "Host(`svelte.example.com`)".to_string(),
					domains: vec!["svelte.example.com".to_string()],
				}],
			});
		}

		Ok(AppInfoRes { services })
	}

	pub fn app_get_compose(&self) -> Result<GetComposeRes> {
		let compose = self.compose.clone().ok_or(Error::AppNotFound)?;

		Ok(GetComposeRes {
			compose,
			database: self.database,
		})
	}

	pub fn app_set_compose(&mut self, req: &SaveComposeReq) -> Result<()> {
		self.compose = Some(req.compose.clone());
		if req.create_database {
			// database does never get deleted
			self.database = req.create_database;
		}

		Ok(())
	}

	pub fn app_compose_command(&mut self, cmd: &ComposeCommand) -> Result<()> {
		match cmd {
			ComposeCommand::Start
			| ComposeCommand::Up
			| ComposeCommand::Restart => {
				self.started = Some(true);
			}
			ComposeCommand::Stop => {
				self.started = Some(false);
			}
		}

		Ok(())
	}

	pub fn app_logs(&self, lines: Option<u32>) -> Result<String> {
		let _compose = self.compose.as_ref().ok_or(Error::AppNotFound)?;

		let Some(lines) = lines else {
			return Ok(MOCK_LOGS.to_string());
		};

		// well this does not take the last lines but should be enough for a mock
		let logs: Vec<&str> = MOCK_LOGS.lines().take(lines as usize).collect();

		Ok(logs.join("\n"))
	}
}

const STATES: &[ServiceState] = &[
	ServiceState::Empty,
	ServiceState::Created,
	ServiceState::Running,
	ServiceState::Unhealthy,
	ServiceState::Paused,
	ServiceState::Restarting,
	ServiceState::Exited,
	ServiceState::Removing,
	ServiceState::Dead,
	ServiceState::Unknown,
];

fn random_service_state(started: Option<bool>) -> ServiceState {
	let mut rng = rand::rng();

	// if the state was selected to be started
	if matches!(started, Some(false)) {
		return ServiceState::Exited;
	}

	// the most likely state is running so it should be the case 70 percent
	// of the time
	if rng.random_bool(0.7) {
		return ServiceState::Running;
	}

	let rng = rng.random_range(0..STATES.len());
	STATES[rng].clone()
}

const fn service_state_to_str(state: &ServiceState) -> &str {
	match state {
		ServiceState::Empty => "Empty",
		ServiceState::Created => "Created",
		ServiceState::Running => "Running",
		ServiceState::Unhealthy => "Unhealthy",
		ServiceState::Paused => "Paused",
		ServiceState::Restarting => "Restarting",
		ServiceState::Exited => "Exited",
		ServiceState::Removing => "Removing",
		ServiceState::Dead => "Dead",
		ServiceState::Unknown => "Unknown",
	}
}
