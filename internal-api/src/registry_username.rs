use std::borrow::Cow;
use std::fmt;
use std::str::FromStr;

use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// needs to be an an alphanumeric string with dashes and underscores
/// at least 2 characters long
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RegistryUsername(String);

impl fmt::Display for RegistryUsername {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::Display::fmt(&self.0, f)
	}
}

impl AsRef<str> for RegistryUsername {
	fn as_ref(&self) -> &str {
		&self.0
	}
}

impl From<RegistryUsername> for String {
	fn from(name: RegistryUsername) -> Self {
		name.0
	}
}

const BLACKLIST: &[&str] = &[
	// create is not allowed because the webui might want use /users/create
	"create", // -
	// used for the server itself to login
	"internal",
];

impl FromStr for RegistryUsername {
	type Err = InvalidRegistryUsername;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let last_idx = s.len().saturating_sub(1);

		let valid = s.len() >= 3
			&& s.len() <= 42
			&& s.as_bytes().iter().enumerate().all(|(i, &c)| {
				let first_or_last = i == 0 || i == last_idx;

				c.is_ascii_lowercase()
					|| c.is_ascii_digit()
					|| (!first_or_last && (c == b'-' || c == b'_'))
			}) // -
			&& !BLACKLIST.contains(&s);

		valid
			.then(|| Self(s.into()))
			.ok_or(InvalidRegistryUsername {})
	}
}

#[derive(Debug, thiserror::Error)]
#[error(
	"Invalid registry username - must be at least 2 characters long and contain \
	only lowercase alphanumeric characters, dashes, and underscores (`create` is \
	not allowed)"
)]
pub struct InvalidRegistryUsername {}

impl Serialize for RegistryUsername {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		serializer.collect_str(&self)
	}
}

impl<'de> Deserialize<'de> for RegistryUsername {
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

	impl ToSql for RegistryUsername {
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

	impl<'r> FromSql<'r> for RegistryUsername {
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

	impl ParamData for RegistryUsername {
		fn is_null(&self) -> bool {
			false
		}
	}
}
