use derive_builder::UninitializedFieldError;
use serde::Deserialize;
use thiserror::Error;

/// Error type for Backlog API operations.
///
/// This enum covers all possible errors that can occur when interacting with the Backlog API,
/// including HTTP errors, JSON parsing errors, validation errors, and API-specific errors.
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
}

/// Result type alias using the [`Error`] type.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display_builder_field_missing() {
        let err = Error::BuilderFieldMissing {
            field: "name".to_string(),
        };
        assert_eq!(
            err.to_string(),
            "Builder error: required field 'name' not set",
            "BuilderFieldMissing should format field name correctly"
        );
    }

    #[test]
    fn test_error_display_http_status() {
        let err = Error::HttpStatus {
            status: 404,
            errors: vec![BacklogApiErrorEntry {
                message: "Not found".to_string(),
                code: 3,
                more_info: None,
            }],
            errors_summary: "Not found".to_string(),
        };
        let err_string = err.to_string();
        assert!(
            err_string.contains("404"),
            "HttpStatus error should contain status code: {}",
            err_string
        );
        assert!(
            err_string.contains("Not found"),
            "HttpStatus error should contain message: {}",
            err_string
        );
    }

    #[test]
    fn test_error_display_url_construction() {
        let err = Error::UrlConstruction("invalid path".to_string());
        assert_eq!(
            err.to_string(),
            "Failed to construct URL: invalid path",
            "UrlConstruction should format message correctly"
        );
    }

    #[test]
    fn test_error_display_file_read() {
        let err = Error::FileRead {
            path: "/tmp/test.txt".to_string(),
            message: "permission denied".to_string(),
        };
        let err_string = err.to_string();
        assert!(
            err_string.contains("/tmp/test.txt"),
            "FileRead error should contain path: {}",
            err_string
        );
        assert!(
            err_string.contains("permission denied"),
            "FileRead error should contain message: {}",
            err_string
        );
    }

    #[test]
    fn test_from_uninitialized_field_error() {
        let builder_err = UninitializedFieldError::new("test_field");
        let err: Error = builder_err.into();
        assert!(
            matches!(err, Error::BuilderFieldMissing { field } if field == "test_field"),
            "Should convert UninitializedFieldError to BuilderFieldMissing"
        );
    }

    #[test]
    fn test_deserialize_error_response() {
        let json = r#"{
            "errors": [
                {"message": "Invalid request", "code": 1, "moreInfo": ""}
            ]
        }"#;
        let response: BacklogApiErrorResponse =
            serde_json::from_str(json).expect("should deserialize error response");
        assert_eq!(response.errors.len(), 1, "should have one error entry");
        assert_eq!(response.errors[0].message, "Invalid request");
        assert_eq!(response.errors[0].code, 1);
    }

    #[test]
    fn test_deserialize_error_response_without_more_info() {
        let json = r#"{
            "errors": [
                {"message": "Error", "code": 1}
            ]
        }"#;
        let response: BacklogApiErrorResponse =
            serde_json::from_str(json).expect("should deserialize error response without moreInfo");
        assert_eq!(response.errors[0].more_info, None);
    }

    #[test]
    fn test_deserialize_error_response_with_more_info() {
        let json = r#"{
            "errors": [
                {"message": "Error", "code": 1, "moreInfo": "additional details"}
            ]
        }"#;
        let response: BacklogApiErrorResponse =
            serde_json::from_str(json).expect("should deserialize error response with moreInfo");
        assert_eq!(
            response.errors[0].more_info,
            Some("additional details".to_string())
        );
    }

    #[test]
    fn test_deserialize_multiple_errors() {
        let json = r#"{
            "errors": [
                {"message": "Error 1", "code": 1, "moreInfo": ""},
                {"message": "Error 2", "code": 2, "moreInfo": "details"}
            ]
        }"#;
        let response: BacklogApiErrorResponse =
            serde_json::from_str(json).expect("should deserialize multiple errors");
        assert_eq!(response.errors.len(), 2);
        assert_eq!(response.errors[0].message, "Error 1");
        assert_eq!(response.errors[1].message, "Error 2");
    }
}
