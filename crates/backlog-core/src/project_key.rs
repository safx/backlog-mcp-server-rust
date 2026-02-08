use super::error::Error;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::sync::LazyLock;

/// Maximum length for project keys (1-25 characters)
const MAX_PROJECT_KEY_LENGTH: usize = 25;

// NOTE: Regex uses {1,25} which must match MAX_PROJECT_KEY_LENGTH
static PROJECT_KEY_REGEXP: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[_A-Z0-9]{1,25}$").expect("valid regex pattern"));

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Hash)]
pub struct ProjectKey(String);

/// A type that identify the project, and is unique through the space.
///
/// ProjectKey must be between 1 and 25 characters and should contain
/// only alphanumerical and underscore characters.
impl ProjectKey {
    /// Converts a string slice to a key without checking
    /// that the string contains valid characters.
    ///
    /// # Safety
    ///
    /// The key passed in must be valid characters.
    pub(crate) fn from_str_unchecked(key: &str) -> Self {
        ProjectKey(key.to_string())
    }
}

impl AsRef<str> for ProjectKey {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl FromStr for ProjectKey {
    type Err = Error;

    /// Parses this string slice into `ProjectKey`.
    ///
    /// # Errors
    ///
    /// Will return [`Err`] if it's not possible to parse this string slice into
    /// the ProjectKey.
    fn from_str(key: &str) -> Result<Self, Self::Err> {
        // Length check is redundant with regex {1,25}, but uses constant for consistency
        if key.is_empty() || key.len() > MAX_PROJECT_KEY_LENGTH || !PROJECT_KEY_REGEXP.is_match(key)
        {
            return Err(Error::InvalidProjectKey(key.to_string()));
        }
        Ok(ProjectKey(key.to_string()))
    }
}

impl From<ProjectKey> for String {
    fn from(key: ProjectKey) -> Self {
        key.0
    }
}

impl std::fmt::Display for ProjectKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[test]
fn test_project_key_from_str() {
    assert_eq!(
        ProjectKey::from_str("BLG"),
        Ok(ProjectKey::from_str_unchecked("BLG"))
    );
    assert_eq!(
        ProjectKey::from_str(""),
        Err(Error::InvalidProjectKey(String::from("")))
    );
    assert_eq!(
        ProjectKey::from_str("B$G"),
        Err(Error::InvalidProjectKey(String::from("B$G")))
    );
    assert_eq!(
        ProjectKey::from_str("TOO_LONG_PROJECT_KEY_LN25"),
        Ok(ProjectKey::from_str_unchecked("TOO_LONG_PROJECT_KEY_LN25"))
    );
    assert_eq!(
        ProjectKey::from_str("TOO_LONG_PROJECT_KEY_LEN26"),
        Err(Error::InvalidProjectKey(String::from(
            "TOO_LONG_PROJECT_KEY_LEN26"
        )))
    );
}
