use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};

use crate::AppState;
use crate::error::{Error, Result};
use crate::users::Users;
use crate::users::data::{Session, UpdateUser, User};
use crate::users::utils::RightsAny;
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

async fn token_auth(
	user: AuthedUser<RightsAny>,
) -> Result<Json<Authenticated>> {
	Ok(Json(Authenticated {
		user: user.user.into(),
		session: user.session,
	}))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveUserReq {
	pub name: String,
	pub password: Option<String>,
}

async fn save(
	mut user: AuthedUser<RightsAny>,
	State(users): State<Users>,
	conn: ConnOwned,
	Json(req): Json<SaveUserReq>,
) -> Result<Json<Authenticated>> {
	let users = users.with_conn(conn.conn());

	let mut update_user = UpdateUser {
		id: user.user.id,
		name: req.name,
		password: None,
	};

	if let Some(password) = req.password {
		let hashed = bcrypt::hash(password.as_str(), bcrypt::DEFAULT_COST)
			.map_err(|e| Error::Internal(e.to_string()))?;

		update_user.password = Some(hashed);
	}

	users.update(&update_user).await?;

	user.user.name = update_user.name;

	Ok(Json(Authenticated {
		user: user.user,
		session: user.session,
	}))
}

async fn logout(
	State(users): State<Users>,
	user: AuthedUser<RightsAny>,
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
		.route("/save", post(save))
		.route("/logout", post(logout))
}
