use backlog_core::identifier::{MilestoneId, ProjectId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[cfg(feature = "schemars")]
use schemars::JsonSchema;

/// Represents a milestone in Backlog.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct Milestone {
    pub id: MilestoneId,
    pub project_id: ProjectId,
    pub name: String,
    pub description: Option<String>,
    pub start_date: Option<DateTime<Utc>>,
    pub release_due_date: Option<DateTime<Utc>>,
    pub archived: bool,
    pub display_order: Option<i32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_milestone_deserialize() {
        let json = r##"{"id":1,"projectId":100,"name":"Sprint 1","description":"First sprint","startDate":"2024-01-01T00:00:00Z","releaseDueDate":"2024-01-14T00:00:00Z","archived":false,"displayOrder":0}"##;
        let milestone: Milestone =
            serde_json::from_str(json).expect("should deserialize Milestone from JSON");
        assert_eq!(milestone.name, "Sprint 1");
        assert!(!milestone.archived);
    }

    #[test]
    fn test_milestone_deserialize_minimal() {
        let json = r##"{"id":2,"projectId":100,"name":"Backlog","archived":false}"##;
        let milestone: Milestone =
            serde_json::from_str(json).expect("should deserialize Milestone with minimal fields");
        assert_eq!(milestone.name, "Backlog");
        assert!(milestone.description.is_none());
    }
}
