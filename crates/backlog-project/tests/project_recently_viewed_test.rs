use backlog_project::GetRecentlyViewedProjectsParamsBuilder;
use serde_json::json;
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

mod common;
use backlog_core::identifier::Identifier;
use common::setup_project_api;

#[tokio::test]
async fn test_get_recently_viewed_projects_no_params() {
    let mock_server = MockServer::start().await;
    let project_api = setup_project_api(&mock_server).await;

    let response_body = json!([
        {
            "project": {
                "id": 1,
                "projectKey": "TEST",
                "name": "Test Project",
                "chartEnabled": true,
                "subtaskingEnabled": true,
                "projectLeaderCanEditProjectLeader": false,
                "useWiki": true,
                "useFileSharing": true,
                "useWikiTreeView": true,
                "useOriginalImageSizeAtWiki": false,
                "textFormattingRule": "markdown",
                "archived": false,
                "displayOrder": 1,
                "useDevAttributes": true
            },
            "updated": "2024-04-14T07:00:00Z"
        },
        {
            "project": {
                "id": 2,
                "projectKey": "DEMO",
                "name": "Demo Project",
                "chartEnabled": false,
                "subtaskingEnabled": true,
                "projectLeaderCanEditProjectLeader": true,
                "useWiki": true,
                "useFileSharing": false,
                "useWikiTreeView": false,
                "useOriginalImageSizeAtWiki": true,
                "textFormattingRule": "backlog",
                "archived": false,
                "displayOrder": 2,
                "useDevAttributes": false
            },
            "updated": "2024-04-14T06:00:00Z"
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/api/v2/users/myself/recentlyViewedProjects"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let params = GetRecentlyViewedProjectsParamsBuilder::default()
        .build()
        .unwrap();
    let result = project_api.get_recently_viewed_projects(params).await;

    assert!(result.is_ok());
    let projects = result.unwrap();
    assert_eq!(projects.len(), 2);
    assert_eq!(projects[0].project.id.value(), 1);
    assert_eq!(projects[0].project.project_key.to_string(), "TEST");
    assert_eq!(projects[0].project.name, "Test Project");
    assert_eq!(projects[1].project.id.value(), 2);
    assert_eq!(projects[1].project.project_key.to_string(), "DEMO");
    assert_eq!(projects[1].project.name, "Demo Project");
    assert_eq!(
        projects[0].updated.to_rfc3339(),
        "2024-04-14T07:00:00+00:00"
    );
    assert_eq!(
        projects[1].updated.to_rfc3339(),
        "2024-04-14T06:00:00+00:00"
    );
}

#[tokio::test]
async fn test_get_recently_viewed_projects_with_order_asc() {
    let mock_server = MockServer::start().await;
    let project_api = setup_project_api(&mock_server).await;

    let response_body = json!([
        {
            "project": {
                "id": 3,
                "projectKey": "OLD",
                "name": "Old Project",
                "chartEnabled": true,
                "subtaskingEnabled": false,
                "projectLeaderCanEditProjectLeader": false,
                "useWiki": false,
                "useFileSharing": true,
                "useWikiTreeView": false,
                "useOriginalImageSizeAtWiki": false,
                "textFormattingRule": "markdown",
                "archived": false,
                "displayOrder": 3,
                "useDevAttributes": false
            },
            "updated": "2024-01-03T09:00:00Z"
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/api/v2/users/myself/recentlyViewedProjects"))
        .and(query_param("order", "asc"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let params = GetRecentlyViewedProjectsParamsBuilder::default()
        .order("asc")
        .build()
        .unwrap();
    let result = project_api.get_recently_viewed_projects(params).await;

    assert!(result.is_ok());
    let projects = result.unwrap();
    assert_eq!(projects.len(), 1);
    assert_eq!(projects[0].project.name, "Old Project");
}

#[tokio::test]
async fn test_get_recently_viewed_projects_with_pagination() {
    let mock_server = MockServer::start().await;
    let project_api = setup_project_api(&mock_server).await;

    let response_body = json!([
        {
            "project": {
                "id": 10,
                "projectKey": "PAGE10",
                "name": "Page 10 Project",
                "chartEnabled": true,
                "subtaskingEnabled": true,
                "projectLeaderCanEditProjectLeader": false,
                "useWiki": true,
                "useFileSharing": true,
                "useWikiTreeView": true,
                "useOriginalImageSizeAtWiki": false,
                "textFormattingRule": "markdown",
                "archived": false,
                "displayOrder": 10,
                "useDevAttributes": true
            },
            "updated": "2024-04-14T07:00:00Z"
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/api/v2/users/myself/recentlyViewedProjects"))
        .and(query_param("offset", "10"))
        .and(query_param("count", "50"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let params = GetRecentlyViewedProjectsParamsBuilder::default()
        .offset(10u32)
        .count(50u32)
        .build()
        .unwrap();
    let result = project_api.get_recently_viewed_projects(params).await;

    assert!(result.is_ok());
    let projects = result.unwrap();
    assert_eq!(projects.len(), 1);
    assert_eq!(projects[0].project.name, "Page 10 Project");
}

#[tokio::test]
async fn test_get_recently_viewed_projects_with_all_params() {
    let mock_server = MockServer::start().await;
    let project_api = setup_project_api(&mock_server).await;

    let response_body = json!([]);

    Mock::given(method("GET"))
        .and(path("/api/v2/users/myself/recentlyViewedProjects"))
        .and(query_param("order", "desc"))
        .and(query_param("offset", "100"))
        .and(query_param("count", "20"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let params = GetRecentlyViewedProjectsParamsBuilder::default()
        .order("desc")
        .offset(100u32)
        .count(20u32)
        .build()
        .unwrap();
    let result = project_api.get_recently_viewed_projects(params).await;

    assert!(result.is_ok());
    let projects = result.unwrap();
    assert_eq!(projects.len(), 0);
}

#[tokio::test]
async fn test_get_recently_viewed_projects_server_error() {
    let mock_server = MockServer::start().await;
    let project_api = setup_project_api(&mock_server).await;

    Mock::given(method("GET"))
        .and(path("/api/v2/users/myself/recentlyViewedProjects"))
        .respond_with(ResponseTemplate::new(500).set_body_string("Internal Server Error"))
        .mount(&mock_server)
        .await;

    let params = GetRecentlyViewedProjectsParamsBuilder::default()
        .build()
        .unwrap();
    let result = project_api.get_recently_viewed_projects(params).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_recently_viewed_projects_unauthorized() {
    let mock_server = MockServer::start().await;
    let project_api = setup_project_api(&mock_server).await;

    let error_body = json!({
        "errors": [
            {
                "message": "Authentication failure",
                "code": 11,
                "moreInfo": ""
            }
        ]
    });

    Mock::given(method("GET"))
        .and(path("/api/v2/users/myself/recentlyViewedProjects"))
        .respond_with(ResponseTemplate::new(401).set_body_json(&error_body))
        .mount(&mock_server)
        .await;

    let params = GetRecentlyViewedProjectsParamsBuilder::default()
        .build()
        .unwrap();
    let result = project_api.get_recently_viewed_projects(params).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_recently_viewed_projects_empty_result() {
    let mock_server = MockServer::start().await;
    let project_api = setup_project_api(&mock_server).await;

    let response_body = json!([]);

    Mock::given(method("GET"))
        .and(path("/api/v2/users/myself/recentlyViewedProjects"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let params = GetRecentlyViewedProjectsParamsBuilder::default()
        .build()
        .unwrap();
    let result = project_api.get_recently_viewed_projects(params).await;

    assert!(result.is_ok());
    let projects = result.unwrap();
    assert_eq!(projects.len(), 0);
}
