use crate::error::Error;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use std::sync::LazyLock;

/// Maximum length for repository names (1-100 characters)
const MAX_REPOSITORY_NAME_LENGTH: usize = 100;

// NOTE: Regex uses {0,99} for additional chars (total 1-100), matching MAX_REPOSITORY_NAME_LENGTH
static REPOSITORY_NAME_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^[a-zA-Z0-9][a-zA-Z0-9_.-]{0,99}$").expect("valid regex pattern")
});

/// A type of string represents Git repository name.
/// Only single-byte alphanumeric characters, underscores, hyphens, and dots can be used.
/// Only one-byte alphanumeric characters can be used as the first character.
/// The length must be 1 to 100 characters.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RepositoryName(String);

impl FromStr for RepositoryName {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty()
            || s.len() > MAX_REPOSITORY_NAME_LENGTH
            || !REPOSITORY_NAME_REGEX.is_match(s)
        {
            return Err(Error::InvalidRepositoryName(s.to_string()));
        }

        Ok(RepositoryName(s.to_string()))
    }
}

impl AsRef<str> for RepositoryName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for RepositoryName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

mod tests {
    #[test]
    fn test_repository_name_from_str() {
        use super::RepositoryName;
        use crate::Error;
        use std::str::FromStr;

        assert_eq!(
            RepositoryName::from_str("valid-repo.name"),
            Ok(RepositoryName("valid-repo.name".to_string()))
        );
        assert_eq!(
            RepositoryName::from_str(""),
            Err(Error::InvalidRepositoryName("".to_string()))
        );
        assert_eq!(
            RepositoryName::from_str("a".repeat(101).as_str()),
            Err(Error::InvalidRepositoryName("a".repeat(101)))
        );
        assert_eq!(
            RepositoryName::from_str("invalid repo name"),
            Err(Error::InvalidRepositoryName(
                "invalid repo name".to_string()
            ))
        );
    }
}
