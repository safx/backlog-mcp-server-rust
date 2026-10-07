#[cfg(feature = "writable")]
use backlog_api_core::{HttpMethod, IntoRequest};
#[cfg(feature = "writable")]
use backlog_core::ProjectIdOrKey;

#[cfg(feature = "writable")]
pub type DeleteVersionResponse = backlog_domain_models::Milestone;

#[cfg(feature = "writable")]
#[derive(Debug, Clone)]
pub struct DeleteVersionParams {
    pub project_id_or_key: ProjectIdOrKey,
    pub version_id: backlog_core::identifier::MilestoneId,
}

#[cfg(feature = "writable")]
impl DeleteVersionParams {
    pub fn new(
        project_id_or_key: impl Into<ProjectIdOrKey>,
        version_id: impl Into<backlog_core::identifier::MilestoneId>,
    ) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
            version_id: version_id.into(),
        }
    }
}

#[cfg(feature = "writable")]
impl IntoRequest for DeleteVersionParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Delete
    }

    fn path(&self) -> String {
        format!(
            "/api/v2/projects/{}/versions/{}",
            self.project_id_or_key, self.version_id
        )
    }
}
