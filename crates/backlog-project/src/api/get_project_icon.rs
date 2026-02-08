use backlog_api_core::IntoDownloadRequest;
use backlog_core::ProjectIdOrKey;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GetProjectIconParams {
    pub project_id_or_key: ProjectIdOrKey,
}

impl GetProjectIconParams {
    pub fn new(project_id_or_key: impl Into<ProjectIdOrKey>) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
        }
    }
}

impl IntoDownloadRequest for GetProjectIconParams {
    fn path(&self) -> String {
        format!("/api/v2/projects/{}/image", self.project_id_or_key)
    }
}
