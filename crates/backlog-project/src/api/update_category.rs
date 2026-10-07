#[cfg(feature = "writable")]
use backlog_api_core::{HttpMethod, IntoRequest};
#[cfg(feature = "writable")]
use backlog_core::ProjectIdOrKey;
#[cfg(feature = "writable")]
use serde::Serialize;

#[cfg(feature = "writable")]
pub type UpdateCategoryResponse = backlog_domain_models::Category;

#[cfg(feature = "writable")]
#[derive(Debug, Clone, Serialize)]
pub struct UpdateCategoryParams {
    #[serde(skip)]
    pub project_id_or_key: ProjectIdOrKey,
    #[serde(skip)]
    pub category_id: backlog_core::identifier::CategoryId,
    pub name: String,
}

#[cfg(feature = "writable")]
impl UpdateCategoryParams {
    pub fn new(
        project_id_or_key: impl Into<ProjectIdOrKey>,
        category_id: impl Into<backlog_core::identifier::CategoryId>,
        name: impl Into<String>,
    ) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
            category_id: category_id.into(),
            name: name.into(),
        }
    }
}

#[cfg(feature = "writable")]
impl IntoRequest for UpdateCategoryParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Patch
    }

    fn path(&self) -> String {
        format!(
            "/api/v2/projects/{}/categories/{}",
            self.project_id_or_key, self.category_id
        )
    }

    fn to_form(&self) -> impl Serialize {
        self
    }
}
