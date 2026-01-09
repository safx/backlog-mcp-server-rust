#[cfg(feature = "writable")]
use crate::models::Issue;
#[cfg(feature = "writable")]
use backlog_api_core::{HttpMethod, IntoRequest};
#[cfg(feature = "writable")]
use backlog_core::IssueIdOrKey;

/// Response type for deleting an issue
#[cfg(feature = "writable")]
pub type DeleteIssueResponse = Issue;

/// Parameters for deleting a specific issue.
/// Corresponds to `DELETE /api/v2/issues/:issueIdOrKey`.
#[cfg(feature = "writable")]
#[derive(Debug, Clone, PartialEq)]
pub struct DeleteIssueParams {
    pub issue_id_or_key: IssueIdOrKey,
}

#[cfg(feature = "writable")]
impl DeleteIssueParams {
    pub fn new(issue_id_or_key: impl Into<IssueIdOrKey>) -> Self {
        Self {
            issue_id_or_key: issue_id_or_key.into(),
        }
    }
}

#[cfg(feature = "writable")]
impl IntoRequest for DeleteIssueParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Delete
    }

    fn path(&self) -> String {
        format!("/api/v2/issues/{}", self.issue_id_or_key)
    }
}

#[cfg(all(test, feature = "writable"))]
mod tests {
    use super::*;
    use backlog_core::IssueKey;
    use std::str::FromStr;

    #[test]
    fn test_delete_issue_params_new() {
        let issue_key = IssueKey::from_str("TEST-123").unwrap();
        let params = DeleteIssueParams::new(issue_key.clone());
        assert_eq!(params.issue_id_or_key, IssueIdOrKey::Key(issue_key));
    }

    #[test]
    fn test_delete_issue_params_into_request() {
        let issue_key = IssueKey::from_str("TEST-123").unwrap();
        let params = DeleteIssueParams::new(issue_key);

        assert_eq!(params.method(), HttpMethod::Delete);
        assert_eq!(params.path(), "/api/v2/issues/TEST-123");
    }
}
