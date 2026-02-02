use serde::{Deserialize, Serialize};

/// Represents a Nulab account
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NulabAccount {
    pub nulab_id: String,
    pub name: String,
    pub unique_id: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    #[test]
    fn test_nulab_account_deserialization() {
        let json_str = r#"{
            "nulabId": "nulab-12345-abcde",
            "name": "鈴木一郎",
            "uniqueId": "suzuki-ichiro-001"
        }"#;

        let account: NulabAccount = serde_json::from_str(json_str).unwrap();
        assert_eq!(account.nulab_id, "nulab-12345-abcde");
        assert_eq!(account.name, "鈴木一郎");
        assert_eq!(account.unique_id, "suzuki-ichiro-001");
    }

    #[test]
    fn test_nulab_account_serialization() {
        let account = NulabAccount {
            nulab_id: "nulab-99999".to_string(),
            name: "Test User".to_string(),
            unique_id: "test-unique-123".to_string(),
        };

        let json = serde_json::to_string(&account).unwrap();
        let value: Value = serde_json::from_str(&json).unwrap();

        // Verify camelCase field names
        assert_eq!(value["nulabId"], "nulab-99999");
        assert_eq!(value["uniqueId"], "test-unique-123");
        // Verify snake_case is NOT used
        assert!(value.get("nulab_id").is_none());
        assert!(value.get("unique_id").is_none());
    }

    #[test]
    fn test_nulab_account_round_trip() {
        let original = NulabAccount {
            nulab_id: "nulab-abc".to_string(),
            name: "田中花子".to_string(),
            unique_id: "tanaka-hanako".to_string(),
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: NulabAccount = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_nulab_account_empty_strings() {
        let json_str = r#"{
            "nulabId": "",
            "name": "",
            "uniqueId": ""
        }"#;

        let account: NulabAccount = serde_json::from_str(json_str).unwrap();
        assert_eq!(account.nulab_id, "");
        assert_eq!(account.name, "");
        assert_eq!(account.unique_id, "");
    }
}
