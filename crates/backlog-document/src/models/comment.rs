use backlog_core::{
    User,
    identifier::{DocumentId, UserId},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A comment on a document page.
///
/// `status_id` and `comment_type` are kept raw because the official docs do not
/// enumerate their values. `replies` is assumed to share this shape.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct DocumentComment {
    pub id: String,
    pub document_id: DocumentId,
    pub status_id: i32,
    /// ProseMirror document serialized as a JSON string.
    pub content: String,
    pub plain: String,
    pub comment_type: String,
    pub created_user_id: UserId,
    pub created: DateTime<Utc>,
    pub updated_user_id: UserId,
    pub updated: DateTime<Utc>,
    pub created_user: User,
    #[serde(default)]
    pub replies: Vec<DocumentComment>,
}
