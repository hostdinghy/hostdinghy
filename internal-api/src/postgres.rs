use serde::{Deserialize, Serialize};

/// A request to get a list of all databases.
///
/// URL: `/postgres/databases`
/// Method: `GET`
/// Authentication: Yes
pub struct PostgresDatabasesReq;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", transparent)]
pub struct PostgresDatabasesRes(pub Vec<String>);

/// A request to create new database and user
///
/// URL: `/postgres/databases`
/// Method: `POST`
/// Authentication: Yes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDatabaseReq {
	pub name: String,
	// password and username is automatically generated
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDatabaseRes {
	pub name: String,
	// username is the same as name
	// pub username: String,
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
	pub name: String,
	pub password: String,
}

/// A request to get a database dump.
///
/// URL: `/postgres/databases/:database/dump`
/// Method: `GET`
/// Authentication: Yes
pub struct PostgresDatabaseDumpReq;
