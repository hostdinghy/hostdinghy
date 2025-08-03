use clap::Parser;
use pg::{UniqueId, db::ConnOwned, time::DateTime};

use crate::{
	AppState,
	teams::data::Team,
	users::data::{Rights, UnsafeUser, User},
};

/// Will always create a root user
#[derive(Debug, Parser)]
pub struct CreateUser {
	username: String,
	password: String,
	#[clap(long)]
	json: bool,
}

pub async fn create_user(
	conn: &mut ConnOwned,
	state: &AppState,
	cu: CreateUser,
) {
	let users = state.users.with_conn(conn.conn());
	let teams = state.teams.with_conn(conn.conn());

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

	let user = User::from(user);

	if cu.json {
		print!("{}", serde_json::to_string(&user).unwrap());
	} else {
		println!("created new user {user:#?}");
	}
}
