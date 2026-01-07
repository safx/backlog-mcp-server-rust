use backlog_core::{
    Star, User,
    identifier::{ProjectId, WikiAttachmentId, WikiId, WikiTagId},
};
use backlog_file::models::SharedFile;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct Wiki {
    pub id: WikiId,
    pub project_id: ProjectId,
    pub name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<WikiTag>,
    pub created_user: User,
    pub created: DateTime<Utc>,
    pub updated_user: User,
    pub updated: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct WikiAttachment {
    pub id: WikiAttachmentId,
    pub name: String,
    pub size: u64,
    #[serde(rename = "createdUser")]
    pub created_user: User,
    pub created: DateTime<Utc>,
}

/// Represents the count of wiki pages in a project.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WikiCount {
    /// The number of wiki pages
    pub count: u32,
}

/// Represents detailed information about a wiki page, including content, attachments, and stars.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WikiDetail {
    /// The unique identifier for the wiki page.
    pub id: WikiId,
    /// The project ID this wiki page belongs to.
    pub project_id: ProjectId,
    /// The name/title of the wiki page.
    pub name: String,
    /// The content of the wiki page (usually in Markdown format).
    pub content: String,
    /// Tags associated with this wiki page.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<WikiTag>,
    /// File attachments associated with this wiki page.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attachments: Vec<WikiAttachment>,
    /// Shared files linked to this wiki page.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub shared_files: Vec<SharedFile>,
    /// Stars given to this wiki page.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stars: Vec<Star>,
    /// The user who created this wiki page.
    pub created_user: User,
    /// The timestamp when this wiki page was created.
    pub created: DateTime<Utc>,
    /// The user who last updated this wiki page.
    pub updated_user: User,
    /// The timestamp when this wiki page was last updated.
    pub updated: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct WikiTag {
    pub id: WikiTagId,
    pub name: String,
}

/// Represents a single entry in the wiki page history.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct WikiHistory {
    /// The wiki page ID this history entry belongs to.
    #[serde(rename = "pageId")]
    pub page_id: WikiId,
    /// The version number of this history entry.
    pub version: u32,
    /// The name/title of the wiki page at this version.
    pub name: String,
    /// The content of the wiki page at this version.
    pub content: String,
    /// The user who created this version.
    #[serde(rename = "createdUser")]
    pub created_user: User,
    /// The timestamp when this version was created.
    pub created: DateTime<Utc>,
}

/// Represents the sort order for wiki history entries.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Default)]
pub enum HistoryOrder {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    #[default]
    Desc,
}


#[cfg(test)]
mod tests {
    use super::*;
    use backlog_core::{
        Language, Role,
        identifier::{Identifier, UserId},
    };
    use serde_json;

    #[test]
    fn test_wiki_history_deserialization() {
        let json = r#"{
            "pageId": 1,
            "version": 2,
            "name": "Updated Page",
            "content": "New content",
            "createdUser": {
                "id": 123,
                "name": "john",
                "roleType": 1,
                "lang": "ja",
                "mailAddress": "john@example.com"
            },
            "created": "2014-06-24T05:04:48Z"
        }"#;

        let history: WikiHistory = serde_json::from_str(json).unwrap();
        assert_eq!(history.page_id.value(), 1);
        assert_eq!(history.version, 2);
        assert_eq!(history.name, "Updated Page");
        assert_eq!(history.content, "New content");
        assert_eq!(history.created_user.name, "john");
    }

    #[test]
    fn test_wiki_history_serialization() {
        let history = WikiHistory {
            page_id: WikiId::new(1),
            version: 2,
            name: "Test Page".to_string(),
            content: "Test content".to_string(),
            created_user: User {
                id: UserId::new(123),
                user_id: Some("john".to_string()),
                name: "john".to_string(),
                role_type: Role::User,
                lang: Some(Language::Japanese),
                mail_address: "john@example.com".to_string(),
                last_login_time: None,
            },
            created: chrono::DateTime::parse_from_rfc3339("2014-06-24T05:04:48Z")
                .unwrap()
                .with_timezone(&Utc),
        };

        let json = serde_json::to_string(&history).unwrap();
        assert!(json.contains("\"pageId\":1"));
        assert!(json.contains("\"version\":2"));
        assert!(json.contains("\"name\":\"Test Page\""));
    }

    #[test]
    fn test_history_order_default() {
        let order = HistoryOrder::default();
        assert_eq!(order, HistoryOrder::Desc);
    }

    #[test]
    fn test_history_order_serialization() {
        let asc = HistoryOrder::Asc;
        let desc = HistoryOrder::Desc;

        assert_eq!(serde_json::to_string(&asc).unwrap(), "\"asc\"");
        assert_eq!(serde_json::to_string(&desc).unwrap(), "\"desc\"");
    }

    #[test]
    fn test_history_order_deserialization() {
        let asc: HistoryOrder = serde_json::from_str("\"asc\"").unwrap();
        let desc: HistoryOrder = serde_json::from_str("\"desc\"").unwrap();

        assert_eq!(asc, HistoryOrder::Asc);
        assert_eq!(desc, HistoryOrder::Desc);
    }
}
