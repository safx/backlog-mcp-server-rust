use backlog_api_core::IntoRequest;
use backlog_core::ProjectIdOrKey;
use backlog_domain_models::Project;

pub type GetProjectDetailResponse = Project;

// GET /api/v2/projects/:projectIdOrKey
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GetProjectDetailParams {
    pub project_id_or_key: ProjectIdOrKey,
}

impl GetProjectDetailParams {
    pub fn new(project_id_or_key: impl Into<ProjectIdOrKey>) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
        }
    }
}

impl IntoRequest for GetProjectDetailParams {
    fn path(&self) -> String {
        format!("/api/v2/projects/{}", self.project_id_or_key)
    }
}
