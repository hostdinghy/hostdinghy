pub mod main;
pub mod registry;
pub mod utils;

use axum::Router;

use crate::AppState;

pub fn routes() -> Router<AppState> {
	Router::new()
		.merge(main::routes())
		.merge(registry::routes())
}
