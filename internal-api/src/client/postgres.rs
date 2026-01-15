use bytes::Bytes;
use futures::{Stream, StreamExt as _, TryStream, stream::BoxStream};
use reqwest::Body;

use crate::{
	Error,
	client::{ApiServerClient, Result},
	database_name::DatabaseName,
	error::WithMessage,
	postgres::{CreateDatabaseReq, CreateDatabaseRes},
};

#[derive(Debug, Clone)]
pub struct ApiServerPostgresClient<'a> {
	inner: &'a ApiServerClient,
}

impl<'a> ApiServerPostgresClient<'a> {
	pub(crate) fn new(inner: &'a ApiServerClient) -> Self {
		Self { inner }
	}

	pub async fn databases(&self) -> Result<Vec<String>> {
		self.inner
			.send_json(self.inner.get("/postgres/databases"))
			.await
	}

	pub async fn create_database(
		&self,
		name: &DatabaseName,
	) -> Result<CreateDatabaseRes> {
		self.inner
			.send_json(
				self.inner
					.post("/postgres/databases")
					.json(&CreateDatabaseReq { name: name.clone() }),
			)
			.await
	}

	pub async fn restore_database<S>(
		&self,
		name: &DatabaseName,
		stream: S,
	) -> Result<()>
	where
		S: TryStream<Ok = Bytes, Error = Error> + Send + 'static,
	{
		self.inner
			.send(
				self.inner
					.put(&format!("/postgres/databases/{name}/restore"))
					.body(Body::wrap_stream(stream)),
			)
			.await
			.map(|_| ())
	}

	pub async fn dump_database(
		&self,
		name: &DatabaseName,
	) -> Result<BoxStream<'static, Result<Bytes>>> {
		self.inner
			.send(self.inner.get(&format!("/postgres/databases/{name}/dump")))
			.await
			.map(|res| {
				res.bytes_stream()
					.map(|r| r.with_message("dump failed"))
					.boxed()
			})
	}

	// pub async fn delete_user(&self, username: &str) -> Result<()> {
	// 	self.inner
	// 		.send_json(
	// 			self.inner.delete(&format!("/registry/users/{username}")),
	// 		)
	// 		.await
	// }
}
