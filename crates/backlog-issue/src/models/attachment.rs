use backlog_core::{User, identifier::AttachmentId}; // Corrected path for AttachmentId
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[cfg(feature = "schemars")]
use schemars::JsonSchema;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct Attachment {
    pub id: AttachmentId,
    pub name: String,
    pub size: u64,
    #[serde(rename = "createdUser")]
    pub created_user: User,
    pub created: DateTime<Utc>,
}
