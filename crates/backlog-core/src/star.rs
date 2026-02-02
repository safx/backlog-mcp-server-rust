use crate::{User, identifier::StarId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// Conditionally import and derive JsonSchema
#[cfg(feature = "schemars")]
use schemars::JsonSchema;

/// Represents a "star" given to a comment.
///
/// Users can star comments to mark them as noteworthy or for quick reference.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct Star {
    /// The ID of the star.
    pub id: StarId,
    /// Optional comment associated with the star.
    pub comment: Option<String>,
    /// URL related to the star (often points to the starred item).
    pub url: String,
    /// The user who gave the star.
    pub presenter: User,
    /// The timestamp of when the star was given.
    pub created: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Role;
    use crate::identifier::{Identifier, UserId};
    use serde_json::Value;

    fn create_test_user() -> User {
        User {
            id: UserId::new(1),
            user_id: Some("admin".to_string()),
            name: "山田太郎".to_string(),
            role_type: Role::Admin,
            lang: Some(crate::Language::Japanese),
            mail_address: "yamada@example.com".to_string(),
            last_login_time: Some("2024-01-15T09:30:00Z".parse().unwrap()),
        }
    }

    #[test]
    fn test_star_deserialization() {
        let json_str = r#"{
            "id": 12345,
            "comment": "重要なコメント",
            "url": "https://example.backlog.jp/view/PROJ-123",
            "presenter": {
                "id": 1,
                "userId": "admin",
                "name": "山田太郎",
                "roleType": 1,
                "lang": "ja",
                "mailAddress": "yamada@example.com",
                "lastLoginTime": "2024-01-15T09:30:00Z"
            },
            "created": "2024-01-20T14:30:00Z"
        }"#;

        let star: Star = serde_json::from_str(json_str).unwrap();
        assert_eq!(star.id.value(), 12345);
        assert_eq!(star.comment, Some("重要なコメント".to_string()));
        assert_eq!(star.url, "https://example.backlog.jp/view/PROJ-123");
        assert_eq!(star.presenter.name, "山田太郎");
        assert_eq!(star.presenter.id.value(), 1);

        let expected_dt: DateTime<Utc> = "2024-01-20T14:30:00Z".parse().unwrap();
        assert_eq!(star.created, expected_dt);
    }

    #[test]
    fn test_star_deserialization_comment_null() {
        let json_str = r#"{
            "id": 67890,
            "comment": null,
            "url": "https://example.backlog.jp/view/PROJ-456",
            "presenter": {
                "id": 2,
                "userId": null,
                "name": "Guest User",
                "roleType": 4,
                "lang": null,
                "mailAddress": "guest@example.com",
                "lastLoginTime": null
            },
            "created": "2024-02-01T00:00:00Z"
        }"#;

        let star: Star = serde_json::from_str(json_str).unwrap();
        assert_eq!(star.id.value(), 67890);
        assert_eq!(star.comment, None);
        assert_eq!(star.presenter.user_id, None);
        assert_eq!(star.presenter.role_type, Role::Viewer);
    }

    #[test]
    fn test_star_serialization() {
        let star = Star {
            id: StarId::new(11111),
            comment: Some("Test comment".to_string()),
            url: "https://test.backlog.jp/view/TEST-1".to_string(),
            presenter: create_test_user(),
            created: "2024-03-15T12:00:00Z".parse().unwrap(),
        };

        let json = serde_json::to_string(&star).unwrap();
        let actual: Value = serde_json::from_str(&json).unwrap();

        // Verify camelCase field names
        assert!(actual.get("id").is_some());
        assert!(actual.get("comment").is_some());
        assert!(actual.get("url").is_some());
        assert!(actual.get("presenter").is_some());
        assert!(actual.get("created").is_some());

        assert_eq!(actual["id"], 11111);
        assert_eq!(actual["comment"], "Test comment");
    }

    #[test]
    fn test_star_serialization_comment_none() {
        let star = Star {
            id: StarId::new(22222),
            comment: None,
            url: "https://test.backlog.jp/view/TEST-2".to_string(),
            presenter: create_test_user(),
            created: "2024-03-15T12:00:00Z".parse().unwrap(),
        };

        let json = serde_json::to_string(&star).unwrap();
        let actual: Value = serde_json::from_str(&json).unwrap();

        assert!(actual["comment"].is_null());
    }

    #[test]
    fn test_star_round_trip() {
        let original = Star {
            id: StarId::new(33333),
            comment: Some("Round trip test".to_string()),
            url: "https://test.backlog.jp/view/TEST-3".to_string(),
            presenter: create_test_user(),
            created: "2024-03-15T12:00:00Z".parse().unwrap(),
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: Star = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }
}
