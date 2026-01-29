use backlog_core::identifier::{DocumentId, ProjectId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::models::tag::DocumentTag;

/// Response type for add_document and delete_document APIs
///
/// Unlike DocumentDetail, this type uses userId fields instead of full User objects.
/// Note: json and plain fields may be null in delete_document responses.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct DocumentResponse {
    pub id: DocumentId,
    pub project_id: ProjectId,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json: Option<JsonValue>, // ProseMirror JSON (null in delete responses)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plain: Option<String>, // Plain text (null in delete responses)
    pub status_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
    pub created_user_id: u32,
    pub created: DateTime<Utc>,
    pub updated_user_id: u32,
    pub updated: DateTime<Utc>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<DocumentTag>,
}
