use chuchi::{Chuchi, api};
use pg::db::ConnOwned;

use crate::error::{Error, Result};
use crate::users::api::{Authenticated, LogoutReq, TokenAuthReq};
use crate::users::{Users, api::LoginReq};

use super::utils::AuthedUser;

// or #[api("/api/login")]
// async fn login(req: LoginReq, auth_token: Header<String>)
#[api(LoginReq)]
async fn login(
	req: LoginReq,
	users: &Users,
	db: ConnOwned,
) -> Result<Authenticated> {
	let users = users.with_conn(db.conn());

	let user = users.by_username(req.username.as_str()).await?;
	let Some(user) = user else {
		return Err(Error::LoginIncorrect);
	};

	if bcrypt::verify(req.password.as_str(), user.password.as_str())
		.unwrap_or(false)
	{
		let session = users.new_session(&user.id).await?;

		Ok(Authenticated {
			user: user.into(),
			session,
		})
	} else {
		Err(Error::LoginIncorrect)
	}
}

#[api(TokenAuthReq)]
async fn token_auth(user: AuthedUser) -> Result<Authenticated> {
	Ok(Authenticated {
		user: user.user.into(),
		session: user.session,
	})
}

#[api(LogoutReq)]
async fn logout(
	_req: LogoutReq,
	user: AuthedUser,
	users: &Users,
	db: ConnOwned,
) -> Result<()> {
	let users = users.with_conn(db.conn());

	users.delete_session(&user.session.token).await?;

	Ok(())
}

pub fn routes(server: &mut Chuchi) {
	server.add_route(login);
	server.add_route(token_auth);
	server.add_route(logout);
}
