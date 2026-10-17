use api::{
	apps::AppId,
	error::{Error, WithMessage as _},
	fs::{
		FsExtractReq, FsInfoRes, FsMoveReq, FsPathBody, FsPathReq,
		FsPermissionsReq, FsWriteReq, FsZipReq,
	},
};
use axum::{
	Json, Router,
	body::Body,
	extract::{Path, Query},
	http::header::{CONTENT_DISPOSITION, CONTENT_LENGTH, CONTENT_TYPE},
	response::{IntoResponse, Response},
	routing::{get, post, put},
};
use futures::TryStreamExt as _;
use tokio::{fs::File, io};
use tokio_util::io::{ReaderStream, StreamReader};

use crate::{
	fs::{AppFs, blocking, io_err},
	server::{Authenticated, router::AppState},
	utils::cmd::cmd,
};

async fn info(
	_auth: Authenticated,
	Path(id): Path<AppId>,
	Query(req): Query<FsPathReq>,
) -> Result<Json<FsInfoRes>, Error> {
	let fs = AppFs::open(&id).await?;
	blocking(move || fs.info(&req.path)).await.map(Json)
}

async fn read(
	_auth: Authenticated,
	Path(id): Path<AppId>,
	Query(req): Query<FsPathReq>,
) -> Result<Response, Error> {
	let fs = AppFs::open(&id).await?;
	let (full, len) = blocking(move || fs.file(&req.path)).await?;
	let file = File::open(full)
		.await
		.map_err(|e| io_err("failed to open file", e))?;

	Ok((
		[
			(CONTENT_TYPE, "application/octet-stream".to_string()),
			(CONTENT_LENGTH, len.to_string()),
		],
		Body::from_stream(ReaderStream::new(file)),
	)
		.into_response())
}

/// A file as it is, a directory as .tar.gz
async fn download(
	_auth: Authenticated,
	Path(id): Path<AppId>,
	Query(req): Query<FsPathReq>,
) -> Result<Response, Error> {
	let fs = AppFs::open(&id).await?;
	let name = match req.path.is_root() {
		true => id.to_string(),
		false => req.path.name().to_string(),
	};
	// keep the header simple, the client may name it differently
	let name: String = name
		.chars()
		.map(|c| {
			if c.is_ascii_graphic() && c != '"' {
				c
			} else {
				'_'
			}
		})
		.collect();

	let full = blocking(move || fs.follow(&req.path)).await?;
	if full.is_file() {
		let file = File::open(&full)
			.await
			.map_err(|e| io_err("failed to open file", e))?;
		let len = file.metadata().await.map(|m| m.len()).unwrap_or(0);

		return Ok((
			[
				(CONTENT_TYPE, "application/octet-stream".to_string()),
				(CONTENT_LENGTH, len.to_string()),
				(
					CONTENT_DISPOSITION,
					format!("attachment; filename=\"{name}\""),
				),
			],
			Body::from_stream(ReaderStream::new(file)),
		)
			.into_response());
	}

	// tar doesn't follow symlinks, it stores them as links
	let dir = full.to_string_lossy().into_owned();
	let mut stdout = cmd(&["tar", "-czf", "-", "-C", &dir, "."])
		.spawn_readable_stdout()
		.await?;
	// an error during the archive ends the stream with an error
	stdout.wait_for_child_exit(true);

	Ok((
		[
			(CONTENT_TYPE, "application/gzip".to_string()),
			(
				CONTENT_DISPOSITION,
				format!("attachment; filename=\"{name}.tar.gz\""),
			),
		],
		Body::from_stream(ReaderStream::new(stdout)),
	)
		.into_response())
}

async fn write(
	_auth: Authenticated,
	Path(id): Path<AppId>,
	Query(req): Query<FsWriteReq>,
	body: Body,
) -> Result<(), Error> {
	let fs = AppFs::open(&id).await?;
	let n_fs = fs.clone();
	let (target, tmp) =
		blocking(move || n_fs.prepare_write(&req.path, req.overwrite)).await?;

	let mut body =
		StreamReader::new(body.into_data_stream().map_err(io::Error::other));

	let res = async {
		let mut file = File::create_new(&tmp).await?;
		io::copy(&mut body, &mut file).await?;
		file.sync_all().await
	}
	.await;

	if let Err(e) = res {
		let _ = tokio::fs::remove_file(&tmp).await;
		return Err(e).with_message("Failed to write the file");
	}

	blocking(move || fs.finish_write(&target, &tmp)).await
}

async fn mkdir(
	_auth: Authenticated,
	Path(id): Path<AppId>,
	Json(req): Json<FsPathBody>,
) -> Result<(), Error> {
	let fs = AppFs::open(&id).await?;
	blocking(move || fs.mkdir(&req.path)).await
}

async fn delete(
	_auth: Authenticated,
	Path(id): Path<AppId>,
	Json(req): Json<FsPathBody>,
) -> Result<(), Error> {
	let fs = AppFs::open(&id).await?;
	blocking(move || fs.delete(&req.path)).await
}

async fn rename(
	_auth: Authenticated,
	Path(id): Path<AppId>,
	Json(req): Json<FsMoveReq>,
) -> Result<(), Error> {
	let fs = AppFs::open(&id).await?;
	blocking(move || fs.rename(&req.from, &req.to)).await
}

async fn copy(
	_auth: Authenticated,
	Path(id): Path<AppId>,
	Json(req): Json<FsMoveReq>,
) -> Result<(), Error> {
	let fs = AppFs::open(&id).await?;
	blocking(move || fs.copy(&req.from, &req.to)).await
}

async fn permissions(
	_auth: Authenticated,
	Path(id): Path<AppId>,
	Json(req): Json<FsPermissionsReq>,
) -> Result<(), Error> {
	let fs = AppFs::open(&id).await?;
	blocking(move || fs.permissions(&req)).await
}

async fn zip(
	_auth: Authenticated,
	Path(id): Path<AppId>,
	Json(req): Json<FsZipReq>,
) -> Result<(), Error> {
	let fs = AppFs::open(&id).await?;
	blocking(move || fs.zip(&req.path, &req.to)).await
}

async fn extract(
	_auth: Authenticated,
	Path(id): Path<AppId>,
	Json(req): Json<FsExtractReq>,
) -> Result<(), Error> {
	let fs = AppFs::open(&id).await?;
	blocking(move || fs.extract(&req.path, &req.to)).await
}

/// nested under `/apps/{id}/fs`
pub fn routes() -> Router<AppState> {
	Router::new()
		.route("/", get(info))
		.route("/read", get(read))
		.route("/download", get(download))
		.route("/write", put(write))
		.route("/mkdir", post(mkdir))
		.route("/delete", post(delete))
		.route("/move", post(rename))
		.route("/copy", post(copy))
		.route("/permissions", post(permissions))
		.route("/zip", post(zip))
		.route("/extract", post(extract))
}
