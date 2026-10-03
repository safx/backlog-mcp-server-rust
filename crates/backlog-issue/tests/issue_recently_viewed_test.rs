mod common;
use common::*;

use backlog_core::identifier::Identifier;
use backlog_issue::GetRecentlyViewedIssuesParamsBuilder;

#[tokio::test]
async fn test_get_recently_viewed_issues_no_params() {
    let mock_server = wiremock::MockServer::start().await;
    let issue_api = setup_issue_api(&mock_server).await;

    let response_body = json!([
        {
            "issue": {
                "id": 10,
                "projectId": 1,
                "issueKey": "PROJ-10",
                "keyId": 10,
                "summary": "Recently viewed issue 1",
                "description": "This is a recently viewed issue",
                "issueType": {"id": 1, "projectId":1, "name": "Bug", "color": "#ff0000", "displayOrder": 0},
                "priority": {"id": 2, "name": "High"},
                "category": [],
                "versions": [],
                "milestone": [],
                "createdUser": {"id": 1, "userId": "john", "name": "John Doe", "roleType": 1, "mailAddress": "john@example.com", "lastLoginTime": "2025-04-01T06:35:39Z"},
                "created": "2024-03-14T06:35:39Z",
                "updated": "2024-04-13T06:35:39Z",
                "status": {"id": 1, "projectId": 1, "name": "Open", "color": "#ff0000", "displayOrder": 1}
            },
            "updated": "2024-04-14T07:00:00Z"
        },
        {
            "issue": {
                "id": 20,
                "projectId": 2,
                "issueKey": "PROJ2-20",
                "keyId": 20,
                "summary": "Recently viewed issue 2",
                "description": "Another recently viewed issue",
                "issueType": {"id": 2, "projectId":2, "name": "Task", "color": "#00ff00", "displayOrder": 1},
                "priority": {"id": 3, "name": "Normal"},
                "category": [],
                "versions": [],
                "milestone": [],
                "createdUser": {"id": 1, "userId": "john", "name": "John Doe", "roleType": 1, "mailAddress": "john@example.com", "lastLoginTime": "2025-04-01T06:35:39Z"},
                "created": "2024-03-14T06:35:39Z",
                "updated": "2024-04-13T06:35:39Z",
                "status": {"id": 2, "projectId": 2, "name": "In Progress", "color": "#0000ff", "displayOrder": 2}
            },
            "updated": "2024-04-14T06:00:00Z"
        },
    ]);

    Mock::given(method("GET"))
        .and(path("/api/v2/users/myself/recentlyViewedIssues"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let params = GetRecentlyViewedIssuesParamsBuilder::default()
        .build()
        .unwrap();
    let result = issue_api.get_recently_viewed_issues(params).await;
    assert!(result.is_ok());
    let issues = result.unwrap();
    assert_eq!(issues.len(), 2);
    assert_eq!(issues[0].issue.id.value(), 10);
    assert_eq!(issues[0].issue.summary, "Recently viewed issue 1");
    assert_eq!(issues[1].issue.id.value(), 20);
    assert_eq!(issues[1].issue.summary, "Recently viewed issue 2");
    assert_eq!(issues[0].updated.to_rfc3339(), "2024-04-14T07:00:00+00:00");
    assert_eq!(issues[1].updated.to_rfc3339(), "2024-04-14T06:00:00+00:00");
    assert_eq!(issues[0].issue.updated, "2024-04-13T06:35:39Z");
}

#[tokio::test]
async fn test_get_recently_viewed_issues_with_order_asc() {
    let mock_server = wiremock::MockServer::start().await;
    let issue_api = setup_issue_api(&mock_server).await;

    let response_body = json!([
        {
            "issue": {
                "id": 5,
                "projectId": 1,
                "issueKey": "OLD-5",
                "keyId": 5,
                "summary": "Older viewed issue",
                "description": "Viewed earlier",
                "issueType": {"id": 1, "projectId":1, "name": "Bug", "color": "#ff0000", "displayOrder": 0},
                "priority": {"id": 2, "name": "High"},
                "category": [],
                "versions": [],
                "milestone": [],
                "createdUser": {"id": 1, "userId": "john", "name": "John Doe", "roleType": 1, "mailAddress": "john@example.com", "lastLoginTime": "2025-04-01T06:35:39Z"},
                "created": "2024-01-01T06:35:39Z",
                "updated": "2024-01-02T06:35:39Z",
                "status": {"id": 1, "projectId": 1, "name": "Open", "color": "#ff0000", "displayOrder": 1}
            },
            "updated": "2024-01-03T09:00:00Z"
        },
    ]);

    Mock::given(method("GET"))
        .and(path("/api/v2/users/myself/recentlyViewedIssues"))
        .and(query_param("order", "asc"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let params = GetRecentlyViewedIssuesParamsBuilder::default()
        .order("asc".to_string())
        .build()
        .unwrap();
    let result = issue_api.get_recently_viewed_issues(params).await;
    assert!(result.is_ok());
    let issues = result.unwrap();
    assert_eq!(issues.len(), 1);
    assert_eq!(issues[0].issue.summary, "Older viewed issue");
}

#[tokio::test]
async fn test_get_recently_viewed_issues_with_pagination() {
    let mock_server = wiremock::MockServer::start().await;
    let issue_api = setup_issue_api(&mock_server).await;

    let response_body = json!([
        {
            "issue": {
                "id": 100,
                "projectId": 1,
                "issueKey": "PAGE-100",
                "keyId": 100,
                "summary": "Page 2 Issue",
                "description": "Second page issue",
                "issueType": {"id": 1, "projectId":1, "name": "Bug", "color": "#ff0000", "displayOrder": 0},
                "priority": {"id": 2, "name": "High"},
                "category": [],
                "versions": [],
                "milestone": [],
                "createdUser": {"id": 1, "userId": "john", "name": "John Doe", "roleType": 1, "mailAddress": "john@example.com", "lastLoginTime": "2025-04-01T06:35:39Z"},
                "created": "2024-03-14T06:35:39Z",
                "updated": "2024-04-13T06:35:39Z",
                "status": {"id": 1, "projectId": 1, "name": "Open", "color": "#ff0000", "displayOrder": 1}
            },
            "updated": "2024-04-14T07:00:00Z"
        },
    ]);

    Mock::given(method("GET"))
        .and(path("/api/v2/users/myself/recentlyViewedIssues"))
        .and(query_param("offset", "10"))
        .and(query_param("count", "5"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let params = GetRecentlyViewedIssuesParamsBuilder::default()
        .offset(10u32)
        .count(5u32)
        .build()
        .unwrap();
    let result = issue_api.get_recently_viewed_issues(params).await;
    assert!(result.is_ok());
    let issues = result.unwrap();
    assert_eq!(issues.len(), 1);
    assert_eq!(issues[0].issue.issue_key, "PAGE-100".parse().unwrap());
}

#[tokio::test]
async fn test_get_recently_viewed_issues_all_params() {
    let mock_server = wiremock::MockServer::start().await;
    let issue_api = setup_issue_api(&mock_server).await;

    let response_body = json!([]);

    Mock::given(method("GET"))
        .and(path("/api/v2/users/myself/recentlyViewedIssues"))
        .and(query_param("order", "desc"))
        .and(query_param("offset", "50"))
        .and(query_param("count", "100"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let params = GetRecentlyViewedIssuesParamsBuilder::default()
        .order("desc".to_string())
        .offset(50u32)
        .count(100u32)
        .build()
        .unwrap();
    let result = issue_api.get_recently_viewed_issues(params).await;
    assert!(result.is_ok());
    let issues = result.unwrap();
    assert!(issues.is_empty());
}

#[tokio::test]
async fn test_get_recently_viewed_issues_server_error() {
    let mock_server = wiremock::MockServer::start().await;
    let issue_api = setup_issue_api(&mock_server).await;

    Mock::given(method("GET"))
        .and(path("/api/v2/users/myself/recentlyViewedIssues"))
        .respond_with(ResponseTemplate::new(500))
        .mount(&mock_server)
        .await;

    let params = GetRecentlyViewedIssuesParamsBuilder::default()
        .build()
        .unwrap();
    let result = issue_api.get_recently_viewed_issues(params).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_recently_viewed_issues_unauthorized() {
    let mock_server = wiremock::MockServer::start().await;
    let issue_api = setup_issue_api(&mock_server).await;

    Mock::given(method("GET"))
        .and(path("/api/v2/users/myself/recentlyViewedIssues"))
        .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
            "errors": [{"message": "Authentication failure."}]
        })))
        .mount(&mock_server)
        .await;

    let params = GetRecentlyViewedIssuesParamsBuilder::default()
        .build()
        .unwrap();
    let result = issue_api.get_recently_viewed_issues(params).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_recently_viewed_issues_empty_result() {
    let mock_server = wiremock::MockServer::start().await;
    let issue_api = setup_issue_api(&mock_server).await;

    let response_body = json!([]);

    Mock::given(method("GET"))
        .and(path("/api/v2/users/myself/recentlyViewedIssues"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let params = GetRecentlyViewedIssuesParamsBuilder::default()
        .build()
        .unwrap();
    let result = issue_api.get_recently_viewed_issues(params).await;
    assert!(result.is_ok());
    let issues = result.unwrap();
    assert_eq!(issues.len(), 0);
}
