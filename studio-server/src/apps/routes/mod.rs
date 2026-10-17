pub mod compose;
pub mod fs;
pub mod main;
pub mod utils;

use axum::Router;

use crate::AppState;

pub fn routes() -> Router<AppState> {
	Router::new()
		.merge(main::routes())
		.merge(compose::routes())
		.merge(fs::routes())
}
