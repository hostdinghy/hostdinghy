use internal_api::apps::AppId;
use pg::{
	Connection, Database, FromRow, Result, ToRow, UniqueId,
	db::Conn,
	filter,
	table::{Table, table::TableWithConn},
	time::DateTime,
	whr,
};

use crate::apps::data::{self, App, AppsBuilderTrait, AppsTrait};

const MIGRATIONS: &[(&str, &str)] = migration_files!("create-apps");

#[derive(Debug, Clone)]
pub struct AppsBuilder {
	apps: Table,
}

impl AppsBuilder {
	pub async fn new(db: &Database) -> Self {
		let this = Self {
			apps: Table::new("apps"),
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

	fn with_connection<'a>(&'a self, conn: Connection<'a>) -> Apps<'a> {
		Apps {
			apps: self.apps.with_conn(conn),
		}
	}
}

impl AppsBuilderTrait for AppsBuilder {
	fn with_conn<'a>(&'a self, conn: Conn<'a>) -> data::AppsWithConn<'a> {
		Box::new(self.with_connection(conn.pg()))
	}
}

#[derive(Debug, FromRow, ToRow)]
struct AppRow {
	id: AppId,
	name: String,
	team_id: UniqueId,
	server_id: UniqueId,
	created_on: DateTime,
}

impl From<AppRow> for App {
	fn from(row: AppRow) -> Self {
		Self {
			id: row.id,
			name: row.name,
			team_id: row.team_id,
			server_id: row.server_id,
			created_on: row.created_on,
		}
	}
}

pub struct Apps<'a> {
	apps: TableWithConn<'a>,
}

#[async_trait::async_trait]
impl AppsTrait for Apps<'_> {
	async fn all(&self, team_id: &Option<UniqueId>) -> Result<Vec<App>> {
		let filter = team_id
			.as_ref()
			.map(|team_id| filter!(team_id))
			.unwrap_or_else(|| filter!());

		self.apps
			.select::<AppRow>(filter)
			.await
			.map(|r| r.into_iter().map(Into::into).collect())
	}

	async fn by_id(
		&self,
		id: &AppId,
		team_id: &Option<UniqueId>,
	) -> Result<Option<App>> {
		let mut filter = filter!(id);
		if let Some(team_id) = team_id {
			filter.and_where(whr!(team_id));
		};

		self.apps
			.select_opt::<AppRow>(filter)
			.await
			.map(|r| r.map(Into::into))
	}

	async fn insert(&self, app: &App) -> Result<()> {
		let row = AppRow {
			id: app.id.clone(),
			name: app.name.clone(),
			team_id: app.team_id,
			server_id: app.server_id,
			created_on: app.created_on,
		};

		self.apps.insert(&row).await
	}
}
