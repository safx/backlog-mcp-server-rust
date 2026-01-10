use crate::models::NotificationForComment;
use backlog_api_core::IntoRequest;
use backlog_core::IssueIdOrKey;
use backlog_core::identifier::CommentId;
use serde::Serialize;

/// Parameters for retrieving comment notifications
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCommentNotificationsParams {
    #[serde(skip)]
    pub issue_id_or_key: IssueIdOrKey,
    #[serde(skip)]
    pub comment_id: CommentId,
}

/// Response type for comment notifications list
pub type GetCommentNotificationsResponse = Vec<NotificationForComment>;

impl GetCommentNotificationsParams {
    pub fn new(issue_id_or_key: impl Into<IssueIdOrKey>, comment_id: impl Into<CommentId>) -> Self {
        Self {
            issue_id_or_key: issue_id_or_key.into(),
            comment_id: comment_id.into(),
        }
    }
}

impl IntoRequest for GetCommentNotificationsParams {
    fn path(&self) -> String {
        format!(
            "/api/v2/issues/{}/comments/{}/notifications",
            self.issue_id_or_key, self.comment_id
        )
    }

    fn to_query(&self) -> impl Serialize {
        serde_json::json!({})
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::IssueApi;
    use backlog_core::{IssueKey, identifier::Identifier};
    use client::test_utils::setup_client;
    use std::str::FromStr;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    const MOCK_NOTIFICATIONS_RESPONSE: &str = r#"[
        {
            "id": 12345,
            "alreadyRead": false,
            "reason": 2,
            "user": {
                "id": 1,
                "userId": "testuser",
                "name": "Test User",
                "roleType": 1,
                "lang": "ja",
                "mailAddress": "test@example.com"
            },
            "resourceAlreadyRead": false
        },
        {
            "id": 12346,
            "alreadyRead": true,
            "reason": 1,
            "user": {
                "id": 2,
                "userId": "testuser2", 
                "name": "Test User 2",
                "roleType": 2,
                "lang": "en",
                "mailAddress": "test2@example.com"
            },
            "resourceAlreadyRead": true
        }
    ]"#;

    #[tokio::test]
    async fn test_get_comment_notifications_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v2/issues/TEST-1/comments/123/notifications"))
            .respond_with(ResponseTemplate::new(200).set_body_string(MOCK_NOTIFICATIONS_RESPONSE))
            .mount(&mock_server)
            .await;

        let client = setup_client(&mock_server).await;
        let api = IssueApi::new(client);

        let params =
            GetCommentNotificationsParams::new(IssueKey::from_str("TEST-1").unwrap(), 123u32);
        let result = api.get_comment_notifications(params).await;

        assert!(result.is_ok());
        let notifications = result.unwrap();
        assert_eq!(notifications.len(), 2);

        // Verify first notification
        let first_notification = &notifications[0];
        assert_eq!(first_notification.id.value(), 12345);
        assert!(!first_notification.already_read);
        assert_eq!(first_notification.user.id.value(), 1);
        assert_eq!(
            first_notification.user.user_id,
            Some("testuser".to_string())
        );

        // Verify second notification
        let second_notification = &notifications[1];
        assert_eq!(second_notification.id.value(), 12346);
        assert!(second_notification.already_read);
        assert_eq!(second_notification.user.id.value(), 2);
    }

    #[tokio::test]
    async fn test_get_comment_notifications_empty_response() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v2/issues/TEST-2/comments/456/notifications"))
            .respond_with(ResponseTemplate::new(200).set_body_string("[]"))
            .mount(&mock_server)
            .await;

        let client = setup_client(&mock_server).await;
        let api = IssueApi::new(client);

        let params =
            GetCommentNotificationsParams::new(IssueKey::from_str("TEST-2").unwrap(), 456u32);
        let result = api.get_comment_notifications(params).await;

        assert!(result.is_ok());
        let notifications = result.unwrap();
        assert_eq!(notifications.len(), 0);
    }

    #[tokio::test]
    async fn test_get_comment_notifications_issue_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
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

        let params = GetCommentNotificationsParams::new(
            IssueKey::from_str("NONEXISTENT-1").unwrap(),
            123u32,
        );
        let result = api.get_comment_notifications(params).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_comment_notifications_comment_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v2/issues/TEST-1/comments/99999/notifications"))
            .respond_with(ResponseTemplate::new(404).set_body_string(
                r#"{"errors":[{"message":"Comment not found","code":6,"moreInfo":""}]}"#,
            ))
            .mount(&mock_server)
            .await;

        let client = setup_client(&mock_server).await;
        let api = IssueApi::new(client);

        let params =
            GetCommentNotificationsParams::new(IssueKey::from_str("TEST-1").unwrap(), 99999u32);
        let result = api.get_comment_notifications(params).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_comment_notifications_path_construction() {
        let params =
            GetCommentNotificationsParams::new(IssueKey::from_str("PROJECT-123").unwrap(), 456u32);
        assert_eq!(
            params.path(),
            "/api/v2/issues/PROJECT-123/comments/456/notifications"
        );

        let params_with_key =
            GetCommentNotificationsParams::new(IssueKey::from_str("PROJ-1").unwrap(), 789u32);
        assert_eq!(
            params_with_key.path(),
            "/api/v2/issues/PROJ-1/comments/789/notifications"
        );
    }
}
