use backlog_core::identifier::{CategoryId, ProjectId};
use serde::{Deserialize, Serialize};

#[cfg(feature = "schemars")]
use schemars::JsonSchema;

/// Represents a category in Backlog.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub id: CategoryId,
    pub project_id: ProjectId,
    pub name: String,
    pub display_order: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_category_deserialize() {
        let json = r##"{"id":1,"projectId":100,"name":"Backend","displayOrder":0}"##;
        let category: Category =
            serde_json::from_str(json).expect("should deserialize Category from JSON");
        assert_eq!(category.name, "Backend");
        assert_eq!(category.display_order, 0);
    }
}
