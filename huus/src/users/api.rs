use chuchi::api::{Method, Request};
use serde::{Deserialize, Serialize};

use crate::{
	error::Error,
	users::data::{Session, User},
};

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

impl Request for LoginReq {
	type Response = Authenticated;
	type Error = Error;

	const PATH: &'static str = "/api/users/login";
	const METHOD: Method = Method::POST;
}

// login via the session token
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenAuthReq;

impl Request for TokenAuthReq {
	type Response = Authenticated;
	type Error = Error;

	const PATH: &'static str = "/api/users/tokenauth";
	const METHOD: Method = Method::POST;
	const HEADERS: &'static [&'static str] = &["session-token"];
}

// login via the session token
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogoutReq;

impl Request for LogoutReq {
	type Response = ();
	type Error = Error;

	const PATH: &'static str = "/api/users/logout";
	const METHOD: Method = Method::POST;
	const HEADERS: &'static [&'static str] = &["session-token"];
}

// // login via the session token
// #[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct DeleteReq;

// impl Request for DeleteReq {
// 	type Response = ();
// 	type Error = Error;

// 	// api is automatically added by nginx
// 	const PATH: &'static str = "/api/users/delete";
// 	const METHOD: Method = Method::POST;
// 	const HEADERS: &'static [&'static str] = &["session-token"];
// }
