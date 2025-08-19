use backlog_api_client::{
    ApiError, CoreError, ProjectIdOrKey, PullRequestNumber, RepositoryIdOrName,
};
use rmcp::ErrorData as McpError;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("API error: {0}")]
    Api(ApiError),

    #[error("Parameter error: {0}")]
    Parameter(String),

    #[error("Server error: {0}")]
    Server(String),

    #[error("Milestone named '{original_name}' not found in project '{project_id_or_key}'.")]
    MilestoneNotFoundByName {
        project_id_or_key: ProjectIdOrKey,
        original_name: String,
        suggestions: Option<Vec<String>>,
    },

    #[error("Nothing to update. Please provide a summary and/or a description.")]
    NothingToUpdate,

    #[error(
        "Pull request attachment with ID '{attachment_id}' not found in pull request #{pr_number} of repository '{repo_id_or_name}' in project '{project_id_or_key}'."
    )]
    PullRequestAttachmentNotFound {
        project_id_or_key: ProjectIdOrKey,
        repo_id_or_name: RepositoryIdOrName,
        pr_number: PullRequestNumber,
        attachment_id: u32,
    },

    #[error("Access denied to project '{project}'. Allowed projects: {allowed_projects:?}")]
    ProjectAccessDenied {
        project: String,
        allowed_projects: Vec<String>,
    },

    #[error("{0}")]
    ProjectNotFound(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<ApiError> for Error {
    fn from(err: ApiError) -> Self {
        Error::Api(err)
    }
}

impl From<CoreError> for Error {
    fn from(err: CoreError) -> Self {
        Error::Api(err.into())
    }
}

impl From<Error> for McpError {
    fn from(err: Error) -> Self {
        match err {
            Error::Server(msg) => McpError::internal_error(msg, None),
            Error::Parameter(msg) => McpError::invalid_params(msg, None),
            Error::Api(api_error) => {
                // Further match on the specific ApiError variant
                match api_error {
                    ApiError::HttpStatus {
                        status,
                        errors_summary,
                        ..
                    } => {
                        // errors_summary already contains a good summary from ApiError's Display
                        McpError::invalid_request(
                            format!("Backlog API Error (HTTP {status}): {errors_summary}"),
                            None,
                        )
                    }
                    ApiError::Json(serde_error) => {
                        let detailed_message = format!(
                            "Failed to parse a successful response from Backlog API: {serde_error}. \
                            This might indicate an unexpected API format change or a misconfiguration \
                            (e.g., wrong server URL pointing to a non-Backlog service that returned 200 OK with different JSON). \
                            Please verify settings."
                        );
                        McpError::internal_error(detailed_message, None) // Internal because the server got a 200 OK but couldn't parse it.
                    }
                    _ => McpError::invalid_request(api_error.to_string(), None), // Fallback for other ApiError variants
                }
            }
            Error::MilestoneNotFoundByName {
                project_id_or_key,
                original_name,
                suggestions,
            } => {
                let mut message = format!(
                    "Milestone named '{original_name}' not found in project '{project_id_or_key}'.",
                );
                if let Some(suggs) = suggestions
                    && !suggs.is_empty()
                {
                    message.push_str(&format!(" Did you mean one of: {suggs:?}?"));
                }
                message.push_str(&format!(" You can list all available milestones using the 'get_version_milestone_list' tool for project '{project_id_or_key}'."));
                McpError::invalid_params(message, None)
            }
            Error::NothingToUpdate => McpError::invalid_params(err.to_string(), None),
            Error::PullRequestAttachmentNotFound { .. } => {
                // The Display impl from ThisError will format the message
                McpError::invalid_request(err.to_string(), None) // Or invalid_params
            }
            Error::ProjectAccessDenied { .. } => McpError::invalid_params(err.to_string(), None),
            Error::ProjectNotFound(_) => McpError::invalid_params(err.to_string(), None),
        }
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(err: std::string::FromUtf8Error) -> Self {
        Error::Server(format!("Failed to decode UTF-8 string: {err}"))
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Server(format!("JSON serialization/deserialization error: {err}"))
    }
}
