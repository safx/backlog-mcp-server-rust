mod common;

#[cfg(test)]
mod get_watching_count_test {
    use serde_json::json;
    use wiremock::{Mock, MockServer, ResponseTemplate, matchers};

    use crate::common::setup_user_api;
    use backlog_core::identifier::UserId;
    use backlog_user::GetWatchingCountParams;

    #[tokio::test]
    async fn test_get_watching_count_success() {
        let mock_server = MockServer::start().await;

        let response_json = json!({
            "count": 138
        });

        Mock::given(matchers::method("GET"))
            .and(matchers::path("/api/v2/users/123/watchings/count"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&response_json))
            .mount(&mock_server)
            .await;

        let api = setup_user_api(&mock_server).await;
        let params = GetWatchingCountParams::new(UserId::from(123));
        let result = api.get_watching_count(params).await;

        assert!(result.is_ok());
        let response = result.expect("get_watching_count should succeed");
        assert_eq!(response.count, 138);
    }

    #[tokio::test]
    async fn test_get_watching_count_with_filters() {
        let mock_server = MockServer::start().await;

        let response_json = json!({
            "count": 42
        });

        Mock::given(matchers::method("GET"))
            .and(matchers::path("/api/v2/users/456/watchings/count"))
            .and(matchers::query_param("resourceAlreadyRead", "true"))
            .and(matchers::query_param("alreadyRead", "false"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&response_json))
            .mount(&mock_server)
            .await;

        let api = setup_user_api(&mock_server).await;
        let params = GetWatchingCountParams::new(UserId::from(456))
            .with_resource_already_read(true)
            .with_already_read(false);
        let result = api.get_watching_count(params).await;

        assert!(result.is_ok());
        let response = result.expect("get_watching_count should succeed with filters");
        assert_eq!(response.count, 42);
    }

    #[tokio::test]
    async fn test_get_watching_count_zero() {
        let mock_server = MockServer::start().await;

        let response_json = json!({
            "count": 0
        });

        Mock::given(matchers::method("GET"))
            .and(matchers::path("/api/v2/users/789/watchings/count"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&response_json))
            .mount(&mock_server)
            .await;

        let api = setup_user_api(&mock_server).await;
        let params = GetWatchingCountParams::new(UserId::from(789));
        let result = api.get_watching_count(params).await;

        assert!(result.is_ok());
        let response = result.expect("get_watching_count should succeed with zero count");
        assert_eq!(response.count, 0);
    }

    #[tokio::test]
    async fn test_get_watching_count_with_partial_filters() {
        let mock_server = MockServer::start().await;

        let response_json = json!({
            "count": 25
        });

        Mock::given(matchers::method("GET"))
            .and(matchers::path("/api/v2/users/999/watchings/count"))
            .and(matchers::query_param("resourceAlreadyRead", "false"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&response_json))
            .mount(&mock_server)
            .await;

        let api = setup_user_api(&mock_server).await;
        let params =
            GetWatchingCountParams::new(UserId::from(999)).with_resource_already_read(false);
        let result = api.get_watching_count(params).await;

        assert!(result.is_ok());
        let response = result.expect("get_watching_count should succeed with partial filters");
        assert_eq!(response.count, 25);
    }

    #[tokio::test]
    async fn test_get_watching_count_error() {
        let mock_server = MockServer::start().await;

        let error_json = json!({
            "errors": [
                {
                    "message": "No user found.",
                    "code": 5,
                    "moreInfo": ""
                }
            ]
        });

        Mock::given(matchers::method("GET"))
            .and(matchers::path("/api/v2/users/12345/watchings/count"))
            .respond_with(ResponseTemplate::new(404).set_body_json(&error_json))
            .mount(&mock_server)
            .await;

        let api = setup_user_api(&mock_server).await;
        let params = GetWatchingCountParams::new(UserId::from(12345));
        let result = api.get_watching_count(params).await;

        assert!(result.is_err());
    }
}
