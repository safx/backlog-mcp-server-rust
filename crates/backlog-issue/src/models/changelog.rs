use backlog_core::identifier::{AttachmentId, CustomFieldId};
use serde::{Deserialize, Serialize};

// Conditionally import and derive JsonSchema
#[cfg(feature = "schemars")]
use schemars::JsonSchema;

use crate::models::CustomFieldTypeId;

/// Represents an entry in the change log associated with a comment.
///
/// This details a specific modification that occurred, such as a change to an issue's
/// status, assignee, or other attributes, recorded as part of a comment.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct ChangeLogEntry {
    /// The field that was changed (e.g., "status", "assignee").
    pub field: String,
    /// The new value of the field after the change.
    pub new_value: Option<String>,
    /// The original value of the field before the change.
    pub original_value: Option<String>,
    /// Information about an attachment, if the change log entry relates to one.
    /// The structure of this field can vary, so it's represented as a generic JSON value.
    pub attachment_info: Option<AttachmentInfo>,
    /// Information about an attribute, if the change log entry relates to one.
    /// The structure of this field can vary, so it's represented as a generic JSON value.
    pub attribute_info: Option<CustomFieldInfo>,
    /// Information about a notification, if the change log entry relates to one.
    /// The structure of this field can vary, so it's represented as a generic JSON value.
    pub notification_info: Option<serde_json::Value>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct AttachmentInfo {
    pub id: AttachmentId,
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct CustomFieldInfo {
    pub id: CustomFieldId,
    pub type_id: CustomFieldTypeId,
}
