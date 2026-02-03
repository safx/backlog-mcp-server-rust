use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum Error {
    #[error("Invalid space key: {0}")]
    InvalidSpaceKey(String),

    #[error("Invalid project key: {0}")]
    InvalidProjectKey(String),

    #[error("Invalid project id or key: {0}")]
    InvalidProjectIdOrKey(String),

    #[error("Invalid issue id or key: {0}")]
    InvalidIssueIdOrKey(String),

    #[error("Invalid issue key: {0}")]
    InvalidIssueKey(String),

    #[error("Invalid issue key id: {0} (must be non-zero)")]
    InvalidIssueKeyId(u32),

    #[error("Invalid role type: {0}")]
    InvalidRole(String),

    #[error("Invalid role id: {0}")]
    InvalidRoleId(i32),

    #[error("Invalid document id: {0}")]
    InvalidDocumentId(String),

    #[error("Invalid repository name: {0}")]
    InvalidRepositoryName(String),

    #[error("Invalid repository id or name: {0}")]
    InvalidRepositoryIdOrName(String),

    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),
}

pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display_invalid_space_key() {
        let err = Error::InvalidSpaceKey("abc".to_string());
        assert_eq!(err.to_string(), "Invalid space key: abc");
    }

    #[test]
    fn test_error_display_invalid_project_key() {
        let err = Error::InvalidProjectKey("test-project".to_string());
        assert_eq!(err.to_string(), "Invalid project key: test-project");
    }

    #[test]
    fn test_error_display_invalid_project_id_or_key() {
        let err = Error::InvalidProjectIdOrKey("invalid".to_string());
        assert_eq!(err.to_string(), "Invalid project id or key: invalid");
    }

    #[test]
    fn test_error_display_invalid_issue_id_or_key() {
        let err = Error::InvalidIssueIdOrKey("bad-issue".to_string());
        assert_eq!(err.to_string(), "Invalid issue id or key: bad-issue");
    }

    #[test]
    fn test_error_display_invalid_issue_key() {
        let err = Error::InvalidIssueKey("PROJ-abc".to_string());
        assert_eq!(err.to_string(), "Invalid issue key: PROJ-abc");
    }

    #[test]
    fn test_error_display_invalid_issue_key_id() {
        let err = Error::InvalidIssueKeyId(0);
        assert_eq!(
            err.to_string(),
            "Invalid issue key id: 0 (must be non-zero)"
        );
    }

    #[test]
    fn test_error_display_invalid_role() {
        let err = Error::InvalidRole("superadmin".to_string());
        assert_eq!(err.to_string(), "Invalid role type: superadmin");
    }

    #[test]
    fn test_error_display_invalid_parameter() {
        let err = Error::InvalidParameter("count must be positive".to_string());
        assert_eq!(err.to_string(), "Invalid parameter: count must be positive");
    }
}
