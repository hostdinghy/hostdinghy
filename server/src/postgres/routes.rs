use std::time::Duration;

use api::{
	Error,
	error::WithMessage,
	postgres::{
		CreateDatabaseReq, CreateDatabaseRes, DatabaseName, NewPasswordRes,
		PostgresDatabasesRes,
	},
};
use axum::{
	Json, Router,
	body::Body,
	extract::Path,
	routing::{get, post, put},
};
use chuchi_crypto::token::Token;
use futures::TryStreamExt;
use tokio::{
	io::{self, AsyncReadExt as _, AsyncWriteExt as _},
	time::sleep,
};
use tokio_util::io::{ReaderStream, StreamReader};

use crate::{
	postgres::{Client, utils},
	server::{Authenticated, router::AppState},
};

async fn databases(
	_auth: Authenticated,
) -> Result<Json<PostgresDatabasesRes>, Error> {
	let client = Client::new().await?;

	let dbs = client
		.list_databases()
		.await
		.with_message("Failed to list Postgres databases")?;

	// postgres and the studio's own database cannot be used through the api
	// (see DatabaseName), so don't list them
	let dbs = dbs
		.into_iter()
		.filter(|name| name.parse::<DatabaseName>().is_ok())
		.collect();

	Ok(Json(PostgresDatabasesRes(dbs)))
}

async fn create_database(
	_auth: Authenticated,
	Json(req): Json<CreateDatabaseReq>,
) -> Result<Json<CreateDatabaseRes>, Error> {
	let client = Client::new().await?;

	// this is technically a race condition but it should never occur
	if client
		.database_exists(req.name.as_ref())
		.await
		.with_message("db error")?
	{
		return Err(Error::DatabaseAlreadyExists);
	}

	let password = Token::<32>::new().to_string();

	client
		.create_user(req.name.as_ref(), &password)
		.await
		.with_message("Failed to create Postgres user")?;

	client
		.create_database(req.name.as_ref(), req.name.as_ref())
		.await
		.with_message("Failed to create Postgres database")?;

	Ok(Json(CreateDatabaseRes {
		name: req.name.into(),
		password,
	}))
}

async fn new_password(
	_auth: Authenticated,
	Path(name): Path<DatabaseName>,
) -> Result<Json<NewPasswordRes>, Error> {
	let client = Client::new().await?;

	if !client
		.database_exists(name.as_ref())
		.await
		.with_message("db error")?
	{
		return Err(Error::DatabaseNotFound);
	}

	let password = Token::<32>::new().to_string();

	client
		.update_password(name.as_ref(), &password)
		.await
		.with_message("Failed to update Postgres user password")?;

	Ok(Json(NewPasswordRes {
		name: name.into(),
		password,
	}))
}

async fn restore_database(
	_auth: Authenticated,
	Path(name): Path<DatabaseName>,
	body: Body,
) -> Result<(), Error> {
	let client = Client::new().await?;

	if !client
		.database_exists(name.as_ref())
		.await
		.with_message("db error")?
	{
		return Err(Error::DatabaseNotFound);
	}

	let mut body = StreamReader::new(
		body.into_data_stream()
			.map_err(|err| io::Error::new(io::ErrorKind::Other, err)),
	);

	// archives made with pg_dump -Fc start with PGDMP, anything else
	// is treated as plain sql
	let mut magic = Vec::with_capacity(5);
	(&mut body)
		.take(5)
		.read_to_end(&mut magic)
		.await
		.with_message("Failed to read the restore file")?;

	let mut child = if magic == b"PGDMP" {
		utils::restore_database(name.as_ref()).await
	} else {
		utils::restore_database_sql(name.as_ref()).await
	}
	.with_message("Failed to start Postgres restore process")?;

	let res = async {
		child.write_all(&magic).await?;
		io::copy(&mut body, &mut child).await
	}
	.await;

	// wait also closes stdin, the restore only finishes after that.
	// If writing failed because the process exited, its error is better
	let res = match child.wait().await {
		Ok(()) => res.with_message("Failed to restore Postgres database"),
		Err(e) => Err(e).with_message("Postgres restore failed"),
	};

	if res.is_err() {
		// read the rest of the upload, a proxy in between (traefik, nginx)
		// might turn an answer before the end of the request into a 502
		let _ = io::copy(&mut body, &mut io::sink()).await;
	}
	res?;

	Ok(())
}

async fn dump_database(
	_auth: Authenticated,
	Path(name): Path<DatabaseName>,
) -> Result<Body, Error> {
	let client = Client::new().await?;

	if !client
		.database_exists(name.as_ref())
		.await
		.with_message("db error")?
	{
		return Err(Error::DatabaseNotFound);
	}

	let mut child = utils::dump_database(name.as_ref())
		.await
		.with_message("Failed to start Postgres dump process")?;

	// lets wait until the process hopefully has decided
	// if the operation will work or not
	sleep(Duration::from_millis(10)).await;

	if child.exited_with_error() {
		match child.wait().await {
			Ok(()) => unreachable!("child exit status changed"),
			Err(e) => return Err(e.into()),
		};
	}

	// make sure to wait until the child has exited in the stream
	child.wait_for_child_exit(true);

	Ok(Body::from_stream(ReaderStream::new(child)))
}

pub fn routes() -> Router<AppState> {
	Router::new()
		.route("/databases", get(databases).post(create_database))
		.route("/databases/{name}/password", post(new_password))
		.route("/databases/{name}/restore", put(restore_database))
		.route("/databases/{name}/dump", get(dump_database))
}
