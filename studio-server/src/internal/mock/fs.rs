//! An in memory file system per app, with the same rules as the server

use std::collections::BTreeMap;

use bytes::Bytes;
use internal_api::{
	error::Error,
	fs::{FsEntry, FsInfoRes, FsKind, FsPath, FsPermissionsReq},
};
use pg::time::DateTime;

use super::storage::MOCK_COMPOSE;

#[derive(Debug, Clone)]
struct Node {
	kind: FsKind,
	content: Bytes,
	mode: u32,
	uid: u32,
	gid: u32,
	modified: DateTime,
}

impl Node {
	fn dir() -> Self {
		Self {
			kind: FsKind::Directory,
			content: Bytes::new(),
			mode: 0o755,
			uid: 0,
			gid: 0,
			modified: DateTime::now(),
		}
	}

	fn file(content: impl Into<Bytes>) -> Self {
		Self {
			kind: FsKind::File,
			content: content.into(),
			mode: 0o644,
			..Self::dir()
		}
	}

	fn entry(&self, name: &str) -> FsEntry {
		FsEntry {
			name: name.into(),
			kind: self.kind,
			size: self.content.len() as u64,
			mode: self.mode,
			uid: self.uid,
			gid: self.gid,
			modified: self.modified,
			link_target: None,
		}
	}
}

fn invalid(msg: &str) -> Error {
	Error::InvalidFileOperation(msg.into())
}

/// The paths are the keys, "" is the app directory
#[derive(Debug, Clone)]
pub struct MockFs {
	nodes: BTreeMap<FsPath, Node>,
}

impl MockFs {
	pub fn new() -> Self {
		let mut fs = Self {
			nodes: BTreeMap::new(),
		};
		fs.nodes.insert(FsPath::root(), Node::dir());

		let files: &[(&str, &[u8])] = &[
			("compose.yml", MOCK_COMPOSE.as_bytes()),
			(".env", b"DATABASE_URL=postgres://app@db/app\nDEBUG=false\n"),
			("data/config.toml", b"[server]\nport = 8080\nname = \"demo\"\n"),
			("data/users.csv", b"id,name,email\n1,Ada,ada@example.com\n2,Linus,linus@example.com\n"),
			("data/uploads/logo.png", b"\x89PNG\r\n\x1a\n\0\0\0\rIHDR mock"),
			("logs/app.log", b"2025-01-01 12:00:00 started\n2025-01-01 12:00:01 listening on :8080\n"),
		];
		for (path, content) in files {
			let path: FsPath = path.parse().unwrap();
			fs.mkdir_all(&path.parent().unwrap());
			fs.nodes.insert(path, Node::file(content.to_vec()));
		}
		// a container user owns the uploads
		for (path, node) in fs.nodes.iter_mut() {
			if path.starts_with(&"data/uploads".parse().unwrap()) {
				node.uid = 1000;
				node.gid = 1000;
			}
		}

		fs
	}

	fn node(&self, path: &FsPath) -> Result<&Node, Error> {
		self.nodes.get(path).ok_or(Error::FileNotFound)
	}

	fn check_changeable(&self, path: &FsPath) -> Result<(), Error> {
		if path.is_root() {
			return Err(invalid("the app directory itself can't be changed"));
		}
		if path.as_ref() == "compose.yml" {
			return Err(invalid(
				"compose.yml can only be changed in the settings",
			));
		}
		Ok(())
	}

	fn mkdir_all(&mut self, path: &FsPath) {
		let mut cur = FsPath::root();
		for seg in path.segments() {
			cur = cur.join(seg).unwrap();
			self.nodes.entry(cur.clone()).or_insert_with(Node::dir);
		}
	}

	/// Checks that the parents are directories and creates them
	fn prepare_parent(&mut self, path: &FsPath) -> Result<(), Error> {
		let mut cur = FsPath::root();
		for seg in path.parent().unwrap_or_default().segments() {
			cur = cur.join(seg).unwrap();
			if let Some(n) = self.nodes.get(&cur)
				&& n.kind != FsKind::Directory
			{
				return Err(invalid("a parent is not a directory"));
			}
		}
		self.mkdir_all(&path.parent().unwrap_or_default());
		Ok(())
	}

	/// All paths at or inside path
	fn subtree(&self, path: &FsPath) -> Vec<FsPath> {
		self.nodes
			.keys()
			.filter(|p| p.starts_with(path))
			.cloned()
			.collect()
	}

	pub fn info(&self, path: &FsPath) -> Result<FsInfoRes, Error> {
		let node = self.node(path)?;
		let children = (node.kind == FsKind::Directory).then(|| {
			let mut children: Vec<_> = self
				.nodes
				.iter()
				.filter(|(p, _)| p.parent().as_ref() == Some(path))
				.map(|(p, n)| n.entry(p.name()))
				.collect();
			children.sort_by(|a, b| {
				(a.kind != FsKind::Directory)
					.cmp(&(b.kind != FsKind::Directory))
					.then_with(|| {
						a.name.to_lowercase().cmp(&b.name.to_lowercase())
					})
			});
			children
		});

		Ok(FsInfoRes {
			entry: node.entry(path.name()),
			children,
		})
	}

	pub fn read(&self, path: &FsPath) -> Result<Bytes, Error> {
		let node = self.node(path)?;
		if node.kind != FsKind::File {
			return Err(invalid("it is not a file"));
		}
		Ok(node.content.clone())
	}

	/// a directory gets a listing instead of an archive
	pub fn download(&self, path: &FsPath) -> Result<(String, Bytes), Error> {
		let node = self.node(path)?;
		let name = if path.is_root() { "app" } else { path.name() };
		if node.kind == FsKind::File {
			return Ok((name.into(), node.content.clone()));
		}

		let listing: String = self
			.subtree(path)
			.iter()
			.map(|p| format!("{p}\n"))
			.collect();
		Ok((format!("{name}-mock-listing.txt"), listing.into()))
	}

	pub fn write(
		&mut self,
		path: &FsPath,
		overwrite: bool,
		content: Bytes,
	) -> Result<(), Error> {
		self.check_changeable(path)?;
		self.prepare_parent(path)?;

		match self.nodes.get_mut(path) {
			Some(n) if n.kind == FsKind::Directory => {
				Err(invalid("it is a directory"))
			}
			Some(_) if !overwrite => Err(Error::FileAlreadyExists),
			Some(n) => {
				n.content = content;
				n.modified = DateTime::now();
				Ok(())
			}
			None => {
				self.nodes.insert(path.clone(), Node::file(content));
				Ok(())
			}
		}
	}

	pub fn mkdir(&mut self, path: &FsPath) -> Result<(), Error> {
		self.check_changeable(path)?;
		if self.nodes.contains_key(path) {
			return Err(Error::FileAlreadyExists);
		}
		self.prepare_parent(path)?;
		self.mkdir_all(path);
		Ok(())
	}

	pub fn delete(&mut self, path: &FsPath) -> Result<(), Error> {
		self.check_changeable(path)?;
		self.node(path)?;
		for p in self.subtree(path) {
			self.nodes.remove(&p);
		}
		Ok(())
	}

	fn prepare_target(
		&mut self,
		from: Option<&FsPath>,
		to: &FsPath,
	) -> Result<(), Error> {
		self.check_changeable(to)?;
		if from.is_some_and(|from| to.starts_with(from)) {
			return Err(invalid("it can't be put inside of itself"));
		}
		if self.nodes.contains_key(to) {
			return Err(Error::FileAlreadyExists);
		}
		self.prepare_parent(to)
	}

	/// copies or moves everything at from to to
	fn transfer(
		&mut self,
		from: &FsPath,
		to: &FsPath,
		remove: bool,
	) -> Result<(), Error> {
		if remove {
			self.check_changeable(from)?;
		} else if from.is_root() {
			return Err(invalid("the app directory itself can't be copied"));
		}
		self.node(from)?;
		self.prepare_target(Some(from), to)?;

		for p in self.subtree(from) {
			let rest = &p.as_ref()[from.as_ref().len()..];
			let new: FsPath = format!("{to}{rest}").parse().unwrap();
			let node = match remove {
				true => self.nodes.remove(&p).unwrap(),
				false => self.nodes[&p].clone(),
			};
			self.nodes.insert(new, node);
		}
		Ok(())
	}

	pub fn rename(&mut self, from: &FsPath, to: &FsPath) -> Result<(), Error> {
		self.transfer(from, to, true)
	}

	pub fn copy(&mut self, from: &FsPath, to: &FsPath) -> Result<(), Error> {
		self.transfer(from, to, false)
	}

	pub fn permissions(&mut self, req: &FsPermissionsReq) -> Result<(), Error> {
		let paths = match req.recursive {
			true => self.subtree(&req.path),
			false => vec![req.path.clone()],
		};
		self.node(&req.path)?;

		for p in paths {
			let node = self.nodes.get_mut(&p).unwrap();
			if let Some(mode) = req.mode {
				node.mode = mode & 0o7777;
			}
			if let Some(uid) = req.uid {
				node.uid = uid;
			}
			if let Some(gid) = req.gid {
				node.gid = gid;
			}
		}
		Ok(())
	}

	pub fn zip(&mut self, path: &FsPath, to: &FsPath) -> Result<(), Error> {
		self.node(path)?;
		self.prepare_target(None, to)?;
		self.nodes
			.insert(to.clone(), Node::file(&b"PK\x05\x06 mock zip"[..]));
		Ok(())
	}

	pub fn extract(&mut self, path: &FsPath, to: &FsPath) -> Result<(), Error> {
		self.read(path)?;
		if internal_api::fs::ArchiveKind::from_name(path.name()).is_none() {
			return Err(invalid(
				"only .zip, .tar, .tar.gz and .tgz can be extracted",
			));
		}
		self.prepare_target(None, to)?;
		self.mkdir_all(to);
		self.nodes.insert(
			to.join("extracted.txt").unwrap(),
			Node::file(format!("mock: extracted from {path}\n")),
		);
		Ok(())
	}
}
