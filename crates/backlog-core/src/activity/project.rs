use crate::identifier::{IssueId, ProjectId};
use serde::{Deserialize, Serialize};

/// Simplified project representation for activity contexts
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ActivityProject {
    pub id: ProjectId,
    pub project_key: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
}

/// Simplified issue representation for activity contexts
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ActivityIssue {
    pub id: IssueId,
    #[serde(rename = "keyId")]
    pub key_id: i64,
    pub summary: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl ActivityProject {
    /// Create from JSON value (for migration from Phase 1)
    pub fn from_json_value(value: &serde_json::Value) -> Option<Self> {
        value
            .get("id")
            .and_then(|v| v.as_u64())
            .and_then(|id| u32::try_from(id).ok())
            .map(|id| ActivityProject {
                id: ProjectId::from(id),
                project_key: value
                    .get("projectKey")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                name: value
                    .get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                archived: value.get("archived").and_then(|v| v.as_bool()),
            })
    }
}

impl ActivityIssue {
    /// Create from JSON value (for migration from Phase 1)
    pub fn from_json_value(value: &serde_json::Value) -> Option<Self> {
        value
            .get("id")
            .and_then(|v| v.as_u64())
            .and_then(|id| u32::try_from(id).ok())
            .map(|id| ActivityIssue {
                id: IssueId::from(id),
                key_id: value.get("keyId").and_then(|v| v.as_i64()).unwrap_or(0),
                summary: value
                    .get("summary")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                description: value
                    .get("description")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string()),
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::identifier::Identifier;

    #[test]
    fn test_activity_project_creation() {
        let project = ActivityProject {
            id: ProjectId::new(1),
            project_key: "TEST".to_string(),
            name: "Test Project".to_string(),
            archived: Some(false),
        };

        assert_eq!(project.id.value(), 1);
        assert_eq!(project.project_key, "TEST");
        assert_eq!(project.name, "Test Project");
        assert_eq!(project.archived, Some(false));
    }

    #[test]
    fn test_activity_issue_creation() {
        let issue = ActivityIssue {
            id: IssueId::new(100),
            key_id: 1,
            summary: "Test Issue".to_string(),
            description: Some("Test description".to_string()),
        };

        assert_eq!(issue.id.value(), 100);
        assert_eq!(issue.key_id, 1);
        assert_eq!(issue.summary, "Test Issue");
        assert_eq!(issue.description, Some("Test description".to_string()));
    }

    #[test]
    fn test_activity_project_serialization() {
        let project = ActivityProject {
            id: ProjectId::new(2),
            project_key: "PROJ".to_string(),
            name: "Project Name".to_string(),
            archived: None,
        };

        let json = serde_json::to_string(&project).unwrap();
        assert!(json.contains("\"id\":2"));
        assert!(json.contains("\"projectKey\":\"PROJ\""));
        assert!(json.contains("\"name\":\"Project Name\""));
        assert!(!json.contains("\"archived\"")); // Should be omitted when None
    }

    #[test]
    fn test_activity_project_from_json_value() {
        let json = serde_json::json!({
            "id": 3,
            "projectKey": "JSON",
            "name": "JSON Project",
            "archived": true
        });

        let project = ActivityProject::from_json_value(&json).unwrap();
        assert_eq!(project.id.value(), 3);
        assert_eq!(project.project_key, "JSON");
        assert_eq!(project.name, "JSON Project");
        assert_eq!(project.archived, Some(true));
    }

    #[test]
    fn test_activity_issue_from_json_value() {
        let json = serde_json::json!({
            "id": 200,
            "keyId": 5,
            "summary": "JSON Issue",
            "description": "Description from JSON"
        });

        let issue = ActivityIssue::from_json_value(&json).unwrap();
        assert_eq!(issue.id.value(), 200);
        assert_eq!(issue.key_id, 5);
        assert_eq!(issue.summary, "JSON Issue");
        assert_eq!(issue.description, Some("Description from JSON".to_string()));
    }
}
