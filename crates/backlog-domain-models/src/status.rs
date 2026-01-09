use backlog_core::identifier::{ProjectId, StatusId};
use serde::{Deserialize, Serialize};

#[cfg(feature = "schemars")]
use schemars::JsonSchema;

/// Represents a status definition within a project.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct Status {
    /// Unique identifier for the status.
    pub id: StatusId,
    /// Identifier of the project this status belongs to.
    pub project_id: ProjectId,
    /// Name of the status.
    pub name: String,
    /// Color of the status in hex format (e.g., "#e30000").
    pub color: String,
    /// Order in which the status is displayed.
    pub display_order: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_deserialize() {
        let json =
            r##"{"id":1,"projectId":100,"name":"Open","color":"#ea2c00","displayOrder":1000}"##;
        let status: Status =
            serde_json::from_str(json).expect("should deserialize Status from JSON");
        assert_eq!(status.name, "Open");
        assert_eq!(status.color, "#ea2c00");
    }
}
