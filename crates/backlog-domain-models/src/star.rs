use backlog_core::{User, identifier::StarId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Star {
    pub id: StarId,
    pub comment: Option<String>,
    pub url: String,
    pub title: String,
    pub presenter: User,
    pub created: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use backlog_core::{Language, Role, identifier::UserId};

    #[test]
    fn test_star_deserialization() {
        let json = r#"{
            "id": 75,
            "comment": null,
            "url": "https://xx.backlog.jp/view/BLG-1",
            "title": "[BLG-1] first issue | 課題の表示 - Backlog",
            "presenter": {
                "id": 1,
                "userId": "admin",
                "name": "admin",
                "roleType": 1,
                "lang": "ja",
                "mailAddress": "admin@example.com",
                "lastLoginTime": "2024-01-01T00:00:00Z"
            },
            "created": "2024-06-01T10:00:00Z"
        }"#;

        let star: Star = serde_json::from_str(json).unwrap();
        assert_eq!(star.id, StarId::new(75));
        assert_eq!(star.comment, None);
        assert_eq!(star.url, "https://xx.backlog.jp/view/BLG-1");
        assert_eq!(star.title, "[BLG-1] first issue | 課題の表示 - Backlog");
        assert_eq!(star.presenter.id, UserId::new(1));
        assert_eq!(star.presenter.name, "admin");
        assert_eq!(star.created.to_rfc3339(), "2024-06-01T10:00:00+00:00");
    }

    #[test]
    fn test_star_with_comment_deserialization() {
        let json = r#"{
            "id": 100,
            "comment": "Great work!",
            "url": "https://xx.backlog.jp/view/BLG-2",
            "title": "[BLG-2] second issue",
            "presenter": {
                "id": 2,
                "userId": "user1",
                "name": "Test User",
                "roleType": 2,
                "lang": "en",
                "mailAddress": "user@example.com",
                "lastLoginTime": null
            },
            "created": "2024-06-02T15:30:00Z"
        }"#;

        let star: Star = serde_json::from_str(json).unwrap();
        assert_eq!(star.id, StarId::new(100));
        assert_eq!(star.comment, Some("Great work!".to_string()));
        assert_eq!(star.presenter.role_type, Role::User);
        assert_eq!(star.presenter.lang, Some(Language::English));
        assert_eq!(star.presenter.last_login_time, None);
    }

    #[test]
    fn test_star_array_deserialization() {
        let json = r#"[
            {
                "id": 1,
                "comment": null,
                "url": "https://xx.backlog.jp/view/TEST-1",
                "title": "Test Issue 1",
                "presenter": {
                    "id": 1,
                    "userId": "admin",
                    "name": "Admin",
                    "roleType": 1,
                    "lang": "ja",
                    "mailAddress": "admin@example.com",
                    "lastLoginTime": "2024-01-01T00:00:00Z"
                },
                "created": "2024-01-01T00:00:00Z"
            },
            {
                "id": 2,
                "comment": "Nice!",
                "url": "https://xx.backlog.jp/view/TEST-2",
                "title": "Test Issue 2",
                "presenter": {
                    "id": 2,
                    "userId": "user",
                    "name": "User",
                    "roleType": 2,
                    "lang": "en",
                    "mailAddress": "user@example.com",
                    "lastLoginTime": null
                },
                "created": "2024-01-02T00:00:00Z"
            }
        ]"#;

        let stars: Vec<Star> = serde_json::from_str(json).unwrap();
        assert_eq!(stars.len(), 2);
        assert_eq!(stars[0].id, StarId::new(1));
        assert_eq!(stars[1].id, StarId::new(2));
        assert_eq!(stars[1].comment, Some("Nice!".to_string()));
    }
}
