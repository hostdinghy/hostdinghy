use serde::{Deserialize, Serialize};

pub use crate::registry_username::RegistryUsername;

/// A request to get a list of all registry users.
///
/// URL: `/registry/users`
/// Method: `GET`
/// Authentication: Yes
pub struct RegistryUsersReq;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", transparent)]
pub struct RegistryUsersRes(pub Vec<String>);

/// A request to create new user
///
/// URL: `/registry/users`
/// Method: `POST`
/// Authentication: Yes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserReq {
	pub username: RegistryUsername,
	// password is automatically generated
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserRes {
	pub username: String,
	pub password: String,
}

/// A request to delete a user
///
/// URL: `/registry/users/:username`
/// Method: `DELETE`
/// Authentication: Yes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteUserReq;
