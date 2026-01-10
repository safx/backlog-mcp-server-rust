mod common;

#[cfg(test)]
mod activity_api_tests {
    use super::common::setup_activity_api;
    use backlog_core::identifier::{ActivityId, Identifier};
    use serde_json::json;
    use wiremock::matchers::{method, path, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_get_activity_success() {
        let mock_server = MockServer::start().await;
        let activity_id = ActivityId::from(12345);

        Mock::given(method("GET"))
            .and(path("/api/v2/activities/12345"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "id": 12345,
                "project": {
                    "id": 101,
                    "projectKey": "TEST",
                    "name": "Test Project",
                    "chartEnabled": false,
                    "subtaskingEnabled": false,
                    "projectLeaderCanEditProjectLeader": false,
                    "useWikiTreeView": false,
                    "textFormattingRule": "backlog",
                    "archived": false,
                    "displayOrder": 0,
                    "useDevAttributes": true,
                    "useWiki": true,
                    "useFileSharing": true,
                    "useOriginalImageSizeAtWiki": false
                },
                "type": 1,
                "content": {
                    "id": 456,
                    "key_id": 789,
                    "summary": "Test issue",
                    "description": "Test description"
                },
                "notifications": [],
                "createdUser": {
                    "id": 12345,
                    "userId": "testuser",
                    "name": "Test User",
                    "roleType": 2,
                    "lang": "ja",
                    "mailAddress": "test@example.com",
                    "nulabAccount": {
                        "nulabId": "nulabtest",
                        "name": "Test Nulab User",
                        "uniqueId": "unique123"
                    },
                    "keyword": "test keyword"
                },
                "created": "2024-01-01T10:00:00Z"
            })))
            .mount(&mock_server)
            .await;

        let api = setup_activity_api(&mock_server).await;
        let result = api.get_activity(activity_id).await;

        assert!(result.is_ok());
        let activity = result.expect("get_activity should return activity for valid ID");
        assert_eq!(activity.id.value(), 12345);
        // Use helper method to access project id
        assert_eq!(activity.project_id(), Some(101));
    }

    #[tokio::test]
    async fn test_get_activity_not_found() {
        let mock_server = MockServer::start().await;
        let activity_id = ActivityId::from(99999);

        Mock::given(method("GET"))
            .and(path("/api/v2/activities/99999"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({
                "errors": [
                    {
                        "message": "No activity found",
                        "code": 5,
                        "moreInfo": ""
                    }
                ]
            })))
            .mount(&mock_server)
            .await;

        let api = setup_activity_api(&mock_server).await;
        let result = api.get_activity(activity_id).await;

        let err = result.expect_err("should return 404 error for non-existent activity");
        assert!(matches!(
            err,
            backlog_api_core::Error::HttpStatus { status: 404, .. }
        ));
    }

    #[tokio::test]
    async fn test_get_activity_unauthorized() {
        let mock_server = MockServer::start().await;
        let activity_id = ActivityId::from(12345);

        Mock::given(method("GET"))
            .and(path_regex(r"/api/v2/activities/\d+"))
            .respond_with(ResponseTemplate::new(401).set_body_json(json!({
                "errors": [
                    {
                        "message": "Unauthorized",
                        "code": 11,
                        "moreInfo": ""
                    }
                ]
            })))
            .mount(&mock_server)
            .await;

        let api = setup_activity_api(&mock_server).await;
        let result = api.get_activity(activity_id).await;

        let err = result.expect_err("should return 401 error");
        assert!(matches!(
            err,
            backlog_api_core::Error::HttpStatus { status: 401, .. }
        ));
    }

    #[tokio::test]
    async fn test_get_activity_forbidden() {
        let mock_server = MockServer::start().await;
        let activity_id = ActivityId::from(12345);

        Mock::given(method("GET"))
            .and(path_regex(r"/api/v2/activities/\d+"))
            .respond_with(ResponseTemplate::new(403).set_body_json(json!({
                "errors": [
                    {
                        "message": "Forbidden",
                        "code": 13,
                        "moreInfo": ""
                    }
                ]
            })))
            .mount(&mock_server)
            .await;

        let api = setup_activity_api(&mock_server).await;
        let result = api.get_activity(activity_id).await;

        let err = result.expect_err("should return 403 error");
        assert!(matches!(
            err,
            backlog_api_core::Error::HttpStatus { status: 403, .. }
        ));
    }

    #[tokio::test]
    async fn test_get_activity_server_error() {
        let mock_server = MockServer::start().await;
        let activity_id = ActivityId::from(12345);

        Mock::given(method("GET"))
            .and(path_regex(r"/api/v2/activities/\d+"))
            .respond_with(ResponseTemplate::new(500).set_body_json(json!({
                "errors": [
                    {
                        "message": "Internal Server Error",
                        "code": 99,
                        "moreInfo": ""
                    }
                ]
            })))
            .mount(&mock_server)
            .await;

        let api = setup_activity_api(&mock_server).await;
        let result = api.get_activity(activity_id).await;

        let err = result.expect_err("should return 500 error");
        assert!(matches!(
            err,
            backlog_api_core::Error::HttpStatus { status: 500, .. }
        ));
    }
}
