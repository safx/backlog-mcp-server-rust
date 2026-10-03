use backlog_wiki::GetRecentlyViewedWikisParamsBuilder;
use serde_json::json;
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

mod common;
use common::{Identifier, setup_wiki_api};

#[tokio::test]
async fn test_get_recently_viewed_wikis_no_params() {
    let mock_server = MockServer::start().await;
    let wiki_api = setup_wiki_api(&mock_server).await;

    let response_body = json!([
        {
            "page": {
                "id": 1,
                "projectId": 100,
                "name": "Home",
                "tags": [
                    {"id": 1, "name": "important"}
                ],
                "createdUser": {
                    "id": 1,
                    "userId": "admin",
                    "name": "Admin User",
                    "roleType": 1,
                    "lang": "ja",
                    "mailAddress": "admin@example.com"
                },
                "created": "2024-01-01T09:00:00Z",
                "updatedUser": {
                    "id": 1,
                    "userId": "admin",
                    "name": "Admin User",
                    "roleType": 1,
                    "lang": "ja",
                    "mailAddress": "admin@example.com"
                },
                "updated": "2024-01-15T10:30:00Z"
            },
            "updated": "2024-04-14T07:00:00Z"
        },
        {
            "page": {
                "id": 2,
                "projectId": 100,
                "name": "API Documentation",
                "tags": [],
                "createdUser": {
                    "id": 2,
                    "userId": "dev",
                    "name": "Developer",
                    "roleType": 2,
                    "lang": "ja",
                    "mailAddress": "dev@example.com"
                },
                "created": "2024-01-02T10:00:00Z",
                "updatedUser": {
                    "id": 2,
                    "userId": "dev",
                    "name": "Developer",
                    "roleType": 2,
                    "lang": "ja",
                    "mailAddress": "dev@example.com"
                },
                "updated": "2024-01-16T11:00:00Z"
            },
            "updated": "2024-04-14T06:00:00Z"
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/api/v2/users/myself/recentlyViewedWikis"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let params = GetRecentlyViewedWikisParamsBuilder::default()
        .build()
        .unwrap();
    let result = wiki_api.get_recently_viewed_wikis(params).await;

    assert!(result.is_ok());
    let wikis = result.unwrap();
    assert_eq!(wikis.len(), 2);
    assert_eq!(wikis[0].page.id.value(), 1);
    assert_eq!(wikis[0].page.name, "Home");
    assert_eq!(wikis[1].page.id.value(), 2);
    assert_eq!(wikis[1].page.name, "API Documentation");
    assert_eq!(wikis[0].updated.to_rfc3339(), "2024-04-14T07:00:00+00:00");
    assert_eq!(wikis[1].updated.to_rfc3339(), "2024-04-14T06:00:00+00:00");
    assert_eq!(
        wikis[0].page.updated.to_rfc3339(),
        "2024-01-15T10:30:00+00:00"
    );
}

#[tokio::test]
async fn test_get_recently_viewed_wikis_with_order_asc() {
    let mock_server = MockServer::start().await;
    let wiki_api = setup_wiki_api(&mock_server).await;

    let response_body = json!([
        {
            "page": {
                "id": 3,
                "projectId": 100,
                "name": "Old Wiki",
                "tags": [],
                "createdUser": {
                    "id": 1,
                    "userId": "admin",
                    "name": "Admin User",
                    "roleType": 1,
                    "lang": "ja",
                    "mailAddress": "admin@example.com"
                },
                "created": "2023-12-01T09:00:00Z",
                "updatedUser": {
                    "id": 1,
                    "userId": "admin",
                    "name": "Admin User",
                    "roleType": 1,
                    "lang": "ja",
                    "mailAddress": "admin@example.com"
                },
                "updated": "2023-12-01T09:00:00Z"
            },
            "updated": "2024-01-03T09:00:00Z"
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/api/v2/users/myself/recentlyViewedWikis"))
        .and(query_param("order", "asc"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let params = GetRecentlyViewedWikisParamsBuilder::default()
        .order("asc")
        .build()
        .unwrap();
    let result = wiki_api.get_recently_viewed_wikis(params).await;

    assert!(result.is_ok());
    let wikis = result.unwrap();
    assert_eq!(wikis.len(), 1);
    assert_eq!(wikis[0].page.name, "Old Wiki");
}

#[tokio::test]
async fn test_get_recently_viewed_wikis_with_pagination() {
    let mock_server = MockServer::start().await;
    let wiki_api = setup_wiki_api(&mock_server).await;

    let response_body = json!([
        {
            "page": {
                "id": 10,
                "projectId": 100,
                "name": "Page 10",
                "tags": [],
                "createdUser": {
                    "id": 1,
                    "userId": "admin",
                    "name": "Admin User",
                    "roleType": 1,
                    "lang": "ja",
                    "mailAddress": "admin@example.com"
                },
                "created": "2024-01-10T09:00:00Z",
                "updatedUser": {
                    "id": 1,
                    "userId": "admin",
                    "name": "Admin User",
                    "roleType": 1,
                    "lang": "ja",
                    "mailAddress": "admin@example.com"
                },
                "updated": "2024-01-10T09:00:00Z"
            },
            "updated": "2024-04-14T07:00:00Z"
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/api/v2/users/myself/recentlyViewedWikis"))
        .and(query_param("offset", "10"))
        .and(query_param("count", "50"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let params = GetRecentlyViewedWikisParamsBuilder::default()
        .offset(10u32)
        .count(50u32)
        .build()
        .unwrap();
    let result = wiki_api.get_recently_viewed_wikis(params).await;

    assert!(result.is_ok());
    let wikis = result.unwrap();
    assert_eq!(wikis.len(), 1);
    assert_eq!(wikis[0].page.name, "Page 10");
}

#[tokio::test]
async fn test_get_recently_viewed_wikis_with_all_params() {
    let mock_server = MockServer::start().await;
    let wiki_api = setup_wiki_api(&mock_server).await;

    let response_body = json!([]);

    Mock::given(method("GET"))
        .and(path("/api/v2/users/myself/recentlyViewedWikis"))
        .and(query_param("order", "desc"))
        .and(query_param("offset", "100"))
        .and(query_param("count", "20"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let params = GetRecentlyViewedWikisParamsBuilder::default()
        .order("desc")
        .offset(100u32)
        .count(20u32)
        .build()
        .unwrap();
    let result = wiki_api.get_recently_viewed_wikis(params).await;

    assert!(result.is_ok());
    let wikis = result.unwrap();
    assert_eq!(wikis.len(), 0);
}

#[tokio::test]
async fn test_get_recently_viewed_wikis_server_error() {
    let mock_server = MockServer::start().await;
    let wiki_api = setup_wiki_api(&mock_server).await;

    Mock::given(method("GET"))
        .and(path("/api/v2/users/myself/recentlyViewedWikis"))
        .respond_with(ResponseTemplate::new(500).set_body_string("Internal Server Error"))
        .mount(&mock_server)
        .await;

    let params = GetRecentlyViewedWikisParamsBuilder::default()
        .build()
        .unwrap();
    let result = wiki_api.get_recently_viewed_wikis(params).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_recently_viewed_wikis_unauthorized() {
    let mock_server = MockServer::start().await;
    let wiki_api = setup_wiki_api(&mock_server).await;

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
        .and(path("/api/v2/users/myself/recentlyViewedWikis"))
        .respond_with(ResponseTemplate::new(401).set_body_json(&error_body))
        .mount(&mock_server)
        .await;

    let params = GetRecentlyViewedWikisParamsBuilder::default()
        .build()
        .unwrap();
    let result = wiki_api.get_recently_viewed_wikis(params).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_recently_viewed_wikis_empty_result() {
    let mock_server = MockServer::start().await;
    let wiki_api = setup_wiki_api(&mock_server).await;

    let response_body = json!([]);

    Mock::given(method("GET"))
        .and(path("/api/v2/users/myself/recentlyViewedWikis"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let params = GetRecentlyViewedWikisParamsBuilder::default()
        .build()
        .unwrap();
    let result = wiki_api.get_recently_viewed_wikis(params).await;

    assert!(result.is_ok());
    let wikis = result.unwrap();
    assert_eq!(wikis.len(), 0);
}
