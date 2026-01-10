use backlog_issue::api::IssueApi;
use client::test_utils::setup_client;
use wiremock::MockServer;

/// Common test setup function
#[allow(dead_code)]
pub async fn setup_issue_api(mock_server: &MockServer) -> IssueApi {
    let client = setup_client(mock_server).await;
    IssueApi::new(client)
}

/// Common imports for tests
#[allow(unused_imports)]
pub use backlog_core::identifier::{
    AttachmentId, CommentId, IssueId, ProjectId, SharedFileId, UserId,
};
#[allow(unused_imports)]
pub use backlog_core::{IssueIdOrKey, Language, Role, User};
#[allow(unused_imports)]
pub use backlog_issue::models::{Attachment, Comment, FileContent, Issue, SharedFile};
#[allow(unused_imports)]
pub use chrono::{TimeZone, Utc};
#[allow(unused_imports)]
pub use serde_json::json;
#[allow(unused_imports)]
pub use wiremock::matchers::{method, path, query_param};
#[allow(unused_imports)]
pub use wiremock::{Mock, ResponseTemplate};

/// Creates a mock User for testing
#[allow(dead_code)]
pub fn create_mock_user(id: u32, name: &str) -> User {
    User {
        id: UserId::new(id),
        user_id: Some(name.to_string()),
        name: name.to_string(),
        role_type: Role::User,
        lang: Some(Language::Japanese),
        mail_address: format!("{name}@example.com"),
        last_login_time: Some(
            chrono::DateTime::parse_from_rfc3339("2024-01-01T00:00:00Z")
                .unwrap()
                .with_timezone(&Utc),
        ),
    }
}

/// Creates a mock Comment for testing
#[allow(dead_code)]
pub fn create_mock_comment(id: u32, content: &str, user_id: u32, user_name: &str) -> Comment {
    let created_time = Utc.with_ymd_and_hms(2024, 1, 1, 12, 0, 0).unwrap();
    Comment {
        id: CommentId::new(id),
        content: Some(content.to_string()),
        change_log: vec![],
        created_user: create_mock_user(user_id, user_name),
        created: created_time,
        updated: created_time,
        stars: vec![],
        notifications: vec![],
    }
}

/// Creates a mock Attachment for testing
#[allow(dead_code)]
pub fn create_mock_attachment(
    id: u32,
    name: &str,
    size: u64,
    user_id: u32,
    user_name: &str,
    created_str: &str,
) -> Attachment {
    Attachment {
        id: AttachmentId::new(id),
        name: name.to_string(),
        size,
        created_user: create_mock_user(user_id, user_name),
        created: chrono::DateTime::parse_from_rfc3339(created_str)
            .unwrap()
            .with_timezone(&Utc),
    }
}

/// Creates a mock SharedFile for testing
#[allow(dead_code)]
pub fn create_mock_shared_file(
    id: u32,
    dir: &str,
    name: &str,
    size: Option<u64>,
    user_id: u32,
    user_name: &str,
    created_str: &str,
) -> SharedFile {
    SharedFile {
        id: SharedFileId::new(id),
        dir: dir.to_string(),
        name: name.to_string(),
        created_user: create_mock_user(user_id, user_name),
        created: chrono::DateTime::parse_from_rfc3339(created_str)
            .unwrap()
            .with_timezone(&Utc),
        updated_user: None,
        updated: None,
        content: match size {
            Some(s) => FileContent::File { size: s },
            None => FileContent::Directory,
        },
    }
}
