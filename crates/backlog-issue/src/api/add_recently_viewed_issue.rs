use crate::models::Issue;
use backlog_api_core::{HttpMethod, IntoRequest};
use backlog_core::IssueIdOrKey;
use serde::Serialize;

pub type AddRecentlyViewedIssueResponse = Issue;

/// Parameters for adding a recently viewed issue
///
/// Corresponds to `POST /api/v2/users/myself/recentlyViewedIssues`.
#[cfg(feature = "writable")]
#[derive(Debug, Clone)]
pub struct AddRecentlyViewedIssueParams {
    /// Issue ID or issue key
    pub issue_id_or_key: IssueIdOrKey,
}

#[cfg(feature = "writable")]
impl From<&AddRecentlyViewedIssueParams> for Vec<(String, String)> {
    fn from(params: &AddRecentlyViewedIssueParams) -> Self {
        vec![(
            "issueIdOrKey".to_string(),
            params.issue_id_or_key.to_string(),
        )]
    }
}

#[cfg(feature = "writable")]
impl IntoRequest for AddRecentlyViewedIssueParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Post
    }

    fn path(&self) -> String {
        "/api/v2/users/myself/recentlyViewedIssues".to_string()
    }

    fn to_form(&self) -> impl Serialize {
        let params: Vec<(String, String)> = self.into();
        params
    }
}

#[cfg(all(test, feature = "writable"))]
mod tests {
    use super::*;
    use backlog_core::{IssueKey, identifier::IssueId};
    use std::str::FromStr;

    #[test]
    fn test_params_with_issue_id() {
        let params = AddRecentlyViewedIssueParams {
            issue_id_or_key: IssueIdOrKey::Id(IssueId::new(12345)),
        };

        let form_params: Vec<(String, String)> = (&params).into();
        assert_eq!(form_params.len(), 1);
        assert_eq!(form_params[0].0, "issueIdOrKey");
        assert_eq!(form_params[0].1, "12345");
    }

    #[test]
    fn test_params_with_issue_key() {
        let params = AddRecentlyViewedIssueParams {
            issue_id_or_key: IssueIdOrKey::Key(IssueKey::from_str("TEST-123").unwrap()),
        };

        let form_params: Vec<(String, String)> = (&params).into();
        assert_eq!(form_params.len(), 1);
        assert_eq!(form_params[0].0, "issueIdOrKey");
        assert_eq!(form_params[0].1, "TEST-123");
    }

    #[test]
    fn test_path() {
        let params = AddRecentlyViewedIssueParams {
            issue_id_or_key: IssueIdOrKey::Id(IssueId::new(1)),
        };
        assert_eq!(params.path(), "/api/v2/users/myself/recentlyViewedIssues");
    }

    #[test]
    fn test_method() {
        let params = AddRecentlyViewedIssueParams {
            issue_id_or_key: IssueIdOrKey::Id(IssueId::new(1)),
        };
        assert_eq!(params.method(), HttpMethod::Post);
    }
}
