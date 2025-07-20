use api::{error::Error, requests::AppInfoRes};
use axum::{Json, Router, routing::get};

use crate::server::{Authenticated, router::AppState};

async fn app_info(_auth: Authenticated) -> Result<Json<AppInfoRes>, Error> {
	todo!()
}

pub fn routes() -> Router<AppState> {
	Router::new().route("/:id", get(app_info))
}
