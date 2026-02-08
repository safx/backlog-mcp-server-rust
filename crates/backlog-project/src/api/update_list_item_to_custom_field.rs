#[cfg(feature = "writable")]
use backlog_api_core::{HttpMethod, IntoRequest};
#[cfg(feature = "writable")]
use backlog_core::{ProjectIdOrKey, identifier::CustomFieldId};
#[cfg(feature = "writable")]
use backlog_domain_models::CustomFieldType;
#[cfg(feature = "writable")]
use serde::Serialize;

/// Represents a successful response from updating a list item in a custom field.
///
/// Corresponds to `PATCH /api/v2/projects/:projectIdOrKey/customFields/:id/items/:itemId`.
#[cfg(feature = "writable")]
pub type UpdateListItemToCustomFieldResponse = CustomFieldType;

/// Parameters for updating a list item in a list type custom field.
///
/// This API updates an existing item in a list-type custom field (single or multiple selection).
/// Only administrators and project administrators can call this API.
///
/// Corresponds to `PATCH /api/v2/projects/:projectIdOrKey/customFields/:id/items/:itemId`.
#[cfg(feature = "writable")]
#[derive(Debug, Clone, Serialize)]
pub struct UpdateListItemToCustomFieldParams {
    #[serde(skip)]
    pub project_id_or_key: ProjectIdOrKey,
    #[serde(skip)]
    pub custom_field_id: CustomFieldId,
    #[serde(skip)]
    pub item_id: u32,
    pub name: String,
}

#[cfg(feature = "writable")]
impl UpdateListItemToCustomFieldParams {
    /// Creates new parameters for updating a list item in a custom field.
    pub fn new(
        project_id_or_key: impl Into<ProjectIdOrKey>,
        custom_field_id: CustomFieldId,
        item_id: u32,
        name: impl Into<String>,
    ) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
            custom_field_id,
            item_id,
            name: name.into(),
        }
    }
}

#[cfg(feature = "writable")]
impl IntoRequest for UpdateListItemToCustomFieldParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Patch
    }

    fn path(&self) -> String {
        format!(
            "/api/v2/projects/{}/customFields/{}/items/{}",
            self.project_id_or_key, self.custom_field_id, self.item_id
        )
    }

    fn to_form(&self) -> impl Serialize {
        self
    }
}
