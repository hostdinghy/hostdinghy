#[macro_use]
mod utils;
mod apps;
mod error;
mod routes;
mod users;

use std::{fs, path::Path};

use chuchi::{Chuchi, Resource};
use clap::Parser;
use pg::{Database, db::Db};
use serde::Deserialize;

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

#[derive(Debug, Clone, Deserialize, Resource)]
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

fn init(server: &mut Chuchi) {
	users::routes::routes(server);
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

	let users = users::database::UsersBuilder::new(&database).await;

	match args.subcmd {
		Some(SubCommand::CreateUser(c)) => {
			users::cli::create_user(&mut conn, &users, c).await;
			return;
		}
		None => {}
	}

	// we don't need it anymore
	drop(conn);

	let mut server = chuchi::build("0.0.0.0:3030").await.unwrap();

	server.add_resource(cfg);
	server.add_resource(db);
	server.add_resource::<users::data::Users>(Box::new(users));

	if args.enable_cors || cfg!(debug_assertions) {
		server.add_catcher(routes::cors::CorsHeaders);
	}

	init(&mut server);

	tokio::try_join!(server.run_task()).unwrap();
}
