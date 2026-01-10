mod common;

#[cfg(test)]
mod delete_watching_test {
    use serde_json::json;
    use wiremock::{matchers, Mock, MockServer, ResponseTemplate};

    use crate::common::setup_watching_api;
    use backlog_core::identifier::WatchingId;

    #[tokio::test]
    #[cfg(feature = "writable")]
    async fn test_delete_watching_success() {
        let mock_server = MockServer::start().await;

        let response_json = json!({
            "id": 123,
            "resourceAlreadyRead": true,
            "note": "This was being watched",
            "type": "issue",
            "issue": {
                "id": 456,
                "projectId": 1,
                "issueKey": "PROJ-456",
                "keyId": 456,
                "issueType": {
                    "id": 1,
                    "projectId": 1,
                    "name": "Task",
                    "color": "#7ea800",
                    "displayOrder": 0
                },
                "summary": "Task that was watched",
                "description": "Task description",
                "resolution": {
                    "id": 4,
                    "name": "Fixed"
                },
                "priority": {
                    "id": 2,
                    "name": "Normal"
                },
                "status": {
                    "id": 4,
                    "projectId": 1,
                    "name": "Closed",
                    "color": "#393939",
                    "displayOrder": 4000
                },
                "assignee": null,
                "category": [],
                "versions": [],
                "milestone": [],
                "startDate": null,
                "dueDate": null,
                "estimatedHours": null,
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
                "created": "2024-01-01T10:00:00Z",
                "updatedUser": {
                    "id": 2,
                    "userId": "dev",
                    "name": "Developer",
                    "roleType": 2,
                    "lang": "ja",
                    "mailAddress": "dev@example.com",
                    "nulabAccount": null,
                    "keyword": "dev"
                },
                "updated": "2024-01-20T16:00:00Z",
                "customFields": [],
                "attachments": [],
                "sharedFiles": [],
                "stars": []
            },
            "lastContentUpdated": "2024-01-20T16:00:00Z",
            "created": "2024-01-05T10:00:00Z",
            "updated": "2024-01-20T16:00:00Z"
        });

        Mock::given(matchers::method("DELETE"))
            .and(matchers::path("/api/v2/watchings/123"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&response_json))
            .mount(&mock_server)
            .await;

        let api = setup_watching_api(&mock_server).await;
        let watching_id = WatchingId::from(123);
        let result = api.delete(watching_id).await;

        assert!(result.is_ok());
        let deleted_watching = result.unwrap();
        assert_eq!(deleted_watching.id, WatchingId::from(123));
        assert_eq!(
            deleted_watching.note,
            Some("This was being watched".to_string())
        );
        assert!(deleted_watching.issue.is_some());
    }

    #[tokio::test]
    #[cfg(feature = "writable")]
    async fn test_delete_watching_not_found() {
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

        Mock::given(matchers::method("DELETE"))
            .and(matchers::path("/api/v2/watchings/999"))
            .respond_with(ResponseTemplate::new(404).set_body_json(&error_json))
            .mount(&mock_server)
            .await;

        let api = setup_watching_api(&mock_server).await;
        let result = api.delete(WatchingId::from(999)).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    #[cfg(feature = "writable")]
    async fn test_delete_watching_unauthorized() {
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

        Mock::given(matchers::method("DELETE"))
            .and(matchers::path("/api/v2/watchings/456"))
            .respond_with(ResponseTemplate::new(401).set_body_json(&error_json))
            .mount(&mock_server)
            .await;

        let api = setup_watching_api(&mock_server).await;
        let result = api.delete(WatchingId::from(456)).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    #[cfg(feature = "writable")]
    async fn test_delete_watching_forbidden() {
        let mock_server = MockServer::start().await;

        let error_json = json!({
            "errors": [
                {
                    "message": "You do not have permission to delete this watching.",
                    "code": 11,
                    "moreInfo": ""
                }
            ]
        });

        Mock::given(matchers::method("DELETE"))
            .and(matchers::path("/api/v2/watchings/123"))
            .respond_with(ResponseTemplate::new(403).set_body_json(&error_json))
            .mount(&mock_server)
            .await;

        let api = setup_watching_api(&mock_server).await;
        let result = api.delete(WatchingId::from(123)).await;

        let err = result.expect_err("should return 403 error");
        assert!(matches!(
            err,
            backlog_api_core::Error::HttpStatus { status: 403, .. }
        ));
    }

    #[tokio::test]
    #[cfg(feature = "writable")]
    async fn test_delete_watching_server_error() {
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

        Mock::given(matchers::method("DELETE"))
            .and(matchers::path("/api/v2/watchings/123"))
            .respond_with(ResponseTemplate::new(500).set_body_json(&error_json))
            .mount(&mock_server)
            .await;

        let api = setup_watching_api(&mock_server).await;
        let result = api.delete(WatchingId::from(123)).await;

        let err = result.expect_err("should return 500 error");
        assert!(matches!(
            err,
            backlog_api_core::Error::HttpStatus { status: 500, .. }
        ));
    }
}
