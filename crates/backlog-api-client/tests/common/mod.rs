//! Common test utilities for backlog-api-client tests.

use backlog_api_client::client::BacklogApiClient;
use wiremock::MockServer;

/// Creates a BacklogApiClient configured to use a mock server.
///
/// # Arguments
/// * `mock_server` - The wiremock MockServer to connect to
///
/// # Returns
/// A configured BacklogApiClient with API key authentication
pub async fn setup_api_client(mock_server: &MockServer) -> BacklogApiClient {
    BacklogApiClient::new(&mock_server.uri())
        .expect("Client creation should succeed with valid mock server URI")
        .with_api_key("test-api-key")
}

/// Creates a BacklogApiClient without authentication for testing constructor behavior.
///
/// # Arguments
/// * `mock_server` - The wiremock MockServer to connect to
///
/// # Returns
/// A BacklogApiClient without any authentication configured
pub async fn setup_api_client_no_auth(mock_server: &MockServer) -> BacklogApiClient {
    BacklogApiClient::new(&mock_server.uri())
        .expect("Client creation should succeed with valid mock server URI")
}

/// Common imports for tests
#[allow(unused_imports)]
pub use backlog_api_core::Error as ApiError;
#[allow(unused_imports)]
pub use serde_json::json;
#[allow(unused_imports)]
pub use wiremock::matchers::{method, path};
#[allow(unused_imports)]
pub use wiremock::{Mock, ResponseTemplate};
