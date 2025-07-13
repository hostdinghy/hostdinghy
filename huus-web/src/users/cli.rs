use clap::Parser;
use pg::{UniqueId, db::ConnOwned, time::DateTime};

use crate::{
	teams::{
		data::{Team, TeamsBuilderTrait},
		database::TeamsBuilder,
	},
	users::{
		data::{Rights, UnsafeUser, UsersBuilderTrait},
		database::UsersBuilder,
	},
};

/// Will always create a root user
#[derive(Debug, Parser)]
pub struct CreateUser {
	username: String,
	password: String,
}

pub async fn create_user(
	conn: &mut ConnOwned,
	users: &UsersBuilder,
	teams: &TeamsBuilder,
	cu: CreateUser,
) {
	let users = users.with_conn(conn.conn());
	let teams = teams.with_conn(conn.conn());

	let team = Team {
		id: UniqueId::new(),
		name: cu.username.clone(),
		created_on: DateTime::now(),
	};
	teams.insert(&team).await.unwrap();

	let user = UnsafeUser {
		id: UniqueId::new(),
		team_id: team.id,
		username: cu.username.clone(),
		name: cu.username,
		password: bcrypt::hash(cu.password.as_str(), 10)
			.expect("failed to hash password"),
		rights: Rights {
			admin: true,
			root: true,
		},
		created_on: DateTime::now(),
	};
	users.insert(&user).await.unwrap();

	println!("created new user {user:#?}");
}
