mod common;

#[cfg(test)]
mod add_watching_test {
    use serde_json::json;
    use wiremock::{matchers, Mock, MockServer, ResponseTemplate};

    use crate::common::setup_watching_api;
    use backlog_core::identifier::{Identifier, IssueId};
    use backlog_core::{IssueIdOrKey, IssueKey};
    use backlog_watching::AddWatchingParams;
    use std::str::FromStr;

    #[tokio::test]
    #[cfg(feature = "writable")]
    async fn test_add_watching_with_issue_id() {
        let mock_server = MockServer::start().await;

        let response_json = json!({
            "id": 123,
            "resourceAlreadyRead": false,
            "note": "Watching this issue",
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
                "summary": "New feature implementation",
                "description": "Implement new feature",
                "resolution": null,
                "priority": {
                    "id": 2,
                    "name": "Normal"
                },
                "status": {
                    "id": 1,
                    "projectId": 1,
                    "name": "Open",
                    "color": "#ed8077",
                    "displayOrder": 1000
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
                "created": "2024-01-20T10:00:00Z",
                "updatedUser": {
                    "id": 1,
                    "userId": "admin",
                    "name": "Administrator",
                    "roleType": 1,
                    "lang": "ja",
                    "mailAddress": "admin@example.com",
                    "nulabAccount": null,
                    "keyword": "admin"
                },
                "updated": "2024-01-20T10:00:00Z",
                "customFields": [],
                "attachments": [],
                "sharedFiles": [],
                "stars": []
            },
            "lastContentUpdated": null,
            "created": "2024-01-20T10:00:00Z",
            "updated": "2024-01-20T10:00:00Z"
        });

        Mock::given(matchers::method("POST"))
            .and(matchers::path("/api/v2/watchings"))
            .and(matchers::header(
                "Content-Type",
                "application/x-www-form-urlencoded",
            ))
            .and(matchers::body_string_contains("issueIdOrKey=456"))
            .and(matchers::body_string_contains("note=Watching+this+issue"))
            .respond_with(ResponseTemplate::new(201).set_body_json(&response_json))
            .mount(&mock_server)
            .await;

        let api = setup_watching_api(&mock_server).await;
        let params = AddWatchingParams::new(IssueIdOrKey::Id(IssueId::from(456)))
            .with_note("Watching this issue");
        let result = api.add(params).await;

        assert!(result.is_ok());
        let watching = result.unwrap();
        assert_eq!(watching.id.value(), 123);
        assert_eq!(watching.note, Some("Watching this issue".to_string()));
        assert!(watching.issue.is_some());
    }

    #[tokio::test]
    #[cfg(feature = "writable")]
    async fn test_add_watching_with_issue_key() {
        let mock_server = MockServer::start().await;

        let response_json = json!({
            "id": 456,
            "resourceAlreadyRead": false,
            "type": "issue",
            "issue": {
                "id": 789,
                "projectId": 1,
                "issueKey": "TEST-123",
                "keyId": 123,
                "issueType": {
                    "id": 2,
                    "projectId": 1,
                    "name": "Bug",
                    "color": "#990000",
                    "displayOrder": 1
                },
                "summary": "Bug fix",
                "description": "",
                "resolution": null,
                "priority": {
                    "id": 2,
                    "name": "Normal"
                },
                "status": {
                    "id": 1,
                    "projectId": 1,
                    "name": "Open",
                    "color": "#ed8077",
                    "displayOrder": 1000
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
                "created": "2024-01-21T11:00:00Z",
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
                "updated": "2024-01-21T11:00:00Z",
                "customFields": [],
                "attachments": [],
                "sharedFiles": [],
                "stars": []
            },
            "created": "2024-01-21T11:00:00Z",
            "updated": "2024-01-21T11:00:00Z"
        });

        Mock::given(matchers::method("POST"))
            .and(matchers::path("/api/v2/watchings"))
            .and(matchers::body_string_contains("issueIdOrKey=TEST-123"))
            .respond_with(ResponseTemplate::new(201).set_body_json(&response_json))
            .mount(&mock_server)
            .await;

        let api = setup_watching_api(&mock_server).await;
        let params =
            AddWatchingParams::new(IssueIdOrKey::Key(IssueKey::from_str("TEST-123").unwrap()));
        let result = api.add(params).await;

        if let Err(ref e) = result {
            eprintln!("Error: {e:?}");
        }
        assert!(result.is_ok());
        let watching = result.unwrap();
        assert_eq!(watching.id.value(), 456);
        assert_eq!(watching.note, None);
    }

    #[tokio::test]
    #[cfg(feature = "writable")]
    async fn test_add_watching_already_exists() {
        let mock_server = MockServer::start().await;

        let error_json = json!({
            "errors": [
                {
                    "message": "You cannot add same watching twice.",
                    "code": 14,
                    "moreInfo": ""
                }
            ]
        });

        Mock::given(matchers::method("POST"))
            .and(matchers::path("/api/v2/watchings"))
            .respond_with(ResponseTemplate::new(400).set_body_json(&error_json))
            .mount(&mock_server)
            .await;

        let api = setup_watching_api(&mock_server).await;
        let params = AddWatchingParams::new(IssueIdOrKey::Id(IssueId::from(999)));
        let result = api.add(params).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    #[cfg(feature = "writable")]
    async fn test_add_watching_unauthorized() {
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
            .and(matchers::path("/api/v2/watchings"))
            .respond_with(ResponseTemplate::new(401).set_body_json(&error_json))
            .mount(&mock_server)
            .await;

        let api = setup_watching_api(&mock_server).await;
        let params = AddWatchingParams::new(IssueIdOrKey::Id(IssueId::from(123)));
        let result = api.add(params).await;

        let err = result.expect_err("should return 401 error");
        assert!(matches!(
            err,
            backlog_api_core::Error::HttpStatus { status: 401, .. }
        ));
    }

    #[tokio::test]
    #[cfg(feature = "writable")]
    async fn test_add_watching_forbidden() {
        let mock_server = MockServer::start().await;

        let error_json = json!({
            "errors": [
                {
                    "message": "You do not have permission to watch this issue.",
                    "code": 11,
                    "moreInfo": ""
                }
            ]
        });

        Mock::given(matchers::method("POST"))
            .and(matchers::path("/api/v2/watchings"))
            .respond_with(ResponseTemplate::new(403).set_body_json(&error_json))
            .mount(&mock_server)
            .await;

        let api = setup_watching_api(&mock_server).await;
        let params = AddWatchingParams::new(IssueIdOrKey::Id(IssueId::from(123)));
        let result = api.add(params).await;

        let err = result.expect_err("should return 403 error");
        assert!(matches!(
            err,
            backlog_api_core::Error::HttpStatus { status: 403, .. }
        ));
    }

    #[tokio::test]
    #[cfg(feature = "writable")]
    async fn test_add_watching_server_error() {
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
            .and(matchers::path("/api/v2/watchings"))
            .respond_with(ResponseTemplate::new(500).set_body_json(&error_json))
            .mount(&mock_server)
            .await;

        let api = setup_watching_api(&mock_server).await;
        let params = AddWatchingParams::new(IssueIdOrKey::Id(IssueId::from(123)));
        let result = api.add(params).await;

        let err = result.expect_err("should return 500 error");
        assert!(matches!(
            err,
            backlog_api_core::Error::HttpStatus { status: 500, .. }
        ));
    }
}
