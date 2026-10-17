/*!
Files of an app, everything stays inside `$HOSTDINGHY_DIR/<app id>`.

[`FsPath`] can't contain `..`, but the app's containers can create symlinks
in their volumes. So every path is resolved with its parent directories
canonicalized and checked to be inside the app directory. Operations that
change something work on the last segment itself without following it
(deleting a symlink deletes the link), the rest follows it and checks the
target.

The functions here are blocking, call them with [`blocking`].
*/

pub mod routes;

use std::{
	cmp::Ordering,
	fs::{self, File, Metadata, Permissions},
	io::{self, ErrorKind},
	os::unix::fs::{MetadataExt as _, PermissionsExt as _, lchown, symlink},
	path::{Component, Path, PathBuf},
};

use api::{
	apps::AppId,
	error::Error,
	fs::{ArchiveKind, FsEntry, FsInfoRes, FsKind, FsPath, FsPermissionsReq},
};
use chuchi_crypto::token::Token;
use chuchi_postgres::time::DateTime;
use tokio::task;

use crate::utils::{hostdinghy_dir, is_dir};

pub const COMPOSE_FILE: &str = "compose.yml";

/// Runs a blocking file operation on the blocking thread pool
pub async fn blocking<T, F>(f: F) -> Result<T, Error>
where
	F: FnOnce() -> Result<T, Error> + Send + 'static,
	T: Send + 'static,
{
	task::spawn_blocking(f)
		.await
		.map_err(|e| Error::any("file operation panicked", e))?
}

/// Maps not found and already exists to their api errors
pub fn io_err(message: &str, e: io::Error) -> Error {
	match e.kind() {
		ErrorKind::NotFound => Error::FileNotFound,
		ErrorKind::AlreadyExists => Error::FileAlreadyExists,
		ErrorKind::NotADirectory => {
			Error::InvalidFileOperation("a parent is not a directory".into())
		}
		ErrorKind::IsADirectory => {
			Error::InvalidFileOperation("it is a directory".into())
		}
		ErrorKind::DirectoryNotEmpty => {
			Error::InvalidFileOperation("the directory is not empty".into())
		}
		_ => Error::any(message, e),
	}
}

fn invalid(msg: &str) -> Error {
	Error::InvalidFileOperation(msg.into())
}

#[derive(Debug, Clone)]
pub struct AppFs {
	/// canonical
	root: PathBuf,
}

impl AppFs {
	/// Creates the app directory if it doesn't exist yet, a new app only
	/// gets one with its first compose file. The studio checks that the
	/// app exists.
	pub async fn open(id: &AppId) -> Result<Self, Error> {
		let dir = hostdinghy_dir()?.join(id.as_ref());
		if !is_dir(&dir).await {
			match tokio::fs::create_dir(&dir).await {
				Ok(()) => {}
				// created in the meantime
				Err(e) if e.kind() == ErrorKind::AlreadyExists => {}
				Err(e) => {
					return Err(Error::any(
						"Failed to create app directory",
						e,
					));
				}
			}
		}

		let root = tokio::fs::canonicalize(&dir)
			.await
			.map_err(|_| Error::AppNotFound)?;

		Ok(Self { root })
	}

	fn check_inside(&self, path: &Path) -> Result<(), Error> {
		if path.starts_with(&self.root) {
			Ok(())
		} else {
			Err(invalid("it points outside of the app directory"))
		}
	}

	/// The path with the parent directories resolved, the last segment
	/// isn't followed if it is a symlink. It doesn't need to exist.
	pub fn nofollow(&self, path: &FsPath) -> Result<PathBuf, Error> {
		let Some(parent) = path.parent() else {
			return Ok(self.root.clone());
		};

		let parent = self.follow(&parent)?;
		Ok(parent.join(path.name()))
	}

	/// The path with all symlinks resolved, it needs to exist
	pub fn follow(&self, path: &FsPath) -> Result<PathBuf, Error> {
		let mut full = self.root.clone();
		full.extend(path.segments());

		let canon = fs::canonicalize(&full)
			.map_err(|e| io_err("failed to resolve path", e))?;
		self.check_inside(&canon)?;

		Ok(canon)
	}

	/// The app directory and its compose file can't be changed here
	pub fn check_changeable(&self, path: &FsPath) -> Result<(), Error> {
		if path.is_root() {
			return Err(invalid("the app directory itself can't be changed"));
		}
		if path.as_ref() == COMPOSE_FILE {
			return Err(invalid(
				"compose.yml can only be changed in the settings",
			));
		}

		Ok(())
	}

	/// Creates the directory and missing parents, each new one gets the
	/// owner of its parent
	pub fn mkdir_all(&self, path: &FsPath) -> Result<PathBuf, Error> {
		let mut cur = FsPath::root();
		let mut cur_full = self.root.clone();

		for seg in path.segments() {
			cur = cur.join(seg).map_err(|e| Error::any("invalid path", e))?;
			let full = cur_full.join(seg);

			match fs::symlink_metadata(&full) {
				Ok(meta) if meta.is_dir() => cur_full = full,
				// a symlink to a directory inside is fine
				Ok(meta) if meta.is_symlink() => {
					let target = self.follow(&cur)?;
					if !target.is_dir() {
						return Err(invalid("a parent is not a directory"));
					}
					cur_full = target;
				}
				Ok(_) => return Err(invalid("a parent is not a directory")),
				Err(e) if e.kind() == ErrorKind::NotFound => {
					fs::create_dir(&full)
						.map_err(|e| io_err("failed to create directory", e))?;
					inherit_owner(&full, &cur_full)?;
					cur_full = full;
				}
				Err(e) => return Err(io_err("failed to read directory", e)),
			}
		}

		Ok(cur_full)
	}

	pub fn info(&self, path: &FsPath) -> Result<FsInfoRes, Error> {
		let full = self.follow(path)?;
		let meta = fs::metadata(&full)
			.map_err(|e| io_err("failed to read metadata", e))?;
		let this = entry(path.name(), &meta, None);

		let children = if meta.is_dir() {
			let mut children = vec![];
			let dir = fs::read_dir(&full)
				.map_err(|e| io_err("failed to read directory", e))?;

			for child in dir {
				let child =
					child.map_err(|e| io_err("failed to read directory", e))?;
				let meta = match child.metadata() {
					Ok(m) => m,
					// removed in the meantime
					Err(_) => continue,
				};
				let link = meta
					.is_symlink()
					.then(|| fs::read_link(child.path()).ok())
					.flatten()
					.map(|t| t.to_string_lossy().into_owned());

				children.push(entry(
					&child.file_name().to_string_lossy(),
					&meta,
					link,
				));
			}

			children.sort_by(|a, b| match (a.kind, b.kind) {
				(FsKind::Directory, FsKind::Directory) => cmp_names(a, b),
				(FsKind::Directory, _) => Ordering::Less,
				(_, FsKind::Directory) => Ordering::Greater,
				_ => cmp_names(a, b),
			});

			Some(children)
		} else {
			None
		};

		Ok(FsInfoRes {
			entry: this,
			children,
		})
	}

	/// A regular file to read
	pub fn file(&self, path: &FsPath) -> Result<(PathBuf, u64), Error> {
		let full = self.follow(path)?;
		let meta = fs::metadata(&full)
			.map_err(|e| io_err("failed to read metadata", e))?;
		if !meta.is_file() {
			return Err(invalid("it is not a file"));
		}

		Ok((full, meta.len()))
	}

	/// Prepares writing a file: creates the parents and returns the target
	/// and a temporary file next to it, rename it when done
	pub fn prepare_write(
		&self,
		path: &FsPath,
		overwrite: bool,
	) -> Result<(PathBuf, PathBuf), Error> {
		self.check_changeable(path)?;
		let parent = path.parent().unwrap_or_default();
		let parent_full = self.mkdir_all(&parent)?;
		let target = parent_full.join(path.name());

		match fs::symlink_metadata(&target) {
			Ok(meta) if meta.is_dir() => {
				return Err(invalid("it is a directory"));
			}
			Ok(_) if !overwrite => return Err(Error::FileAlreadyExists),
			Ok(_) => {}
			Err(e) if e.kind() == ErrorKind::NotFound => {}
			Err(e) => return Err(io_err("failed to read metadata", e)),
		}

		let tmp = parent_full.join(format!(
			".{}.hostdinghy-{}",
			path.name(),
			Token::<8>::new()
		));

		Ok((target, tmp))
	}

	/// Moves a written temporary file into place, it keeps the mode and
	/// owner of the file it replaces or gets the owner of the directory
	pub fn finish_write(&self, target: &Path, tmp: &Path) -> Result<(), Error> {
		let res = (|| {
			match fs::metadata(target) {
				Ok(old) => {
					fs::set_permissions(tmp, old.permissions())
						.map_err(|e| io_err("failed to set mode", e))?;
					lchown(tmp, Some(old.uid()), Some(old.gid()))
						.map_err(|e| io_err("failed to set owner", e))?;
				}
				Err(_) => {
					fs::set_permissions(tmp, Permissions::from_mode(0o644))
						.map_err(|e| io_err("failed to set mode", e))?;
					inherit_owner(tmp, target.parent().unwrap())?;
				}
			}

			// replaces a symlink instead of writing through it
			fs::rename(tmp, target)
				.map_err(|e| io_err("failed to move the file into place", e))
		})();

		if res.is_err() {
			let _ = fs::remove_file(tmp);
		}
		res
	}

	pub fn mkdir(&self, path: &FsPath) -> Result<(), Error> {
		self.check_changeable(path)?;
		// a missing parent is not an error, it gets created
		if let Ok(full) = self.nofollow(path)
			&& fs::symlink_metadata(full).is_ok()
		{
			return Err(Error::FileAlreadyExists);
		}

		self.mkdir_all(path).map(|_| ())
	}

	pub fn delete(&self, path: &FsPath) -> Result<(), Error> {
		self.check_changeable(path)?;
		let full = self.nofollow(path)?;
		let meta = fs::symlink_metadata(&full)
			.map_err(|e| io_err("failed to read metadata", e))?;

		// remove_dir_all doesn't follow symlinks
		if meta.is_dir() {
			fs::remove_dir_all(&full)
		} else {
			fs::remove_file(&full)
		}
		.map_err(|e| io_err("failed to delete", e))
	}

	/// Checks a move or copy target: not inside the source, parent exists
	/// (created if missing) and the target itself doesn't
	fn prepare_target(
		&self,
		from: Option<&FsPath>,
		to: &FsPath,
	) -> Result<PathBuf, Error> {
		self.check_changeable(to)?;
		if from.is_some_and(|from| to.starts_with(from)) {
			return Err(invalid("it can't be put inside of itself"));
		}

		let parent = self.mkdir_all(&to.parent().unwrap_or_default())?;
		let target = parent.join(to.name());
		if fs::symlink_metadata(&target).is_ok() {
			return Err(Error::FileAlreadyExists);
		}

		Ok(target)
	}

	pub fn rename(&self, from: &FsPath, to: &FsPath) -> Result<(), Error> {
		self.check_changeable(from)?;
		let src = self.nofollow(from)?;
		fs::symlink_metadata(&src)
			.map_err(|e| io_err("failed to read metadata", e))?;
		let target = self.prepare_target(Some(from), to)?;

		fs::rename(&src, &target).map_err(|e| io_err("failed to move", e))
	}

	pub fn copy(&self, from: &FsPath, to: &FsPath) -> Result<(), Error> {
		if from.is_root() {
			return Err(invalid("the app directory itself can't be copied"));
		}
		let src = self.nofollow(from)?;
		fs::symlink_metadata(&src)
			.map_err(|e| io_err("failed to read metadata", e))?;
		let target = self.prepare_target(Some(from), to)?;

		let res = copy_all(&src, &target);
		if res.is_err() {
			let _ = remove_any(&target);
		}
		res
	}

	pub fn permissions(&self, req: &FsPermissionsReq) -> Result<(), Error> {
		let full = self.nofollow(&req.path)?;
		let meta = fs::symlink_metadata(&full)
			.map_err(|e| io_err("failed to read metadata", e))?;
		if meta.is_symlink() {
			return Err(invalid(
				"symlinks have no permissions of their own, change the target",
			));
		}

		set_permissions(&full, req)?;

		if req.recursive && meta.is_dir() {
			walk(&full, &mut |path, meta| {
				// a symlink would change something somewhere else
				if meta.is_symlink() {
					return Ok(());
				}
				set_permissions(path, req)
			})?;
		}

		Ok(())
	}

	/// Packs a file or directory into a new zip file
	pub fn zip(&self, path: &FsPath, to: &FsPath) -> Result<(), Error> {
		let src = self.follow(path)?;
		// the zip may be inside the directory, it is skipped while packing
		let target = self.prepare_target(None, to)?;
		// the root has no name, use the app directory's name
		let base = match path.is_root() {
			true => self.root.file_name().unwrap().to_string_lossy().into(),
			false => path.name().to_string(),
		};

		let tmp = target.with_file_name(format!(".{}.hostdinghy", to.name()));
		let res = write_zip(&src, &base, &tmp).and_then(|_| {
			inherit_owner(&tmp, target.parent().unwrap())?;
			fs::rename(&tmp, &target)
				.map_err(|e| io_err("failed to move the zip into place", e))
		});
		if res.is_err() {
			let _ = fs::remove_file(&tmp);
		}
		res
	}

	/// Extracts an archive into a new directory
	pub fn extract(&self, path: &FsPath, to: &FsPath) -> Result<(), Error> {
		let (src, _) = self.file(path)?;
		let kind = ArchiveKind::from_name(path.name()).ok_or_else(|| {
			invalid("only .zip, .tar, .tar.gz and .tgz can be extracted")
		})?;
		let target = self.prepare_target(None, to)?;

		// everything inside is created here and no symlinks are extracted,
		// so nothing can point outside of the new directory
		fs::create_dir(&target)
			.map_err(|e| io_err("failed to create directory", e))?;
		let parent = target.parent().unwrap();
		inherit_owner(&target, parent)?;

		let res = match kind {
			ArchiveKind::Zip => extract_zip(&src, &target),
			ArchiveKind::Tar => extract_tar(File::open(&src), &target, false),
			ArchiveKind::TarGz => extract_tar(File::open(&src), &target, true),
		}
		.and_then(|_| {
			// everything gets the owner of the directory it is extracted to
			let owner = fs::metadata(parent)
				.map_err(|e| io_err("failed to read metadata", e))?;
			walk(&target, &mut |path, _| {
				lchown(path, Some(owner.uid()), Some(owner.gid()))
					.map_err(|e| io_err("failed to set owner", e))
			})
		});

		if res.is_err() {
			let _ = fs::remove_dir_all(&target);
		}
		res
	}
}

fn cmp_names(a: &FsEntry, b: &FsEntry) -> Ordering {
	a.name
		.to_lowercase()
		.cmp(&b.name.to_lowercase())
		.then_with(|| a.name.cmp(&b.name))
}

pub fn entry(name: &str, meta: &Metadata, link: Option<String>) -> FsEntry {
	let ft = meta.file_type();
	let kind = if ft.is_symlink() {
		FsKind::Symlink
	} else if ft.is_dir() {
		FsKind::Directory
	} else if ft.is_file() {
		FsKind::File
	} else {
		FsKind::Other
	};

	FsEntry {
		name: name.to_string(),
		kind,
		size: if kind == FsKind::File { meta.len() } else { 0 },
		mode: meta.mode() & 0o7777,
		uid: meta.uid(),
		gid: meta.gid(),
		modified: meta
			.modified()
			.map(DateTime::from_std)
			.unwrap_or_else(|_| DateTime::from_secs(meta.mtime())),
		link_target: link,
	}
}

fn inherit_owner(path: &Path, dir: &Path) -> Result<(), Error> {
	let meta =
		fs::metadata(dir).map_err(|e| io_err("failed to read metadata", e))?;
	lchown(path, Some(meta.uid()), Some(meta.gid()))
		.map_err(|e| io_err("failed to set owner", e))
}

fn set_permissions(path: &Path, req: &FsPermissionsReq) -> Result<(), Error> {
	if req.uid.is_some() || req.gid.is_some() {
		lchown(path, req.uid, req.gid)
			.map_err(|e| io_err("failed to set owner", e))?;
	}
	if let Some(mode) = req.mode {
		fs::set_permissions(path, Permissions::from_mode(mode & 0o7777))
			.map_err(|e| io_err("failed to set mode", e))?;
	}

	Ok(())
}

/// Calls f for everything inside dir (not dir itself), doesn't follow
/// symlinks
fn walk(
	dir: &Path,
	f: &mut dyn FnMut(&Path, &Metadata) -> Result<(), Error>,
) -> Result<(), Error> {
	let entries =
		fs::read_dir(dir).map_err(|e| io_err("failed to read directory", e))?;
	for entry in entries {
		let entry = entry.map_err(|e| io_err("failed to read directory", e))?;
		let path = entry.path();
		let meta = fs::symlink_metadata(&path)
			.map_err(|e| io_err("failed to read metadata", e))?;

		f(&path, &meta)?;
		if meta.is_dir() {
			walk(&path, f)?;
		}
	}

	Ok(())
}

/// Copies files, directories and symlinks (as symlinks) with their mode
/// and owner
fn copy_all(src: &Path, dst: &Path) -> Result<(), Error> {
	let meta = fs::symlink_metadata(src)
		.map_err(|e| io_err("failed to read metadata", e))?;

	if meta.is_symlink() {
		let target =
			fs::read_link(src).map_err(|e| io_err("failed to read link", e))?;
		symlink(target, dst).map_err(|e| io_err("failed to copy link", e))?;
	} else if meta.is_dir() {
		fs::create_dir(dst)
			.map_err(|e| io_err("failed to create directory", e))?;
		let entries = fs::read_dir(src)
			.map_err(|e| io_err("failed to read directory", e))?;
		for entry in entries {
			let entry =
				entry.map_err(|e| io_err("failed to read directory", e))?;
			copy_all(&entry.path(), &dst.join(entry.file_name()))?;
		}
		fs::set_permissions(dst, meta.permissions())
			.map_err(|e| io_err("failed to set mode", e))?;
	} else if meta.is_file() {
		fs::copy(src, dst).map_err(|e| io_err("failed to copy", e))?;
	} else {
		// sockets, devices
		return Ok(());
	}

	lchown(dst, Some(meta.uid()), Some(meta.gid()))
		.map_err(|e| io_err("failed to set owner", e))
}

fn remove_any(path: &Path) -> io::Result<()> {
	match fs::symlink_metadata(path)? {
		m if m.is_dir() => fs::remove_dir_all(path),
		_ => fs::remove_file(path),
	}
}

fn write_zip(src: &Path, base: &str, tmp: &Path) -> Result<(), Error> {
	use std::io::Write as _;
	use zip::{CompressionMethod, ZipWriter, write::SimpleFileOptions};

	let file = File::create_new(tmp)
		.map_err(|e| io_err("failed to create the zip", e))?;
	let mut zip = ZipWriter::new(file);
	let zip_err =
		|e: zip::result::ZipError| Error::any("failed to write zip", e);

	let add = |zip: &mut ZipWriter<File>,
	           path: &Path,
	           name: &str,
	           meta: &Metadata|
	 -> Result<(), Error> {
		let opts = SimpleFileOptions::default()
			.compression_method(CompressionMethod::Deflated)
			.unix_permissions(meta.mode() & 0o777);
		if meta.is_dir() {
			zip.add_directory(name, opts).map_err(zip_err)?;
		} else if meta.is_file() {
			zip.start_file(name, opts).map_err(zip_err)?;
			let mut file = File::open(path)
				.map_err(|e| io_err("failed to read file", e))?;
			io::copy(&mut file, zip)
				.map_err(|e| io_err("failed to write zip", e))?;
			zip.flush().map_err(|e| io_err("failed to write zip", e))?;
		}
		// symlinks and others are skipped
		Ok(())
	};

	let meta =
		fs::metadata(src).map_err(|e| io_err("failed to read metadata", e))?;
	add(&mut zip, src, base, &meta)?;

	if meta.is_dir() {
		let mut entries = vec![];
		walk(src, &mut |path, meta| {
			// skip our own temporary file
			if path != tmp {
				entries.push((path.to_path_buf(), meta.clone()));
			}
			Ok(())
		})?;

		for (path, meta) in entries {
			let rel = path.strip_prefix(src).unwrap();
			let name = format!("{base}/{}", rel.to_string_lossy());
			add(&mut zip, &path, &name, &meta)?;
		}
	}

	zip.finish().map_err(zip_err)?;
	Ok(())
}

/// A relative path with only normal components
fn safe_rel(path: &Path) -> Option<PathBuf> {
	let mut out = PathBuf::new();
	for c in path.components() {
		match c {
			Component::Normal(c) => out.push(c),
			Component::CurDir => {}
			_ => return None,
		}
	}

	(!out.as_os_str().is_empty()).then_some(out)
}

fn create_extracted(
	target: &Path,
	rel: &Path,
	mode: Option<u32>,
	content: &mut dyn io::Read,
) -> Result<(), Error> {
	let full = target.join(rel);
	if let Some(parent) = full.parent() {
		fs::create_dir_all(parent)
			.map_err(|e| io_err("failed to create directory", e))?;
	}

	let mut file = File::create_new(&full)
		.map_err(|e| io_err("failed to create file", e))?;
	io::copy(content, &mut file).map_err(|e| io_err("failed to extract", e))?;
	fs::set_permissions(&full, Permissions::from_mode(mode.unwrap_or(0o644)))
		.map_err(|e| io_err("failed to set mode", e))
}

fn extract_zip(src: &Path, target: &Path) -> Result<(), Error> {
	let file =
		File::open(src).map_err(|e| io_err("failed to open the archive", e))?;
	let mut zip = zip::ZipArchive::new(file)
		.map_err(|e| Error::any("failed to read the zip", e))?;

	for i in 0..zip.len() {
		let mut entry = zip
			.by_index(i)
			.map_err(|e| Error::any("failed to read the zip", e))?;
		let Some(rel) = entry.enclosed_name().as_deref().and_then(safe_rel)
		else {
			continue;
		};
		let mode = entry.unix_mode().map(|m| m & 0o777);

		if entry.is_dir() {
			fs::create_dir_all(target.join(&rel))
				.map_err(|e| io_err("failed to create directory", e))?;
		} else if entry.is_file() {
			create_extracted(target, &rel, mode, &mut entry)?;
		}
		// symlinks are skipped
	}

	Ok(())
}

fn extract_tar(
	file: io::Result<File>,
	target: &Path,
	gz: bool,
) -> Result<(), Error> {
	let file = file.map_err(|e| io_err("failed to open the archive", e))?;
	let reader: Box<dyn io::Read> = match gz {
		true => Box::new(flate2::read::GzDecoder::new(file)),
		false => Box::new(file),
	};
	let mut archive = tar::Archive::new(reader);
	let entries = archive
		.entries()
		.map_err(|e| io_err("failed to read the archive", e))?;

	for entry in entries {
		let mut entry =
			entry.map_err(|e| io_err("failed to read the archive", e))?;
		let path = entry
			.path()
			.map_err(|e| io_err("failed to read the archive", e))?
			.into_owned();
		let Some(rel) = safe_rel(&path) else {
			continue;
		};
		let mode = entry.header().mode().ok().map(|m| m & 0o777);

		match entry.header().entry_type() {
			tar::EntryType::Directory => {
				fs::create_dir_all(target.join(&rel))
					.map_err(|e| io_err("failed to create directory", e))?;
			}
			tar::EntryType::Regular | tar::EntryType::Continuous => {
				create_extracted(target, &rel, mode, &mut entry)?;
			}
			// links and special files are skipped
			_ => {}
		}
	}

	Ok(())
}
