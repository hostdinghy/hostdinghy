use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};

use crate::AppState;
use crate::error::{Error, Result};
use crate::users::Users;
use crate::users::data::{Session, User};
use crate::utils::ConnOwned;

use super::utils::AuthedUser;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginReq {
	pub username: String,
	pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Authenticated {
	pub session: Session,
	pub user: User,
}

async fn login(
	State(users): State<Users>,
	conn: ConnOwned,
	Json(req): Json<LoginReq>,
) -> Result<Json<Authenticated>> {
	let users = users.with_conn(conn.conn());

	let user = users.by_username(req.username.as_str()).await?;
	let Some(user) = user else {
		return Err(Error::LoginIncorrect);
	};

	if bcrypt::verify(req.password.as_str(), user.password.as_str())
		.unwrap_or(false)
	{
		let session = users.new_session(&user.id).await?;

		Ok(Json(Authenticated {
			user: user.into(),
			session,
		}))
	} else {
		Err(Error::LoginIncorrect)
	}
}

async fn token_auth(user: AuthedUser) -> Result<Json<Authenticated>> {
	Ok(Json(Authenticated {
		user: user.user.into(),
		session: user.session,
	}))
}

async fn logout(
	State(users): State<Users>,
	user: AuthedUser,
	conn: ConnOwned,
) -> Result<()> {
	let users = users.with_conn(conn.conn());

	users.delete_session(&user.session.token).await?;

	Ok(())
}

pub fn routes() -> Router<AppState> {
	Router::new()
		.route("/login", post(login))
		.route("/tokenauth", post(token_auth))
		.route("/logout", post(logout))
}
