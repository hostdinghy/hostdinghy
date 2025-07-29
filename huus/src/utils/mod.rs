use std::{
	env::{self, VarError},
	io::{self, Cursor},
	path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use tokio::fs;

use crate::utils::cli::{CliError, WithMessage};

pub mod cli;
pub mod cmd;
pub mod compose;

pub fn huus_dir() -> Result<PathBuf, CliError> {
	match env::var("HUUS_DIR") {
		Ok(dir) => Ok(dir.into()),
		Err(VarError::NotPresent) => Err(CliError::HuusDirNotPresent),
		Err(e) => Err(CliError::any(
			"Failed to get HUUS_DIR environment variable",
			e,
		)),
	}
}

pub async fn write_toml<T: Serialize, P: AsRef<Path>>(
	data: &T,
	path: P,
) -> Result<(), CliError> {
	let s = toml::to_string(data)
		.with_message("Failed to serialize data to TOML")?;

	fs::write(path, s)
		.await
		.with_message("Failed to write TOML file")
}

pub async fn read_toml<T: for<'de> Deserialize<'de>, P: AsRef<Path>>(
	path: P,
) -> Result<T, CliError> {
	let s = fs::read_to_string(path)
		.await
		.with_message("Failed to read TOML file")?;

	toml::from_str(&s).with_message("Failed to deserialize TOML data")
}

pub async fn is_dir(path: impl AsRef<Path>) -> bool {
	fs::metadata(path).await.map_or(false, |m| m.is_dir())
}

pub async fn is_file(path: impl AsRef<Path>) -> bool {
	fs::metadata(path).await.map_or(false, |m| m.is_file())
}

#[cfg(target_os = "linux")]
fn getuid() -> u32 {
	unsafe { libc::getuid() }
}

#[cfg(not(target_os = "linux"))]
fn getuid() -> u32 {
	panic!("cannot determine uid on non-linux systems");
}

// Check if this program is running as root
pub async fn verify_root() -> Result<(), CliError> {
	let uid = getuid();

	if uid != 0 {
		return Err(CliError::NotRoot);
	}

	// because we are running as root this might mean
	// we are running from within sudo which could
	// mean /etc/environment is not read
	// so we read it manually
	let etc_env = match fs::read_to_string("/etc/environment").await {
		Ok(o) => o,
		Err(e) if e.kind() == io::ErrorKind::NotFound => String::new(),
		Err(e) => {
			return Err(CliError::any("Failed to read /etc/environment", e));
		}
	};
	dotenvy::from_read_override(Cursor::new(etc_env))
		.with_message("Failed to read /etc/environment")?;

	Ok(())
}
