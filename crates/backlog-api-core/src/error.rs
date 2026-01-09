use derive_builder::UninitializedFieldError;
use serde::Deserialize;
use thiserror::Error;

// Add use statement for backlog_core so its Error type can be referenced.
// use backlog_core; // This line is redundant as backlog_core::Error is used with its full path.

#[derive(Debug, Error)]
pub enum Error {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("URL error: {0}")]
    Url(#[from] url::ParseError),

    #[error("Validation error: {0}")]
    Validation(#[from] backlog_core::Error),

    /// Builder required field missing (from derive_builder)
    #[error("Builder error: required field '{field}' not set")]
    BuilderFieldMissing { field: String },

    /// URL path construction failed
    #[error("Failed to construct URL: {0}")]
    UrlConstruction(String),

    /// File read operation failed
    #[error("Failed to read file '{path}': {message}")]
    FileRead { path: String, message: String },

    /// HTTP request building failed
    #[error("Failed to build HTTP request: {0}")]
    RequestBuild(String),

    /// Authentication token contains invalid characters
    #[error("Invalid authentication token: {0}")]
    InvalidAuthToken(String),

    /// Received unexpected HTTP status code
    #[error("Unexpected HTTP status {status}: {body}")]
    UnexpectedStatus { status: u16, body: String },

    /// Error response body could not be parsed
    #[error("HTTP error {status} with unparseable body: {body}")]
    UnparseableErrorResponse { status: u16, body: String },

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Backlog API Error (HTTP {status}): {errors_summary}")]
    HttpStatus {
        status: u16,
        errors: Vec<BacklogApiErrorEntry>,
        errors_summary: String, // Pre-formatted summary of errors
    },
    // Consider HttpErrorWithUnparsedBody { status: u16, body: String } later if needed
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<UninitializedFieldError> for Error {
    fn from(err: UninitializedFieldError) -> Self {
        Self::BuilderFieldMissing {
            field: err.field_name().to_string(),
        }
    }
}

/// Represents a single error entry from the Backlog API.
#[derive(Debug, Deserialize)]
pub struct BacklogApiErrorEntry {
    pub message: String,
    pub code: i64,
    #[serde(rename = "moreInfo")]
    pub more_info: Option<String>, // API can return empty string, map to Option
}

/// Represents the error response structure from the Backlog API.
#[derive(Debug, Deserialize)]
pub struct BacklogApiErrorResponse {
    pub errors: Vec<BacklogApiErrorEntry>,
}
