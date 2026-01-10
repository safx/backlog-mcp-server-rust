mod common;

use backlog_core::identifier::{ActivityId, ActivityTypeId, Identifier};
use backlog_space::api::GetSpaceRecentUpdatesParams;
use common::*;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path},
};

#[tokio::test]
async fn test_get_space_recent_updates_success() {
    let server = MockServer::start().await;
    let space_api = setup_space_api(&server).await;

    let mock_response = serde_json::json!([
        {
            "id": 143592,
            "project": {
                "id": 1,
                "projectKey": "EXAMPLE",
                "name": "Example Project",
                "chartEnabled": false,
                "subtaskingEnabled": false,
                "projectLeaderCanEditProjectLeader": false,
                "useWiki": true,
                "useFileSharing": true,
                "useWikiTreeView": false,
                "useOriginalImageSizeAtWiki": false,
                "textFormattingRule": "markdown",
                "archived": false,
                "displayOrder": 0,
                "useDevAttributes": false
            },
            "type": 1,
            "content": {
                "id": 1234,
                "key_id": 100,
                "summary": "Fix bug in login",
                "description": "Fixed authentication issue"
            },
            "notifications": [],
            "createdUser": {
                "id": 1001,
                "userId": "admin",
                "name": "Admin User",
                "roleType": 1,
                "lang": null,
                "mailAddress": "admin@example.com",
                "lastLoginTime": "2024-01-01T00:00:00Z"
            },
            "created": "2024-01-15T10:30:00Z"
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/api/v2/space/activities"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
        .mount(&server)
        .await;

    let params = GetSpaceRecentUpdatesParams::default();
    let result = space_api.get_space_recent_updates(params).await;

    assert!(result.is_ok());
    let activities = result.expect("get_space_recent_updates should succeed");
    assert_eq!(activities.len(), 1);

    let activity = &activities[0];
    assert_eq!(activity.id.value(), 143592);
    assert_eq!(activity.type_id, 1);
    assert_eq!(activity.project_name(), Some("Example Project"));
}

#[tokio::test]
async fn test_get_space_recent_updates_with_filters() {
    let server = MockServer::start().await;
    let space_api = setup_space_api(&server).await;

    let mock_response = serde_json::json!([]);

    Mock::given(method("GET"))
        .and(path("/api/v2/space/activities"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
        .mount(&server)
        .await;

    let params = GetSpaceRecentUpdatesParams {
        activity_type_ids: Some(vec![ActivityTypeId::new(1), ActivityTypeId::new(2)]),
        count: Some(50),
        order: Some("desc".to_string()),
        ..Default::default()
    };

    let result = space_api.get_space_recent_updates(params).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_space_recent_updates_with_pagination() {
    let server = MockServer::start().await;
    let space_api = setup_space_api(&server).await;

    let mock_response = serde_json::json!([]);

    Mock::given(method("GET"))
        .and(path("/api/v2/space/activities"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
        .mount(&server)
        .await;

    let params = GetSpaceRecentUpdatesParams {
        min_id: Some(ActivityId::new(100)),
        max_id: Some(ActivityId::new(200)),
        ..Default::default()
    };

    let result = space_api.get_space_recent_updates(params).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_space_recent_updates_unauthorized() {
    let server = MockServer::start().await;
    let space_api = setup_space_api(&server).await;

    Mock::given(method("GET"))
        .and(path("/api/v2/space/activities"))
        .respond_with(ResponseTemplate::new(401))
        .mount(&server)
        .await;

    let params = GetSpaceRecentUpdatesParams::default();
    let result = space_api.get_space_recent_updates(params).await;
    assert!(result.is_err());
    let err = result.unwrap_err();
    // Note: This test returns 401 with no body, which becomes UnparseableErrorResponse
    assert!(matches!(
        err,
        backlog_api_core::Error::UnparseableErrorResponse { status: 401, .. }
    ));
}
