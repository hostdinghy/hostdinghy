use std::{
	env::{self, VarError},
	path::PathBuf,
};

use crate::utils::cli::CliError;

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
