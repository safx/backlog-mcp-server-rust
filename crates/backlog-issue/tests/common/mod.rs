use backlog_issue::api::IssueApi;
use client::test_utils::setup_client;
use wiremock::MockServer;

/// Common test setup function
pub async fn setup_issue_api(mock_server: &MockServer) -> IssueApi {
    let client = setup_client(mock_server).await;
    IssueApi::new(client)
}

/// Common imports for tests
pub use backlog_core::identifier::{
    AttachmentId, CommentId, IssueId, ProjectId, SharedFileId, UserId,
};
pub use backlog_core::{IssueIdOrKey, Language, Role, User};
pub use backlog_issue::models::{Attachment, Comment, FileContent, Issue, SharedFile};
pub use chrono::{TimeZone, Utc};
pub use serde_json::json;
pub use wiremock::matchers::{method, path, query_param};
pub use wiremock::{Mock, ResponseTemplate};
