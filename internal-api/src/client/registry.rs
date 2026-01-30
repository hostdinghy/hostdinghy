use crate::{
	client::{ApiServerClient, Result},
	registry::{CreateUserReq, CreateUserRes},
	registry_username::RegistryUsername,
};

#[derive(Debug, Clone)]
pub struct ApiServerRegistryClient<'a> {
	inner: &'a ApiServerClient,
}

impl<'a> ApiServerRegistryClient<'a> {
	pub(crate) fn new(inner: &'a ApiServerClient) -> Self {
		Self { inner }
	}

	pub async fn users(&self) -> Result<Vec<String>> {
		self.inner
			.send_json(self.inner.get("/registry/users"))
			.await
	}

	pub async fn create_user(
		&self,
		username: &RegistryUsername,
	) -> Result<CreateUserRes> {
		self.inner
			.send_json(self.inner.post("/registry/users").json(
				&CreateUserReq {
					username: username.clone(),
				},
			))
			.await
	}

	pub async fn delete_user(&self, username: &RegistryUsername) -> Result<()> {
		self.inner
			.send_json(
				self.inner.delete(&format!("/registry/users/{username}")),
			)
			.await
	}
}
