use backlog_api_core::IntoRequest;
use backlog_core::ProjectIdOrKey;

pub type GetIssueTypeListResponse = Vec<backlog_domain_models::IssueType>;

// GET /api/v2/projects/:projectIdOrKey/issueTypes
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GetIssueTypeListParams {
    pub project_id_or_key: ProjectIdOrKey,
}

impl GetIssueTypeListParams {
    pub fn new(project_id_or_key: impl Into<ProjectIdOrKey>) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
        }
    }
}

impl IntoRequest for GetIssueTypeListParams {
    fn path(&self) -> String {
        format!("/api/v2/projects/{}/issueTypes", self.project_id_or_key)
    }
}
