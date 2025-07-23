use std::borrow::Cow;
use std::fmt;
use std::str::FromStr;

use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// needs to be an an alphanumeric string with dashes and underscores
/// at least 2 characters long
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AppId(String);

impl fmt::Display for AppId {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::Display::fmt(&self.0, f)
	}
}

impl AsRef<str> for AppId {
	fn as_ref(&self) -> &str {
		&self.0
	}
}

impl From<AppId> for String {
	fn from(app_id: AppId) -> Self {
		app_id.0
	}
}

impl FromStr for AppId {
	type Err = InvalidAppId;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let invalid = s.len() < 2
				|| !s.as_bytes().iter().all(|&c| {
					c.is_ascii_alphanumeric() || c == b'-' || c == b'_'
				})
				// create is not allowed because the ui uses /app/create
				|| s == "create";

		(!invalid).then(|| Self(s.into())).ok_or(InvalidAppId {})
	}
}

#[derive(Debug, thiserror::Error)]
#[error(
	"Invalid app id - must be at least 2 characters long and contain \
	only alphanumeric characters, dashes, and underscores (`create` is \
	not allowed)"
)]
pub struct InvalidAppId {}

impl Serialize for AppId {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		serializer.collect_str(&self)
	}
}

impl<'de> Deserialize<'de> for AppId {
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

	impl ToSql for AppId {
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

	impl<'r> FromSql<'r> for AppId {
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

	impl ParamData for AppId {
		fn is_null(&self) -> bool {
			false
		}
	}
}
