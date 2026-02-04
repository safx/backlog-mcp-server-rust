use crate::error::Error;
use crate::identifier::Identifier;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{str::FromStr, sync::LazyLock};

#[cfg(feature = "schemars")]
use schemars::JsonSchema;

/// Length of document ID hex string (32 lowercase hex characters)
const DOCUMENT_ID_HEX_LENGTH: usize = 32;

// NOTE: Regex uses {32} which must match DOCUMENT_ID_HEX_LENGTH
static DOCUMENT_ID_REGEXP: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[0-9a-f]{32}$").expect("valid regex pattern"));

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct DocumentId(pub String);

impl DocumentId {
    pub fn unsafe_new(value: String) -> Self {
        Self(value)
    }
}

impl Identifier for DocumentId {
    type Id = String;
    fn value(&self) -> Self::Id {
        self.0.clone()
    }
}

impl From<String> for DocumentId {
    fn from(value: String) -> Self {
        DocumentId(value)
    }
}

impl FromStr for DocumentId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Length check is redundant with regex {32}, but uses constant for consistency
        if s.len() != DOCUMENT_ID_HEX_LENGTH || !DOCUMENT_ID_REGEXP.is_match(s) {
            return Err(Error::InvalidDocumentId(s.to_string()));
        }
        Ok(DocumentId(s.to_string()))
    }
}

impl std::fmt::Display for DocumentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::hash::Hash for DocumentId {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str_valid() {
        let valid_id = "a1b2c3d4e5f6789012345678901234ab";
        let result = DocumentId::from_str(valid_id).unwrap();
        assert_eq!(result.0, valid_id);
    }

    #[test]
    fn test_from_str_invalid_length() {
        let short_id = "abc123";
        let result = DocumentId::from_str(short_id);
        assert!(result.is_err());
        assert_eq!(result, Err(Error::InvalidDocumentId(short_id.to_string())));
    }

    #[test]
    fn test_from_str_invalid_chars() {
        let invalid_id = "g1b2c3d4e5f6789012345678901234ab"; // 'g' is not hex
        let result = DocumentId::from_str(invalid_id);
        assert!(result.is_err());
        assert_eq!(
            result,
            Err(Error::InvalidDocumentId(invalid_id.to_string()))
        );
    }

    #[test]
    fn test_from_str_uppercase() {
        let uppercase_id = "A1B2C3D4E5F6789012345678901234AB";
        let result = DocumentId::from_str(uppercase_id);
        assert!(result.is_err()); // Only lowercase hex is valid
    }

    #[test]
    fn test_display() {
        let id = DocumentId::unsafe_new("abc123def456789012345678901234ab".to_string());
        assert_eq!(id.to_string(), "abc123def456789012345678901234ab");
    }

    #[test]
    fn test_identifier_value() {
        let id = DocumentId::unsafe_new("test1234567890abcdef1234567890ab".to_string());
        assert_eq!(id.value(), "test1234567890abcdef1234567890ab");
    }
}
