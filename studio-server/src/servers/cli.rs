use clap::Parser;
use internal_api::requests::ApiToken;
use pg::{UniqueId, db::ConnOwned, time::DateTime};

use crate::{AppState, servers::data::Server};

#[derive(Debug, Parser)]
pub struct CreateServer {
	name: String,
	team_id: UniqueId,
	addr: String,
	tls_cert: String,
}

pub async fn create_server(
	conn: &mut ConnOwned,
	state: &AppState,
	cu: CreateServer,
) {
	let servers = state.servers.with_conn(conn.conn());

	let server = Server {
		id: UniqueId::new(),
		team_id: cu.team_id,
		name: cu.name,
		addr: cu.addr,
		api_token: ApiToken::new(),
		tls_cert: cu.tls_cert,
		created_on: DateTime::now(),
	};
	servers.insert(&server).await.unwrap();

	println!("created new server {server:#?}");
}
