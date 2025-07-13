use api::error::Error;
use axum::{extract::FromRequestParts, http::request::Parts};
use hyper::header::AUTHORIZATION;
use subtle::ConstantTimeEq as _;

use crate::server::{config::ApiToken, router::AppState};

pub struct Authenticated {
	// addr: SocketAddr,
}

impl FromRequestParts<AppState> for Authenticated {
	type Rejection = Error;

	fn from_request_parts(
		parts: &mut Parts,
		state: &AppState,
	) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
		async move {
			let headers = &parts.headers;

			let token: ApiToken = headers
				.get(AUTHORIZATION)
				.and_then(|v| v.to_str().ok())
				.and_then(|s| s.strip_prefix("Bearer "))
				.and_then(|s| s.parse().ok())
				.ok_or(Error::MissingApiToken)?;

			let correct_token =
				state.cfg.api_token.as_ref().ok_or(Error::InvalidApiToken)?;

			// this is probably not necessary but why not
			let choice = correct_token.as_ref().ct_eq(token.as_ref());

			bool::from(choice)
				.then_some(Self {})
				.ok_or(Error::InvalidApiToken)
		}
	}
}
