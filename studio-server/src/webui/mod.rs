use core::fmt;
use std::ops;

use axum::extract::FromRef;

use crate::AppState;

pub mod routes;

#[derive(Debug, Clone, FromRef)]
#[repr(transparent)]
pub struct DistDir(pub String);

impl ops::Deref for DistDir {
	type Target = String;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl fmt::Display for DistDir {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::Display::fmt(&self.0, f)
	}
}

impl FromRef<AppState> for DistDir {
	fn from_ref(state: &AppState) -> Self {
		state.dist_dir.clone()
	}
}
