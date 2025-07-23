use std::{
	collections::HashMap,
	sync::{Arc, RwLock},
};

use internal_api::app_id::AppId;
use pg::{Result, UniqueId, db::Conn};

use super::data::{App, AppsBuilderTrait, AppsTrait, AppsWithConn};

pub struct AppsBuilder {
	inner: Arc<Apps>,
}

impl AppsBuilder {
	pub fn new() -> Self {
		Self {
			inner: Arc::new(Apps::new()),
		}
	}
}

impl AppsBuilderTrait for AppsBuilder {
	fn with_conn<'a>(&'a self, _conn: Conn<'a>) -> AppsWithConn<'a> {
		Box::new(self.inner.clone())
	}
}

pub struct Apps {
	apps: RwLock<HashMap<AppId, App>>,
}

impl Apps {
	pub fn new() -> Self {
		Self {
			apps: RwLock::new(HashMap::new()),
		}
	}
}

#[async_trait::async_trait]
impl AppsTrait for Arc<Apps> {
	async fn all(&self) -> Result<Vec<App>> {
		let inner = self.apps.read().unwrap();
		Ok(inner.values().cloned().collect())
	}

	async fn all_by_team(&self, team_id: &UniqueId) -> Result<Vec<App>> {
		let inner = self.apps.read().unwrap();
		let apps = inner
			.values()
			.filter(|app| &app.team_id == team_id)
			.cloned()
			.collect();

		Ok(apps)
	}

	async fn by_id(&self, id: &AppId) -> Result<Option<App>> {
		let inner = self.apps.read().unwrap();
		Ok(inner.get(id).cloned())
	}

	async fn insert(&self, app: &App) -> Result<()> {
		let mut inner = self.apps.write().unwrap();
		inner.insert(app.id.clone(), app.clone());
		Ok(())
	}
}
