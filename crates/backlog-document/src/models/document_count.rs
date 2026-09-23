use serde::{Deserialize, Serialize};

/// Number of documents in a single project, without keyword filtering.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct DocumentCount {
    pub count: u32,
}
