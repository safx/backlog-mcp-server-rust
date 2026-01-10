#[cfg(feature = "writable")]
use backlog_issue::AddRecentlyViewedIssueParams;
use serde_json::json;
use wiremock::matchers::{body_string, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

mod common;
use backlog_core::identifier::Identifier;
use backlog_core::{IssueIdOrKey, IssueKey};
use common::setup_issue_api;
use std::str::FromStr;

#[cfg(feature = "writable")]
#[tokio::test]
async fn test_add_recently_viewed_issue_with_id() {
    let mock_server = MockServer::start().await;
    let issue_api = setup_issue_api(&mock_server).await;

    let response_body = json!({
        "id": 12345,
        "projectId": 1,
        "issueKey": "TEST-1",
        "keyId": 1,
        "issueType": {
            "id": 1,
            "projectId": 1,
            "name": "Bug",
            "color": "#990000",
            "displayOrder": 1
        },
        "summary": "Test Issue",
        "description": "This is a test issue",
        "resolutions": null,
        "priority": {
            "id": 3,
            "name": "Normal"
        },
        "status": {
            "id": 1,
            "projectId": 1,
            "name": "Open",
            "color": "#ed8077",
            "displayOrder": 1
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
            "name": "Admin",
            "roleType": 1,
            "lang": "ja",
            "mailAddress": "admin@example.com"
        },
        "created": "2023-01-01T00:00:00Z",
        "updatedUser": {
            "id": 1,
            "userId": "admin",
            "name": "Admin",
            "roleType": 1,
            "lang": "ja",
            "mailAddress": "admin@example.com"
        },
        "updated": "2023-01-01T00:00:00Z",
        "customFields": [],
        "attachments": [],
        "sharedFiles": [],
        "stars": []
    });

    Mock::given(method("POST"))
        .and(path("/api/v2/users/myself/recentlyViewedIssues"))
        .and(body_string("issueIdOrKey=12345"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let params = AddRecentlyViewedIssueParams {
        issue_id_or_key: IssueIdOrKey::Id(12345.into()),
    };
    let result = issue_api.add_recently_viewed_issue(params).await;

    assert!(result.is_ok());
    let issue = result.unwrap();
    assert_eq!(issue.id.value(), 12345);
    assert_eq!(issue.issue_key.to_string(), "TEST-1");
    assert_eq!(issue.summary, "Test Issue");
}

#[cfg(feature = "writable")]
#[tokio::test]
async fn test_add_recently_viewed_issue_with_key() {
    let mock_server = MockServer::start().await;
    let issue_api = setup_issue_api(&mock_server).await;

    let response_body = json!({
        "id": 12346,
        "projectId": 1,
        "issueKey": "TEST-2",
        "keyId": 2,
        "issueType": {
            "id": 2,
            "projectId": 1,
            "name": "Task",
            "color": "#4488cc",
            "displayOrder": 2
        },
        "summary": "Another Test Issue",
        "description": "This is another test issue",
        "resolutions": null,
        "priority": {
            "id": 2,
            "name": "High"
        },
        "status": {
            "id": 2,
            "projectId": 1,
            "name": "In Progress",
            "color": "#4488cc",
            "displayOrder": 2
        },
        "assignee": {
            "id": 2,
            "userId": "user1",
            "name": "User 1",
            "roleType": 2,
            "lang": "ja",
            "mailAddress": "user1@example.com"
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
            "name": "Admin",
            "roleType": 1,
            "lang": "ja",
            "mailAddress": "admin@example.com"
        },
        "created": "2023-01-02T00:00:00Z",
        "updatedUser": {
            "id": 2,
            "userId": "user1",
            "name": "User 1",
            "roleType": 2,
            "lang": "ja",
            "mailAddress": "user1@example.com"
        },
        "updated": "2023-01-02T10:00:00Z",
        "customFields": [],
        "attachments": [],
        "sharedFiles": [],
        "stars": []
    });

    Mock::given(method("POST"))
        .and(path("/api/v2/users/myself/recentlyViewedIssues"))
        .and(body_string("issueIdOrKey=TEST-2"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let params = AddRecentlyViewedIssueParams {
        issue_id_or_key: IssueIdOrKey::Key(IssueKey::from_str("TEST-2").unwrap()),
    };
    let result = issue_api.add_recently_viewed_issue(params).await;

    assert!(result.is_ok());
    let issue = result.unwrap();
    assert_eq!(issue.id.value(), 12346);
    assert_eq!(issue.issue_key.to_string(), "TEST-2");
    assert_eq!(issue.summary, "Another Test Issue");
    assert_eq!(issue.assignee.as_ref().unwrap().name, "User 1");
}

#[cfg(feature = "writable")]
#[tokio::test]
async fn test_add_recently_viewed_issue_not_found() {
    let mock_server = MockServer::start().await;
    let issue_api = setup_issue_api(&mock_server).await;

    let error_body = json!({
        "errors": [
            {
                "message": "No issue found.",
                "code": 5,
                "moreInfo": ""
            }
        ]
    });

    Mock::given(method("POST"))
        .and(path("/api/v2/users/myself/recentlyViewedIssues"))
        .and(body_string("issueIdOrKey=INVALID-999"))
        .respond_with(ResponseTemplate::new(404).set_body_json(&error_body))
        .mount(&mock_server)
        .await;

    let params = AddRecentlyViewedIssueParams {
        issue_id_or_key: IssueIdOrKey::Key(IssueKey::from_str("INVALID-999").unwrap()),
    };
    let result = issue_api.add_recently_viewed_issue(params).await;

    assert!(result.is_err());
}

#[cfg(feature = "writable")]
#[tokio::test]
async fn test_add_recently_viewed_issue_unauthorized() {
    let mock_server = MockServer::start().await;
    let issue_api = setup_issue_api(&mock_server).await;

    let error_body = json!({
        "errors": [
            {
                "message": "Authentication failure",
                "code": 11,
                "moreInfo": ""
            }
        ]
    });

    Mock::given(method("POST"))
        .and(path("/api/v2/users/myself/recentlyViewedIssues"))
        .respond_with(ResponseTemplate::new(401).set_body_json(&error_body))
        .mount(&mock_server)
        .await;

    let params = AddRecentlyViewedIssueParams {
        issue_id_or_key: IssueIdOrKey::Id(12345.into()),
    };
    let result = issue_api.add_recently_viewed_issue(params).await;

    assert!(result.is_err());
}

#[cfg(feature = "writable")]
#[tokio::test]
async fn test_add_recently_viewed_issue_server_error() {
    let mock_server = MockServer::start().await;
    let issue_api = setup_issue_api(&mock_server).await;

    Mock::given(method("POST"))
        .and(path("/api/v2/users/myself/recentlyViewedIssues"))
        .respond_with(ResponseTemplate::new(500).set_body_string("Internal Server Error"))
        .mount(&mock_server)
        .await;

    let params = AddRecentlyViewedIssueParams {
        issue_id_or_key: IssueIdOrKey::Id(12345.into()),
    };
    let result = issue_api.add_recently_viewed_issue(params).await;

    assert!(result.is_err());
}

#[cfg(feature = "writable")]
#[tokio::test]
async fn test_add_recently_viewed_issue_with_custom_fields() {
    let mock_server = MockServer::start().await;
    let issue_api = setup_issue_api(&mock_server).await;

    let response_body = json!({
        "id": 12347,
        "projectId": 1,
        "issueKey": "TEST-3",
        "keyId": 3,
        "issueType": {
            "id": 1,
            "projectId": 1,
            "name": "Bug",
            "color": "#990000",
            "displayOrder": 1
        },
        "summary": "Issue with Custom Fields",
        "description": "This issue has custom fields",
        "resolutions": null,
        "priority": {
            "id": 3,
            "name": "Normal"
        },
        "status": {
            "id": 1,
            "projectId": 1,
            "name": "Open",
            "color": "#ed8077",
            "displayOrder": 1
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
            "name": "Admin",
            "roleType": 1,
            "lang": "ja",
            "mailAddress": "admin@example.com"
        },
        "created": "2023-01-03T00:00:00Z",
        "updatedUser": {
            "id": 1,
            "userId": "admin",
            "name": "Admin",
            "roleType": 1,
            "lang": "ja",
            "mailAddress": "admin@example.com"
        },
        "updated": "2023-01-03T00:00:00Z",
        "customFields": [
            {
                "id": 1,
                "fieldTypeId": 1,
                "name": "Text Field",
                "value": "Sample Text"
            },
            {
                "id": 2,
                "fieldTypeId": 2,
                "name": "Number Field",
                "value": 42
            }
        ],
        "attachments": [],
        "sharedFiles": [],
        "stars": []
    });

    Mock::given(method("POST"))
        .and(path("/api/v2/users/myself/recentlyViewedIssues"))
        .and(body_string("issueIdOrKey=TEST-3"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let params = AddRecentlyViewedIssueParams {
        issue_id_or_key: IssueIdOrKey::Key(IssueKey::from_str("TEST-3").unwrap()),
    };
    let result = issue_api.add_recently_viewed_issue(params).await;

    assert!(result.is_ok());
    let issue = result.unwrap();
    assert_eq!(issue.id.value(), 12347);
    assert_eq!(issue.custom_fields.len(), 2);
}

#[cfg(feature = "writable")]
#[tokio::test]
async fn test_add_recently_viewed_issue_already_viewed() {
    let mock_server = MockServer::start().await;
    let issue_api = setup_issue_api(&mock_server).await;

    // Even if already viewed, API returns the issue successfully
    let response_body = json!({
        "id": 12345,
        "projectId": 1,
        "issueKey": "TEST-1",
        "keyId": 1,
        "issueType": {
            "id": 1,
            "projectId": 1,
            "name": "Bug",
            "color": "#990000",
            "displayOrder": 1
        },
        "summary": "Already Viewed Issue",
        "description": "This issue was already in the recently viewed list",
        "resolutions": null,
        "priority": {
            "id": 3,
            "name": "Normal"
        },
        "status": {
            "id": 1,
            "projectId": 1,
            "name": "Open",
            "color": "#ed8077",
            "displayOrder": 1
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
            "name": "Admin",
            "roleType": 1,
            "lang": "ja",
            "mailAddress": "admin@example.com"
        },
        "created": "2023-01-01T00:00:00Z",
        "updatedUser": {
            "id": 1,
            "userId": "admin",
            "name": "Admin",
            "roleType": 1,
            "lang": "ja",
            "mailAddress": "admin@example.com"
        },
        "updated": "2023-01-01T00:00:00Z",
        "customFields": [],
        "attachments": [],
        "sharedFiles": [],
        "stars": []
    });

    Mock::given(method("POST"))
        .and(path("/api/v2/users/myself/recentlyViewedIssues"))
        .and(body_string("issueIdOrKey=TEST-1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let params = AddRecentlyViewedIssueParams {
        issue_id_or_key: IssueIdOrKey::Key(IssueKey::from_str("TEST-1").unwrap()),
    };
    let result = issue_api.add_recently_viewed_issue(params).await;

    assert!(result.is_ok());
    let issue = result.unwrap();
    assert_eq!(issue.summary, "Already Viewed Issue");
}
