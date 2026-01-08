use backlog_core::identifier::DocumentId;
use serde::{Deserialize, Serialize};

#[cfg(feature = "schemars")]
use schemars::JsonSchema;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct DocumentTreeNode {
    pub id: DocumentId,
    pub name: String, // Document title, or None for root tree nodes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_type: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<DocumentTreeNode>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct DocumentTreeRootNode {
    pub id: String, // "Active" or "Trash"
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<DocumentTreeNode>,
}
