mod common;

#[cfg(test)]
mod get_watching_test {
    use serde_json::json;
    use wiremock::{matchers, Mock, MockServer, ResponseTemplate};

    use crate::common::setup_watching_api;
    use backlog_core::identifier::WatchingId;

    #[tokio::test]
    async fn test_get_watching_success() {
        let mock_server = MockServer::start().await;

        let response_json = json!({
            "id": 123,
            "resourceAlreadyRead": false,
            "note": "Watching this important issue",
            "type": "issue",
            "issue": {
                "id": 456,
                "projectId": 1,
                "issueKey": "PROJ-123",
                "keyId": 123,
                "issueType": {
                    "id": 1,
                    "projectId": 1,
                    "name": "Bug",
                    "color": "#990000",
                    "displayOrder": 0
                },
                "summary": "Critical bug fix",
                "description": "This needs immediate attention",
                "resolution": null,
                "priority": {
                    "id": 3,
                    "name": "High"
                },
                "status": {
                    "id": 1,
                    "projectId": 1,
                    "name": "Open",
                    "color": "#ed8077",
                    "displayOrder": 1000
                },
                "assignee": {
                    "id": 2,
                    "userId": "developer",
                    "name": "Developer",
                    "roleType": 2,
                    "lang": "ja",
                    "mailAddress": "dev@example.com",
                    "nulabAccount": null,
                    "keyword": "dev"
                },
                "category": [],
                "versions": [],
                "milestone": [],
                "startDate": null,
                "dueDate": "2024-02-01",
                "estimatedHours": 8.0,
                "actualHours": null,
                "parentIssueId": null,
                "createdUser": {
                    "id": 1,
                    "userId": "admin",
                    "name": "Administrator",
                    "roleType": 1,
                    "lang": "ja",
                    "mailAddress": "admin@example.com",
                    "nulabAccount": null,
                    "keyword": "admin"
                },
                "created": "2024-01-01T09:00:00Z",
                "updatedUser": {
                    "id": 2,
                    "userId": "developer",
                    "name": "Developer",
                    "roleType": 2,
                    "lang": "ja",
                    "mailAddress": "dev@example.com",
                    "nulabAccount": null,
                    "keyword": "dev"
                },
                "updated": "2024-01-15T14:30:00Z",
                "customFields": [],
                "attachments": [],
                "sharedFiles": [],
                "stars": []
            },
            "lastContentUpdated": "2024-01-15T14:30:00Z",
            "created": "2024-01-10T10:00:00Z",
            "updated": "2024-01-15T14:30:00Z",
            "alreadyRead": true
        });

        Mock::given(matchers::method("GET"))
            .and(matchers::path("/api/v2/watchings/123"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&response_json))
            .mount(&mock_server)
            .await;

        let api = setup_watching_api(&mock_server).await;
        let result = api.get(WatchingId::from(123)).await;

        assert!(result.is_ok());
        let watching = result.unwrap();
        assert_eq!(watching.id, WatchingId::from(123));
        assert!(!watching.resource_already_read);
        assert_eq!(
            watching.note,
            Some("Watching this important issue".to_string())
        );
        assert!(watching.already_read);
        assert!(watching.issue.is_some());

        let issue = watching.issue.unwrap();
        assert_eq!(issue.issue_key.to_string(), "PROJ-123");
        assert_eq!(issue.summary, "Critical bug fix");
    }

    #[tokio::test]
    async fn test_get_watching_minimal() {
        let mock_server = MockServer::start().await;

        let response_json = json!({
            "id": 456,
            "type": "issue",
            "created": "2024-01-01T00:00:00Z",
            "updated": "2024-01-15T00:00:00Z"
        });

        Mock::given(matchers::method("GET"))
            .and(matchers::path("/api/v2/watchings/456"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&response_json))
            .mount(&mock_server)
            .await;

        let api = setup_watching_api(&mock_server).await;
        let result = api.get(WatchingId::from(456)).await;

        assert!(result.is_ok());
        let watching = result.unwrap();
        assert_eq!(watching.id, WatchingId::from(456));
        assert!(!watching.resource_already_read);
        assert_eq!(watching.note, None);
        assert!(!watching.already_read);
        assert!(watching.issue.is_none());
    }

    #[tokio::test]
    async fn test_get_watching_not_found() {
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

        Mock::given(matchers::method("GET"))
            .and(matchers::path("/api/v2/watchings/999"))
            .respond_with(ResponseTemplate::new(404).set_body_json(&error_json))
            .mount(&mock_server)
            .await;

        let api = setup_watching_api(&mock_server).await;
        let result = api.get(WatchingId::from(999)).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_watching_unauthorized() {
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

        Mock::given(matchers::method("GET"))
            .and(matchers::path("/api/v2/watchings/123"))
            .respond_with(ResponseTemplate::new(401).set_body_json(&error_json))
            .mount(&mock_server)
            .await;

        let api = setup_watching_api(&mock_server).await;
        let result = api.get(WatchingId::from(123)).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_watching_forbidden() {
        let mock_server = MockServer::start().await;

        let error_json = json!({
            "errors": [
                {
                    "message": "You do not have permission to view this watching.",
                    "code": 11,
                    "moreInfo": ""
                }
            ]
        });

        Mock::given(matchers::method("GET"))
            .and(matchers::path("/api/v2/watchings/123"))
            .respond_with(ResponseTemplate::new(403).set_body_json(&error_json))
            .mount(&mock_server)
            .await;

        let api = setup_watching_api(&mock_server).await;
        let result = api.get(WatchingId::from(123)).await;

        let err = result.expect_err("should return 403 error");
        assert!(matches!(
            err,
            backlog_api_core::Error::HttpStatus { status: 403, .. }
        ));
    }

    #[tokio::test]
    async fn test_get_watching_server_error() {
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

        Mock::given(matchers::method("GET"))
            .and(matchers::path("/api/v2/watchings/123"))
            .respond_with(ResponseTemplate::new(500).set_body_json(&error_json))
            .mount(&mock_server)
            .await;

        let api = setup_watching_api(&mock_server).await;
        let result = api.get(WatchingId::from(123)).await;

        let err = result.expect_err("should return 500 error");
        assert!(matches!(
            err,
            backlog_api_core::Error::HttpStatus { status: 500, .. }
        ));
    }
}
