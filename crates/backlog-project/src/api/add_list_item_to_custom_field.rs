#[cfg(feature = "writable")]
use backlog_api_core::{HttpMethod, IntoRequest};
#[cfg(feature = "writable")]
use backlog_core::{ProjectIdOrKey, identifier::CustomFieldId};
#[cfg(feature = "writable")]
use backlog_domain_models::CustomFieldType;
#[cfg(feature = "writable")]
use serde::Serialize;

/// Represents a successful response from adding a list item to a custom field.
///
/// Corresponds to `POST /api/v2/projects/:projectIdOrKey/customFields/:id/items`.
#[cfg(feature = "writable")]
pub type AddListItemToCustomFieldResponse = CustomFieldType;

/// Parameters for adding a list item to a list type custom field.
///
/// This API adds a new item to an existing list-type custom field (single or multiple selection).
/// Only administrators can call this API if the option "Add items in adding or editing issues"
/// is disabled in settings.
///
/// Corresponds to `POST /api/v2/projects/:projectIdOrKey/customFields/:id/items`.
#[cfg(feature = "writable")]
#[derive(Debug, Clone, Serialize)]
pub struct AddListItemToCustomFieldParams {
    #[serde(skip)]
    pub project_id_or_key: ProjectIdOrKey,
    #[serde(skip)]
    pub custom_field_id: CustomFieldId,
    pub name: String,
}

#[cfg(feature = "writable")]
impl AddListItemToCustomFieldParams {
    /// Creates new parameters for adding a list item to a custom field.
    pub fn new(
        project_id_or_key: impl Into<ProjectIdOrKey>,
        custom_field_id: CustomFieldId,
        name: impl Into<String>,
    ) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
            custom_field_id,
            name: name.into(),
        }
    }
}

#[cfg(feature = "writable")]
impl IntoRequest for AddListItemToCustomFieldParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Post
    }

    fn path(&self) -> String {
        format!(
            "/api/v2/projects/{}/customFields/{}/items",
            self.project_id_or_key, self.custom_field_id
        )
    }

    fn to_form(&self) -> impl Serialize {
        self
    }
}
