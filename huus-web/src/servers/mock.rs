use std::{
	collections::HashMap,
	sync::{Arc, RwLock},
};

use pg::{Result, UniqueId, db::Conn};

use super::data::{Server, ServersBuilderTrait, ServersTrait, ServersWithConn};

pub struct ServersBuilder {
	inner: Arc<Servers>,
}

impl ServersBuilder {
	pub fn new() -> Self {
		Self {
			inner: Arc::new(Servers::new()),
		}
	}
}

impl ServersBuilderTrait for ServersBuilder {
	fn with_conn<'a>(&'a self, _conn: Conn<'a>) -> ServersWithConn<'a> {
		Box::new(self.inner.clone())
	}
}

pub struct Servers {
	servers: RwLock<HashMap<UniqueId, Server>>,
}

impl Servers {
	pub fn new() -> Self {
		Self {
			servers: RwLock::new(HashMap::new()),
		}
	}
}

#[async_trait::async_trait]
impl ServersTrait for Arc<Servers> {
	async fn all(&self, team_id: &Option<UniqueId>) -> Result<Vec<Server>> {
		let inner = self.servers.read().unwrap();
		if let Some(team_id) = team_id {
			let servers = inner
				.values()
				.filter(|server| &server.team_id == team_id)
				.cloned()
				.collect();
			Ok(servers)
		} else {
			Ok(inner.values().cloned().collect())
		}
	}

	async fn by_id(
		&self,
		id: &UniqueId,
		team_id: &Option<UniqueId>,
	) -> Result<Option<Server>> {
		let inner = self.servers.read().unwrap();

		let server = inner
			.get(id)
			// Filter by team_id if provided
			.filter(|server| {
				team_id.map(|t| server.team_id == t).unwrap_or(true)
			})
			.cloned();

		Ok(server)
	}

	async fn insert(&self, server: &Server) -> Result<()> {
		let mut inner = self.servers.write().unwrap();
		inner.insert(server.id, server.clone());
		Ok(())
	}
}
