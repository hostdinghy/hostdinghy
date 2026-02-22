#[macro_use]
mod utils;
mod apps;
mod error;
mod internal;
mod servers;
mod teams;
mod users;
mod webui;

use std::{fs, sync::Arc};

use axum::Router;
use axum::extract::FromRef;
use axum::http::{Method, header};
use clap::Parser;
use pg::{Database, db::Db};
use serde::Deserialize;
use tokio::net::TcpListener;
use tokio::signal;
use tower_http::cors::{self, CorsLayer};
use tower_http::trace::TraceLayer;

use crate::internal::ApiClient;
use crate::webui::DistDir;

#[derive(Debug, Parser)]
#[command(version)]
struct Args {
	#[clap(subcommand)]
	subcmd: Option<SubCommand>,
	#[clap(long)]
	enable_cors: bool,
	#[clap(long, default_value = "studio_server=info,tower_http=debug,warn")]
	tracing: String,
	#[clap(long, default_value_t = default_config_path())]
	config: String,
	#[clap(long, default_value_t = default_dist_dir())]
	dist_dir: String,
	#[clap(long)]
	internal_api_mock: Option<bool>,
}

fn default_config_path() -> String {
	if cfg!(debug_assertions) {
		"./config.toml".into()
	} else {
		"/data/config.toml".into()
	}
}

fn default_dist_dir() -> String {
	if cfg!(debug_assertions) {
		"../studio-webui/dist".into()
	} else {
		"dist".into()
	}
}

#[derive(Debug, Parser)]
enum SubCommand {
	CreateUser(users::cli::CreateUser),
	CreateServer(servers::cli::CreateServer),
	GenerateApiToken(servers::cli::GenerateApiToken),
	CreateMockServer(servers::cli::CreateMockServer),
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct Config {
	database: DbConf,
	/// Any string value which will be displayed somewhere in the UI.
	#[serde(default = "default_env")]
	environment: String,
}

fn default_env() -> String {
	if cfg!(debug_assertions) {
		"debug".to_string()
	} else {
		"production".to_string()
	}
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct DbConf {
	pub host: String,
	pub name: String,
	pub user: String,
	pub password: String,
}

#[derive(Clone)]
pub struct AppState {
	teams: teams::Teams,
	users: users::Users,
	servers: servers::Servers,
	apps: apps::Apps,
	api_client: ApiClient,
	db: Db,
	dist_dir: DistDir,
	cfg: Arc<Config>,
}

impl FromRef<AppState> for ApiClient {
	fn from_ref(state: &AppState) -> Self {
		state.api_client.clone()
	}
}

impl FromRef<AppState> for Db {
	fn from_ref(state: &AppState) -> Self {
		state.db.clone()
	}
}

impl FromRef<AppState> for Arc<Config> {
	fn from_ref(state: &AppState) -> Self {
		state.cfg.clone()
	}
}

fn create_app(state: AppState, enable_cors: bool) -> Router {
	let mut app = Router::new()
		.nest("/api/teams", teams::routes::routes())
		.nest("/api/users", users::routes::routes())
		.nest("/api/servers", servers::routes::routes())
		.nest("/api/apps", apps::routes::routes())
		.merge(webui::routes::routes())
		.layer(TraceLayer::new_for_http());

	if enable_cors {
		app = app.layer(
			CorsLayer::new()
				.allow_origin(cors::Any)
				.allow_methods([
					Method::GET,
					Method::POST,
					Method::PUT,
					Method::PATCH,
					Method::DELETE,
					Method::OPTIONS,
				])
				.allow_headers([
					header::CONTENT_TYPE,
					"session-token".parse().unwrap(),
				]),
		);
	}

	app.with_state(state)
}

#[tokio::main]
async fn main() {
	let args = Args::parse();

	let cfg_string =
		fs::read_to_string(args.config).expect("failed to read config.toml");
	let cfg: Config =
		toml::from_str(&cfg_string).expect("failed to read config.toml");

	tracing_subscriber::fmt()
		.with_env_filter(args.tracing)
		.init();

	let db_cfg = &cfg.database;
	let database = Database::with_host(
		&db_cfg.host,
		&db_cfg.name,
		&db_cfg.user,
		&db_cfg.password,
	)
	.await
	.expect("could not connect to the database");
	let db = Db::from(database.clone());
	let mut conn = db.get().await.unwrap();

	let teams = teams::database::TeamsBuilder::new(&database).await;
	let users = users::database::UsersBuilder::new(&database).await;
	let servers = servers::database::ServersBuilder::new(&database).await;
	let apps = apps::database::AppsBuilder::new(&database).await;
	let api_client = ApiClient::new(
		args.internal_api_mock.unwrap_or(cfg!(debug_assertions)),
	);

	let state = AppState {
		teams: Arc::new(teams),
		users: Arc::new(users),
		servers: Arc::new(servers),
		apps: Arc::new(apps),
		api_client: api_client.clone(),
		db,
		dist_dir: DistDir(args.dist_dir),
		cfg: Arc::new(cfg),
	};

	api_client.populate_mock_data(&mut conn, &state).await;

	match args.subcmd {
		Some(SubCommand::CreateUser(c)) => {
			users::cli::create_user(&mut conn, &state, c).await;
			return;
		}
		Some(SubCommand::CreateServer(c)) => {
			servers::cli::create_server(&mut conn, &state, c).await;
			return;
		}
		Some(SubCommand::GenerateApiToken(c)) => {
			servers::cli::generate_api_token(c);
			return;
		}
		Some(SubCommand::CreateMockServer(c)) => {
			servers::cli::create_mock_server(&mut conn, &state, c).await;
			return;
		}
		None => {}
	}

	// we don't need it anymore
	drop(conn);

	let app = create_app(state, args.enable_cors || cfg!(debug_assertions));

	let listener = TcpListener::bind("0.0.0.0:3030")
		.await
		.expect("failed to bind to port 3030");

	println!("Server starting on 0.0.0.0:3030");

	axum::serve(listener, app)
		.with_graceful_shutdown(shutdown_signal())
		.await
		.unwrap();
}

async fn shutdown_signal() {
	let ctrl_c = async {
		signal::ctrl_c()
			.await
			.expect("failed to install Ctrl+C handler");
	};

	#[cfg(unix)]
	let terminate = async {
		signal::unix::signal(signal::unix::SignalKind::terminate())
			.expect("failed to install signal handler")
			.recv()
			.await;
	};

	#[cfg(not(unix))]
	let terminate = std::future::pending::<()>();

	tokio::select! {
		_ = ctrl_c => {},
		_ = terminate => {},
	}
}
