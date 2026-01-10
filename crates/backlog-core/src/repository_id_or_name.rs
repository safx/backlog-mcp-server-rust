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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str_id() {
        let result = RepositoryIdOrName::from_str("123").unwrap();
        assert!(matches!(result, RepositoryIdOrName::Id(_)));
        if let RepositoryIdOrName::Id(id) = result {
            assert_eq!(id.value(), 123);
        }
    }

    #[test]
    fn test_from_str_name() {
        let result = RepositoryIdOrName::from_str("my-repo").unwrap();
        assert!(matches!(result, RepositoryIdOrName::Name(_)));
        assert_eq!(result.to_string(), "my-repo");
    }

    #[test]
    fn test_from_str_zero_as_name() {
        // "0" is valid as a repository name (single alphanumeric char)
        let result = RepositoryIdOrName::from_str("0").unwrap();
        assert!(matches!(result, RepositoryIdOrName::Name(_)));
        assert_eq!(result.to_string(), "0");
    }

    #[test]
    fn test_from_str_invalid() {
        let result = RepositoryIdOrName::from_str("invalid repo");
        assert!(result.is_err());
        assert_eq!(
            result,
            Err(Error::InvalidRepositoryIdOrName("invalid repo".to_string()))
        );
    }

    #[test]
    fn test_display_id() {
        let id_or_name = RepositoryIdOrName::Id(RepositoryId::new(456));
        assert_eq!(id_or_name.to_string(), "456");
    }

    #[test]
    fn test_display_name() {
        let name = RepositoryName::from_str("test-repo").unwrap();
        let id_or_name = RepositoryIdOrName::Name(name);
        assert_eq!(id_or_name.to_string(), "test-repo");
    }

    #[test]
    fn test_from_repository_id() {
        let id = RepositoryId::new(789);
        let id_or_name: RepositoryIdOrName = id.into();
        assert!(matches!(id_or_name, RepositoryIdOrName::Id(_)));
    }

    #[test]
    fn test_from_repository_name() {
        let name = RepositoryName::from_str("my-project").unwrap();
        let id_or_name: RepositoryIdOrName = name.into();
        assert!(matches!(id_or_name, RepositoryIdOrName::Name(_)));
    }

    #[test]
    fn test_into_string() {
        let id_or_name = RepositoryIdOrName::Id(RepositoryId::new(123));
        let s: String = id_or_name.into();
        assert_eq!(s, "123");

        let name = RepositoryName::from_str("repo-name").unwrap();
        let id_or_name = RepositoryIdOrName::Name(name);
        let s: String = id_or_name.into();
        assert_eq!(s, "repo-name");
    }
}
