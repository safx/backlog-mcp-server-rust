use backlog_api_core::IntoRequest;
use backlog_core::ProjectIdOrKey;

pub type GetCategoryListResponse = Vec<backlog_domain_models::Category>;

// GET /api/v2/projects/:projectIdOrKey/categories
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GetCategoryListParams {
    pub project_id_or_key: ProjectIdOrKey,
}

impl GetCategoryListParams {
    pub fn new(project_id_or_key: impl Into<ProjectIdOrKey>) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
        }
    }
}

impl IntoRequest for GetCategoryListParams {
    fn path(&self) -> String {
        format!("/api/v2/projects/{}/categories", self.project_id_or_key)
    }
}
