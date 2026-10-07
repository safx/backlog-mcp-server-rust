#[cfg(feature = "writable")]
use backlog_api_core::{HttpMethod, IntoRequest};
#[cfg(feature = "writable")]
use backlog_core::ProjectIdOrKey;
#[cfg(feature = "writable")]
use serde::Serialize;

#[cfg(feature = "writable")]
pub type DeleteIssueTypeResponse = backlog_domain_models::IssueType;

#[cfg(feature = "writable")]
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteIssueTypeParams {
    #[serde(skip)]
    pub project_id_or_key: ProjectIdOrKey,
    #[serde(skip)]
    pub issue_type_id: backlog_core::identifier::IssueTypeId,
    pub substitute_issue_type_id: backlog_core::identifier::IssueTypeId,
}

#[cfg(feature = "writable")]
impl DeleteIssueTypeParams {
    pub fn new(
        project_id_or_key: impl Into<ProjectIdOrKey>,
        issue_type_id: impl Into<backlog_core::identifier::IssueTypeId>,
        substitute_issue_type_id: impl Into<backlog_core::identifier::IssueTypeId>,
    ) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
            issue_type_id: issue_type_id.into(),
            substitute_issue_type_id: substitute_issue_type_id.into(),
        }
    }
}

#[cfg(feature = "writable")]
impl IntoRequest for DeleteIssueTypeParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Delete
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
