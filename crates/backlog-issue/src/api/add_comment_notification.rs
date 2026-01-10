#[cfg(feature = "writable")]
use crate::models::Comment;
#[cfg(feature = "writable")]
use backlog_api_core::{HttpMethod, IntoRequest};
#[cfg(feature = "writable")]
use backlog_api_macros::ToFormParams;
#[cfg(feature = "writable")]
use backlog_core::{
    IssueIdOrKey,
    identifier::{CommentId, UserId},
};
#[cfg(feature = "writable")]
use serde::Serialize;

/// Response type for adding comment notification
#[cfg(feature = "writable")]
pub type AddCommentNotificationResponse = Comment;

/// Parameters for adding notifications to a comment
#[cfg(feature = "writable")]
#[derive(Debug, Clone, ToFormParams)]
pub struct AddCommentNotificationParams {
    #[form(skip)]
    pub issue_id_or_key: IssueIdOrKey,
    #[form(skip)]
    pub comment_id: CommentId,
    #[form(array, name = "notifiedUserId")]
    pub notified_user_ids: Vec<UserId>,
}

#[cfg(feature = "writable")]
impl AddCommentNotificationParams {
    pub fn new(
        issue_id_or_key: impl Into<IssueIdOrKey>,
        comment_id: impl Into<CommentId>,
        notified_user_ids: Vec<UserId>,
    ) -> Self {
        Self {
            issue_id_or_key: issue_id_or_key.into(),
            comment_id: comment_id.into(),
            notified_user_ids,
        }
    }
}

#[cfg(feature = "writable")]
impl IntoRequest for AddCommentNotificationParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Post
    }

    fn path(&self) -> String {
        format!(
            "/api/v2/issues/{}/comments/{}/notifications",
            self.issue_id_or_key, self.comment_id
        )
    }

    fn to_form(&self) -> impl Serialize {
        let params: Vec<(String, String)> = self.into();
        params
    }
}

#[cfg(all(test, feature = "writable"))]
mod tests {
    use super::*;
    use crate::api::IssueApi;
    use backlog_core::{IssueKey, identifier::Identifier};
    use client::test_utils::setup_client;
    use std::str::FromStr;
    use wiremock::matchers::{body_string_contains, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    const MOCK_COMMENT_RESPONSE: &str = r#"{
        "id": 12345,
        "content": "Test comment content",
        "changeLog": [],
        "createdUser": {
            "id": 1,
            "userId": "testuser",
            "name": "Test User",
            "roleType": 1,
            "lang": "ja",
            "mailAddress": "test@example.com"
        },
        "created": "2013-08-05T06:15:06Z",
        "updated": "2013-08-05T06:15:06Z",
        "stars": [],
        "notifications": [
            {
                "id": 98765,
                "alreadyRead": false,
                "reason": 2,
                "user": {
                    "id": 2,
                    "userId": "notifieduser",
                    "name": "Notified User",
                    "roleType": 1,
                    "lang": "ja",
                    "mailAddress": "notified@example.com"
                },
                "resourceAlreadyRead": false
            }
        ]
    }"#;

    #[tokio::test]
    async fn test_add_comment_notification_success_single_user() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v2/issues/TEST-1/comments/123/notifications"))
            .and(body_string_contains("notifiedUserId%5B%5D=456")) // URL encoded notifiedUserId[]=456
            .respond_with(ResponseTemplate::new(200).set_body_string(MOCK_COMMENT_RESPONSE))
            .mount(&mock_server)
            .await;

        let client = setup_client(&mock_server).await;
        let api = IssueApi::new(client);

        let params = AddCommentNotificationParams::new(
            IssueKey::from_str("TEST-1").unwrap(),
            123u32,
            vec![UserId::new(456)],
        );
        let result = api.add_comment_notification(params).await;

        assert!(result.is_ok());
        let comment = result.unwrap();
        assert_eq!(comment.id.value(), 12345);
        assert_eq!(comment.notifications.len(), 1);
        assert_eq!(comment.notifications[0].user.id.value(), 2);
    }

    #[tokio::test]
    async fn test_add_comment_notification_success_multiple_users() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v2/issues/TEST-2/comments/789/notifications"))
            .and(body_string_contains("notifiedUserId%5B%5D=111"))
            .and(body_string_contains("notifiedUserId%5B%5D=222"))
            .respond_with(ResponseTemplate::new(200).set_body_string(MOCK_COMMENT_RESPONSE))
            .mount(&mock_server)
            .await;

        let client = setup_client(&mock_server).await;
        let api = IssueApi::new(client);

        let params = AddCommentNotificationParams::new(
            IssueKey::from_str("TEST-2").unwrap(),
            789u32,
            vec![UserId::new(111), UserId::new(222)],
        );
        let result = api.add_comment_notification(params).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_add_comment_notification_issue_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path(
                "/api/v2/issues/NONEXISTENT-1/comments/123/notifications",
            ))
            .respond_with(ResponseTemplate::new(404).set_body_string(
                r#"{"errors":[{"message":"Issue not found","code":6,"moreInfo":""}]}"#,
            ))
            .mount(&mock_server)
            .await;

        let client = setup_client(&mock_server).await;
        let api = IssueApi::new(client);

        let params = AddCommentNotificationParams::new(
            IssueKey::from_str("NONEXISTENT-1").unwrap(),
            123u32,
            vec![UserId::new(456)],
        );
        let result = api.add_comment_notification(params).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_add_comment_notification_comment_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v2/issues/TEST-1/comments/99999/notifications"))
            .respond_with(ResponseTemplate::new(404).set_body_string(
                r#"{"errors":[{"message":"Comment not found","code":6,"moreInfo":""}]}"#,
            ))
            .mount(&mock_server)
            .await;

        let client = setup_client(&mock_server).await;
        let api = IssueApi::new(client);

        let params = AddCommentNotificationParams::new(
            IssueKey::from_str("TEST-1").unwrap(),
            99999u32,
            vec![UserId::new(456)],
        );
        let result = api.add_comment_notification(params).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_add_comment_notification_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v2/issues/TEST-1/comments/123/notifications"))
            .respond_with(ResponseTemplate::new(403).set_body_string(
                r#"{"errors":[{"message":"Only the comment creator can add notifications","code":11,"moreInfo":""}]}"#,
            ))
            .mount(&mock_server)
            .await;

        let client = setup_client(&mock_server).await;
        let api = IssueApi::new(client);

        let params = AddCommentNotificationParams::new(
            IssueKey::from_str("TEST-1").unwrap(),
            123u32,
            vec![UserId::new(456)],
        );
        let result = api.add_comment_notification(params).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_add_comment_notification_invalid_user() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v2/issues/TEST-1/comments/123/notifications"))
            .respond_with(ResponseTemplate::new(400).set_body_string(
                r#"{"errors":[{"message":"Invalid user ID","code":2,"moreInfo":""}]}"#,
            ))
            .mount(&mock_server)
            .await;

        let client = setup_client(&mock_server).await;
        let api = IssueApi::new(client);

        let params = AddCommentNotificationParams::new(
            IssueKey::from_str("TEST-1").unwrap(),
            123u32,
            vec![UserId::new(99999)],
        );
        let result = api.add_comment_notification(params).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_add_comment_notification_path_construction() {
        let params = AddCommentNotificationParams::new(
            IssueKey::from_str("PROJECT-123").unwrap(),
            456u32,
            vec![UserId::new(789)],
        );
        assert_eq!(
            params.path(),
            "/api/v2/issues/PROJECT-123/comments/456/notifications"
        );
    }

    #[tokio::test]
    async fn test_add_comment_notification_empty_users_list() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v2/issues/TEST-1/comments/123/notifications"))
            .respond_with(ResponseTemplate::new(400).set_body_string(
                r#"{"errors":[{"message":"notifiedUserId is required","code":2,"moreInfo":""}]}"#,
            ))
            .mount(&mock_server)
            .await;

        let client = setup_client(&mock_server).await;
        let api = IssueApi::new(client);

        let params = AddCommentNotificationParams::new(
            IssueKey::from_str("TEST-1").unwrap(),
            123u32,
            vec![],
        );
        let result = api.add_comment_notification(params).await;

        assert!(result.is_err());
    }
}
