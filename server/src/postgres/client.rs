use crate::utils::cli::{CliError, WithMessage};
use tokio_postgres::{Client as PgClient, Config, NoTls};

pub struct Client {
	client: PgClient,
}

impl Client {
	pub async fn new() -> Result<Self, CliError> {
		let mut config = Config::new();
		config
			.host("/var/run/postgresql")
			.user("root")
			.dbname("postgres");

		let (client, connection) = config
			.connect(NoTls)
			.await
			.with_message("Failed to connect to PostgreSQL")?;

		// Spawn the connection task
		tokio::spawn(async move {
			if let Err(e) = connection.await {
				eprintln!("PostgreSQL connection error: {}", e);
			}
		});

		Ok(Self { client })
	}

	pub async fn create_superuser(
		&self,
		name: &str,
		password: &str,
	) -> Result<(), CliError> {
		let sql = format!(
			"CREATE USER {} WITH LOGIN SUPERUSER CREATEDB CREATEROLE \
            INHERIT NOREPLICATION CONNECTION LIMIT -1 PASSWORD '{}'",
			name, password
		);

		self.client
			.execute(&sql, &[])
			.await
			.with_message("Failed to create superuser")?;

		Ok(())
	}

	pub async fn create_user(
		&self,
		name: &str,
		password: &str,
	) -> Result<(), CliError> {
		let sql = format!(
			"CREATE USER {} WITH LOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE \
            INHERIT NOREPLICATION CONNECTION LIMIT -1 PASSWORD '{}'",
			name, password
		);

		self.client
			.execute(&sql, &[])
			.await
			.with_message(format!("Failed to create user {name}"))?;

		Ok(())
	}

	pub async fn drop_user(&self, name: &str) -> Result<(), CliError> {
		let sql = format!("DROP USER IF EXISTS {}", name);

		self.client
			.execute(&sql, &[])
			.await
			.with_message("Failed to drop user")?;

		Ok(())
	}

	pub async fn create_database(
		&self,
		name: &str,
		user: &str,
	) -> Result<(), CliError> {
		let sql = format!(
			"CREATE DATABASE {} WITH OWNER = {} ENCODING = 'UTF8' \
            CONNECTION LIMIT = -1",
			name, user
		);

		self.client
			.execute(&sql, &[])
			.await
			.with_message(format!("Failed to create database {name}"))?;

		Ok(())
	}

	pub async fn list_databases(&self) -> Result<Vec<String>, CliError> {
		let sql = "SELECT datname FROM pg_database WHERE datistemplate = false";

		let rows = self
			.client
			.query(sql, &[])
			.await
			.with_message("Failed to list databases")?;

		let databases = rows
			.into_iter()
			.map(|row| row.get::<_, String>(0))
			.collect();

		Ok(databases)
	}

	pub async fn database_exists(&self, name: &str) -> Result<bool, CliError> {
		let sql = "SELECT EXISTS(SELECT 1 FROM pg_database WHERE datname = $1)";

		let row = self.client.query_one(sql, &[&name]).await.with_message(
			format!("Failed to check if database {name} exists"),
		)?;

		Ok(row.get::<_, bool>(0))
	}

	pub async fn drop_database(&self, name: &str) -> Result<(), CliError> {
		let sql = format!("DROP DATABASE IF EXISTS {}", name);

		self.client
			.execute(&sql, &[])
			.await
			.with_message("Failed to drop database")?;

		Ok(())
	}

	pub async fn list_users(&self) -> Result<Vec<String>, CliError> {
		let sql = "SELECT usename FROM pg_user";

		let rows = self
			.client
			.query(sql, &[])
			.await
			.with_message("Failed to list users")?;

		let users = rows
			.into_iter()
			.map(|row| row.get::<_, String>(0))
			.collect();

		Ok(users)
	}
}
