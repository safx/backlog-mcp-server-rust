#[cfg(feature = "writable")]
use backlog_api_core::{HttpMethod, IntoRequest};
#[cfg(feature = "writable")]
use backlog_core::ProjectIdOrKey;
#[cfg(feature = "writable")]
use serde::Serialize;

#[cfg(feature = "writable")]
pub type AddCategoryResponse = backlog_domain_models::Category;

#[cfg(feature = "writable")]
#[derive(Debug, Clone, Serialize)]
pub struct AddCategoryParams {
    #[serde(skip)]
    pub project_id_or_key: ProjectIdOrKey,
    pub name: String,
}

#[cfg(feature = "writable")]
impl AddCategoryParams {
    pub fn new(project_id_or_key: impl Into<ProjectIdOrKey>, name: impl Into<String>) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
            name: name.into(),
        }
    }
}

#[cfg(feature = "writable")]
impl IntoRequest for AddCategoryParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Post
    }

    fn path(&self) -> String {
        format!("/api/v2/projects/{}/categories", self.project_id_or_key)
    }

    fn to_form(&self) -> impl Serialize {
        self
    }
}
