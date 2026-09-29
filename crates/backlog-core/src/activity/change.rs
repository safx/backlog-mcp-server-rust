use serde::{Deserialize, Serialize};

/// One entry of an activity's `content.changes`; the API uses snake_case keys here.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Change {
    pub field: String,
    pub new_value: String,
    pub old_value: Option<String>,
    #[serde(rename = "type")]
    pub change_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Comment {
    pub id: i64,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GroupProjectActivity {
    pub id: i64,
    #[serde(rename = "type")]
    pub type_id: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_change_serialization() {
        let change = Change {
            field: "status".to_string(),
            new_value: "Closed".to_string(),
            old_value: Some("Open".to_string()),
            change_type: "standard".to_string(),
        };

        let json = serde_json::to_string(&change).unwrap();
        assert!(json.contains("\"field\":\"status\""));
        assert!(json.contains("\"new_value\":\"Closed\""));
        assert!(json.contains("\"old_value\":\"Open\""));
        assert!(json.contains("\"type\":\"standard\""));
    }

    #[test]
    fn test_change_deserialization() {
        let json = r#"{
            "field": "priority",
            "new_value": "High",
            "old_value": null,
            "type": "standard"
        }"#;

        let change: Change = serde_json::from_str(json).unwrap();
        assert_eq!(change.field, "priority");
        assert_eq!(change.new_value, "High");
        assert_eq!(change.old_value, None);
        assert_eq!(change.change_type, "standard");
    }

    #[test]
    fn test_comment_serialization() {
        let comment = Comment {
            id: 12345,
            content: "This is a comment".to_string(),
        };

        let json = serde_json::to_string(&comment).unwrap();
        assert!(json.contains("\"id\":12345"));
        assert!(json.contains("\"content\":\"This is a comment\""));
    }

    #[test]
    fn test_comment_deserialization() {
        let json = r#"{
            "id": 67890,
            "content": "Test comment content"
        }"#;

        let comment: Comment = serde_json::from_str(json).unwrap();
        assert_eq!(comment.id, 67890);
        assert_eq!(comment.content, "Test comment content");
    }

    #[test]
    fn test_group_project_activity_serialization() {
        let activity = GroupProjectActivity {
            id: 999,
            type_id: 5,
        };

        let json = serde_json::to_string(&activity).unwrap();
        assert!(json.contains("\"id\":999"));
        assert!(json.contains("\"type\":5"));
    }

    #[test]
    fn test_group_project_activity_deserialization() {
        let json = r#"{
            "id": 123,
            "type": 7
        }"#;

        let activity: GroupProjectActivity = serde_json::from_str(json).unwrap();
        assert_eq!(activity.id, 123);
        assert_eq!(activity.type_id, 7);
    }
}
