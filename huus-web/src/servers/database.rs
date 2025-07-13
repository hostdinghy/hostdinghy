use internal_api::requests::ApiToken;
use pg::{
	Connection, Database, FromRow, Result, ToRow, UniqueId,
	db::Conn,
	filter,
	table::{Table, table::TableWithConn},
	time::DateTime,
};

use crate::servers::data::{self, Server, ServersBuilderTrait, ServersTrait};

const MIGRATIONS: &[(&str, &str)] = migration_files!("create-servers");

#[derive(Debug, Clone)]
pub struct ServersBuilder {
	servers: Table,
}

impl ServersBuilder {
	pub async fn new(db: &Database) -> Self {
		let this = Self {
			servers: Table::new("servers"),
		};

		let migrations = db.migrations();
		let mut conn = db.get().await.unwrap();

		for (name, sql) in MIGRATIONS {
			migrations
				.add(&mut conn, name, sql)
				.await
				.expect("failed to run migration");
		}

		this
	}

	fn with_connection<'a>(&'a self, conn: Connection<'a>) -> Servers<'a> {
		Servers {
			servers: self.servers.with_conn(conn),
		}
	}
}

impl ServersBuilderTrait for ServersBuilder {
	fn with_conn<'a>(&'a self, conn: Conn<'a>) -> data::ServersWithConn<'a> {
		Box::new(self.with_connection(conn.pg()))
	}
}

#[derive(Debug, FromRow, ToRow)]
struct ServerRow {
	id: UniqueId,
	team_id: UniqueId,
	addr: String,
	name: String,
	api_token: ApiToken,
	tls_cert: String,
	created_on: DateTime,
}

impl From<ServerRow> for Server {
	fn from(row: ServerRow) -> Self {
		Self {
			id: row.id,
			team_id: row.team_id,
			addr: row.addr,
			name: row.name,
			api_token: row.api_token,
			tls_cert: row.tls_cert,
			created_on: row.created_on,
		}
	}
}

pub struct Servers<'a> {
	servers: TableWithConn<'a>,
}

#[async_trait::async_trait]
impl ServersTrait for Servers<'_> {
	async fn all(&self) -> Result<Vec<Server>> {
		self.servers
			.select::<ServerRow>(filter!())
			.await
			.map(|r| r.into_iter().map(Into::into).collect())
	}

	async fn all_by_team(&self, team_id: &UniqueId) -> Result<Vec<Server>> {
		self.servers
			.select::<ServerRow>(filter!(team_id))
			.await
			.map(|r| r.into_iter().map(Into::into).collect())
	}

	async fn by_id(&self, id: &UniqueId) -> Result<Option<Server>> {
		self.servers
			.select_opt::<ServerRow>(filter!(id))
			.await
			.map(|r| r.map(Into::into))
	}

	async fn insert(&self, server: &Server) -> Result<()> {
		let row = ServerRow {
			id: server.id,
			team_id: server.team_id,
			addr: server.addr.clone(),
			name: server.name.clone(),
			api_token: server.api_token.clone(),
			tls_cert: server.tls_cert.clone(),
			created_on: server.created_on,
		};

		self.servers.insert(&row).await
	}
}
