mod common;

#[cfg(test)]
mod update_watching_test {
    use serde_json::json;
    use wiremock::{matchers, Mock, MockServer, ResponseTemplate};

    use crate::common::setup_watching_api;
    use backlog_core::identifier::WatchingId;
    use backlog_watching::UpdateWatchingParams;

    #[tokio::test]
    #[cfg(feature = "writable")]
    async fn test_update_watching_with_note() {
        let mock_server = MockServer::start().await;

        let response_json = json!({
            "id": 123,
            "resourceAlreadyRead": false,
            "note": "Updated watching note",
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
                "summary": "Task to watch",
                "description": "Task description",
                "resolution": null,
                "priority": {
                    "id": 2,
                    "name": "Normal"
                },
                "status": {
                    "id": 2,
                    "projectId": 1,
                    "name": "Processing",
                    "color": "#4488c5",
                    "displayOrder": 2000
                },
                "assignee": {
                    "id": 3,
                    "userId": "dev",
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
                "created": "2024-01-10T10:00:00Z",
                "updatedUser": {
                    "id": 3,
                    "userId": "dev",
                    "name": "Developer",
                    "roleType": 2,
                    "lang": "ja",
                    "mailAddress": "dev@example.com",
                    "nulabAccount": null,
                    "keyword": "dev"
                },
                "updated": "2024-01-22T15:00:00Z",
                "customFields": [],
                "attachments": [],
                "sharedFiles": [],
                "stars": []
            },
            "lastContentUpdated": "2024-01-22T15:00:00Z",
            "created": "2024-01-15T10:00:00Z",
            "updated": "2024-01-22T16:00:00Z"
        });

        Mock::given(matchers::method("PATCH"))
            .and(matchers::path("/api/v2/watchings/123"))
            .and(matchers::header(
                "Content-Type",
                "application/x-www-form-urlencoded",
            ))
            .and(matchers::body_string_contains("note=Updated+watching+note"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&response_json))
            .mount(&mock_server)
            .await;

        let api = setup_watching_api(&mock_server).await;
        let params =
            UpdateWatchingParams::new(WatchingId::from(123)).with_note("Updated watching note");
        let result = api.update(params).await;

        assert!(result.is_ok());
        let watching = result.unwrap();
        assert_eq!(watching.id, WatchingId::from(123));
        assert_eq!(watching.note, Some("Updated watching note".to_string()));
    }

    #[tokio::test]
    #[cfg(feature = "writable")]
    async fn test_update_watching_remove_note() {
        let mock_server = MockServer::start().await;

        let response_json = json!({
            "id": 456,
            "resourceAlreadyRead": true,
            "note": "",
            "type": "issue",
            "issue": {
                "id": 789,
                "projectId": 1,
                "issueKey": "TEST-789",
                "keyId": 789,
                "issueType": {
                    "id": 2,
                    "projectId": 1,
                    "name": "Bug",
                    "color": "#990000",
                    "displayOrder": 1
                },
                "summary": "Bug to fix",
                "description": "Bug description",
                "resolution": null,
                "priority": {
                    "id": 3,
                    "name": "High"
                },
                "status": {
                    "id": 3,
                    "projectId": 1,
                    "name": "Resolved",
                    "color": "#b0be3c",
                    "displayOrder": 3000
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
                    "id": 2,
                    "userId": "user",
                    "name": "User",
                    "roleType": 2,
                    "lang": "ja",
                    "mailAddress": "user@example.com",
                    "nulabAccount": null,
                    "keyword": "user"
                },
                "created": "2024-01-05T09:00:00Z",
                "updatedUser": {
                    "id": 2,
                    "userId": "user",
                    "name": "User",
                    "roleType": 2,
                    "lang": "ja",
                    "mailAddress": "user@example.com",
                    "nulabAccount": null,
                    "keyword": "user"
                },
                "updated": "2024-01-23T10:00:00Z",
                "customFields": [],
                "attachments": [],
                "sharedFiles": [],
                "stars": []
            },
            "lastContentUpdated": "2024-01-23T10:00:00Z",
            "created": "2024-01-10T10:00:00Z",
            "updated": "2024-01-23T11:00:00Z"
        });

        Mock::given(matchers::method("PATCH"))
            .and(matchers::path("/api/v2/watchings/456"))
            .and(matchers::body_string_contains("note="))
            .respond_with(ResponseTemplate::new(200).set_body_json(&response_json))
            .mount(&mock_server)
            .await;

        let api = setup_watching_api(&mock_server).await;
        let params = UpdateWatchingParams::new(WatchingId::from(456)).with_note("");
        let result = api.update(params).await;

        assert!(result.is_ok());
        let watching = result.unwrap();
        assert_eq!(watching.id, WatchingId::from(456));
        assert_eq!(watching.note, Some("".to_string()));
    }

    #[tokio::test]
    #[cfg(feature = "writable")]
    async fn test_update_watching_not_found() {
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

        Mock::given(matchers::method("PATCH"))
            .and(matchers::path("/api/v2/watchings/999"))
            .respond_with(ResponseTemplate::new(404).set_body_json(&error_json))
            .mount(&mock_server)
            .await;

        let api = setup_watching_api(&mock_server).await;
        let params = UpdateWatchingParams::new(WatchingId::from(999)).with_note("This will fail");
        let result = api.update(params).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    #[cfg(feature = "writable")]
    async fn test_update_watching_unauthorized() {
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

        Mock::given(matchers::method("PATCH"))
            .and(matchers::path("/api/v2/watchings/123"))
            .respond_with(ResponseTemplate::new(401).set_body_json(&error_json))
            .mount(&mock_server)
            .await;

        let api = setup_watching_api(&mock_server).await;
        let params = UpdateWatchingParams::new(WatchingId::from(123)).with_note("Update note");
        let result = api.update(params).await;

        let err = result.expect_err("should return 401 error");
        assert!(matches!(
            err,
            backlog_api_core::Error::HttpStatus { status: 401, .. }
        ));
    }

    #[tokio::test]
    #[cfg(feature = "writable")]
    async fn test_update_watching_forbidden() {
        let mock_server = MockServer::start().await;

        let error_json = json!({
            "errors": [
                {
                    "message": "You do not have permission to update this watching.",
                    "code": 11,
                    "moreInfo": ""
                }
            ]
        });

        Mock::given(matchers::method("PATCH"))
            .and(matchers::path("/api/v2/watchings/123"))
            .respond_with(ResponseTemplate::new(403).set_body_json(&error_json))
            .mount(&mock_server)
            .await;

        let api = setup_watching_api(&mock_server).await;
        let params = UpdateWatchingParams::new(WatchingId::from(123)).with_note("Update note");
        let result = api.update(params).await;

        let err = result.expect_err("should return 403 error");
        assert!(matches!(
            err,
            backlog_api_core::Error::HttpStatus { status: 403, .. }
        ));
    }

    #[tokio::test]
    #[cfg(feature = "writable")]
    async fn test_update_watching_server_error() {
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

        Mock::given(matchers::method("PATCH"))
            .and(matchers::path("/api/v2/watchings/123"))
            .respond_with(ResponseTemplate::new(500).set_body_json(&error_json))
            .mount(&mock_server)
            .await;

        let api = setup_watching_api(&mock_server).await;
        let params = UpdateWatchingParams::new(WatchingId::from(123)).with_note("Update note");
        let result = api.update(params).await;

        let err = result.expect_err("should return 500 error");
        assert!(matches!(
            err,
            backlog_api_core::Error::HttpStatus { status: 500, .. }
        ));
    }
}
