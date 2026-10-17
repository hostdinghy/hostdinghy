//! Files of an app, relative to its directory (`$HOSTDINGHY_DIR/<app id>`)
//!
//! All requests take the path as query or json field, `""` is the app
//! directory itself.

use std::{fmt, str::FromStr};

use chuchi_postgres::time::DateTime;
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error as _};

/// A relative path inside an app directory
///
/// Segments are separated by `/` and can't be empty, `.` or `..`, so the
/// path can't leave the directory by itself (symlinks are checked by the
/// server). The empty path is the app directory.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct FsPath(String);

impl FsPath {
	pub fn root() -> Self {
		Self::default()
	}

	pub fn is_root(&self) -> bool {
		self.0.is_empty()
	}

	pub fn segments(&self) -> impl Iterator<Item = &str> {
		self.0.split('/').filter(|s| !s.is_empty())
	}

	/// The last segment, empty for the root
	pub fn name(&self) -> &str {
		self.0.rsplit('/').next().unwrap_or("")
	}

	/// None for the root
	pub fn parent(&self) -> Option<FsPath> {
		if self.is_root() {
			return None;
		}

		Some(Self(
			self.0.rsplit_once('/').map_or("", |(p, _)| p).to_string(),
		))
	}

	pub fn join(&self, name: &str) -> Result<FsPath, InvalidFsPath> {
		if self.is_root() {
			name.parse()
		} else {
			format!("{}/{name}", self.0).parse()
		}
	}

	/// If self is other or inside of it
	pub fn starts_with(&self, other: &FsPath) -> bool {
		other.is_root()
			|| self.0 == other.0
			|| self
				.0
				.strip_prefix(&other.0)
				.is_some_and(|r| r.starts_with('/'))
	}
}

impl fmt::Display for FsPath {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::Display::fmt(&self.0, f)
	}
}

impl AsRef<str> for FsPath {
	fn as_ref(&self) -> &str {
		&self.0
	}
}

impl FromStr for FsPath {
	type Err = InvalidFsPath;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		// allow a leading or trailing slash, the ui uses them in urls
		let s = s.trim_matches('/');
		if s.is_empty() {
			return Ok(Self::root());
		}

		let valid = s.len() <= 4096
			&& s.split('/').all(|seg| {
				!seg.is_empty()
					&& seg != "." && seg != ".."
					&& seg.len() <= 255
					&& !seg.contains('\0')
			});

		valid.then(|| Self(s.into())).ok_or(InvalidFsPath {})
	}
}

#[derive(Debug, thiserror::Error)]
#[error("Invalid path - segments separated by / and not empty, `.` or `..`")]
pub struct InvalidFsPath {}

impl Serialize for FsPath {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		serializer.serialize_str(&self.0)
	}
}

impl<'de> Deserialize<'de> for FsPath {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		let s: std::borrow::Cow<'_, str> =
			Deserialize::deserialize(deserializer)?;
		s.parse().map_err(D::Error::custom)
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FsKind {
	File,
	Directory,
	Symlink,
	/// sockets, devices...
	Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FsEntry {
	/// empty for the app directory
	pub name: String,
	pub kind: FsKind,
	/// in bytes, 0 for directories
	pub size: u64,
	/// the permission bits, e.g. 0o644
	pub mode: u32,
	pub uid: u32,
	pub gid: u32,
	pub modified: DateTime,
	/// where a symlink points to
	pub link_target: Option<String>,
}

/// Info about a file or directory, with the entries of a directory
///
/// URL: `/apps/:id/fs?path=<path>`
/// Method: `GET`
/// Authentication: Yes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FsInfoRes {
	pub entry: FsEntry,
	/// sorted, directories first, None if it isn't a directory
	pub children: Option<Vec<FsEntry>>,
}

/// URL: `/apps/:id/fs?path=<path>` (`GET`)
///
/// Or `/apps/:id/fs/read?path=<path>` (`GET`): the content of a file
///
/// Or `/apps/:id/fs/download?path=<path>` (`GET`): a file, or a directory
/// as `.tar.gz`, with a content-disposition header
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FsPathReq {
	#[serde(default)]
	pub path: FsPath,
}

/// Writes the request body to a file, creates missing parent directories
///
/// URL: `/apps/:id/fs/write?path=<path>&overwrite=<bool>`
/// Method: `PUT`
/// Authentication: Yes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FsWriteReq {
	pub path: FsPath,
	/// replace an existing file, otherwise it is an error
	#[serde(default)]
	pub overwrite: bool,
}

/// URL: `/apps/:id/fs/mkdir` (`POST`, json): creates missing parents too
///
/// Or `/apps/:id/fs/delete` (`POST`, json): directories with everything
/// inside
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FsPathBody {
	pub path: FsPath,
}

/// URL: `/apps/:id/fs/move` or `/apps/:id/fs/copy`
/// Method: `POST`
/// Authentication: Yes
///
/// `to` is the new path (not the directory to move into) and must not exist
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FsMoveReq {
	pub from: FsPath,
	pub to: FsPath,
}

/// Changes mode and/or owner
///
/// URL: `/apps/:id/fs/permissions`
/// Method: `POST`
/// Authentication: Yes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FsPermissionsReq {
	pub path: FsPath,
	/// permission bits, e.g. 0o755
	pub mode: Option<u32>,
	pub uid: Option<u32>,
	pub gid: Option<u32>,
	/// for everything inside a directory too (symlinks are skipped)
	#[serde(default)]
	pub recursive: bool,
}

/// Packs a file or directory into a new `.zip` file
///
/// URL: `/apps/:id/fs/zip`
/// Method: `POST`
/// Authentication: Yes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FsZipReq {
	pub path: FsPath,
	/// the archive to create, must not exist
	pub to: FsPath,
}

/// Extracts a `.zip`, `.tar` or `.tar.gz`/`.tgz` into a new directory
///
/// URL: `/apps/:id/fs/extract`
/// Method: `POST`
/// Authentication: Yes
///
/// Symlinks and other special entries in the archive are skipped.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FsExtractReq {
	pub path: FsPath,
	/// the directory to create, must not exist
	pub to: FsPath,
}

/// The archive formats extract supports, by file name
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArchiveKind {
	Zip,
	Tar,
	TarGz,
}

impl ArchiveKind {
	pub fn from_name(name: &str) -> Option<Self> {
		let name = name.to_ascii_lowercase();
		if name.ends_with(".zip") {
			Some(Self::Zip)
		} else if name.ends_with(".tar.gz") || name.ends_with(".tgz") {
			Some(Self::TarGz)
		} else if name.ends_with(".tar") {
			Some(Self::Tar)
		} else {
			None
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn paths() {
		assert!("".parse::<FsPath>().unwrap().is_root());
		assert_eq!("/a/b/".parse::<FsPath>().unwrap().as_ref(), "a/b");
		for bad in ["a/../b", "..", "a//b", "./a", "a/.", "a\0b"] {
			assert!(bad.parse::<FsPath>().is_err(), "{bad}");
		}

		let p: FsPath = "data/config.toml".parse().unwrap();
		assert_eq!(p.name(), "config.toml");
		assert_eq!(p.parent().unwrap().as_ref(), "data");
		assert!(p.parent().unwrap().parent().unwrap().is_root());
		assert!(p.starts_with(&"data".parse().unwrap()));
		assert!(!p.starts_with(&"dat".parse().unwrap()));
		assert!(p.starts_with(&FsPath::root()));
	}
}
