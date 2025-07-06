use super::{
	Users,
	data::{Session, Token, User, UsersWithConn},
};
use crate::error::Error;

use axum::extract::{FromRef, FromRequestParts};
use axum::http::HeaderMap;
use axum::http::request::Parts;
use pg::db::Db;

pub struct AuthedUser {
	pub session: Session,
	pub user: User,
}

impl<S> FromRequestParts<S> for AuthedUser
where
	S: Send + Sync,
	Users: FromRef<S>,
	Db: FromRef<S>,
{
	type Rejection = Error;

	async fn from_request_parts(
		parts: &mut Parts,
		state: &S,
	) -> Result<Self, Self::Rejection> {
		let users = Users::from_ref(state);
		let db = Db::from_ref(state);

		let conn = db.get().await?;
		let users = users.with_conn(conn.conn());

		let session = session_from_headers(&parts.headers, &users).await?;

		let user = authenticated_user(&session, &users).await?;

		Ok(Self { session, user })
	}
}

pub async fn session_from_headers(
	headers: &HeaderMap,
	users: &UsersWithConn<'_>,
) -> Result<Session, Error> {
	let token: Token = headers
		.get("session-token")
		.ok_or(Error::MissingSessionToken)?
		.to_str()
		.map_err(|_| Error::MissingSessionToken)?
		.parse()
		.map_err(|_| Error::MissingSessionToken)?;

	users
		.session_by_token(&token)
		.await?
		.ok_or(Error::InvalidSessionToken)
}

pub async fn authenticated_user(
	session: &Session,
	users: &UsersWithConn<'_>,
) -> Result<User, Error> {
	users.by_id(&session.user_id).await?.ok_or(Error::NotFound)
}
