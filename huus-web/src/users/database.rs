use pg::{
	Connection, Database, FromRow, Result, ToRow, UniqueId,
	db::Conn,
	filter,
	table::{Table, table::TableWithConn},
	time::{DateTime, Timeout},
	whr,
};

use crate::users::data::UnsafeUser;

use super::data::{self, Session, Token, User, UsersBuilderTrait, UsersTrait};

const MIGRATIONS: &[(&str, &str)] = migration_files!("create-users");

#[derive(Debug, Clone)]
pub struct UsersBuilder {
	users: Table,
	sessions: Table,
}

impl UsersBuilder {
	pub async fn new(db: &Database) -> Self {
		let this = Self {
			users: Table::new("users"),
			sessions: Table::new("user_sessions"),
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

	fn with_connection<'a>(&'a self, conn: Connection<'a>) -> Users<'a> {
		Users {
			users: self.users.with_conn(conn),
			sessions: self.sessions.with_conn(conn),
		}
	}
}

impl UsersBuilderTrait for UsersBuilder {
	fn with_conn<'a>(&'a self, conn: Conn<'a>) -> data::UsersWithConn<'a> {
		Box::new(self.with_connection(conn.pg()))
	}
}

#[derive(Debug, FromRow, ToRow)]
struct UserRow {
	id: UniqueId,
	username: String,
	name: String,
	password: String,
	created_on: DateTime,
}

impl From<UserRow> for UnsafeUser {
	fn from(row: UserRow) -> Self {
		Self {
			id: row.id,
			username: row.username,
			name: row.name,
			password: row.password,
			created_on: row.created_on,
		}
	}
}

impl From<UserRow> for User {
	fn from(row: UserRow) -> Self {
		Self {
			id: row.id,
			username: row.username,
			name: row.name,
			created_on: row.created_on,
		}
	}
}

#[derive(Debug, FromRow, ToRow)]
struct SessionRow {
	token: Token,
	timeout: Timeout,
	user_id: UniqueId,
	created_on: DateTime,
}

impl From<Session> for SessionRow {
	fn from(session: Session) -> Self {
		Self {
			token: session.token,
			timeout: session.timeout,
			user_id: session.user_id,
			created_on: session.created_on,
		}
	}
}

impl From<SessionRow> for Session {
	fn from(row: SessionRow) -> Self {
		Self {
			token: row.token,
			timeout: row.timeout,
			user_id: row.user_id,
			created_on: row.created_on,
		}
	}
}

pub struct Users<'a> {
	users: TableWithConn<'a>,
	sessions: TableWithConn<'a>,
}

#[async_trait::async_trait]
impl UsersTrait for Users<'_> {
	async fn by_id(&self, id: &UniqueId) -> Result<Option<User>> {
		self.users
			.select_opt::<UserRow>(filter!(id))
			.await
			.map(|r| r.map(Into::into))
	}

	async fn by_username(&self, username: &str) -> Result<Option<UnsafeUser>> {
		self.users
			.select_opt::<UserRow>(filter!(&username))
			.await
			.map(|r| r.map(Into::into))
	}

	async fn insert(&self, user: &UnsafeUser) -> Result<()> {
		let row = UserRow {
			id: user.id,
			username: user.username.clone(),
			name: user.name.clone(),
			password: user.password.clone(),
			created_on: user.created_on,
		};

		self.users.insert(&row).await
	}

	async fn new_session(&self, user_id: &UniqueId) -> Result<Session> {
		let session = Session::new(*user_id);
		let row = SessionRow::from(session);

		self.sessions.insert(&row).await?;

		Ok(row.into())
	}

	async fn session_by_token(&self, token: &Token) -> Result<Option<Session>> {
		self.sessions
			.select_opt::<SessionRow>(filter!(token))
			.await
			.map(|r| r.map(Into::into))
	}

	async fn delete_session(&self, token: &Token) -> Result<()> {
		self.sessions.delete(whr!(token)).await
	}
}
