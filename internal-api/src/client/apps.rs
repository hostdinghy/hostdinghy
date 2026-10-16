use bytes::Bytes;
use futures::{StreamExt as _, stream::BoxStream};

use crate::{
	Error,
	app_id::AppId,
	apps::{
		AppInfoRes, CommandEvent, ComposeCommand, GetComposeRes, SaveComposeReq,
	},
	client::{ApiServerClient, Result},
	error::WithMessage,
};

#[derive(Debug, Clone)]
pub struct ApiServerAppsClient<'a> {
	inner: &'a ApiServerClient,
}

impl<'a> ApiServerAppsClient<'a> {
	pub(crate) fn new(inner: &'a ApiServerClient) -> Self {
		Self { inner }
	}

	pub async fn app_info(&self, id: &AppId) -> Result<AppInfoRes> {
		self.inner
			.send_json(self.inner.get(&format!("/apps/{id}")))
			.await
	}

	pub async fn get_compose(&self, id: &AppId) -> Result<GetComposeRes> {
		self.inner
			.send_json(self.inner.get(&format!("/apps/{id}/compose")))
			.await
	}

	pub async fn set_compose(
		&self,
		id: &AppId,
		req: &SaveComposeReq,
	) -> Result<()> {
		self.inner
			.send(self.inner.post(&format!("/apps/{id}/compose")).json(req))
			.await
			.map(|_| ())
	}

	/// Runs the command and waits until it is done
	pub async fn compose_command(
		&self,
		id: &AppId,
		service: Option<&str>,
		cmd: &ComposeCommand,
	) -> Result<()> {
		let output = self
			.inner
			.send(self.inner.post(&compose_command_path(id, service, cmd)))
			.await?
			.text()
			.await
			.with_message("failed to read the command output")?;

		// the last event is the result
		let last = output
			.lines()
			.filter_map(|l| serde_json::from_str::<CommandEvent>(l).ok())
			.last();
		match last {
			Some(CommandEvent::Done) => Ok(()),
			Some(CommandEvent::Error { message }) => {
				Err(Error::any("compose command failed", message))
			}
			_ => Err(Error::any(
				"compose command failed",
				"the output ended without a result",
			)),
		}
	}

	/// Returns the events of the command as json lines, see [`CommandEvent`]
	pub async fn compose_command_stream(
		&self,
		id: &AppId,
		service: Option<&str>,
		cmd: &ComposeCommand,
	) -> Result<BoxStream<'static, Result<Bytes>>> {
		self.inner
			.send(self.inner.post(&compose_command_path(id, service, cmd)))
			.await
			.map(|res| {
				res.bytes_stream()
					.map(|r| r.with_message("compose command failed"))
					.boxed()
			})
	}

	pub async fn app_logs(
		&self,
		id: &AppId,
		lines: Option<u32>,
	) -> Result<String> {
		let mut uri = format!("/apps/{id}/logs");
		if let Some(lines) = lines {
			// todo could this be a query type?
			uri.push_str(&format!("?lines={lines}"));
		}

		let response = self.inner.send(self.inner.get(&uri)).await?;

		response
			.text()
			.await
			.with_message("failed to parse logs response")
	}
}

fn compose_command_path(
	id: &AppId,
	service: Option<&str>,
	cmd: &ComposeCommand,
) -> String {
	match service {
		Some(service) => format!("/apps/{id}/service/{service}/action/{cmd}"),
		None => format!("/apps/{id}/action/{cmd}"),
	}
}
