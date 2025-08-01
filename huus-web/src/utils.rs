use axum::{
	extract::{FromRef, FromRequestParts},
	http::request::Parts,
};
use pg::db::{Conn, Db, Trans};

use crate::error::Error;

macro_rules! migration_files {
	($($file:expr),* $(,)?) => {
		&[
			$(
				(
					$file,
					include_str!(concat!("./migrations/", $file, ".sql")),
				)
			),*
		]
	};
}

pub struct ConnOwned(pub pg::db::ConnOwned);

impl ConnOwned {
	pub fn conn(&self) -> Conn<'_> {
		self.0.conn()
	}

	#[allow(dead_code)]
	pub async fn trans(&mut self) -> Result<Trans<'_>, pg::Error> {
		self.0.trans().await
	}
}

impl<S> FromRequestParts<S> for ConnOwned
where
	S: Send + Sync,
	Db: FromRef<S>,
{
	type Rejection = Error;

	async fn from_request_parts(
		_parts: &mut Parts,
		state: &S,
	) -> Result<Self, Self::Rejection> {
		let db = Db::from_ref(state);
		db.get().await.map(Self).map_err(Into::into)
	}
}
