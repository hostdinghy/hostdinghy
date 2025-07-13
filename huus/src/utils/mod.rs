use std::{
	env::{self, VarError},
	path::{Path, PathBuf},
};

use serde::Serialize;
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

#[cfg(target_os = "linux")]
fn getuid() -> u32 {
	unsafe { libc::getuid() }
}

#[cfg(not(target_os = "linux"))]
fn getuid() -> u32 {
	panic!("cannot determine uid on non-linux systems");
}

// Check if this program is running as root
pub fn verify_root() -> Result<(), CliError> {
	let uid = getuid();

	(uid == 0).then_some(()).ok_or_else(|| CliError::NotRoot)
}
