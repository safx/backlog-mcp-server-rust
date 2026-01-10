use backlog_core::{
    Star, User,
    identifier::{CommentId, Identifier},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[cfg(feature = "schemars")]
use schemars::JsonSchema;

/// Unified comment structure that supports both activity and issue contexts
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    pub id: CommentId,
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_user: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stars: Option<Vec<Star>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notifications: Option<Vec<serde_json::Value>>, // Will be NotificationForComment in future
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_log: Option<Vec<serde_json::Value>>, // Will be ChangeLogEntry in future
}

/// Simplified comment for activity contexts
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct ActivityComment {
    pub id: u32,
    pub content: String,
}

impl From<ActivityComment> for Comment {
    fn from(ac: ActivityComment) -> Self {
        Comment {
            id: CommentId::from(ac.id),
            content: Some(ac.content),
            created_user: None,
            created: None,
            updated: None,
            stars: None,
            notifications: None,
            change_log: None,
        }
    }
}

impl Comment {
    /// Convert to simplified activity comment
    pub fn to_activity_comment(&self) -> Option<ActivityComment> {
        self.content.as_ref().map(|content| ActivityComment {
            id: self.id.value(),
            content: content.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use backlog_core::identifier::Identifier;

    #[test]
    fn test_comment_creation() {
        let comment = Comment {
            id: CommentId::new(123),
            content: Some("Test comment".to_string()),
            created_user: None,
            created: None,
            updated: None,
            stars: None,
            notifications: None,
            change_log: None,
        };

        assert_eq!(comment.id.value(), 123);
        assert_eq!(comment.content, Some("Test comment".to_string()));
    }

    #[test]
    fn test_activity_comment_conversion() {
        let activity_comment = ActivityComment {
            id: 456,
            content: "Activity comment".to_string(),
        };

        let comment: Comment = activity_comment.clone().into();
        assert_eq!(comment.id.value(), 456);
        assert_eq!(comment.content, Some("Activity comment".to_string()));

        let back = comment.to_activity_comment();
        assert!(back.is_some());
        let back = back.unwrap();
        assert_eq!(back.id, 456);
        assert_eq!(back.content, "Activity comment");
    }

    #[test]
    fn test_comment_serialization() {
        let comment = Comment {
            id: CommentId::new(789),
            content: Some("Serialization test".to_string()),
            created_user: None,
            created: None,
            updated: None,
            stars: None,
            notifications: None,
            change_log: None,
        };

        let json = serde_json::to_string(&comment).unwrap();
        assert!(json.contains("\"id\":789"));
        assert!(json.contains("\"content\":\"Serialization test\""));
        // Optional fields should not appear
        assert!(!json.contains("\"createdUser\""));
        assert!(!json.contains("\"created\""));
    }
}
