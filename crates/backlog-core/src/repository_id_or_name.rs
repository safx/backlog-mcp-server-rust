use crate::RepositoryName;
use crate::error::Error;
use crate::identifier::{Identifier, RepositoryId};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// A type of string represents either a RepositoryId or a RepositoryName,
/// allowing for flexible identification of repositories.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RepositoryIdOrName {
    Id(RepositoryId),
    Name(RepositoryName),
}

impl FromStr for RepositoryIdOrName {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Try to parse as RepositoryId (u32) first
        if let Ok(id_val) = u32::from_str(s)
            && id_val > 0
        {
            return Ok(RepositoryIdOrName::Id(RepositoryId::new(id_val)));
        }
        // If not a u32 or not > 0, try to parse as RepositoryName
        match RepositoryName::from_str(s) {
            Ok(name) => Ok(RepositoryIdOrName::Name(name)),
            Err(_) => Err(Error::InvalidRepositoryIdOrName(s.to_string())),
        }
    }
}

impl fmt::Display for RepositoryIdOrName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RepositoryIdOrName::Id(id) => write!(f, "{}", id.value()),
            RepositoryIdOrName::Name(name) => write!(f, "{name}"),
        }
    }
}

impl From<RepositoryId> for RepositoryIdOrName {
    fn from(id: RepositoryId) -> Self {
        RepositoryIdOrName::Id(id)
    }
}

impl From<RepositoryName> for RepositoryIdOrName {
    fn from(name: RepositoryName) -> Self {
        RepositoryIdOrName::Name(name)
    }
}

// This allows RepositoryIdOrName to be easily converted to a String,
// which is useful for constructing URL paths.
impl From<RepositoryIdOrName> for String {
    fn from(val: RepositoryIdOrName) -> Self {
        val.to_string()
    }
}
