use pg::{
	Connection, Database, FromRow, Result, ToRow, UniqueId,
	db::Conn,
	filter,
	table::{Table, table::TableWithConn},
	time::DateTime,
};

use crate::teams::data::{self, Team, TeamsBuilderTrait, TeamsTrait};

const MIGRATIONS: &[(&str, &str)] = migration_files!("create-teams");

#[derive(Debug, Clone)]
pub struct TeamsBuilder {
	teams: Table,
}

impl TeamsBuilder {
	pub async fn new(db: &Database) -> Self {
		let this = Self {
			teams: Table::new("teams"),
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

	fn with_connection<'a>(&'a self, conn: Connection<'a>) -> Teams<'a> {
		Teams {
			teams: self.teams.with_conn(conn),
		}
	}
}

impl TeamsBuilderTrait for TeamsBuilder {
	fn with_conn<'a>(&'a self, conn: Conn<'a>) -> data::TeamsWithConn<'a> {
		Box::new(self.with_connection(conn.pg()))
	}
}

#[derive(Debug, FromRow, ToRow)]
struct TeamRow {
	id: UniqueId,
	name: String,
	created_on: DateTime,
}

impl From<TeamRow> for Team {
	fn from(row: TeamRow) -> Self {
		Self {
			id: row.id,
			name: row.name,
			created_on: row.created_on,
		}
	}
}

pub struct Teams<'a> {
	teams: TableWithConn<'a>,
}

#[async_trait::async_trait]
impl TeamsTrait for Teams<'_> {
	async fn by_id(&self, id: &UniqueId) -> Result<Option<Team>> {
		self.teams
			.select_opt::<TeamRow>(filter!(id))
			.await
			.map(|r| r.map(Into::into))
	}

	async fn insert(&self, team: &Team) -> Result<()> {
		let row = TeamRow {
			id: team.id,
			name: team.name.clone(),
			created_on: team.created_on,
		};

		self.teams.insert(&row).await
	}
}
