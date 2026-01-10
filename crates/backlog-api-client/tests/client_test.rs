//! Tests for BacklogApiClient facade.
//!
//! This test module verifies:
//! - Client construction with valid/invalid URLs
//! - Builder pattern methods (with_api_key, with_auth_token)
//! - API accessor methods return correct types
//!
//! Deep integration tests are handled by individual domain crates.

mod common;

use backlog_api_client::client::BacklogApiClient;
use common::*;
use wiremock::MockServer;

// =============================================================================
// Constructor Tests
// =============================================================================

mod constructor_tests {
    use super::*;

    #[tokio::test]
    async fn test_new_with_valid_https_url_success() {
        let mock_server = MockServer::start().await;
        let result = BacklogApiClient::new(&mock_server.uri());
        assert!(
            result.is_ok(),
            "new() should succeed with valid mock server URL"
        );
    }

    #[tokio::test]
    async fn test_new_with_valid_http_url_success() {
        let result = BacklogApiClient::new("http://localhost:8080");
        assert!(result.is_ok(), "new() should succeed with valid http URL");
    }

    #[test]
    fn test_new_with_invalid_url_error() {
        let result = BacklogApiClient::new("not-a-valid-url");
        assert!(result.is_err(), "new() should fail with invalid URL");
        if let Err(err) = result {
            assert!(
                matches!(err, backlog_api_core::Error::Url(_)),
                "Error should be Url variant, got: {:?}",
                err
            );
        }
    }

    #[test]
    fn test_new_with_empty_url_error() {
        let result = BacklogApiClient::new("");
        assert!(result.is_err(), "new() should fail with empty URL");
        if let Err(err) = result {
            assert!(
                matches!(err, backlog_api_core::Error::Url(_)),
                "Error should be Url variant for empty URL"
            );
        }
    }

    #[test]
    fn test_new_with_missing_scheme_error() {
        let result = BacklogApiClient::new("example.backlog.jp");
        assert!(result.is_err(), "new() should fail without URL scheme");
        if let Err(err) = result {
            assert!(
                matches!(err, backlog_api_core::Error::Url(_)),
                "Error should be Url variant for missing scheme"
            );
        }
    }
}

// =============================================================================
// Builder Method Tests
// =============================================================================

mod builder_tests {
    use super::*;

    #[tokio::test]
    async fn test_with_api_key_builder_pattern() {
        let mock_server = MockServer::start().await;
        let client = BacklogApiClient::new(&mock_server.uri())
            .expect("Client creation should succeed")
            .with_api_key("test-api-key");

        // If we reach here, builder pattern works
        drop(client);
    }

    #[tokio::test]
    async fn test_with_auth_token_builder_pattern() {
        let mock_server = MockServer::start().await;
        let client = BacklogApiClient::new(&mock_server.uri())
            .expect("Client creation should succeed")
            .with_auth_token("oauth-bearer-token");

        drop(client);
    }

    #[tokio::test]
    async fn test_builder_methods_can_be_chained() {
        let mock_server = MockServer::start().await;
        // Note: In practice you'd use one or the other, but chaining should compile
        let client = BacklogApiClient::new(&mock_server.uri())
            .expect("Client creation should succeed")
            .with_api_key("api-key")
            .with_auth_token("token");

        drop(client);
    }

    #[tokio::test]
    async fn test_with_api_key_accepts_owned_string() {
        let mock_server = MockServer::start().await;
        let key = String::from("my-api-key");
        let client = BacklogApiClient::new(&mock_server.uri())
            .expect("Client creation should succeed")
            .with_api_key(key);

        drop(client);
    }
}

// =============================================================================
// API Accessor Tests (Feature-Gated)
// =============================================================================

#[cfg(feature = "issue")]
mod issue_accessor_tests {
    use super::*;

    #[tokio::test]
    async fn test_issue_accessor_returns_issue_api() {
        let mock_server = MockServer::start().await;
        let client = setup_api_client(&mock_server).await;

        let issue_api = client.issue();
        // Type assertion - if this compiles, the return type is correct
        let _: backlog_issue::IssueApi = issue_api;
    }

    #[tokio::test]
    async fn test_issue_accessor_multiple_calls_independent() {
        let mock_server = MockServer::start().await;
        let client = setup_api_client(&mock_server).await;

        let api1 = client.issue();
        let api2 = client.issue();

        // Both should be usable independently
        drop(api1);
        drop(api2);
    }
}

#[cfg(feature = "project")]
mod project_accessor_tests {
    use super::*;

    #[tokio::test]
    async fn test_project_accessor_returns_project_api() {
        let mock_server = MockServer::start().await;
        let client = setup_api_client(&mock_server).await;

        let project_api = client.project();
        let _: backlog_project::ProjectApi = project_api;
    }
}

#[cfg(feature = "space")]
mod space_accessor_tests {
    use super::*;

    #[tokio::test]
    async fn test_space_accessor_returns_space_api() {
        let mock_server = MockServer::start().await;
        let client = setup_api_client(&mock_server).await;

        let space_api = client.space();
        let _: backlog_space::SpaceApi = space_api;
    }
}

#[cfg(feature = "user")]
mod user_accessor_tests {
    use super::*;

    #[tokio::test]
    async fn test_user_accessor_returns_user_api() {
        let mock_server = MockServer::start().await;
        let client = setup_api_client(&mock_server).await;

        let user_api = client.user();
        let _: backlog_user::UserApi = user_api;
    }
}

#[cfg(feature = "document")]
mod document_accessor_tests {
    use super::*;

    #[tokio::test]
    async fn test_document_accessor_returns_document_api() {
        let mock_server = MockServer::start().await;
        let client = setup_api_client(&mock_server).await;

        let doc_api = client.document();
        let _: backlog_document::DocumentApi = doc_api;
    }
}

#[cfg(feature = "git")]
mod git_accessor_tests {
    use super::*;

    #[tokio::test]
    async fn test_git_accessor_returns_git_api() {
        let mock_server = MockServer::start().await;
        let client = setup_api_client(&mock_server).await;

        let git_api = client.git();
        let _: backlog_git::api::GitApi = git_api;
    }
}

#[cfg(feature = "file")]
mod file_accessor_tests {
    use super::*;

    #[tokio::test]
    async fn test_file_accessor_returns_file_api() {
        let mock_server = MockServer::start().await;
        let client = setup_api_client(&mock_server).await;

        let file_api = client.file();
        let _: backlog_file::FileApi = file_api;
    }
}

#[cfg(feature = "wiki")]
mod wiki_accessor_tests {
    use super::*;

    #[tokio::test]
    async fn test_wiki_accessor_returns_wiki_api() {
        let mock_server = MockServer::start().await;
        let client = setup_api_client(&mock_server).await;

        let wiki_api = client.wiki();
        let _: backlog_wiki::WikiApi = wiki_api;
    }
}

#[cfg(feature = "activity")]
mod activity_accessor_tests {
    use super::*;

    #[tokio::test]
    async fn test_activity_accessor_returns_activity_api() {
        let mock_server = MockServer::start().await;
        let client = setup_api_client(&mock_server).await;

        let activity_api = client.activity();
        let _: backlog_activity::ActivityApi = activity_api;
    }
}

#[cfg(feature = "team")]
mod team_accessor_tests {
    use super::*;

    #[tokio::test]
    async fn test_team_accessor_returns_team_api() {
        let mock_server = MockServer::start().await;
        let client = setup_api_client(&mock_server).await;

        let team_api = client.team();
        let _: backlog_team::TeamApi = team_api;
    }
}

#[cfg(feature = "star")]
mod star_accessor_tests {
    use super::*;

    #[tokio::test]
    async fn test_star_accessor_returns_star_api() {
        let mock_server = MockServer::start().await;
        let client = setup_api_client(&mock_server).await;

        let star_api = client.star();
        let _: backlog_star::StarApi = star_api;
    }
}

#[cfg(feature = "rate-limit")]
mod rate_limit_accessor_tests {
    use super::*;

    #[tokio::test]
    async fn test_rate_limit_accessor_returns_rate_limit_api() {
        let mock_server = MockServer::start().await;
        let client = setup_api_client(&mock_server).await;

        let rate_limit_api = client.rate_limit();
        let _: backlog_rate_limit::RateLimitApi = rate_limit_api;
    }
}

#[cfg(feature = "watching")]
mod watching_accessor_tests {
    use super::*;

    #[tokio::test]
    async fn test_watching_accessor_returns_watching_api() {
        let mock_server = MockServer::start().await;
        let client = setup_api_client(&mock_server).await;

        let watching_api = client.watching();
        let _: backlog_watching::WatchingApi = watching_api;
    }
}

#[cfg(feature = "webhook")]
mod webhook_accessor_tests {
    use super::*;

    #[tokio::test]
    async fn test_webhook_accessor_returns_webhook_api() {
        let mock_server = MockServer::start().await;
        let client = setup_api_client(&mock_server).await;

        let webhook_api = client.webhook();
        let _: backlog_webhook::WebhookApi = webhook_api;
    }
}

// =============================================================================
// Integration Smoke Tests
// =============================================================================

#[cfg(feature = "rate-limit")]
mod smoke_tests {
    use super::*;

    #[tokio::test]
    async fn test_rate_limit_api_smoke_test() {
        let mock_server = MockServer::start().await;
        let client = setup_api_client(&mock_server).await;

        let rate_limit_response = json!({
            "rateLimit": {
                "read": { "limit": 600, "remaining": 599, "reset": 1234567890 },
                "update": { "limit": 150, "remaining": 150, "reset": 1234567890 },
                "search": { "limit": 150, "remaining": 150, "reset": 1234567890 },
                "icon": { "limit": 60, "remaining": 60, "reset": 1234567890 }
            }
        });

        Mock::given(method("GET"))
            .and(path("/api/v2/rateLimit"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&rate_limit_response))
            .mount(&mock_server)
            .await;

        let rate_limit_api = client.rate_limit();
        let result = rate_limit_api.get_rate_limit().await;

        assert!(
            result.is_ok(),
            "Smoke test should succeed through full client stack"
        );

        let response = result.expect("get_rate_limit should succeed");
        assert_eq!(
            response.rate_limit.read.limit, 600,
            "Response should be parsed correctly"
        );
    }

    #[tokio::test]
    async fn test_rate_limit_api_error_propagation() {
        let mock_server = MockServer::start().await;
        let client = setup_api_client(&mock_server).await;

        let error_response = json!({
            "errors": [
                {
                    "message": "Authentication failure",
                    "code": 1,
                    "moreInfo": ""
                }
            ]
        });

        Mock::given(method("GET"))
            .and(path("/api/v2/rateLimit"))
            .respond_with(ResponseTemplate::new(401).set_body_json(&error_response))
            .mount(&mock_server)
            .await;

        let rate_limit_api = client.rate_limit();
        let result = rate_limit_api.get_rate_limit().await;

        let err = result.expect_err("Should return 401 error");
        assert!(
            matches!(err, backlog_api_core::Error::HttpStatus { status: 401, .. }),
            "Error should propagate HTTP status correctly"
        );
    }
}
