use std::{
	collections::HashMap,
	sync::{Arc, RwLock},
};

use pg::{Result, UniqueId, db::Conn};

use crate::teams::data::{Team, TeamsBuilderTrait, TeamsWithConn};

use super::data::TeamsTrait;

pub struct TeamsBuilder {
	inner: Arc<Teams>,
}

impl TeamsBuilder {
	pub fn new() -> Self {
		Self {
			inner: Arc::new(Teams::new()),
		}
	}
}

impl TeamsBuilderTrait for TeamsBuilder {
	fn with_conn<'a>(&'a self, _conn: Conn<'a>) -> TeamsWithConn<'a> {
		Box::new(self.inner.clone())
	}
}

pub struct Teams {
	teams: RwLock<HashMap<UniqueId, Team>>,
}

impl Teams {
	pub fn new() -> Self {
		Self {
			teams: RwLock::new(HashMap::new()),
		}
	}
}

#[async_trait::async_trait]
impl TeamsTrait for Arc<Teams> {
	async fn by_id(&self, id: &UniqueId) -> Result<Option<Team>> {
		let inner = self.teams.read().unwrap();
		Ok(inner.get(id).cloned())
	}

	async fn insert(&self, team: &Team) -> Result<()> {
		let mut inner = self.teams.write().unwrap();
		inner.insert(team.id, team.clone());
		Ok(())
	}
}
