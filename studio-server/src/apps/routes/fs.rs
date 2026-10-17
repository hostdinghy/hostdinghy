//! The files of an app, passed through to its server
//! (see [`internal_api::fs`])

use axum::body::Body;
use axum::extract::{Path, Query, State};
use axum::http::header::{CONTENT_DISPOSITION, CONTENT_LENGTH, CONTENT_TYPE};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post, put};
use axum::{Json, Router};
use futures::StreamExt as _;
use internal_api::apps::AppId;
use internal_api::error::WithMessage as _;
use internal_api::fs::{
	FsExtractReq, FsInfoRes, FsMoveReq, FsPathBody, FsPathReq,
	FsPermissionsReq, FsWriteReq, FsZipReq,
};

use crate::AppState;
use crate::apps::Apps;
use crate::apps::routes::utils::{AppWithServer, app_with_server};
use crate::error::Result;
use crate::internal::{ApiClient, ApiServerClient};
use crate::servers::Servers;
use crate::users::utils::AuthedUser;
use crate::users::utils::RightsAny;
use crate::utils::ConnOwned;

/// Checks that the user may access the app and connects to its server
async fn api(
	id: &AppId,
	user: &AuthedUser<RightsAny>,
	apps: &Apps,
	servers: &Servers,
	api_client: &ApiClient,
	conn: &ConnOwned,
) -> Result<ApiServerClient> {
	let apps = apps.with_conn(conn.conn());
	let servers = servers.with_conn(conn.conn());

	let AppWithServer { api, .. } =
		app_with_server(id, user, &apps, &servers, api_client).await?;
	Ok(api)
}

/// the extractors every handler needs
macro_rules! fs_handler {
	(
		$name:ident($($arg:ident: $ty:ty),*) -> $ret:ty
		|$api:ident, $id:ident| $body:expr
	) => {
		pub async fn $name(
			user: AuthedUser<RightsAny>,
			State(apps): State<Apps>,
			State(servers): State<Servers>,
			State(api_client): State<ApiClient>,
			Path($id): Path<AppId>,
			conn: ConnOwned,
			$($arg: $ty),*
		) -> Result<$ret> {
			let $api =
				api(&$id, &user, &apps, &servers, &api_client, &conn).await?;
			$body
		}
	};
}

fs_handler!(info(req: Query<FsPathReq>) -> Json<FsInfoRes> |api, id| {
	Ok(Json(api.fs().info(&id, &req.path).await?))
});

fs_handler!(read(req: Query<FsPathReq>) -> Response |api, id| {
	let stream = api.fs().read(&id, &req.path).await?;
	Ok((
		[(CONTENT_TYPE, "application/octet-stream")],
		Body::from_stream(stream),
	)
		.into_response())
});

fs_handler!(download(req: Query<FsPathReq>) -> Response |api, id| {
	let dl = api.fs().download(&id, &req.path).await?;
	let mut res = (
		[
			(CONTENT_TYPE, dl.content_type),
			(
				CONTENT_DISPOSITION,
				format!("attachment; filename=\"{}\"", dl.filename),
			),
		],
		Body::from_stream(dl.stream),
	)
		.into_response();
	if let Some(len) = dl.length {
		res.headers_mut().insert(CONTENT_LENGTH, len.into());
	}
	Ok(res)
});

fs_handler!(write(req: Query<FsWriteReq>, body: Body) -> Json<()> |api, id| {
	let stream = body
		.into_data_stream()
		.map(|r| r.with_message("failed to read the upload"))
		.boxed();
	api.fs().write(&id, &req, stream).await?;
	Ok(Json(()))
});

fs_handler!(mkdir(req: Json<FsPathBody>) -> Json<()> |api, id| {
	api.fs().mkdir(&id, &req.path).await?;
	Ok(Json(()))
});

fs_handler!(delete(req: Json<FsPathBody>) -> Json<()> |api, id| {
	api.fs().delete(&id, &req.path).await?;
	Ok(Json(()))
});

fs_handler!(rename(req: Json<FsMoveReq>) -> Json<()> |api, id| {
	api.fs().rename(&id, &req).await?;
	Ok(Json(()))
});

fs_handler!(copy(req: Json<FsMoveReq>) -> Json<()> |api, id| {
	api.fs().copy(&id, &req).await?;
	Ok(Json(()))
});

fs_handler!(permissions(req: Json<FsPermissionsReq>) -> Json<()> |api, id| {
	api.fs().permissions(&id, &req).await?;
	Ok(Json(()))
});

fs_handler!(zip(req: Json<FsZipReq>) -> Json<()> |api, id| {
	api.fs().zip(&id, &req).await?;
	Ok(Json(()))
});

fs_handler!(extract(req: Json<FsExtractReq>) -> Json<()> |api, id| {
	api.fs().extract(&id, &req).await?;
	Ok(Json(()))
});

pub fn routes() -> Router<AppState> {
	Router::new()
		.route("/{id}/fs", get(info))
		.route("/{id}/fs/read", get(read))
		.route("/{id}/fs/download", get(download))
		.route("/{id}/fs/write", put(write))
		.route("/{id}/fs/mkdir", post(mkdir))
		.route("/{id}/fs/delete", post(delete))
		.route("/{id}/fs/move", post(rename))
		.route("/{id}/fs/copy", post(copy))
		.route("/{id}/fs/permissions", post(permissions))
		.route("/{id}/fs/zip", post(zip))
		.route("/{id}/fs/extract", post(extract))
}
