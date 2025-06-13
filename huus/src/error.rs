use std::error::Error as StdError;
use std::fmt;

use pg::database::DatabaseError;
use serde::{Deserialize, Serialize};

use chuchi::{
	api::error::{self, Error as ApiError, StatusCode},
	error::{ClientErrorKind, ErrorKind, ServerErrorKind},
	extractor::ExtractorError,
};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Error {
	LoginIncorrect,
	MissingSessionToken,
	InvalidSessionToken,
	InvalidUser,
	NotFound,
	Internal(String),
	Request(String),
}

impl error::ApiError for Error {
	fn from_error(e: ApiError) -> Self {
		match e {
			ApiError::HeadersMissing(_) | ApiError::Deserialize(_) => {
				Self::Request(e.to_string())
			}
			e => Self::Internal(e.to_string()),
		}
	}

	fn status_code(&self) -> StatusCode {
		match self {
			Self::LoginIncorrect
			| Self::MissingSessionToken
			| Self::InvalidSessionToken
			| Self::InvalidUser => StatusCode::FORBIDDEN,
			Self::NotFound => StatusCode::NOT_FOUND,
			Self::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
			Self::Request(_) => StatusCode::BAD_REQUEST,
		}
	}
}

impl ExtractorError for Error {
	fn error_kind(&self) -> ErrorKind {
		match self {
			Self::LoginIncorrect
			| Self::MissingSessionToken
			| Self::InvalidSessionToken
			| Self::InvalidUser => ClientErrorKind::Forbidden.into(),
			Self::NotFound => ClientErrorKind::NotFound.into(),
			Self::Internal(_) => ServerErrorKind::InternalServerError.into(),
			Self::Request(_) => ClientErrorKind::BadRequest.into(),
		}
	}

	fn into_std(self) -> Box<dyn StdError + Send + Sync> {
		Box::new(self)
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
