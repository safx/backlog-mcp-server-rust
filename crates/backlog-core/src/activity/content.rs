use crate::User;
use serde::{Deserialize, Deserializer, Serialize};

use super::{Change, Comment, GroupProjectActivity};

/// Unified content type that merges both content variants
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
#[non_exhaustive]
pub enum Content {
    /// Standard content with comment and changes
    Standard {
        id: i64,
        #[serde(rename = "keyId")]
        key_id: Option<i64>,
        summary: Option<String>,
        description: Option<String>,
        #[serde(default, deserialize_with = "deserialize_comment")]
        comment: Option<Comment>,
        changes: Option<Vec<Change>>,
    },
    /// User management content
    UserManagement {
        users: Option<Vec<User>>,
        #[serde(rename = "groupProjectActivities")]
        group_project_activities: Option<Vec<GroupProjectActivity>>,
        comment: Option<String>,
    },
    /// Detailed content variants (from content.rs)
    IssueCreated(Box<IssueCreatedContent>),
    Issue(Box<IssueContent>),
    IssueDeleted(Box<IssueDeletedContent>),
    IssueMultiUpdate(Box<IssueMultiUpdateContent>),
    Wiki(Box<WikiContent>),
    File(Box<FileContent>),
    Svn(Box<SvnContent>),
    Git(Box<GitContent>),
    GitRepositoryCreated(Box<GitRepositoryCreatedContent>),
    EditMember(Box<EditMemberContent>),
    PullRequest(Box<PullRequestContent>),
    Version(Box<VersionContent>),
    VersionUpdated(Box<VersionUpdatedContent>),
    ProjectTeam(Box<ProjectTeamContent>),
    StatusDeleted(Box<StatusDeletedContent>),
}

// Detailed content types (simplified for Phase 1)
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct IssueCreatedContent {
    pub id: i32,
    #[serde(rename = "keyId")]
    pub key_id: i32,
    pub summary: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct IssueContent {}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct IssueDeletedContent {}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct IssueMultiUpdateContent {}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct WikiContent {}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FileContent {
    pub id: i32,
    pub dir: String,
    pub name: String,
    pub size: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SvnContent {
    pub rev: crate::identifier::SvnRevision,
    pub comment: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GitContent {}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GitRepositoryCreatedContent {}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct EditMemberContent {}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PullRequestContent {}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct VersionContent {}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct VersionUpdatedContent {}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProjectTeamContent {}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct StatusDeletedContent {}

fn deserialize_comment<'de, D>(deserializer: D) -> Result<Option<Comment>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::{self, Visitor};

    struct CommentVisitor;

    impl<'de> Visitor<'de> for CommentVisitor {
        type Value = Option<Comment>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("null, empty string, or Comment object")
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            if value.is_empty() {
                Ok(None)
            } else {
                Err(E::custom("expected empty string or Comment object"))
            }
        }

        fn visit_map<M>(self, map: M) -> Result<Self::Value, M::Error>
        where
            M: de::MapAccess<'de>,
        {
            let comment = Comment::deserialize(de::value::MapAccessDeserializer::new(map))?;
            Ok(Some(comment))
        }
    }

    deserializer.deserialize_any(CommentVisitor)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_content_serialization() {
        let content = Content::Standard {
            id: 123,
            key_id: Some(456),
            summary: Some("Test Summary".to_string()),
            description: Some("Test Description".to_string()),
            comment: Some(Comment {
                id: 789,
                content: "Test comment".to_string(),
            }),
            changes: Some(vec![Change {
                field: "status".to_string(),
                new_value: "Closed".to_string(),
                old_value: "Open".to_string(),
                change_type: "standard".to_string(),
            }]),
        };

        let json = serde_json::to_string(&content).unwrap();
        assert!(json.contains("\"id\":123"));
        assert!(json.contains("\"keyId\":456"));
        assert!(json.contains("\"summary\":\"Test Summary\""));
    }

    #[test]
    fn test_user_management_content() {
        let content = Content::UserManagement {
            users: Some(vec![]),
            group_project_activities: Some(vec![GroupProjectActivity {
                id: 111,
                type_id: 5,
            }]),
            comment: Some("User added".to_string()),
        };

        let json = serde_json::to_string(&content).unwrap();
        assert!(json.contains("\"users\":[]"));
        assert!(json.contains("\"groupProjectActivities\""));
        assert!(json.contains("\"comment\":\"User added\""));
    }

    #[test]
    fn test_issue_created_content() {
        let content = Content::IssueCreated(Box::new(IssueCreatedContent {
            id: 999,
            key_id: 888,
            summary: "New Issue".to_string(),
            description: "Issue description".to_string(),
        }));

        let json = serde_json::to_string(&content).unwrap();
        assert!(json.contains("\"id\":999"));
        assert!(json.contains("\"keyId\":888"));
        assert!(json.contains("\"summary\":\"New Issue\""));
    }

    #[test]
    fn test_content_deserialization_standard() {
        let json = r#"{
            "id": 100,
            "keyId": 200,
            "summary": "Test",
            "description": "Desc",
            "comment": {
                "id": 300,
                "content": "Comment text"
            },
            "changes": []
        }"#;

        let content: Content = serde_json::from_str(json).unwrap();
        match content {
            Content::Standard {
                id,
                key_id,
                summary,
                ..
            } => {
                assert_eq!(id, 100);
                assert_eq!(key_id, Some(200));
                assert_eq!(summary, Some("Test".to_string()));
            }
            _ => panic!("Expected Standard content"),
        }
    }

    #[test]
    fn test_content_with_empty_comment() {
        let json = r#"{
            "id": 100,
            "summary": "Test",
            "comment": ""
        }"#;

        let content: Content = serde_json::from_str(json).unwrap();
        match content {
            Content::Standard { comment, .. } => {
                assert!(comment.is_none());
            }
            _ => panic!("Expected Standard content"),
        }
    }
}
