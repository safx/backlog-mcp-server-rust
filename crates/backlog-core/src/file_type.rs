#[cfg(feature = "schemars")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Represents the type of a shared file.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "lowercase")]
pub enum FileType {
    /// A regular file
    File,
    /// A directory
    Directory,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_type_deserialization() {
        let file: FileType = serde_json::from_str("\"file\"").unwrap();
        assert_eq!(file, FileType::File);

        let directory: FileType = serde_json::from_str("\"directory\"").unwrap();
        assert_eq!(directory, FileType::Directory);
    }

    #[test]
    fn test_file_type_serialization() {
        assert_eq!(serde_json::to_string(&FileType::File).unwrap(), "\"file\"");
        assert_eq!(
            serde_json::to_string(&FileType::Directory).unwrap(),
            "\"directory\""
        );
    }

    #[test]
    fn test_file_type_round_trip() {
        for ft in [FileType::File, FileType::Directory] {
            let json = serde_json::to_string(&ft).unwrap();
            let deserialized: FileType = serde_json::from_str(&json).unwrap();
            assert_eq!(ft, deserialized);
        }
    }

    #[test]
    fn test_file_type_deserialization_invalid() {
        assert!(serde_json::from_str::<FileType>("\"folder\"").is_err());
        assert!(serde_json::from_str::<FileType>("\"File\"").is_err()); // case sensitive
        assert!(serde_json::from_str::<FileType>("\"dir\"").is_err());
    }
}
