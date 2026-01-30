use std::borrow::Cow;
use std::fmt;
use std::str::FromStr;

use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// needs to be an an lowercase alphanumeric string with underscores
/// at least 2 characters long
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DatabaseName(String);

impl fmt::Display for DatabaseName {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::Display::fmt(&self.0, f)
	}
}

impl AsRef<str> for DatabaseName {
	fn as_ref(&self) -> &str {
		&self.0
	}
}

impl From<DatabaseName> for String {
	fn from(app_id: DatabaseName) -> Self {
		app_id.0
	}
}

const BLACKLIST: &[&str] = &[
	// create is not allowed because the webui might want use /databases/create
	"create",
	// the default postgres database and folder
	"postgres",
	"postgresql",
	// internal database
	"hostdinghy",
];

impl FromStr for DatabaseName {
	type Err = InvalidDatabaseName;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let last_idx = s.len().saturating_sub(1);

		let valid = s.len() >= 3
			&& s.len() <= 42
			&& s.as_bytes().iter().enumerate().all(|(i, &c)| {
				let first_or_last = i == 0 || i == last_idx;

				c.is_ascii_lowercase()
					|| c.is_ascii_digit()
					|| (!first_or_last && c == b'_')
			}) // -
			&& !BLACKLIST.contains(&s);

		valid.then(|| Self(s.into())).ok_or(InvalidDatabaseName {})
	}
}

#[derive(Debug, thiserror::Error)]
#[error(
	"Invalid database name - must be at least 2 characters long and contain \
	only lowercase alphanumeric characters, and underscores (`create` is \
	not allowed)"
)]
pub struct InvalidDatabaseName {}

impl Serialize for DatabaseName {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		serializer.collect_str(&self)
	}
}

impl<'de> Deserialize<'de> for DatabaseName {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		let s: Cow<'_, str> = Deserialize::deserialize(deserializer)?;
		Self::from_str(s.as_ref()).map_err(D::Error::custom)
	}
}

#[cfg(feature = "postgres")]
mod impl_postgres {
	use super::*;
	use bytes::BytesMut;
	use chuchi_postgres::filter::ParamData;
	use postgres_types::{FromSql, IsNull, ToSql, Type, to_sql_checked};

	impl ToSql for DatabaseName {
		fn to_sql(
			&self,
			ty: &Type,
			out: &mut BytesMut,
		) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>>
		where
			Self: Sized,
		{
			self.0.to_sql(ty, out)
		}

		fn accepts(ty: &Type) -> bool
		where
			Self: Sized,
		{
			<&str as ToSql>::accepts(ty)
		}

		to_sql_checked!();
	}

	impl<'r> FromSql<'r> for DatabaseName {
		fn from_sql(
			ty: &Type,
			raw: &'r [u8],
		) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
			let s = <&str as FromSql>::from_sql(ty, raw)?;
			s.parse().map_err(Into::into)
		}

		fn accepts(ty: &Type) -> bool {
			<&str as FromSql>::accepts(ty)
		}
	}

	impl ParamData for DatabaseName {
		fn is_null(&self) -> bool {
			false
		}
	}
}
