use crate::User;
use crate::identifier::ActivityId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::Notification;

/// Unified activity structure that supports all activity contexts
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Activity {
    pub id: ActivityId,
    pub project: serde_json::Value, // Phase 1: JSON value to avoid circular dependencies
    #[serde(rename = "type")]
    pub type_id: i32,
    /// Raw content; its shape depends on `type_id` (field names are snake_case on the wire).
    pub content: serde_json::Value,
    /// Empty in recent-update lists; populated by `GET /api/v2/activities/:activityId`.
    pub notifications: Vec<Notification>,
    pub created_user: User,
    pub created: DateTime<Utc>,
}

impl Activity {
    /// Helper method for migration: extract project info from JSON
    pub fn project_id(&self) -> Option<u32> {
        self.project
            .get("id")
            .and_then(|v| v.as_u64())
            .and_then(|id| u32::try_from(id).ok())
    }

    pub fn project_name(&self) -> Option<&str> {
        self.project.get("name").and_then(|v| v.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::activity::{Change, NotificationReason};
    use crate::identifier::Identifier;

    #[test]
    fn test_activity_serialization() {
        let project = serde_json::json!({"id": 1, "name": "Test Project"});

        let activity = Activity {
            id: ActivityId::new(12345),
            project,
            type_id: 1,
            content: serde_json::json!({"id": 100, "key_id": 200, "summary": "Test Summary"}),
            notifications: vec![],
            created_user: User {
                id: crate::identifier::UserId::new(1),
                user_id: Some("testuser".to_string()),
                name: "Test User".to_string(),
                role_type: crate::Role::Admin,
                lang: None,
                mail_address: "test@example.com".to_string(),
                last_login_time: None,
            },
            created: DateTime::parse_from_rfc3339("2024-01-01T10:00:00Z")
                .unwrap()
                .with_timezone(&Utc),
        };

        let json = serde_json::to_string(&activity).unwrap();
        assert!(json.contains("\"id\":12345"));
        assert!(json.contains("\"type\":1"));
        assert!(json.contains("\"createdUser\""));
        assert!(json.contains("\"created\":\"2024-01-01T10:00:00Z\""));
    }

    #[test]
    fn test_activity_deserialization() {
        let json = r#"{
            "id": 67890,
            "project": {"id": 2, "name": "Another Project"},
            "type": 2,
            "content": {
                "id": 300,
                "key_id": 400,
                "summary": "Issue Updated",
                "description": "Description updated",
                "comment": {
                    "id": 500,
                    "content": "Update comment"
                },
                "changes": [{
                    "field": "status",
                    "new_value": "Closed",
                    "old_value": null,
                    "type": "standard"
                }]
            },
            "notifications": [{
                "id": 25,
                "alreadyRead": false,
                "reason": 2,
                "user": {"id": 1, "userId": "admin", "name": "admin", "roleType": 1, "mailAddress": "admin@example.com"},
                "resourceAlreadyRead": false
            }],
            "createdUser": {
                "id": 2,
                "userId": "admin",
                "name": "Administrator",
                "roleType": 1,
                "mailAddress": "admin@example.com"
            },
            "created": "2024-01-02T15:30:00Z"
        }"#;

        let activity: Activity = serde_json::from_str(json).unwrap();
        assert_eq!(activity.id.value(), 67890);
        assert_eq!(activity.type_id, 2);

        let content = &activity.content;
        assert_eq!(content["id"], 300);
        assert_eq!(content["key_id"], 400);
        assert_eq!(content["summary"], "Issue Updated");
        assert_eq!(content["comment"]["content"], "Update comment");
        let changes: Vec<Change> = serde_json::from_value(content["changes"].clone()).unwrap();
        assert_eq!(changes.len(), 1);
        assert_eq!(changes[0].new_value, "Closed");
        assert_eq!(changes[0].old_value, None);
        assert_eq!(activity.notifications.len(), 1);
        assert_eq!(
            activity.notifications[0].reason,
            NotificationReason::IssueCommented
        );
        assert!(activity.notifications[0].user.is_some());
    }

    #[test]
    fn test_project_id_out_of_range() {
        let json = r#"{
            "id": 1,
            "project": {"id": 4294967296, "name": "Overflow"},
            "type": 1,
            "content": {},
            "notifications": [],
            "createdUser": {"id": 1, "name": "Admin", "roleType": 1, "mailAddress": "admin@example.com"},
            "created": "2024-01-03T12:00:00Z"
        }"#;
        let activity: Activity = serde_json::from_str(json).unwrap();
        assert_eq!(activity.project_id(), None);
    }

    #[test]
    fn test_activity_with_user_management_content() {
        let json = r#"{
            "id": 11111,
            "project": {"id": 3},
            "type": 6,
            "content": {
                "users": [{"id": 10, "userId": "newuser", "name": "New User", "roleType": 2, "mailAddress": "newuser@example.com"}],
                "groupProjectActivities": [{"id": 20, "type": 5}],
                "comment": "User added to project"
            },
            "notifications": [],
            "createdUser": {
                "id": 1,
                "name": "Admin",
                "roleType": 1,
                "mailAddress": "admin@example.com"
            },
            "created": "2024-01-03T12:00:00Z"
        }"#;

        let activity: Activity = serde_json::from_str(json).unwrap();
        assert_eq!(activity.id.value(), 11111);
        assert_eq!(activity.type_id, 6);

        let content = &activity.content;
        assert_eq!(content["users"].as_array().map(Vec::len), Some(1));
        assert_eq!(content["groupProjectActivities"][0]["id"], 20);
        assert_eq!(content["comment"], "User added to project");
    }
}
