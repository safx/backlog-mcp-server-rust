use backlog_api_core::{HttpMethod, IntoRequest};
use client::Client;
use serde::{Deserialize, Serialize};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{header, method, path, query_param},
};

// ============================================================================
// Test Infrastructure
// ============================================================================

/// Test request struct implementing IntoRequest for testing purposes
#[derive(Debug, Clone)]
struct TestRequest {
    path_str: String,
    http_method: HttpMethod,
    query_params: Vec<(String, String)>,
    form_params: Vec<(String, String)>,
}

impl TestRequest {
    fn get(path: impl Into<String>) -> Self {
        Self {
            path_str: path.into(),
            http_method: HttpMethod::Get,
            query_params: Vec::new(),
            form_params: Vec::new(),
        }
    }

    fn post(path: impl Into<String>) -> Self {
        Self {
            path_str: path.into(),
            http_method: HttpMethod::Post,
            query_params: Vec::new(),
            form_params: Vec::new(),
        }
    }

    fn put(path: impl Into<String>) -> Self {
        Self {
            path_str: path.into(),
            http_method: HttpMethod::Put,
            query_params: Vec::new(),
            form_params: Vec::new(),
        }
    }

    fn patch(path: impl Into<String>) -> Self {
        Self {
            path_str: path.into(),
            http_method: HttpMethod::Patch,
            query_params: Vec::new(),
            form_params: Vec::new(),
        }
    }

    fn delete(path: impl Into<String>) -> Self {
        Self {
            path_str: path.into(),
            http_method: HttpMethod::Delete,
            query_params: Vec::new(),
            form_params: Vec::new(),
        }
    }

    #[allow(dead_code)]
    fn with_query(mut self, params: Vec<(&str, &str)>) -> Self {
        self.query_params = params
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        self
    }

    #[allow(dead_code)]
    fn with_form(mut self, params: Vec<(&str, &str)>) -> Self {
        self.form_params = params
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        self
    }
}

impl IntoRequest for TestRequest {
    fn method(&self) -> HttpMethod {
        self.http_method
    }

    fn path(&self) -> String {
        self.path_str.clone()
    }

    fn to_query(&self) -> impl Serialize {
        self.query_params.clone()
    }

    fn to_form(&self) -> impl Serialize {
        self.form_params.clone()
    }
}

/// Test response struct for JSON deserialization
#[derive(Debug, Deserialize, PartialEq)]
struct TestResponse {
    id: u32,
    name: String,
    #[serde(default)]
    value: Option<String>,
}

// ============================================================================
// Phase 2: execute() Success Tests
// ============================================================================

#[tokio::test]
async fn test_execute_get_success() {
    let server = MockServer::start().await;
    let client = Client::new(&server.uri()).expect("Client::new should succeed");

    let response_body = serde_json::json!({
        "id": 123,
        "name": "test_entity",
        "value": "some_value"
    });

    Mock::given(method("GET"))
        .and(path("/api/v2/test/123"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&server)
        .await;

    let params = TestRequest::get("/api/v2/test/123");
    let result: Result<TestResponse, _> = client.execute(params).await;

    let response = result.expect("execute should succeed for valid GET request");
    assert_eq!(response.id, 123);
    assert_eq!(response.name, "test_entity");
    assert_eq!(response.value, Some("some_value".to_string()));
}

#[tokio::test]
async fn test_execute_post_success() {
    let server = MockServer::start().await;
    let client = Client::new(&server.uri()).expect("Client::new should succeed");

    let response_body = serde_json::json!({
        "id": 456,
        "name": "created_entity",
        "value": null
    });

    Mock::given(method("POST"))
        .and(path("/api/v2/test"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&server)
        .await;

    let params = TestRequest::post("/api/v2/test")
        .with_form(vec![("field1", "value1"), ("field2", "value2")]);
    let result: Result<TestResponse, _> = client.execute(params).await;

    let response = result.expect("execute should succeed for valid POST request");
    assert_eq!(response.id, 456);
    assert_eq!(response.name, "created_entity");
    assert_eq!(response.value, None);
}

#[tokio::test]
async fn test_execute_put_success() {
    let server = MockServer::start().await;
    let client = Client::new(&server.uri()).expect("Client::new should succeed");

    let response_body = serde_json::json!({
        "id": 789,
        "name": "updated_entity",
        "value": "updated_value"
    });

    Mock::given(method("PUT"))
        .and(path("/api/v2/test/789"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&server)
        .await;

    let params = TestRequest::put("/api/v2/test/789");
    let result: Result<TestResponse, _> = client.execute(params).await;

    let response = result.expect("execute should succeed for valid PUT request");
    assert_eq!(response.id, 789);
    assert_eq!(response.name, "updated_entity");
}

#[tokio::test]
async fn test_execute_patch_success() {
    let server = MockServer::start().await;
    let client = Client::new(&server.uri()).expect("Client::new should succeed");

    let response_body = serde_json::json!({
        "id": 321,
        "name": "patched_entity",
        "value": null
    });

    Mock::given(method("PATCH"))
        .and(path("/api/v2/test/321"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&server)
        .await;

    let params = TestRequest::patch("/api/v2/test/321");
    let result: Result<TestResponse, _> = client.execute(params).await;

    let response = result.expect("execute should succeed for valid PATCH request");
    assert_eq!(response.id, 321);
    assert_eq!(response.name, "patched_entity");
}

#[tokio::test]
async fn test_execute_delete_success() {
    let server = MockServer::start().await;
    let client = Client::new(&server.uri()).expect("Client::new should succeed");

    let response_body = serde_json::json!({
        "id": 654,
        "name": "deleted_entity",
        "value": "deletion_confirmed"
    });

    Mock::given(method("DELETE"))
        .and(path("/api/v2/test/654"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&server)
        .await;

    let params = TestRequest::delete("/api/v2/test/654");
    let result: Result<TestResponse, _> = client.execute(params).await;

    let response = result.expect("execute should succeed for valid DELETE request");
    assert_eq!(response.id, 654);
    assert_eq!(response.name, "deleted_entity");
}

// ============================================================================
// Phase 3: execute() HTTP Error Tests
// ============================================================================

#[tokio::test]
async fn test_execute_400_bad_request() {
    let server = MockServer::start().await;
    let client = Client::new(&server.uri()).expect("Client::new should succeed");

    let error_response = serde_json::json!({
        "errors": [{
            "message": "Invalid request parameters",
            "code": 1,
            "moreInfo": ""
        }]
    });

    Mock::given(method("GET"))
        .and(path("/api/v2/bad_request"))
        .respond_with(ResponseTemplate::new(400).set_body_json(&error_response))
        .mount(&server)
        .await;

    let params = TestRequest::get("/api/v2/bad_request");
    let result: Result<TestResponse, _> = client.execute(params).await;

    let err = result.expect_err("execute should fail with 400 Bad Request");
    assert!(matches!(
        err,
        backlog_api_core::Error::HttpStatus { status: 400, .. }
    ));
}

#[tokio::test]
async fn test_execute_401_unauthorized() {
    let server = MockServer::start().await;
    let client = Client::new(&server.uri()).expect("Client::new should succeed");

    let error_response = serde_json::json!({
        "errors": [{
            "message": "Authentication required",
            "code": 1,
            "moreInfo": ""
        }]
    });

    Mock::given(method("GET"))
        .and(path("/api/v2/protected"))
        .respond_with(ResponseTemplate::new(401).set_body_json(&error_response))
        .mount(&server)
        .await;

    let params = TestRequest::get("/api/v2/protected");
    let result: Result<TestResponse, _> = client.execute(params).await;

    let err = result.expect_err("execute should fail with 401 Unauthorized");
    assert!(matches!(
        err,
        backlog_api_core::Error::HttpStatus { status: 401, .. }
    ));
}

#[tokio::test]
async fn test_execute_403_forbidden() {
    let server = MockServer::start().await;
    let client = Client::new(&server.uri()).expect("Client::new should succeed");

    let error_response = serde_json::json!({
        "errors": [{
            "message": "Access denied",
            "code": 1,
            "moreInfo": ""
        }]
    });

    Mock::given(method("GET"))
        .and(path("/api/v2/forbidden"))
        .respond_with(ResponseTemplate::new(403).set_body_json(&error_response))
        .mount(&server)
        .await;

    let params = TestRequest::get("/api/v2/forbidden");
    let result: Result<TestResponse, _> = client.execute(params).await;

    let err = result.expect_err("execute should fail with 403 Forbidden");
    assert!(matches!(
        err,
        backlog_api_core::Error::HttpStatus { status: 403, .. }
    ));
}

#[tokio::test]
async fn test_execute_404_not_found() {
    let server = MockServer::start().await;
    let client = Client::new(&server.uri()).expect("Client::new should succeed");

    let error_response = serde_json::json!({
        "errors": [{
            "message": "Resource not found",
            "code": 1,
            "moreInfo": ""
        }]
    });

    Mock::given(method("GET"))
        .and(path("/api/v2/not_found"))
        .respond_with(ResponseTemplate::new(404).set_body_json(&error_response))
        .mount(&server)
        .await;

    let params = TestRequest::get("/api/v2/not_found");
    let result: Result<TestResponse, _> = client.execute(params).await;

    let err = result.expect_err("execute should fail with 404 Not Found");
    assert!(matches!(
        err,
        backlog_api_core::Error::HttpStatus { status: 404, .. }
    ));
}

#[tokio::test]
async fn test_execute_500_server_error() {
    let server = MockServer::start().await;
    let client = Client::new(&server.uri()).expect("Client::new should succeed");

    let error_response = serde_json::json!({
        "errors": [{
            "message": "Internal server error",
            "code": 1,
            "moreInfo": ""
        }]
    });

    Mock::given(method("GET"))
        .and(path("/api/v2/server_error"))
        .respond_with(ResponseTemplate::new(500).set_body_json(&error_response))
        .mount(&server)
        .await;

    let params = TestRequest::get("/api/v2/server_error");
    let result: Result<TestResponse, _> = client.execute(params).await;

    let err = result.expect_err("execute should fail with 500 Internal Server Error");
    assert!(matches!(
        err,
        backlog_api_core::Error::HttpStatus { status: 500, .. }
    ));
}

#[tokio::test]
async fn test_execute_unparseable_error_response() {
    let server = MockServer::start().await;
    let client = Client::new(&server.uri()).expect("Client::new should succeed");

    Mock::given(method("GET"))
        .and(path("/api/v2/broken"))
        .respond_with(
            ResponseTemplate::new(500).set_body_string("Internal Server Error - not JSON"),
        )
        .mount(&server)
        .await;

    let params = TestRequest::get("/api/v2/broken");
    let result: Result<TestResponse, _> = client.execute(params).await;

    let err = result.expect_err("execute should fail with unparseable error response");
    assert!(matches!(
        err,
        backlog_api_core::Error::UnparseableErrorResponse { status: 500, .. }
    ));
}

// ============================================================================
// Phase 4: Authentication Tests
// ============================================================================

#[tokio::test]
async fn test_with_auth_token_adds_bearer_header() {
    let server = MockServer::start().await;
    let client = Client::new(&server.uri())
        .expect("Client::new should succeed")
        .with_auth_token("test_token_12345");

    let response_body = serde_json::json!({
        "id": 1,
        "name": "authenticated_resource",
        "value": null
    });

    Mock::given(method("GET"))
        .and(path("/api/v2/authenticated"))
        .and(header("Authorization", "Bearer test_token_12345"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&server)
        .await;

    let params = TestRequest::get("/api/v2/authenticated");
    let result: Result<TestResponse, _> = client.execute(params).await;

    let response = result.expect("execute with auth token should succeed when header matches");
    assert_eq!(response.id, 1);
    assert_eq!(response.name, "authenticated_resource");
}

#[tokio::test]
async fn test_auth_token_on_post_request() {
    let server = MockServer::start().await;
    let client = Client::new(&server.uri())
        .expect("Client::new should succeed")
        .with_auth_token("post_token_67890");

    let response_body = serde_json::json!({
        "id": 10,
        "name": "created_with_auth",
        "value": null
    });

    Mock::given(method("POST"))
        .and(path("/api/v2/create"))
        .and(header("Authorization", "Bearer post_token_67890"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&server)
        .await;

    let params = TestRequest::post("/api/v2/create");
    let result: Result<TestResponse, _> = client.execute(params).await;

    let response = result.expect("POST with auth token should succeed");
    assert_eq!(response.id, 10);
}

#[tokio::test]
async fn test_missing_auth_returns_401() {
    let server = MockServer::start().await;
    let client = Client::new(&server.uri()).expect("Client::new should succeed");

    let error_response = serde_json::json!({
        "errors": [{
            "message": "Authentication required",
            "code": 1,
            "moreInfo": ""
        }]
    });

    Mock::given(method("GET"))
        .and(path("/api/v2/requires_auth"))
        .respond_with(ResponseTemplate::new(401).set_body_json(&error_response))
        .mount(&server)
        .await;

    let params = TestRequest::get("/api/v2/requires_auth");
    let result: Result<TestResponse, _> = client.execute(params).await;

    let err = result.expect_err("execute without auth should fail when auth is required");
    assert!(matches!(
        err,
        backlog_api_core::Error::HttpStatus { status: 401, .. }
    ));
}

#[tokio::test]
async fn test_with_api_key_adds_query_param() {
    let server = MockServer::start().await;
    let client = Client::new(&server.uri())
        .expect("Client::new should succeed")
        .with_api_key("api_key_abc123");

    let response_body = serde_json::json!({
        "id": 2,
        "name": "api_key_resource",
        "value": null
    });

    Mock::given(method("GET"))
        .and(path("/api/v2/resource"))
        .and(query_param("apiKey", "api_key_abc123"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&server)
        .await;

    let params = TestRequest::get("/api/v2/resource");
    let result: Result<TestResponse, _> = client.execute(params).await;

    let response = result.expect("execute with api key should succeed when query param matches");
    assert_eq!(response.id, 2);
    assert_eq!(response.name, "api_key_resource");
}

#[tokio::test]
async fn test_api_key_on_post_request() {
    let server = MockServer::start().await;
    let client = Client::new(&server.uri())
        .expect("Client::new should succeed")
        .with_api_key("post_api_key_xyz");

    let response_body = serde_json::json!({
        "id": 20,
        "name": "created_with_api_key",
        "value": null
    });

    Mock::given(method("POST"))
        .and(path("/api/v2/create_with_key"))
        .and(query_param("apiKey", "post_api_key_xyz"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&server)
        .await;

    let params = TestRequest::post("/api/v2/create_with_key");
    let result: Result<TestResponse, _> = client.execute(params).await;

    let response = result.expect("POST with api key should succeed");
    assert_eq!(response.id, 20);
}

#[tokio::test]
async fn test_with_both_auth_methods() {
    let server = MockServer::start().await;
    let client = Client::new(&server.uri())
        .expect("Client::new should succeed")
        .with_auth_token("bearer_token")
        .with_api_key("api_key");

    let response_body = serde_json::json!({
        "id": 3,
        "name": "dual_auth_resource",
        "value": null
    });

    Mock::given(method("GET"))
        .and(path("/api/v2/dual_auth"))
        .and(header("Authorization", "Bearer bearer_token"))
        .and(query_param("apiKey", "api_key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&server)
        .await;

    let params = TestRequest::get("/api/v2/dual_auth");
    let result: Result<TestResponse, _> = client.execute(params).await;

    let response = result.expect("execute with both auth methods should succeed");
    assert_eq!(response.id, 3);
    assert_eq!(response.name, "dual_auth_resource");
}
