use backlog_core::identifier::ResolutionId;
use serde::{Deserialize, Serialize};

#[cfg(feature = "schemars")]
use schemars::JsonSchema;

/// Represents a resolution in Backlog.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct Resolution {
    pub id: ResolutionId,
    pub name: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolution_deserialize() {
        let json = r##"{"id":1,"name":"Fixed"}"##;
        let resolution: Resolution =
            serde_json::from_str(json).expect("should deserialize Resolution from JSON");
        assert_eq!(resolution.name, "Fixed");
    }
}
