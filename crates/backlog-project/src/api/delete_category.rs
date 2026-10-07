#[cfg(feature = "writable")]
use backlog_api_core::{HttpMethod, IntoRequest};
#[cfg(feature = "writable")]
use backlog_core::ProjectIdOrKey;

#[cfg(feature = "writable")]
pub type DeleteCategoryResponse = backlog_domain_models::Category;

#[cfg(feature = "writable")]
#[derive(Debug, Clone)]
pub struct DeleteCategoryParams {
    pub project_id_or_key: ProjectIdOrKey,
    pub category_id: backlog_core::identifier::CategoryId,
}

#[cfg(feature = "writable")]
impl DeleteCategoryParams {
    pub fn new(
        project_id_or_key: impl Into<ProjectIdOrKey>,
        category_id: impl Into<backlog_core::identifier::CategoryId>,
    ) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
            category_id: category_id.into(),
        }
    }
}

#[cfg(feature = "writable")]
impl IntoRequest for DeleteCategoryParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Delete
    }

    fn path(&self) -> String {
        format!(
            "/api/v2/projects/{}/categories/{}",
            self.project_id_or_key, self.category_id
        )
    }
}
