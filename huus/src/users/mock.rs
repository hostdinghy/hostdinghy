use std::{
	collections::HashMap,
	sync::{Arc, RwLock},
};

use pg::{
	Result, UniqueId,
	db::Conn,
	time::{DateTime, Timeout},
};

use crate::users::data::UnsafeUser;

use super::data::{
	Session, Token, User, UsersBuilderTrait, UsersTrait, UsersWithConn,
};

pub struct UsersBuilder {
	inner: Arc<Users>,
}

impl UsersBuilder {
	pub fn new() -> Self {
		Self {
			inner: Arc::new(Users::new()),
		}
	}
}

impl UsersBuilderTrait for UsersBuilder {
	fn with_conn<'a>(&'a self, _conn: Conn<'a>) -> UsersWithConn<'a> {
		Box::new(self.inner.clone())
	}
}

#[derive(Debug, Clone)]
struct SessionRaw {
	token: Token,
	timeout: Timeout,
	user_id: UniqueId,
	created_on: DateTime,
}

impl From<SessionRaw> for Session {
	fn from(raw: SessionRaw) -> Self {
		Self {
			token: raw.token,
			timeout: raw.timeout,
			user_id: raw.user_id,
			created_on: raw.created_on,
		}
	}
}

pub struct Users {
	users: RwLock<HashMap<UniqueId, UnsafeUser>>,
	sessions: RwLock<HashMap<Token, SessionRaw>>,
}

impl Users {
	pub fn new() -> Self {
		Self {
			users: RwLock::new(HashMap::new()),
			sessions: RwLock::new(HashMap::new()),
		}
	}
}

fn into<T>(v: impl Into<T>) -> T {
	v.into()
}

#[async_trait::async_trait]
impl UsersTrait for Arc<Users> {
	async fn by_id(&self, id: &UniqueId) -> Result<Option<User>> {
		let inner = self.users.read().unwrap();
		Ok(inner.get(id).cloned().map(into))
	}

	async fn by_username(&self, username: &str) -> Result<Option<UnsafeUser>> {
		let inner = self.users.read().unwrap();
		Ok(inner.values().find(|u| u.username == username).cloned())
	}

	async fn insert(&self, user: &UnsafeUser) -> Result<()> {
		let mut inner = self.users.write().unwrap();
		inner.insert(user.id, user.clone());

		Ok(())
	}

	async fn new_session(&self, user_id: &UniqueId) -> Result<Session> {
		let mut inner = self.sessions.write().unwrap();
		let session = Session::new(*user_id);
		let raw = SessionRaw {
			token: session.token.clone(),
			timeout: session.timeout.clone(),
			user_id: session.user_id,
			created_on: session.created_on,
		};

		inner.insert(session.token.clone(), raw);

		Ok(session)
	}

	async fn session_by_token(&self, token: &Token) -> Result<Option<Session>> {
		let inner = self.sessions.read().unwrap();
		Ok(inner.get(token).cloned().map(into))
	}

	async fn delete_session(&self, token: &Token) -> Result<()> {
		let mut inner = self.sessions.write().unwrap();
		inner.remove(token);
		Ok(())
	}
}
