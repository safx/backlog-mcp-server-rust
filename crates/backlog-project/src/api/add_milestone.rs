#[cfg(feature = "writable")]
use backlog_api_core::{HttpMethod, IntoRequest};
#[cfg(feature = "writable")]
use backlog_core::{ApiDate, ProjectIdOrKey};
#[cfg(feature = "writable")]
use serde::Serialize;

#[cfg(feature = "writable")]
pub type AddMilestoneResponse = backlog_domain_models::Milestone;

#[cfg(feature = "writable")]
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddMilestoneParams {
    #[serde(skip)]
    pub project_id_or_key: ProjectIdOrKey,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<ApiDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_due_date: Option<ApiDate>,
}

#[cfg(feature = "writable")]
impl AddMilestoneParams {
    pub fn new(project_id_or_key: impl Into<ProjectIdOrKey>, name: impl Into<String>) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
            name: name.into(),
            description: None,
            start_date: None,
            release_due_date: None,
        }
    }
}

#[cfg(feature = "writable")]
impl IntoRequest for AddMilestoneParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Post
    }

    fn path(&self) -> String {
        format!("/api/v2/projects/{}/versions", self.project_id_or_key)
    }

    fn to_form(&self) -> impl Serialize {
        self
    }
}
