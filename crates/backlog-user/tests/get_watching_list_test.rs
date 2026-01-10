mod common;

use backlog_core::identifier::{Identifier, IssueId, UserId};
use backlog_user::api::{GetWatchingListParams, Order, WatchingSort};
use common::*;
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_get_watching_list_minimal() {
    let mock_server = MockServer::start().await;

    let response_body = r#"[
            {
                "id": 123,
                "resourceAlreadyRead": false,
                "type": "issue",
                "created": "2023-01-01T00:00:00Z",
                "updated": "2023-01-01T00:00:00Z"
            }
        ]"#;

    Mock::given(method("GET"))
        .and(path("/api/v2/users/1/watchings"))
        .respond_with(ResponseTemplate::new(200).set_body_string(response_body))
        .mount(&mock_server)
        .await;

    let api = setup_user_api(&mock_server).await;

    let result = api
        .get_watching_list(UserId::from(1), GetWatchingListParams::default())
        .await;

    assert!(result.is_ok());
    let watchings = result.expect("get_watching_list should succeed");
    assert_eq!(watchings.len(), 1);
    assert_eq!(watchings[0].id.value(), 123);
}

#[tokio::test]
async fn test_get_watching_list_with_all_params() {
    let mock_server = MockServer::start().await;

    let response_body = r#"[
            {
                "id": 456,
                "resourceAlreadyRead": true,
                "note": "Important issue",
                "type": "issue",
                "lastContentUpdated": "2023-01-02T12:00:00Z",
                "created": "2023-01-01T00:00:00Z",
                "updated": "2023-01-02T00:00:00Z"
            }
        ]"#;

    Mock::given(method("GET"))
        .and(path("/api/v2/users/100/watchings"))
        .and(query_param("order", "asc"))
        .and(query_param("sort", "created"))
        .and(query_param("count", "50"))
        .and(query_param("offset", "10"))
        .and(query_param("resourceAlreadyRead", "true"))
        .and(query_param("issueId[]", "123"))
        .and(query_param("issueId[]", "456"))
        .respond_with(ResponseTemplate::new(200).set_body_string(response_body))
        .mount(&mock_server)
        .await;

    let api = setup_user_api(&mock_server).await;

    let params = GetWatchingListParams::builder()
        .order(Order::Asc)
        .sort(WatchingSort::Created)
        .count(50)
        .offset(10)
        .resource_already_read(true)
        .issue_ids(vec![IssueId::from(123), IssueId::from(456)])
        .build()
        .expect("builder should succeed with all fields");

    let result = api.get_watching_list(UserId::from(100), params).await;

    assert!(result.is_ok());
    let watchings = result.expect("get_watching_list should succeed with all params");
    assert_eq!(watchings.len(), 1);
    assert_eq!(watchings[0].id.value(), 456);
    assert!(watchings[0].resource_already_read);
    assert_eq!(watchings[0].note, Some("Important issue".to_string()));
}

#[tokio::test]
async fn test_get_watching_list_empty_response() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v2/users/1/watchings"))
        .respond_with(ResponseTemplate::new(200).set_body_string("[]"))
        .mount(&mock_server)
        .await;

    let api = setup_user_api(&mock_server).await;

    let result = api
        .get_watching_list(UserId::from(1), GetWatchingListParams::default())
        .await;

    assert!(result.is_ok());
    let watchings = result.expect("get_watching_list should succeed with empty response");
    assert_eq!(watchings.len(), 0);
}

#[tokio::test]
async fn test_get_watching_list_unauthorized() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v2/users/1/watchings"))
        .respond_with(ResponseTemplate::new(401))
        .mount(&mock_server)
        .await;

    let api = setup_user_api(&mock_server).await;

    let result = api
        .get_watching_list(UserId::from(1), GetWatchingListParams::default())
        .await;

    assert!(result.is_err());
}
