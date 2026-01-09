mod common;

#[cfg(feature = "writable")]
mod tests {
    use crate::common::*;
    use backlog_star::AddStarParams;
    use wiremock::{matchers::*, MockServer};

    #[tokio::test]
    async fn test_add_star_to_issue_success() {
        let mock_server = MockServer::start().await;
        let api = setup_star_api(&mock_server).await;

        Mock::given(method("POST"))
            .and(path("/api/v2/stars"))
            .and(body_string_contains("issueId=123"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let params = AddStarParams::issue(123u32);
        let result = api.add_star(params).await;

        match result {
            Ok(_) => {}
            Err(e) => panic!("Expected Ok, got Err: {e}"),
        }
    }

    #[tokio::test]
    async fn test_add_star_to_comment_success() {
        let mock_server = MockServer::start().await;
        let api = setup_star_api(&mock_server).await;

        Mock::given(method("POST"))
            .and(path("/api/v2/stars"))
            .and(body_string_contains("commentId=456"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let params = AddStarParams::comment(123u32, 456u32);
        let result = api.add_star(params).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_add_star_to_wiki_success() {
        let mock_server = MockServer::start().await;
        let api = setup_star_api(&mock_server).await;

        Mock::given(method("POST"))
            .and(path("/api/v2/stars"))
            .and(body_string_contains("wikiId=789"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let params = AddStarParams::wiki(789u32);
        let result = api.add_star(params).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_add_star_to_pull_request_success() {
        let mock_server = MockServer::start().await;
        let api = setup_star_api(&mock_server).await;

        Mock::given(method("POST"))
            .and(path("/api/v2/stars"))
            .and(body_string_contains("pullRequestId=10"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let params = AddStarParams::pull_request(10);
        let result = api.add_star(params).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_add_star_to_pull_request_comment_success() {
        let mock_server = MockServer::start().await;
        let api = setup_star_api(&mock_server).await;

        Mock::given(method("POST"))
            .and(path("/api/v2/stars"))
            .and(body_string_contains("pullRequestCommentId=11"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let params = AddStarParams::pull_request_comment(11u32);
        let result = api.add_star(params).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_add_star_already_exists_error() {
        let mock_server = MockServer::start().await;
        let api = setup_star_api(&mock_server).await;

        let error_response = r#"{
            "errors": [
                {
                    "message": "You have already added a star.",
                    "code": 17,
                    "moreInfo": ""
                }
            ]
        }"#;

        Mock::given(method("POST"))
            .and(path("/api/v2/stars"))
            .and(body_string_contains("issueId=123"))
            .respond_with(ResponseTemplate::new(409).set_body_string(error_response))
            .mount(&mock_server)
            .await;

        let params = AddStarParams::issue(123u32);
        let result = api.add_star(params).await;

        assert!(result.is_err());
        let error_message = result.unwrap_err().to_string();
        assert!(error_message.contains("You have already added a star"));
    }

    #[tokio::test]
    async fn test_add_star_resource_not_found() {
        let mock_server = MockServer::start().await;
        let api = setup_star_api(&mock_server).await;

        let error_response = r#"{
            "errors": [
                {
                    "message": "No issue found.",
                    "code": 7,
                    "moreInfo": ""
                }
            ]
        }"#;

        Mock::given(method("POST"))
            .and(path("/api/v2/stars"))
            .and(body_string_contains("issueId=999"))
            .respond_with(ResponseTemplate::new(404).set_body_string(error_response))
            .mount(&mock_server)
            .await;

        let params = AddStarParams::issue(999u32);
        let result = api.add_star(params).await;

        assert!(result.is_err());
        let error_message = result.unwrap_err().to_string();
        assert!(error_message.contains("No issue found"));
    }

    #[tokio::test]
    async fn test_add_star_unexpected_response() {
        let mock_server = MockServer::start().await;
        let api = setup_star_api(&mock_server).await;

        // Return 200 OK instead of expected 204 No Content
        Mock::given(method("POST"))
            .and(path("/api/v2/stars"))
            .and(body_string_contains("issueId=123"))
            .respond_with(ResponseTemplate::new(200).set_body_string("OK"))
            .mount(&mock_server)
            .await;

        let params = AddStarParams::issue(123u32);
        let result = api.add_star(params).await;

        assert!(result.is_err());
        let error_message = result.unwrap_err().to_string();
        assert!(error_message.contains("Unexpected HTTP status 200"));
    }
}
