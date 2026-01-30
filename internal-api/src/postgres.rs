use serde::{Deserialize, Serialize};

pub use crate::database_name::DatabaseName;

/// A request to get a list of databases.
///
/// If the database was created via the webui, a user with the same name
/// will always exist.
///
/// URL: `/postgres/databases`
/// Method: `GET`
/// Authentication: Yes
pub struct PostgresDatabasesReq;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", transparent)]
pub struct PostgresDatabasesRes(pub Vec<String>);

/// A request to create new user
///
/// URL: `/postgres/databases`
/// Method: `POST`
/// Authentication: Yes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDatabaseReq {
	pub name: DatabaseName,
	// user and password is automatically generated
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDatabaseRes {
	/// user has the same name
	pub name: String,
	pub password: String,
}

/// A request to create a new password for the database
///
/// URL: `/postgres/databases/:database/password`
/// Method: `POST`
/// Authentication: Yes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewPasswordReq;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewPasswordRes {
	/// user has the same name
	pub name: String,
	pub password: String,
}

/// A request to upload a dump.
///
/// Provide the archive or sql directly (no json).
///
/// URL: `/postgres/databases/:database/restore`
/// Method: `PUT`
/// Authentication: Yes
pub struct PostgresDatabaseRestoreReq;

/// A request to get a dump of the database.
///
/// Returns the SQL dump directly (no json)
///
/// URL: `/postgres/databases/:database/dump`
/// Method: `GET`
/// Authentication: Yes
pub struct PostgresDatabaseDumpReq;

// todo implement once we have a better security model
// /// A request to delete a database and its user
// ///
// /// URL: `/postgres/databases/:database`
// /// Method: `DELETE`
// /// Authentication: Yes
// #[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct DeleteDatabaseReq;
