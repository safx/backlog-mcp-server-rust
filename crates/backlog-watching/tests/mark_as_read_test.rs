mod common;

#[cfg(test)]
mod mark_as_read_test {
    use serde_json::json;
    use wiremock::{matchers, Mock, MockServer, ResponseTemplate};

    use crate::common::setup_watching_api;
    use backlog_core::identifier::WatchingId;

    #[tokio::test]
    #[cfg(feature = "writable")]
    async fn test_mark_as_read_success() {
        let mock_server = MockServer::start().await;

        Mock::given(matchers::method("POST"))
            .and(matchers::path("/api/v2/watchings/123/markAsRead"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let api = setup_watching_api(&mock_server).await;
        let result = api.mark_as_read(WatchingId::from(123)).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    #[cfg(feature = "writable")]
    async fn test_mark_as_read_not_found() {
        let mock_server = MockServer::start().await;

        let error_json = json!({
            "errors": [
                {
                    "message": "No watching found.",
                    "code": 13,
                    "moreInfo": ""
                }
            ]
        });

        Mock::given(matchers::method("POST"))
            .and(matchers::path("/api/v2/watchings/999/markAsRead"))
            .respond_with(ResponseTemplate::new(404).set_body_json(&error_json))
            .mount(&mock_server)
            .await;

        let api = setup_watching_api(&mock_server).await;
        let result = api.mark_as_read(WatchingId::from(999)).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    #[cfg(feature = "writable")]
    async fn test_mark_as_read_unauthorized() {
        let mock_server = MockServer::start().await;

        let error_json = json!({
            "errors": [
                {
                    "message": "Authentication failure.",
                    "code": 11,
                    "moreInfo": ""
                }
            ]
        });

        Mock::given(matchers::method("POST"))
            .and(matchers::path("/api/v2/watchings/456/markAsRead"))
            .respond_with(ResponseTemplate::new(401).set_body_json(&error_json))
            .mount(&mock_server)
            .await;

        let api = setup_watching_api(&mock_server).await;
        let result = api.mark_as_read(WatchingId::from(456)).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    #[cfg(feature = "writable")]
    async fn test_mark_as_read_already_read() {
        let mock_server = MockServer::start().await;

        Mock::given(matchers::method("POST"))
            .and(matchers::path("/api/v2/watchings/789/markAsRead"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let api = setup_watching_api(&mock_server).await;
        let result = api.mark_as_read(WatchingId::from(789)).await;

        // Even if already read, API returns 204
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[cfg(feature = "writable")]
    async fn test_mark_watching_as_read_forbidden() {
        let mock_server = MockServer::start().await;

        let error_json = json!({
            "errors": [
                {
                    "message": "You do not have permission to mark this watching as read.",
                    "code": 11,
                    "moreInfo": ""
                }
            ]
        });

        Mock::given(matchers::method("POST"))
            .and(matchers::path("/api/v2/watchings/123/markAsRead"))
            .respond_with(ResponseTemplate::new(403).set_body_json(&error_json))
            .mount(&mock_server)
            .await;

        let api = setup_watching_api(&mock_server).await;
        let result = api.mark_as_read(WatchingId::from(123)).await;

        let err = result.expect_err("should return 403 error");
        assert!(matches!(
            err,
            backlog_api_core::Error::HttpStatus { status: 403, .. }
        ));
    }

    #[tokio::test]
    #[cfg(feature = "writable")]
    async fn test_mark_watching_as_read_server_error() {
        let mock_server = MockServer::start().await;

        let error_json = json!({
            "errors": [
                {
                    "message": "Internal server error",
                    "code": 1,
                    "moreInfo": ""
                }
            ]
        });

        Mock::given(matchers::method("POST"))
            .and(matchers::path("/api/v2/watchings/123/markAsRead"))
            .respond_with(ResponseTemplate::new(500).set_body_json(&error_json))
            .mount(&mock_server)
            .await;

        let api = setup_watching_api(&mock_server).await;
        let result = api.mark_as_read(WatchingId::from(123)).await;

        let err = result.expect_err("should return 500 error");
        assert!(matches!(
            err,
            backlog_api_core::Error::HttpStatus { status: 500, .. }
        ));
    }
}
