use backlog_api_core::IntoRequest;
use backlog_core::ProjectIdOrKey;

pub type GetStatusListResponse = Vec<backlog_domain_models::Status>;

// GET /api/v2/projects/:projectIdOrKey/statuses
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GetStatusListParams {
    pub project_id_or_key: ProjectIdOrKey,
}

impl GetStatusListParams {
    pub fn new(project_id_or_key: impl Into<ProjectIdOrKey>) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
        }
    }
}

impl IntoRequest for GetStatusListParams {
    fn path(&self) -> String {
        format!("/api/v2/projects/{}/statuses", self.project_id_or_key)
    }
}
