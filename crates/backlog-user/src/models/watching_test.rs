#[cfg(test)]
mod watching_tests {
    use serde_json;

    use backlog_core::identifier::Identifier;

    use crate::models::watching::{Watching, WatchingType};

    #[test]
    fn test_deserialize_minimal_watching() {
        let json = r#"{
            "id": 123,
            "resourceAlreadyRead": false,
            "type": "issue",
            "created": "2023-01-01T00:00:00Z",
            "updated": "2023-01-01T00:00:00Z"
        }"#;

        let result: Result<Watching, _> = serde_json::from_str(json);
        assert!(result.is_ok());

        let watching = result.expect("valid JSON should deserialize");
        assert_eq!(watching.id.value(), 123);
        assert!(!watching.resource_already_read);
        assert_eq!(watching.watching_type, WatchingType::Issue);
        assert!(watching.note.is_none());
        assert!(watching.issue.is_none());
        assert!(watching.last_content_updated.is_none());
    }

    #[test]
    fn test_deserialize_watching_with_note() {
        let json = r#"{
            "id": 456,
            "resourceAlreadyRead": true,
            "note": "Important issue to track",
            "type": "issue",
            "lastContentUpdated": "2023-01-02T12:00:00Z",
            "created": "2023-01-01T00:00:00Z",
            "updated": "2023-01-02T00:00:00Z"
        }"#;

        let result: Result<Watching, _> = serde_json::from_str(json);
        assert!(result.is_ok());

        let watching = result.expect("valid JSON should deserialize");
        assert_eq!(watching.id.value(), 456);
        assert!(watching.resource_already_read);
        assert_eq!(watching.note, Some("Important issue to track".to_string()));
        assert!(watching.last_content_updated.is_some());
    }

    #[test]
    fn test_watching_type_serialization() {
        assert_eq!(
            serde_json::to_string(&WatchingType::Issue)
                .expect("WatchingType::Issue should serialize"),
            r#""issue""#
        );

        let result: Result<WatchingType, _> = serde_json::from_str(r#""issue""#);
        assert!(result.is_ok());
        assert_eq!(result.expect("valid JSON should deserialize"), WatchingType::Issue);
    }

    #[test]
    fn test_deserialize_watching_list() {
        let json = r#"[
            {
                "id": 1,
                "resourceAlreadyRead": false,
                "type": "issue",
                "created": "2023-01-01T00:00:00Z",
                "updated": "2023-01-01T00:00:00Z"
            },
            {
                "id": 2,
                "resourceAlreadyRead": true,
                "type": "issue",
                "created": "2023-01-02T00:00:00Z",
                "updated": "2023-01-02T00:00:00Z"
            }
        ]"#;

        let result: Result<Vec<Watching>, _> = serde_json::from_str(json);
        assert!(result.is_ok());

        let watchings = result.expect("valid JSON should deserialize");
        assert_eq!(watchings.len(), 2);
        assert_eq!(watchings[0].id.value(), 1);
        assert_eq!(watchings[1].id.value(), 2);
    }

    #[test]
    fn test_watching_count_deserialization() {
        let json = r#"{"count": 138}"#;
        let result: Result<crate::models::watching::WatchingCount, _> = serde_json::from_str(json);
        assert!(result.is_ok());

        let watching_count = result.expect("valid JSON should deserialize");
        assert_eq!(watching_count.count, 138);
    }

    #[test]
    fn test_watching_count_zero() {
        let json = r#"{"count": 0}"#;
        let result: Result<crate::models::watching::WatchingCount, _> = serde_json::from_str(json);
        assert!(result.is_ok());

        let watching_count = result.expect("valid JSON should deserialize");
        assert_eq!(watching_count.count, 0);
    }
}
