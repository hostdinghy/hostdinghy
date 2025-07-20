#[macro_use]
mod utils;
mod apps;
mod error;
mod internal;
mod servers;
mod teams;
mod users;

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

#[derive(Debug, Parser)]
#[command(version)]
struct Args {
	#[clap(subcommand)]
	subcmd: Option<SubCommand>,
	#[clap(long)]
	enable_cors: bool,
	#[clap(long, default_value = "huus=info,chuchi=info,warn")]
	tracing: String,
	#[clap(long, default_value = "./config.toml")]
	config: String,
	#[clap(long, default_value = "../ui/dist")]
	dist_dir: String,
}

#[derive(Debug, Parser)]
enum SubCommand {
	CreateUser(users::cli::CreateUser),
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct Config {
	database: DbConf,
}

#[derive(Debug, Clone, Deserialize)]
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
	api_client: ApiClient,
	db: Db,
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
	.unwrap();
	let db = Db::from(database.clone());
	let mut conn = db.get().await.unwrap();

	let teams = teams::database::TeamsBuilder::new(&database).await;
	let users = users::database::UsersBuilder::new(&database).await;
	let servers = servers::database::ServersBuilder::new(&database).await;

	match args.subcmd {
		Some(SubCommand::CreateUser(c)) => {
			users::cli::create_user(&mut conn, &users, &teams, c).await;
			return;
		}
		None => {}
	}

	// we don't need it anymore
	drop(conn);

	let state = AppState {
		teams: Arc::new(Box::new(teams)),
		users: Arc::new(Box::new(users)),
		servers: Arc::new(Box::new(servers)),
		api_client: ApiClient::new(cfg!(debug_assertions)),
		db,
		cfg: Arc::new(cfg),
	};

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
