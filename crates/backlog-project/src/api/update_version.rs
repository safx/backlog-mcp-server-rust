#[cfg(feature = "writable")]
use backlog_api_core::{HttpMethod, IntoRequest};
#[cfg(feature = "writable")]
use backlog_core::{ApiDate, ProjectIdOrKey};
#[cfg(feature = "writable")]
use serde::Serialize;

#[cfg(feature = "writable")]
pub type UpdateVersionResponse = backlog_domain_models::Milestone;

#[cfg(feature = "writable")]
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateVersionParams {
    #[serde(skip)]
    pub project_id_or_key: ProjectIdOrKey,
    #[serde(skip)]
    pub version_id: backlog_core::identifier::MilestoneId,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<ApiDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_due_date: Option<ApiDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
}

#[cfg(feature = "writable")]
impl UpdateVersionParams {
    pub fn new(
        project_id_or_key: impl Into<ProjectIdOrKey>,
        version_id: impl Into<backlog_core::identifier::MilestoneId>,
        name: impl Into<String>,
    ) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
            version_id: version_id.into(),
            name: name.into(),
            description: None,
            start_date: None,
            release_due_date: None,
            archived: None,
        }
    }
}

#[cfg(feature = "writable")]
impl IntoRequest for UpdateVersionParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Patch
    }

    fn path(&self) -> String {
        format!(
            "/api/v2/projects/{}/versions/{}",
            self.project_id_or_key, self.version_id
        )
    }

    fn to_form(&self) -> impl Serialize {
        self
    }
}
