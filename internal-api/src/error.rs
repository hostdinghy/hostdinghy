use axum::{
	Json,
	response::{IntoResponse, Response},
};
use http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, thiserror::Error, Serialize, Deserialize)]
pub enum Error {
	/// Gets returned if the app folder or the compose file (if required) could not be found
	#[error("Could not find app folder")]
	AppNotFound,
	#[error("Missing bearer token in request")]
	MissingApiToken,
	#[error("Invalid bearer token in request")]
	InvalidApiToken,
	#[error("Invalid certificate provided")]
	InvalidCertificate,
	#[error("Failed to run command: {command}, message: {message}")]
	Command { command: String, message: String },
	#[error("HOSTDINGHY_DIR environment variable is not set")]
	HostdinghyDirNotPresent,
	#[error("{message} - {error}")]
	Any { message: String, error: String },
}

impl Error {
	pub fn cmd(command: impl ToString, message: impl ToString) -> Self {
		Self::Command {
			command: command.to_string(),
			message: message.to_string(),
		}
	}

	pub fn any(message: impl ToString, e: impl ToString) -> Self {
		Self::Any {
			message: message.to_string(),
			error: e.to_string(),
		}
	}

	pub fn status_code(&self) -> StatusCode {
		match self {
			Self::AppNotFound => StatusCode::NOT_FOUND,
			Self::MissingApiToken => StatusCode::UNAUTHORIZED,
			Self::InvalidApiToken => StatusCode::FORBIDDEN,
			Self::InvalidCertificate => StatusCode::BAD_REQUEST,
			Self::Command { .. }
			| Self::HostdinghyDirNotPresent
			| Self::Any { .. } => StatusCode::INTERNAL_SERVER_ERROR,
		}
	}
}

impl IntoResponse for Error {
	fn into_response(self) -> Response {
		let status = self.status_code();
		(status, Json(self)).into_response()
	}
}

pub trait WithMessage {
	type Ret;

	fn with_message(self, message: impl ToString) -> Self::Ret;
}

impl<T, E> WithMessage for Result<T, E>
where
	E: std::error::Error + Send + Sync + 'static,
{
	type Ret = Result<T, Error>;

	fn with_message(self, message: impl ToString) -> Self::Ret {
		self.map_err(|e| Error::any(message, e))
	}
}
