use clap::Parser;
use pg::{UniqueId, db::ConnOwned, time::DateTime};

use crate::users::{
	data::{UnsafeUser, UsersBuilderTrait},
	database::UsersBuilder,
};

#[derive(Debug, Parser)]
pub struct CreateUser {
	username: String,
	password: String,
}

pub async fn create_user(
	conn: &mut ConnOwned,
	users: &UsersBuilder,
	cu: CreateUser,
) {
	let users = users.with_conn(conn.conn());

	let user = UnsafeUser {
		id: UniqueId::new(),
		username: cu.username.clone(),
		name: cu.username,
		password: bcrypt::hash(cu.password.as_str(), 10)
			.expect("failed to hash password"),
		created_on: DateTime::now(),
	};
	users.insert(&user).await.unwrap();

	println!("created new user {user:#?}");
}
