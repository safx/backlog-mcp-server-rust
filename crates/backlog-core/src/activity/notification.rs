use crate::User;
use crate::identifier::NotificationId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::NotificationReason;

/// Unified notification structure that supports all notification contexts
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Notification {
    pub id: NotificationId,
    pub already_read: bool,
    pub reason: NotificationReason,
    /// Omitted in some documented activity payloads.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_already_read: Option<bool>,

    // Optional fields depending on context
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<serde_json::Value>, // Will be Project in Phase 2

    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue: Option<serde_json::Value>, // Will be Issue in Phase 2

    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<serde_json::Value>, // Will be Comment in Phase 2

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_comment: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender: Option<User>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<DateTime<Utc>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::identifier::Identifier;

    #[test]
    fn test_minimal_notification() {
        let json = r#"{
            "id": 12345,
            "alreadyRead": false,
            "reason": 2,
            "resourceAlreadyRead": false
        }"#;

        let notification: Notification = serde_json::from_str(json).unwrap();
        assert_eq!(notification.id.value(), 12345);
        assert!(!notification.already_read);
        assert_eq!(notification.reason, NotificationReason::IssueCommented);
        assert_eq!(notification.resource_already_read, Some(false));
        assert!(notification.user.is_none());
        assert!(notification.project.is_none());
    }

    #[test]
    fn test_full_notification() {
        let json = r#"{
            "id": 67890,
            "alreadyRead": true,
            "reason": 3,
            "resourceAlreadyRead": true,
            "user": {
                "id": 1,
                "userId": "admin",
                "name": "Administrator",
                "roleType": 1,
                "mailAddress": "admin@example.com"
            },
            "project": {"id": 1, "name": "Test Project"},
            "issue": {"id": 100, "key": "TEST-1"},
            "sender": {
                "id": 2,
                "userId": "user",
                "name": "Test User",
                "roleType": 2,
                "mailAddress": "user@example.com"
            },
            "created": "2024-01-01T10:00:00Z"
        }"#;

        let notification: Notification = serde_json::from_str(json).unwrap();
        assert_eq!(notification.id.value(), 67890);
        assert!(notification.already_read);
        assert_eq!(notification.reason, NotificationReason::IssueCreated);
        assert!(notification.user.is_some());
        assert!(notification.project.is_some());
        assert!(notification.issue.is_some());
        assert!(notification.sender.is_some());
        assert!(notification.created.is_some());
    }

    #[test]
    fn test_notification_serialization_omits_none_fields() {
        let notification = Notification {
            id: NotificationId::new(999),
            already_read: false,
            reason: NotificationReason::FileAdded,
            resource_already_read: Some(true),
            user: None,
            project: None,
            issue: None,
            comment: None,
            pull_request: None,
            pull_request_comment: None,
            sender: None,
            created: None,
        };

        let json = serde_json::to_string(&notification).unwrap();
        assert!(!json.contains("\"user\":null"));
        assert!(!json.contains("\"project\":null"));
        assert!(json.contains("\"id\":999"));
        assert!(json.contains("\"alreadyRead\":false"));
    }
}
