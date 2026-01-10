mod common;

use backlog_core::{
    ProjectIdOrKey, ProjectKey,
    id::{ProjectId, UserId, WebhookId},
};
use backlog_webhook::{GetWebhookListParams, GetWebhookParams, Webhook, WebhookApi};
use common::*;
use wiremock::{Mock, ResponseTemplate, matchers};

#[tokio::test]
async fn test_get_webhook_list_params_path() {
    use backlog_api_core::IntoRequest;

    let params = GetWebhookListParams {
        project_id_or_key: ProjectIdOrKey::from(
            "TEST"
                .parse::<ProjectKey>()
                .expect("TEST is a valid project key"),
        ),
    };
    assert_eq!(params.path(), "/api/v2/projects/TEST/webhooks");

    let params_with_id = GetWebhookListParams {
        project_id_or_key: ProjectIdOrKey::from(ProjectId::new(123)),
    };
    assert_eq!(params_with_id.path(), "/api/v2/projects/123/webhooks");
}

#[tokio::test]
async fn test_get_webhook_list_params_query() {
    use backlog_api_core::IntoRequest;
    use backlog_core::{ProjectIdOrKey, ProjectKey};

    let params = GetWebhookListParams {
        project_id_or_key: ProjectIdOrKey::from(
            "TEST"
                .parse::<ProjectKey>()
                .expect("TEST is a valid project key"),
        ),
    };

    // Query should be empty
    let _query = params.to_query();
}

#[tokio::test]
async fn test_get_webhook_list_success() {
    let mock_server = setup_mock_server().await;
    let response_body = mock_webhook_list_response();

    Mock::given(matchers::method("GET"))
        .and(matchers::path("/api/v2/projects/TEST/webhooks"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let client = client::Client::new(&mock_server.uri())
        .expect("mock server URI should be valid")
        .with_api_key("test-api-key");
    let api = WebhookApi::new(client);

    let result = api
        .get_webhook_list(
            "TEST"
                .parse::<ProjectKey>()
                .expect("TEST is a valid project key"),
        )
        .await;
    assert!(result.is_ok());

    let webhooks = result.expect("get_webhook_list should return webhooks");
    assert_eq!(webhooks.len(), 2);

    let webhook1 = &webhooks[0];
    assert_eq!(webhook1.id, 1);
    assert_eq!(webhook1.name, "webhook1");
    assert_eq!(webhook1.description, "test webhook 1");
    assert_eq!(webhook1.hook_url, "http://example.com/webhook1");
    assert!(!webhook1.all_event);
    assert_eq!(webhook1.activity_type_ids, vec![1, 2, 3, 4, 5]);

    let webhook2 = &webhooks[1];
    assert_eq!(webhook2.id, 2);
    assert_eq!(webhook2.name, "webhook2");
    assert_eq!(webhook2.description, "test webhook 2");
    assert_eq!(webhook2.hook_url, "http://example.com/webhook2");
    assert!(webhook2.all_event);
    assert!(webhook2.activity_type_ids.is_empty());
}

#[tokio::test]
async fn test_get_webhook_list_with_project_id() {
    let mock_server = setup_mock_server().await;
    let response_body = mock_webhook_list_response();

    Mock::given(matchers::method("GET"))
        .and(matchers::path("/api/v2/projects/123/webhooks"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let client = client::Client::new(&mock_server.uri())
        .expect("mock server URI should be valid")
        .with_api_key("test-api-key");
    let api = WebhookApi::new(client);

    let result = api.get_webhook_list(ProjectId::new(123)).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_webhook_list_empty() {
    let mock_server = setup_mock_server().await;
    let response_body = mock_empty_webhook_list_response();

    Mock::given(matchers::method("GET"))
        .and(matchers::path("/api/v2/projects/EMPTY/webhooks"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let client = client::Client::new(&mock_server.uri())
        .expect("mock server URI should be valid")
        .with_api_key("test-api-key");
    let api = WebhookApi::new(client);

    let result = api
        .get_webhook_list(
            "EMPTY"
                .parse::<ProjectKey>()
                .expect("EMPTY is a valid project key"),
        )
        .await;
    assert!(result.is_ok());

    let webhooks = result.expect("get_webhook_list should return empty list");
    assert!(webhooks.is_empty());
}

#[tokio::test]
async fn test_get_webhook_list_error() {
    let mock_server = setup_mock_server().await;
    let response_body = mock_error_response();

    Mock::given(matchers::method("GET"))
        .and(matchers::path("/api/v2/projects/INVALID/webhooks"))
        .respond_with(ResponseTemplate::new(404).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let client = client::Client::new(&mock_server.uri())
        .expect("mock server URI should be valid")
        .with_api_key("test-api-key");
    let api = WebhookApi::new(client);

    let result = api
        .get_webhook_list(
            "INVALID"
                .parse::<ProjectKey>()
                .expect("INVALID is a valid project key"),
        )
        .await;
    assert!(result.is_err());
}

#[test]
fn test_webhook_model_fields() {
    let json = serde_json::json!({
        "id": 1,
        "name": "test webhook",
        "description": "test description",
        "hookUrl": "https://example.com/webhook",
        "allEvent": true,
        "activityTypeIds": [1, 2, 3],
        "createdUser": {
            "id": 1,
            "userId": "admin",
            "name": "admin",
            "roleType": 1,
            "lang": "ja",
            "mailAddress": "admin@example.com"
        },
        "created": "2023-01-01T00:00:00Z",
        "updatedUser": {
            "id": 1,
            "userId": "admin",
            "name": "admin",
            "roleType": 1,
            "lang": "ja",
            "mailAddress": "admin@example.com"
        },
        "updated": "2023-01-01T00:00:00Z"
    });

    let webhook: Webhook =
        serde_json::from_value(json).expect("JSON should deserialize to Webhook");
    assert_eq!(webhook.id, 1);
    assert_eq!(webhook.name, "test webhook");
    assert_eq!(webhook.description, "test description");
    assert_eq!(webhook.hook_url, "https://example.com/webhook");
    assert!(webhook.all_event);
    assert_eq!(webhook.activity_type_ids, vec![1, 2, 3]);
    assert_eq!(webhook.created_user.id, UserId::new(1));
    assert_eq!(webhook.updated_user.id, UserId::new(1));
}

#[tokio::test]
async fn test_get_webhook_params_path() {
    use backlog_api_core::IntoRequest;

    let params = GetWebhookParams {
        project_id_or_key: ProjectIdOrKey::from(
            "TEST"
                .parse::<ProjectKey>()
                .expect("TEST is a valid project key"),
        ),
        webhook_id: WebhookId::new(1),
    };
    assert_eq!(params.path(), "/api/v2/projects/TEST/webhooks/1");

    let params_with_id = GetWebhookParams {
        project_id_or_key: ProjectIdOrKey::from(ProjectId::new(123)),
        webhook_id: WebhookId::new(456),
    };
    assert_eq!(params_with_id.path(), "/api/v2/projects/123/webhooks/456");
}

#[tokio::test]
async fn test_get_webhook_success() {
    let mock_server = setup_mock_server().await;
    let response_body = mock_single_webhook_response();

    Mock::given(matchers::method("GET"))
        .and(matchers::path("/api/v2/projects/TEST/webhooks/1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let client = client::Client::new(&mock_server.uri())
        .expect("mock server URI should be valid")
        .with_api_key("test-api-key");
    let api = WebhookApi::new(client);

    let result = api
        .get_webhook(
            "TEST"
                .parse::<ProjectKey>()
                .expect("TEST is a valid project key"),
            WebhookId::new(1),
        )
        .await;
    assert!(result.is_ok());

    let webhook = result.expect("get_webhook should return webhook");
    assert_eq!(webhook.id, 1);
    assert_eq!(webhook.name, "webhook1");
    assert_eq!(webhook.description, "test webhook 1");
    assert_eq!(webhook.hook_url, "http://example.com/webhook1");
    assert!(!webhook.all_event);
    assert_eq!(webhook.activity_type_ids, vec![1, 2, 3, 4, 5]);
}

#[tokio::test]
async fn test_get_webhook_with_project_id() {
    let mock_server = setup_mock_server().await;
    let response_body = mock_single_webhook_response();

    Mock::given(matchers::method("GET"))
        .and(matchers::path("/api/v2/projects/123/webhooks/456"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let client = client::Client::new(&mock_server.uri())
        .expect("mock server URI should be valid")
        .with_api_key("test-api-key");
    let api = WebhookApi::new(client);

    let result = api
        .get_webhook(ProjectId::new(123), WebhookId::new(456))
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_webhook_not_found() {
    let mock_server = setup_mock_server().await;
    let response_body = mock_error_response();

    Mock::given(matchers::method("GET"))
        .and(matchers::path("/api/v2/projects/TEST/webhooks/999"))
        .respond_with(ResponseTemplate::new(404).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let client = client::Client::new(&mock_server.uri())
        .expect("mock server URI should be valid")
        .with_api_key("test-api-key");
    let api = WebhookApi::new(client);

    let result = api
        .get_webhook(
            "TEST"
                .parse::<ProjectKey>()
                .expect("TEST is a valid project key"),
            WebhookId::new(999),
        )
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_webhook_list_unauthorized() {
    let mock_server = setup_mock_server().await;
    let error_response = serde_json::json!({
        "errors": [{
            "message": "Authentication required",
            "code": 1,
            "moreInfo": ""
        }]
    });

    Mock::given(matchers::method("GET"))
        .and(matchers::path("/api/v2/projects/TEST/webhooks"))
        .respond_with(ResponseTemplate::new(401).set_body_json(&error_response))
        .mount(&mock_server)
        .await;

    let client = client::Client::new(&mock_server.uri())
        .expect("mock server URI should be valid")
        .with_api_key("test-api-key");
    let api = WebhookApi::new(client);

    let result = api
        .get_webhook_list(
            "TEST"
                .parse::<ProjectKey>()
                .expect("TEST is a valid project key"),
        )
        .await;

    let err = result.expect_err("should return 401 error");
    assert!(matches!(
        err,
        backlog_api_core::Error::HttpStatus { status: 401, .. }
    ));
}

#[tokio::test]
async fn test_get_webhook_list_forbidden() {
    let mock_server = setup_mock_server().await;
    let error_response = serde_json::json!({
        "errors": [{
            "message": "You do not have permission to view webhooks",
            "code": 11,
            "moreInfo": ""
        }]
    });

    Mock::given(matchers::method("GET"))
        .and(matchers::path("/api/v2/projects/TEST/webhooks"))
        .respond_with(ResponseTemplate::new(403).set_body_json(&error_response))
        .mount(&mock_server)
        .await;

    let client = client::Client::new(&mock_server.uri())
        .expect("mock server URI should be valid")
        .with_api_key("test-api-key");
    let api = WebhookApi::new(client);

    let result = api
        .get_webhook_list(
            "TEST"
                .parse::<ProjectKey>()
                .expect("TEST is a valid project key"),
        )
        .await;

    let err = result.expect_err("should return 403 error");
    assert!(matches!(
        err,
        backlog_api_core::Error::HttpStatus { status: 403, .. }
    ));
}

#[tokio::test]
async fn test_get_webhook_list_server_error() {
    let mock_server = setup_mock_server().await;
    let error_response = serde_json::json!({
        "errors": [{
            "message": "Internal server error",
            "code": 0,
            "moreInfo": ""
        }]
    });

    Mock::given(matchers::method("GET"))
        .and(matchers::path("/api/v2/projects/TEST/webhooks"))
        .respond_with(ResponseTemplate::new(500).set_body_json(&error_response))
        .mount(&mock_server)
        .await;

    let client = client::Client::new(&mock_server.uri())
        .expect("mock server URI should be valid")
        .with_api_key("test-api-key");
    let api = WebhookApi::new(client);

    let result = api
        .get_webhook_list(
            "TEST"
                .parse::<ProjectKey>()
                .expect("TEST is a valid project key"),
        )
        .await;

    let err = result.expect_err("should return 500 error");
    assert!(matches!(
        err,
        backlog_api_core::Error::HttpStatus { status: 500, .. }
    ));
}

#[tokio::test]
async fn test_get_webhook_unauthorized() {
    let mock_server = setup_mock_server().await;
    let error_response = serde_json::json!({
        "errors": [{
            "message": "Authentication required",
            "code": 1,
            "moreInfo": ""
        }]
    });

    Mock::given(matchers::method("GET"))
        .and(matchers::path("/api/v2/projects/TEST/webhooks/1"))
        .respond_with(ResponseTemplate::new(401).set_body_json(&error_response))
        .mount(&mock_server)
        .await;

    let client = client::Client::new(&mock_server.uri())
        .expect("mock server URI should be valid")
        .with_api_key("test-api-key");
    let api = WebhookApi::new(client);

    let result = api
        .get_webhook(
            "TEST"
                .parse::<ProjectKey>()
                .expect("TEST is a valid project key"),
            WebhookId::new(1),
        )
        .await;

    let err = result.expect_err("should return 401 error");
    assert!(matches!(
        err,
        backlog_api_core::Error::HttpStatus { status: 401, .. }
    ));
}

#[tokio::test]
async fn test_get_webhook_forbidden() {
    let mock_server = setup_mock_server().await;
    let error_response = serde_json::json!({
        "errors": [{
            "message": "You do not have permission to view this webhook",
            "code": 11,
            "moreInfo": ""
        }]
    });

    Mock::given(matchers::method("GET"))
        .and(matchers::path("/api/v2/projects/TEST/webhooks/1"))
        .respond_with(ResponseTemplate::new(403).set_body_json(&error_response))
        .mount(&mock_server)
        .await;

    let client = client::Client::new(&mock_server.uri())
        .expect("mock server URI should be valid")
        .with_api_key("test-api-key");
    let api = WebhookApi::new(client);

    let result = api
        .get_webhook(
            "TEST"
                .parse::<ProjectKey>()
                .expect("TEST is a valid project key"),
            WebhookId::new(1),
        )
        .await;

    let err = result.expect_err("should return 403 error");
    assert!(matches!(
        err,
        backlog_api_core::Error::HttpStatus { status: 403, .. }
    ));
}

#[tokio::test]
async fn test_get_webhook_server_error() {
    let mock_server = setup_mock_server().await;
    let error_response = serde_json::json!({
        "errors": [{
            "message": "Internal server error",
            "code": 0,
            "moreInfo": ""
        }]
    });

    Mock::given(matchers::method("GET"))
        .and(matchers::path("/api/v2/projects/TEST/webhooks/1"))
        .respond_with(ResponseTemplate::new(500).set_body_json(&error_response))
        .mount(&mock_server)
        .await;

    let client = client::Client::new(&mock_server.uri())
        .expect("mock server URI should be valid")
        .with_api_key("test-api-key");
    let api = WebhookApi::new(client);

    let result = api
        .get_webhook(
            "TEST"
                .parse::<ProjectKey>()
                .expect("TEST is a valid project key"),
            WebhookId::new(1),
        )
        .await;

    let err = result.expect_err("should return 500 error");
    assert!(matches!(
        err,
        backlog_api_core::Error::HttpStatus { status: 500, .. }
    ));
}
