use crate::models::Attachment;
use backlog_api_core::IntoRequest;
use backlog_core::IssueIdOrKey;

/// Response type for getting a list of attachments
pub type GetAttachmentListResponse = Vec<Attachment>;

/// Parameters for getting attachment list for a specific issue.
/// Corresponds to `GET /api/v2/issues/:issueIdOrKey/attachments`.
#[derive(Debug, Clone, PartialEq)]
pub struct GetAttachmentListParams {
    pub issue_id_or_key: IssueIdOrKey,
}

impl GetAttachmentListParams {
    pub fn new(issue_id_or_key: impl Into<IssueIdOrKey>) -> Self {
        Self {
            issue_id_or_key: issue_id_or_key.into(),
        }
    }
}

impl IntoRequest for GetAttachmentListParams {
    fn path(&self) -> String {
        format!("/api/v2/issues/{}/attachments", self.issue_id_or_key)
    }
}
