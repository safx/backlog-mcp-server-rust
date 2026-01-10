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
