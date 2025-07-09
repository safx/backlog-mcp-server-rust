#[cfg(feature = "writable")]
use backlog_api_core::IntoRequest;
#[cfg(feature = "writable")]
use backlog_core::identifier::{CommentId, IssueId, PullRequestCommentId, PullRequestId, WikiId};
#[cfg(feature = "writable")]
use serde::Serialize;

/// Parameters for adding a star to a resource.
///
/// # Example
/// ```no_run
/// # use backlog_star::api::AddStarParams;
/// // Add star to an issue
/// let params = AddStarParams::issue(123u32);
///
/// // Add star to a comment
/// let params = AddStarParams::comment(123u32, 456u32);
/// ```
#[cfg(feature = "writable")]
#[derive(Debug, Clone)]
pub struct AddStarParams {
    target: StarTarget,
}

#[cfg(feature = "writable")]
#[derive(Debug, Clone)]
pub enum StarTarget {
    /// Star an issue
    Issue { id: IssueId },
    /// Star a comment
    Comment {
        issue_id: IssueId,
        comment_id: CommentId,
    },
    /// Star a wiki page
    Wiki { id: WikiId },
    /// Star a pull request
    PullRequest { id: PullRequestId },
    /// Star a pull request comment
    PullRequestComment { comment_id: PullRequestCommentId },
}

#[cfg(feature = "writable")]
impl AddStarParams {
    /// Creates parameters for adding a star to an issue.
    pub fn issue(id: impl Into<IssueId>) -> Self {
        Self {
            target: StarTarget::Issue { id: id.into() },
        }
    }

    /// Creates parameters for adding a star to a comment.
    pub fn comment(issue_id: impl Into<IssueId>, comment_id: impl Into<CommentId>) -> Self {
        Self {
            target: StarTarget::Comment {
                issue_id: issue_id.into(),
                comment_id: comment_id.into(),
            },
        }
    }

    /// Creates parameters for adding a star to a wiki page.
    pub fn wiki(id: impl Into<WikiId>) -> Self {
        Self {
            target: StarTarget::Wiki { id: id.into() },
        }
    }

    /// Creates parameters for adding a star to a pull request.
    pub fn pull_request(id: impl Into<PullRequestId>) -> Self {
        Self {
            target: StarTarget::PullRequest { id: id.into() },
        }
    }

    /// Creates parameters for adding a star to a pull request comment.
    pub fn pull_request_comment(comment_id: impl Into<PullRequestCommentId>) -> Self {
        Self {
            target: StarTarget::PullRequestComment {
                comment_id: comment_id.into(),
            },
        }
    }
}

#[cfg(feature = "writable")]
impl IntoRequest for AddStarParams {
    fn method(&self) -> backlog_api_core::HttpMethod {
        backlog_api_core::HttpMethod::Post
    }

    fn path(&self) -> String {
        "/api/v2/stars".to_string()
    }

    fn to_form(&self) -> impl Serialize {
        let params: Vec<(String, String)> = self.into();
        params
    }
}

#[cfg(feature = "writable")]
impl From<&AddStarParams> for Vec<(String, String)> {
    fn from(params: &AddStarParams) -> Self {
        match &params.target {
            StarTarget::Issue { id } => vec![("issueId".to_string(), id.to_string())],
            StarTarget::Comment { comment_id, .. } => {
                vec![("commentId".to_string(), comment_id.to_string())]
            }
            StarTarget::Wiki { id } => vec![("wikiId".to_string(), id.to_string())],
            StarTarget::PullRequest { id } => {
                vec![("pullRequestId".to_string(), id.to_string())]
            }
            StarTarget::PullRequestComment { comment_id } => {
                vec![("pullRequestCommentId".to_string(), comment_id.to_string())]
            }
        }
    }
}

#[cfg(all(test, feature = "writable"))]
mod tests {
    use super::*;
    use backlog_core::identifier::Identifier;

    #[test]
    fn test_add_star_params_issue() {
        let params = AddStarParams::issue(123u32);

        match &params.target {
            StarTarget::Issue { id } => assert_eq!(id.value(), 123),
            _ => panic!("Expected Issue target"),
        }

        let form_params: Vec<(String, String)> = (&params).into();
        assert_eq!(form_params.len(), 1);
        assert_eq!(form_params[0], ("issueId".to_string(), "123".to_string()));
    }

    #[test]
    fn test_add_star_params_comment() {
        let params = AddStarParams::comment(123u32, 456u32);

        match &params.target {
            StarTarget::Comment {
                issue_id,
                comment_id,
            } => {
                assert_eq!(issue_id.value(), 123);
                assert_eq!(comment_id.value(), 456);
            }
            _ => panic!("Expected Comment target"),
        }

        let form_params: Vec<(String, String)> = (&params).into();
        assert_eq!(form_params.len(), 1);
        assert_eq!(form_params[0], ("commentId".to_string(), "456".to_string()));
    }

    #[test]
    fn test_add_star_params_wiki() {
        let params = AddStarParams::wiki(789u32);

        match &params.target {
            StarTarget::Wiki { id } => assert_eq!(id.value(), 789),
            _ => panic!("Expected Wiki target"),
        }

        let form_params: Vec<(String, String)> = (&params).into();
        assert_eq!(form_params.len(), 1);
        assert_eq!(form_params[0], ("wikiId".to_string(), "789".to_string()));
    }

    #[test]
    fn test_add_star_params_pull_request() {
        let params = AddStarParams::pull_request(10u32);

        match &params.target {
            StarTarget::PullRequest { id } => {
                assert_eq!(id.value(), 10);
            }
            _ => panic!("Expected PullRequest target"),
        }

        let form_params: Vec<(String, String)> = (&params).into();
        assert_eq!(form_params.len(), 1);
        assert_eq!(
            form_params[0],
            ("pullRequestId".to_string(), "10".to_string())
        );
    }

    #[test]
    fn test_add_star_params_pull_request_comment() {
        let params = AddStarParams::pull_request_comment(11u32);

        match &params.target {
            StarTarget::PullRequestComment { comment_id } => {
                assert_eq!(comment_id.value(), 11);
            }
            _ => panic!("Expected PullRequestComment target"),
        }

        let form_params: Vec<(String, String)> = (&params).into();
        assert_eq!(form_params.len(), 1);
        assert_eq!(
            form_params[0],
            ("pullRequestCommentId".to_string(), "11".to_string())
        );
    }

    #[test]
    fn test_into_request_path() {
        let params = AddStarParams::issue(123u32);
        assert_eq!(params.path(), "/api/v2/stars");
    }
}
