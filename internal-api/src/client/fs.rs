use bytes::Bytes;
use futures::{StreamExt as _, TryStream, stream::BoxStream};
use reqwest::{Body, header::CONTENT_DISPOSITION};
use serde::Serialize;

use crate::{
	Error,
	app_id::AppId,
	client::{ApiServerClient, Result},
	error::WithMessage,
	fs::{
		FsExtractReq, FsInfoRes, FsMoveReq, FsPath, FsPathBody, FsPathReq,
		FsPermissionsReq, FsWriteReq, FsZipReq,
	},
};

/// A file to download
pub struct FsDownload {
	/// from the content-disposition of the server
	pub filename: String,
	pub content_type: String,
	pub length: Option<u64>,
	pub stream: BoxStream<'static, Result<Bytes>>,
}

#[derive(Debug, Clone)]
pub struct ApiServerFsClient<'a> {
	inner: &'a ApiServerClient,
}

impl<'a> ApiServerFsClient<'a> {
	pub(crate) fn new(inner: &'a ApiServerClient) -> Self {
		Self { inner }
	}

	pub async fn info(&self, id: &AppId, path: &FsPath) -> Result<FsInfoRes> {
		self.inner
			.send_json(
				self.inner
					.get(&format!("/apps/{id}/fs"))
					.query(&FsPathReq { path: path.clone() }),
			)
			.await
	}

	pub async fn read(
		&self,
		id: &AppId,
		path: &FsPath,
	) -> Result<BoxStream<'static, Result<Bytes>>> {
		self.inner
			.send(
				self.inner
					.get(&format!("/apps/{id}/fs/read"))
					.query(&FsPathReq { path: path.clone() }),
			)
			.await
			.map(|res| {
				res.bytes_stream()
					.map(|r| r.with_message("failed to read the file"))
					.boxed()
			})
	}

	pub async fn download(
		&self,
		id: &AppId,
		path: &FsPath,
	) -> Result<FsDownload> {
		let res = self
			.inner
			.send(
				self.inner
					.get(&format!("/apps/{id}/fs/download"))
					.query(&FsPathReq { path: path.clone() }),
			)
			.await?;

		let header = |name| {
			res.headers()
				.get(name)
				.and_then(|v| v.to_str().ok())
				.map(str::to_string)
		};
		let filename = header(CONTENT_DISPOSITION)
			.and_then(|d| {
				d.split_once("filename=\"")
					.and_then(|(_, n)| n.strip_suffix('"'))
					.map(str::to_string)
			})
			.unwrap_or_else(|| "download".into());
		let content_type = header(reqwest::header::CONTENT_TYPE)
			.unwrap_or_else(|| "application/octet-stream".into());

		Ok(FsDownload {
			filename,
			content_type,
			length: res.content_length(),
			stream: res
				.bytes_stream()
				.map(|r| r.with_message("failed to download"))
				.boxed(),
		})
	}

	pub async fn write<S>(
		&self,
		id: &AppId,
		req: &FsWriteReq,
		stream: S,
	) -> Result<()>
	where
		S: TryStream<Ok = Bytes, Error = Error> + Send + 'static,
	{
		self.inner
			.send(
				self.inner
					.put(&format!("/apps/{id}/fs/write"))
					.query(req)
					.body(Body::wrap_stream(stream)),
			)
			.await
			.map(|_| ())
	}

	async fn post(
		&self,
		id: &AppId,
		action: &str,
		req: &impl Serialize,
	) -> Result<()> {
		self.inner
			.send(
				self.inner
					.post(&format!("/apps/{id}/fs/{action}"))
					.json(req),
			)
			.await
			.map(|_| ())
	}

	pub async fn mkdir(&self, id: &AppId, path: &FsPath) -> Result<()> {
		self.post(id, "mkdir", &FsPathBody { path: path.clone() })
			.await
	}

	pub async fn delete(&self, id: &AppId, path: &FsPath) -> Result<()> {
		self.post(id, "delete", &FsPathBody { path: path.clone() })
			.await
	}

	pub async fn rename(&self, id: &AppId, req: &FsMoveReq) -> Result<()> {
		self.post(id, "move", req).await
	}

	pub async fn copy(&self, id: &AppId, req: &FsMoveReq) -> Result<()> {
		self.post(id, "copy", req).await
	}

	pub async fn permissions(
		&self,
		id: &AppId,
		req: &FsPermissionsReq,
	) -> Result<()> {
		self.post(id, "permissions", req).await
	}

	pub async fn zip(&self, id: &AppId, req: &FsZipReq) -> Result<()> {
		self.post(id, "zip", req).await
	}

	pub async fn extract(&self, id: &AppId, req: &FsExtractReq) -> Result<()> {
		self.post(id, "extract", req).await
	}
}
