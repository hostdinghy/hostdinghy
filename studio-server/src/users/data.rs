use std::time::Duration;

use axum::extract::FromRef;
use pg::{
	Result, UniqueId,
	db::Conn,
	time::{DateTime, Timeout},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::AppState;

#[derive(Debug, Clone)]
pub struct UnsafeUser {
	pub id: UniqueId,
	pub team_id: UniqueId,
	pub username: String,
	pub name: String,
	// password hashed with bcrypt
	pub password: String,
	pub rights: Rights,
	pub created_on: DateTime,
}

impl From<UnsafeUser> for User {
	fn from(user: UnsafeUser) -> Self {
		Self {
			id: user.id,
			team_id: user.team_id,
			username: user.username,
			name: user.name,
			rights: user.rights,
			created_on: user.created_on,
		}
	}
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Rights {
	// admin means you can edit your team
	pub admin: bool,
	pub root: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
	pub id: UniqueId,
	pub team_id: UniqueId,
	pub username: String,
	pub name: String,
	pub rights: Rights,
	pub created_on: DateTime,
}

impl User {
	pub fn new(
		team_id: UniqueId,
		username: String,
		name: String,
		rights: Rights,
	) -> Self {
		Self {
			id: UniqueId::new(),
			team_id,
			username,
			name,
			rights,
			created_on: DateTime::now(),
		}
	}
}

pub type Token = crypto::token::Token<32>;

// 30days
const SESSION_TIMEOUT: Duration = Duration::from_secs(30 * 24 * 60 * 60);
// 10 days remaining
const RENEW_AFTER: Duration = Duration::from_secs(10 * 24 * 60 * 60);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Session {
	pub token: Token,
	pub timeout: Timeout,
	#[serde(skip_serializing)]
	pub user_id: UniqueId,
	#[serde(skip_serializing)]
	pub created_on: DateTime,
}

impl Session {
	pub fn new(user_id: UniqueId) -> Self {
		Self {
			token: Token::new(),
			timeout: Timeout::new(SESSION_TIMEOUT),
			user_id,
			created_on: DateTime::now(),
		}
	}

	pub fn is_valid(&self) -> bool {
		!self.timeout.has_elapsed()
	}

	// does not check if the time is valid
	pub fn should_renew(&self) -> bool {
		self.timeout
			.remaining()
			.map(|d| d < RENEW_AFTER)
			.unwrap_or(true)
	}
}

pub type Users = Arc<dyn UsersBuilderTrait + Send + Sync>;
pub type UsersWithConn<'a> = Box<dyn UsersTrait + Send + Sync + 'a>;

impl FromRef<AppState> for Users {
	fn from_ref(state: &AppState) -> Self {
		state.users.clone()
	}
}

pub trait UsersBuilderTrait {
	fn with_conn<'a>(&'a self, conn: Conn<'a>) -> UsersWithConn<'a>;
}

#[derive(Debug, Clone)]
pub struct UpdateUser {
	pub id: UniqueId,
	pub name: String,
	pub password: Option<String>,
}

#[async_trait::async_trait]
pub trait UsersTrait {
	async fn by_id(&self, id: &UniqueId) -> Result<Option<User>>;

	async fn by_username(&self, username: &str) -> Result<Option<UnsafeUser>>;

	async fn insert(&self, user: &UnsafeUser) -> Result<()>;

	async fn update(&self, user: &UpdateUser) -> Result<()>;

	async fn new_session(&self, user_id: &UniqueId) -> Result<Session>;

	async fn session_by_token(&self, token: &Token) -> Result<Option<Session>>;

	async fn delete_session(&self, token: &Token) -> Result<()>;
}
