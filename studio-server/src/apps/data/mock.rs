use std::{
	collections::HashMap,
	sync::{Arc, RwLock},
};

use internal_api::app_id::AppId;
use pg::{Error, Result, UniqueId, db::Conn, try2};

use super::{App, AppsBuilderTrait, AppsTrait, AppsWithConn};

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
	async fn all(&self, team_id: &Option<UniqueId>) -> Result<Vec<App>> {
		let inner = self.apps.read().unwrap();

		if let Some(team_id) = team_id {
			let apps = inner
				.values()
				.filter(|app| &app.team_id == team_id)
				.cloned()
				.collect();
			Ok(apps)
		} else {
			Ok(inner.values().cloned().collect())
		}
	}

	async fn by_id(
		&self,
		id: &AppId,
		team_id: &Option<UniqueId>,
	) -> Result<Option<App>> {
		let inner = self.apps.read().unwrap();

		let app = inner
			.get(id)
			// Filter by team_id if provided
			.filter(|app| team_id.map(|t| app.team_id == t).unwrap_or(true))
			.cloned();

		Ok(app)
	}

	async fn insert(&self, app: &App) -> Result<()> {
		let mut inner = self.apps.write().unwrap();
		if inner.contains_key(&app.id) {
			return Err(Error::unique_violation(Some("id".into())));
		}

		inner.insert(app.id.clone(), app.clone());
		Ok(())
	}
}
