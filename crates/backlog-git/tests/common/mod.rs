use backlog_git::api::GitApi;
use client::test_utils::setup_client;

/// Common test setup function
pub async fn setup_git_api(mock_server: &MockServer) -> GitApi {
    let client = setup_client(mock_server).await;
    GitApi::new(client)
}

/// Common imports for tests
pub use wiremock::matchers::{method, path};
pub use wiremock::{Mock, MockServer, ResponseTemplate};
