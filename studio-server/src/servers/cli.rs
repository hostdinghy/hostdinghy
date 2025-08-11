use base64::{Engine, prelude::BASE64_URL_SAFE_NO_PAD};
use clap::Parser;
use internal_api::requests::ApiToken;
use pg::{UniqueId, db::ConnOwned, time::DateTime};

use crate::{AppState, servers::data::Server};

#[derive(Debug, Parser)]
pub struct CreateServer {
	name: String,
	team_id: UniqueId,
	domain: String,
	api_token: ApiToken,
	tls_cert: String,
}

pub async fn create_server(
	conn: &mut ConnOwned,
	state: &AppState,
	cs: CreateServer,
) {
	let servers = state.servers.with_conn(conn.conn());
	let cert = BASE64_URL_SAFE_NO_PAD
		.decode(cs.tls_cert.trim())
		.expect("invalid cert");
	let cert = String::from_utf8(cert).expect("invalid cert");

	// let's create a server and then check if the server can be connected to
	let server = Server {
		id: UniqueId::new(),
		team_id: cs.team_id,
		name: cs.name,
		domain: cs.domain,
		api_token: cs.api_token,
		tls_cert: cert,
		created_on: DateTime::now(),
	};
	let client = state
		.api_client
		.connect(&server)
		.expect("failed to connect to the server");

	// check if the information of the server works
	let _info = client.info().await.expect("failed to get server info");
	// seems to work else info would have failed now we can insert the server

	servers.insert(&server).await.unwrap();

	println!("created new server {server:#?}");
}

#[derive(Debug, Parser)]
pub struct GenerateApiToken {}

pub fn generate_api_token(gat: GenerateApiToken) {
	eprintln!("generated token: {}", ApiToken::new());
}

#[derive(Debug, Parser)]
pub struct CreateMockServer {
	name: String,
	team_id: UniqueId,
}

pub async fn create_mock_server(
	conn: &mut ConnOwned,
	state: &AppState,
	cms: CreateMockServer,
) {
	assert!(
		state.api_client.is_mock(),
		"this should only be used in mock mode"
	);

	let servers = state.servers.with_conn(conn.conn());

	let server = Server {
		id: UniqueId::new(),
		team_id: cms.team_id,
		name: cms.name.clone(),
		domain: cms.name.clone(),
		api_token: ApiToken::new(),
		tls_cert: cms.name,
		created_on: DateTime::now(),
	};
	servers.insert(&server).await.unwrap();

	println!("created new mock server {server:#?}");
}
