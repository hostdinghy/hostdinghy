use std::fmt;

use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use internal_api::error::Error as ApiError;
use pg::database::DatabaseError;
use serde::{Deserialize, Serialize};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Error {
	LoginIncorrect,
	MissingSessionToken,
	InvalidSessionToken,
	InvalidUser,
	InsufficientRights,
	NotFound,
	Internal(String),
	Request(String),
}

impl Error {
	pub fn status_code(&self) -> StatusCode {
		match self {
			Self::LoginIncorrect | Self::MissingSessionToken => {
				StatusCode::UNAUTHORIZED
			}
			Self::InvalidSessionToken
			| Self::InvalidUser
			| Self::InsufficientRights => StatusCode::FORBIDDEN,

			Self::NotFound => StatusCode::NOT_FOUND,
			Self::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
			Self::Request(_) => StatusCode::BAD_REQUEST,
		}
	}
}

impl IntoResponse for Error {
	fn into_response(self) -> Response {
		let status = self.status_code();
		(status, Json(self)).into_response()
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		fmt::Debug::fmt(self, f)
	}
}

impl std::error::Error for Error {}

impl From<pg::Error> for Error {
	fn from(e: pg::Error) -> Self {
		Self::Internal(e.to_string())
	}
}

impl From<DatabaseError> for Error {
	fn from(e: DatabaseError) -> Self {
		Self::Internal(e.to_string())
	}
}

impl From<ApiError> for Error {
	fn from(e: ApiError) -> Self {
		// todo do better conversion
		Self::Internal(e.to_string())
	}
}
