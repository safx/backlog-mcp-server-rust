use super::error::Error;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::sync::LazyLock;

/// Minimum length for space keys
const MIN_SPACE_KEY_LENGTH: usize = 3;
/// Maximum length for space keys
const MAX_SPACE_KEY_LENGTH: usize = 10;

// NOTE: Regex uses {3,10} which must match MIN/MAX_SPACE_KEY_LENGTH
static SPACE_KEY_REGEXP: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9-]{3,10}$").expect("valid regex pattern"));

/// A type of Backlog's space identifier, also known as "Space ID" in Backlog's help document,
/// is used to access the organization's Backlog space. It is found in the
/// subdomain part of the Space URL, e.g., <https://myspace.backlog.com>,
/// and is unique through the Backlog service.
///
/// SpaceKey must be between 3 and 10 characters and should contain
/// only alphanumerical and hyphen characters.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub struct SpaceKey(String);

impl SpaceKey {
    /// Converts a string slice to a `SpaceKey` without checking
    /// that the string contains valid characters.
    ///
    /// # Safety
    ///
    /// The key passed in must be valid characters.
    #[cfg(test)]
    fn from_str_unchecked(key: &str) -> Self {
        SpaceKey(key.to_string())
    }
}

impl FromStr for SpaceKey {
    type Err = Error;

    /// Parses this string slice into `SpaceKey`.
    ///
    /// # Errors
    ///
    /// Will return [`Err`] if it's not possible to parse this string slice into
    /// the `SpaceKey`.
    fn from_str(key: &str) -> Result<Self, Self::Err> {
        // Length check is redundant with regex {3,10}, but uses constants for consistency
        if key.len() < MIN_SPACE_KEY_LENGTH
            || key.len() > MAX_SPACE_KEY_LENGTH
            || !SPACE_KEY_REGEXP.is_match(key)
        {
            return Err(Error::InvalidSpaceKey(key.to_string()));
        }
        Ok(SpaceKey(key.to_string()))
    }
}

impl From<SpaceKey> for String {
    fn from(key: SpaceKey) -> Self {
        key.0
    }
}

impl AsRef<str> for SpaceKey {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for SpaceKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

mod tests {
    #[test]
    fn test_space_key_from_str() {
        use super::{Error, SpaceKey};
        use std::str::FromStr;

        assert_eq!(
            SpaceKey::from_str("nulab"),
            Ok(SpaceKey::from_str_unchecked("nulab"))
        );
        assert_eq!(
            SpaceKey::from_str("NULAB"),
            Ok(SpaceKey::from_str_unchecked("NULAB"))
        );
        assert_eq!(
            SpaceKey::from_str("nulab-inc"),
            Ok(SpaceKey::from_str_unchecked("nulab-inc"))
        );
        assert_eq!(
            SpaceKey::from_str("nulab_inc"),
            Err(Error::InvalidSpaceKey(String::from("nulab_inc")))
        );
        assert_eq!(
            SpaceKey::from_str("nu"),
            Err(Error::InvalidSpaceKey(String::from("nu")))
        );
        assert_eq!(
            SpaceKey::from_str("too-long10"),
            Ok(SpaceKey::from_str_unchecked("too-long10"))
        );
        assert_eq!(
            SpaceKey::from_str("too-long-11"),
            Err(Error::InvalidSpaceKey(String::from("too-long-11")))
        );
    }
}
