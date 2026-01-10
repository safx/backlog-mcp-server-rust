#[cfg(feature = "writable")]
use backlog_api_core::IntoRequest;
#[cfg(feature = "writable")]
use serde::Serialize;

#[cfg(feature = "writable")]
#[derive(Debug, Clone)]
pub struct ResetUnreadNotificationCountParams;

#[cfg(feature = "writable")]
impl ResetUnreadNotificationCountParams {
    pub fn new() -> Self {
        Self
    }
}

#[cfg(feature = "writable")]
impl Default for ResetUnreadNotificationCountParams {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "writable")]
impl IntoRequest for ResetUnreadNotificationCountParams {
    fn method(&self) -> backlog_api_core::HttpMethod {
        backlog_api_core::HttpMethod::Post
    }

    fn path(&self) -> String {
        "/api/v2/notifications/markAsRead".to_string()
    }

    fn to_form(&self) -> impl Serialize {
        // No parameters needed for this endpoint
        let params: Vec<(String, String)> = Vec::new();
        params
    }
}

#[cfg(all(test, feature = "writable"))]
mod tests {
    use super::*;
    use crate::api::UserApi;
    use client::test_utils::setup_client;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    async fn setup_api(mock_server: &MockServer) -> UserApi {
        let client = setup_client(mock_server).await;
        UserApi::new(client)
    }

    #[tokio::test]
    async fn test_reset_unread_notification_count_success_with_notifications() {
        let mock_server = MockServer::start().await;
        let api = setup_api(&mock_server).await;

        let response_body = serde_json::json!({
            "count": 42
        });

        Mock::given(method("POST"))
            .and(path("/api/v2/notifications/markAsRead"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .mount(&mock_server)
            .await;

        let result = api.reset_unread_notification_count().await;
        assert!(result.is_ok());
        let count =
            result.expect("reset_unread_notification_count should succeed with notifications");
        assert_eq!(count.count, 42);
    }

    #[tokio::test]
    async fn test_reset_unread_notification_count_success_no_notifications() {
        let mock_server = MockServer::start().await;
        let api = setup_api(&mock_server).await;

        let response_body = serde_json::json!({
            "count": 0
        });

        Mock::given(method("POST"))
            .and(path("/api/v2/notifications/markAsRead"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .mount(&mock_server)
            .await;

        let result = api.reset_unread_notification_count().await;
        assert!(result.is_ok());
        let count =
            result.expect("reset_unread_notification_count should succeed with no notifications");
        assert_eq!(count.count, 0);
    }

    #[tokio::test]
    async fn test_reset_unread_notification_count_unauthorized() {
        let mock_server = MockServer::start().await;
        let api = setup_api(&mock_server).await;

        Mock::given(method("POST"))
            .and(path("/api/v2/notifications/markAsRead"))
            .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
                "errors": [{
                    "message": "Unauthorized",
                    "code": 11,
                    "moreInfo": ""
                }]
            })))
            .mount(&mock_server)
            .await;

        let result = api.reset_unread_notification_count().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_reset_unread_notification_count_server_error() {
        let mock_server = MockServer::start().await;
        let api = setup_api(&mock_server).await;

        Mock::given(method("POST"))
            .and(path("/api/v2/notifications/markAsRead"))
            .respond_with(ResponseTemplate::new(500).set_body_json(serde_json::json!({
                "errors": [{
                    "message": "Internal Server Error",
                    "code": 1,
                    "moreInfo": ""
                }]
            })))
            .mount(&mock_server)
            .await;

        let result = api.reset_unread_notification_count().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_reset_unread_notification_count_idempotency() {
        let mock_server = MockServer::start().await;
        let api = setup_api(&mock_server).await;

        // First call - some notifications marked as read
        Mock::given(method("POST"))
            .and(path("/api/v2/notifications/markAsRead"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "count": 10
            })))
            .up_to_n_times(1)
            .mount(&mock_server)
            .await;

        // Second call - no notifications to mark
        Mock::given(method("POST"))
            .and(path("/api/v2/notifications/markAsRead"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "count": 0
            })))
            .mount(&mock_server)
            .await;

        let result1 = api.reset_unread_notification_count().await;
        assert!(result1.is_ok());
        assert_eq!(result1.expect("first reset should succeed").count, 10);

        let result2 = api.reset_unread_notification_count().await;
        assert!(result2.is_ok());
        assert_eq!(result2.expect("second reset should succeed").count, 0);
    }

    #[test]
    fn test_into_request_path() {
        let params = ResetUnreadNotificationCountParams::new();
        assert_eq!(params.path(), "/api/v2/notifications/markAsRead");
    }

    #[test]
    fn test_into_request_method() {
        let params = ResetUnreadNotificationCountParams::new();
        assert_eq!(params.method(), backlog_api_core::HttpMethod::Post);
    }
}
