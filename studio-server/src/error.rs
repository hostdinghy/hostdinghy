use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use internal_api::error::{ComposeError, Error as ApiError};
use pg::database::DatabaseError;
use serde::{Deserialize, Serialize};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "detail", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Error {
	#[error("Compose file error: {0}")]
	Compose(#[from] ComposeError),
	#[error("Login incorrect")]
	LoginIncorrect,
	#[error("Missing session token in request")]
	MissingSessionToken,
	#[error("Invalid session token in request")]
	InvalidSessionToken,
	#[error("Invalid user")]
	InvalidUser,
	#[error("Insufficient rights for this operation")]
	InsufficientRights,
	#[error("Resource not found")]
	NotFound,
	/// gets returned if the internal api server has some error
	#[error("Internal API server error: {0}")]
	InternalApiServer(String),
	#[error("Internal server error: {0}")]
	Internal(String),
	#[error("Request error: {0}")]
	Request(String),
}

impl Error {
	pub fn status_code(&self) -> StatusCode {
		match self {
			Self::Compose(_) => StatusCode::BAD_REQUEST,
			Self::LoginIncorrect | Self::MissingSessionToken => {
				StatusCode::UNAUTHORIZED
			}
			Self::InvalidSessionToken
			| Self::InvalidUser
			| Self::InsufficientRights => StatusCode::FORBIDDEN,

			Self::NotFound => StatusCode::NOT_FOUND,
			Self::InternalApiServer(_) | Self::Internal(_) => {
				StatusCode::INTERNAL_SERVER_ERROR
			}
			Self::Request(_) => StatusCode::BAD_REQUEST,
		}
	}
}

#[derive(Debug, Clone, Serialize)]
struct ErrorResponse {
	pub message: String,
	pub error: Error,
}

impl IntoResponse for Error {
	fn into_response(self) -> Response {
		let status = self.status_code();
		(
			status,
			Json(ErrorResponse {
				message: self.to_string(),
				error: self,
			}),
		)
			.into_response()
	}
}

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
		// todo better error matching
		match e {
			ApiError::Compose(e) => Self::Compose(e),
			ApiError::AppNotFound => Self::NotFound,
			ApiError::Any { .. } => Self::InternalApiServer(e.to_string()),
			e => Self::Internal(e.to_string()),
		}
	}
}
