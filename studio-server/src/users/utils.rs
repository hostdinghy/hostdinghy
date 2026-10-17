use std::marker::PhantomData;

use super::{
	Users,
	data::{Session, Token, User, UsersWithConn},
	download::DownloadTickets,
};
use crate::{error::Error, users::data::Rights};

use axum::extract::{FromRef, FromRequestParts, OriginalUri, Query};
use axum::http::request::Parts;
use axum::http::{HeaderMap, Method};
use pg::{UniqueId, db::Db};
use serde::Deserialize;

pub struct AuthedUser<RightsCheck> {
	pub session: Session,
	pub user: User,
	rights_check: PhantomData<RightsCheck>,
}

impl<RightsCheck> AuthedUser<RightsCheck> {
	pub fn team_for_filter(&self) -> Option<UniqueId> {
		if self.user.rights.root {
			None
		} else {
			Some(self.user.team_id)
		}
	}

	// pub fn can_access_team(&self, team_id: &UniqueId) -> bool {
	// 	self.user.rights.root || self.user.team_id == *team_id
	// }
}

impl<S, RC> FromRequestParts<S> for AuthedUser<RC>
where
	S: Send + Sync,
	Users: FromRef<S>,
	Db: FromRef<S>,
	DownloadTickets: FromRef<S>,
	RC: RightsCheck + Send + Sync + 'static,
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

		let session = match download_ticket(parts) {
			Some(ticket) => {
				// nested routers only see their part of the path
				let path = parts
					.extensions
					.get::<OriginalUri>()
					.map(|u| u.0.path())
					.unwrap_or(parts.uri.path());
				let token = DownloadTickets::from_ref(state)
					.take(&ticket, path)
					.ok_or(Error::InvalidSessionToken)?;
				users
					.session_by_token(&token)
					.await?
					.ok_or(Error::InvalidSessionToken)?
			}
			None => session_from_headers(&parts.headers, &users).await?,
		};

		let user = authenticated_user(&session, &users, RC::check).await?;

		Ok(Self {
			session,
			user,
			rights_check: PhantomData,
		})
	}
}

pub trait RightsCheck {
	fn check(rights: &Rights) -> bool;
}

pub struct RightsAny;

impl RightsCheck for RightsAny {
	fn check(_: &Rights) -> bool {
		true
	}
}

pub struct RightsAdmin;

impl RightsCheck for RightsAdmin {
	fn check(rights: &Rights) -> bool {
		rights.admin || rights.root
	}
}

pub struct RightsRoot;

impl RightsCheck for RightsRoot {
	fn check(rights: &Rights) -> bool {
		rights.root
	}
}

/// the ticket of a download without the session-token header
/// (see [`DownloadTickets`])
fn download_ticket(parts: &Parts) -> Option<String> {
	#[derive(Deserialize)]
	struct TicketQuery {
		#[serde(rename = "download-ticket")]
		ticket: String,
	}

	// downloads are GET requests, tickets are not for anything else
	if parts.method != Method::GET
		|| parts.headers.contains_key("session-token")
	{
		return None;
	}
	Query::<TicketQuery>::try_from_uri(&parts.uri)
		.ok()
		.map(|q| q.0.ticket)
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

pub async fn authenticated_user<Rf>(
	session: &Session,
	users: &UsersWithConn<'_>,
	rights_check: Rf,
) -> Result<User, Error>
where
	Rf: FnOnce(&Rights) -> bool,
{
	let user = users
		.by_id(&session.user_id)
		.await?
		.ok_or(Error::NotFound)?;

	if !rights_check(&user.rights) {
		return Err(Error::InsufficientRights);
	}

	Ok(user)
}
