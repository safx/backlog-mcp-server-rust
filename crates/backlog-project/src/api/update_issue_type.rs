#[cfg(feature = "writable")]
use backlog_api_core::{HttpMethod, IntoRequest};
#[cfg(feature = "writable")]
use backlog_core::ProjectIdOrKey;
#[cfg(feature = "writable")]
use serde::Serialize;

#[cfg(feature = "writable")]
pub type UpdateIssueTypeResponse = backlog_domain_models::IssueType;

#[cfg(feature = "writable")]
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateIssueTypeParams {
    #[serde(skip)]
    pub project_id_or_key: ProjectIdOrKey,
    #[serde(skip)]
    pub issue_type_id: backlog_core::identifier::IssueTypeId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<backlog_domain_models::IssueTypeColor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_description: Option<String>,
}

#[cfg(feature = "writable")]
impl UpdateIssueTypeParams {
    pub fn new(
        project_id_or_key: impl Into<ProjectIdOrKey>,
        issue_type_id: impl Into<backlog_core::identifier::IssueTypeId>,
    ) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
            issue_type_id: issue_type_id.into(),
            name: None,
            color: None,
            template_summary: None,
            template_description: None,
        }
    }
}

#[cfg(feature = "writable")]
impl IntoRequest for UpdateIssueTypeParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Patch
    }

    fn path(&self) -> String {
        format!(
            "/api/v2/projects/{}/issueTypes/{}",
            self.project_id_or_key, self.issue_type_id
        )
    }

    fn to_form(&self) -> impl Serialize {
        self
    }
}
