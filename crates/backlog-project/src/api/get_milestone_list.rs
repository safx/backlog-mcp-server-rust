use backlog_api_core::IntoRequest;
use backlog_core::ProjectIdOrKey;

pub type GetMilestoneListResponse = Vec<backlog_domain_models::Milestone>;

// GET /api/v2/projects/:projectIdOrKey/versions
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GetMilestoneListParams {
    pub project_id_or_key: ProjectIdOrKey,
}

impl GetMilestoneListParams {
    pub fn new(project_id_or_key: impl Into<ProjectIdOrKey>) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
        }
    }
}

impl IntoRequest for GetMilestoneListParams {
    fn path(&self) -> String {
        format!("/api/v2/projects/{}/versions", self.project_id_or_key)
    }
}
