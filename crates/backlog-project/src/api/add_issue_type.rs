#[cfg(feature = "writable")]
use backlog_api_core::{HttpMethod, IntoRequest};
#[cfg(feature = "writable")]
use backlog_core::ProjectIdOrKey;
#[cfg(feature = "writable")]
use serde::Serialize;

#[cfg(feature = "writable")]
pub type AddIssueTypeResponse = backlog_domain_models::IssueType;

#[cfg(feature = "writable")]
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddIssueTypeParams {
    #[serde(skip)]
    pub project_id_or_key: ProjectIdOrKey,
    pub name: String,
    pub color: backlog_domain_models::IssueTypeColor,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_description: Option<String>,
}

#[cfg(feature = "writable")]
impl AddIssueTypeParams {
    pub fn new(
        project_id_or_key: impl Into<ProjectIdOrKey>,
        name: impl Into<String>,
        color: backlog_domain_models::IssueTypeColor,
    ) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
            name: name.into(),
            color,
            template_summary: None,
            template_description: None,
        }
    }
}

#[cfg(feature = "writable")]
impl IntoRequest for AddIssueTypeParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Post
    }

    fn path(&self) -> String {
        format!("/api/v2/projects/{}/issueTypes", self.project_id_or_key)
    }

    fn to_form(&self) -> impl Serialize {
        self
    }
}
