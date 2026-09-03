mod common;

#[cfg(feature = "writable")]
mod writable_tests {
    use super::common::setup_document_api;
    use backlog_core::identifier::{DocumentId, Identifier, ProjectId};
    use backlog_document::api::{
        AddDocumentParams, DeleteDocumentParams, UpdateDocumentContentParams, UpdateDocumentParams,
    };
    use wiremock::matchers::{body_string, body_string_contains, header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    /// Creates mock JSON for DocumentResponse (used by add_document and delete_document)
    ///
    /// Note: DocumentResponse uses createdUserId/updatedUserId (u32) instead of
    /// full User objects that DocumentDetail uses.
    fn create_mock_document_response_json(
        id: &str,
        project_id: u32,
        title: &str,
    ) -> serde_json::Value {
        serde_json::json!({
            "id": id,
            "projectId": project_id,
            "title": title,
            "json": {"type": "doc", "content": []},
            "plain": "Plain text content",
            "statusId": 1,
            "emoji": "📄",
            "createdUserId": 1,
            "created": "2023-12-01T10:00:00Z",
            "updatedUserId": 1,
            "updated": "2023-12-01T10:00:00Z",
            "tags": []
        })
    }

    #[tokio::test]
    async fn test_add_document_with_project_id_only() {
        let mock_server = MockServer::start().await;
        let doc_api = setup_document_api(&mock_server).await;

        let response_body = create_mock_document_response_json(
            "00112233445566778899aabbccddeeff",
            1,
            "New Document",
        );

        Mock::given(method("POST"))
            .and(path("/api/v2/documents"))
            .and(header("Content-Type", "application/x-www-form-urlencoded"))
            .and(body_string_contains("projectId=1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
            .mount(&mock_server)
            .await;

        let params = AddDocumentParams::new(ProjectId::new(1));

        let result = doc_api.add_document(params).await;
        let detail = result.expect("add_document with project_id only should succeed");
        assert_eq!(detail.project_id.value(), 1);
    }

    #[tokio::test]
    async fn test_add_document_with_all_params() {
        let mock_server = MockServer::start().await;
        let doc_api = setup_document_api(&mock_server).await;

        let response_body = create_mock_document_response_json(
            "00112233445566778899aabbccddeeff",
            1,
            "Complete Document",
        );

        Mock::given(method("POST"))
            .and(path("/api/v2/documents"))
            .and(header("Content-Type", "application/x-www-form-urlencoded"))
            .and(body_string_contains("projectId=1"))
            .and(body_string_contains("title=Complete+Document"))
            .and(body_string_contains("content=This+is+the+content"))
            .and(body_string_contains("emoji=%F0%9F%93%84"))
            .and(body_string_contains("addLast=true"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
            .mount(&mock_server)
            .await;

        let params = AddDocumentParams::new(ProjectId::new(1))
            .title("Complete Document")
            .content("This is the content")
            .emoji("📄")
            .add_last(true);

        let result = doc_api.add_document(params).await;
        let detail = result.expect("add_document with all params should succeed");
        assert_eq!(detail.title, "Complete Document");
    }

    #[tokio::test]
    async fn test_add_document_with_parent_id() {
        let mock_server = MockServer::start().await;
        let doc_api = setup_document_api(&mock_server).await;

        let parent_id_str = "aabbccddeeff00112233445566778899";
        let response_body = create_mock_document_response_json(
            "00112233445566778899aabbccddeeff",
            1,
            "Child Document",
        );

        Mock::given(method("POST"))
            .and(path("/api/v2/documents"))
            .and(header("Content-Type", "application/x-www-form-urlencoded"))
            .and(body_string_contains("projectId=1"))
            .and(body_string_contains("title=Child+Document"))
            .and(body_string_contains(format!("parentId={}", parent_id_str)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
            .mount(&mock_server)
            .await;

        let params = AddDocumentParams::new(ProjectId::new(1))
            .title("Child Document")
            .parent_id(DocumentId::unsafe_new(parent_id_str.to_string()));

        let result = doc_api.add_document(params).await;
        let detail = result.expect("add_document with parent_id should succeed");
        assert_eq!(detail.title, "Child Document");
    }

    #[tokio::test]
    async fn test_add_document_with_title_only() {
        let mock_server = MockServer::start().await;
        let doc_api = setup_document_api(&mock_server).await;

        let response_body =
            create_mock_document_response_json("00112233445566778899aabbccddeeff", 1, "Title Only");

        Mock::given(method("POST"))
            .and(path("/api/v2/documents"))
            .and(header("Content-Type", "application/x-www-form-urlencoded"))
            .and(body_string_contains("projectId=1"))
            .and(body_string_contains("title=Title+Only"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
            .mount(&mock_server)
            .await;

        let params = AddDocumentParams::new(ProjectId::new(1)).title("Title Only");

        let result = doc_api.add_document(params).await;
        let detail = result.expect("add_document with title only should succeed");
        assert_eq!(detail.title, "Title Only");
    }

    #[tokio::test]
    async fn test_delete_document_success() {
        let mock_server = MockServer::start().await;
        let doc_api = setup_document_api(&mock_server).await;

        let document_id_str = "00112233445566778899aabbccddeeff";
        let response_body =
            create_mock_document_response_json(document_id_str, 1, "Deleted Document");

        Mock::given(method("DELETE"))
            .and(path(format!("/api/v2/documents/{}", document_id_str)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
            .mount(&mock_server)
            .await;

        let params = DeleteDocumentParams::new(DocumentId::unsafe_new(document_id_str.to_string()));

        let result = doc_api.delete_document(params).await;
        let detail = result.expect("delete_document should succeed");
        assert_eq!(detail.title, "Deleted Document");
        assert_eq!(detail.id.to_string(), document_id_str);
    }

    #[tokio::test]
    async fn test_delete_document_not_found() {
        let mock_server = MockServer::start().await;
        let doc_api = setup_document_api(&mock_server).await;

        let document_id_str = "nonexistent00000000000000000000";

        Mock::given(method("DELETE"))
            .and(path(format!("/api/v2/documents/{}", document_id_str)))
            .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
                "errors": [{
                    "message": "Document not found",
                    "code": 6
                }]
            })))
            .mount(&mock_server)
            .await;

        let params = DeleteDocumentParams::new(DocumentId::unsafe_new(document_id_str.to_string()));

        let result = doc_api.delete_document(params).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_document_title_and_emoji() {
        let mock_server = MockServer::start().await;
        let doc_api = setup_document_api(&mock_server).await;

        let document_id_str = "00112233445566778899aabbccddeeff";
        let response_body = create_mock_document_response_json(document_id_str, 1, "Updated Title");

        Mock::given(method("PATCH"))
            .and(path(format!("/api/v2/documents/{}", document_id_str)))
            .and(header("Content-Type", "application/x-www-form-urlencoded"))
            .and(body_string("title=Updated+Title&emoji=%F0%9F%93%9D"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
            .mount(&mock_server)
            .await;

        let params = UpdateDocumentParams::new(DocumentId::unsafe_new(document_id_str.to_string()))
            .title("Updated Title")
            .emoji("📝");

        let result = doc_api.update_document(params).await;
        let detail = result.expect("update_document should succeed");
        assert_eq!(detail.title, "Updated Title");
    }

    #[tokio::test]
    async fn test_update_document_empty_emoji_sends_key() {
        let mock_server = MockServer::start().await;
        let doc_api = setup_document_api(&mock_server).await;

        let document_id_str = "00112233445566778899aabbccddeeff";
        let mut response_body = create_mock_document_response_json(document_id_str, 1, "No Emoji");
        response_body["emoji"] = serde_json::Value::Null;

        Mock::given(method("PATCH"))
            .and(path(format!("/api/v2/documents/{}", document_id_str)))
            .and(body_string("emoji="))
            .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
            .mount(&mock_server)
            .await;

        let params = UpdateDocumentParams::new(DocumentId::unsafe_new(document_id_str.to_string()))
            .emoji("");

        let result = doc_api.update_document(params).await;
        let detail = result.expect("empty emoji should be sent as emoji=");
        assert!(detail.emoji.is_none());
    }

    #[tokio::test]
    async fn test_update_document_without_fields_returns_error() {
        let mock_server = MockServer::start().await;
        let doc_api = setup_document_api(&mock_server).await;

        let document_id_str = "00112233445566778899aabbccddeeff";

        Mock::given(method("PATCH"))
            .and(path(format!("/api/v2/documents/{}", document_id_str)))
            .and(body_string(""))
            .respond_with(ResponseTemplate::new(400).set_body_json(serde_json::json!({
                "errors": [{
                    "message": "Specify title or emoji.",
                    "code": 7,
                    "moreInfo": ""
                }]
            })))
            .mount(&mock_server)
            .await;

        let params = UpdateDocumentParams::new(DocumentId::unsafe_new(document_id_str.to_string()));

        let result = doc_api.update_document(params).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_document_accepts_null_user_ids() {
        let mock_server = MockServer::start().await;
        let doc_api = setup_document_api(&mock_server).await;

        let document_id_str = "00112233445566778899aabbccddeeff";
        let mut response_body = create_mock_document_response_json(document_id_str, 1, "Title");
        response_body["createdUserId"] = serde_json::Value::Null;
        response_body["updatedUserId"] = serde_json::Value::Null;

        Mock::given(method("PATCH"))
            .and(path(format!("/api/v2/documents/{}", document_id_str)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
            .mount(&mock_server)
            .await;

        let params = UpdateDocumentParams::new(DocumentId::unsafe_new(document_id_str.to_string()))
            .title("Title");

        let result = doc_api.update_document(params).await;
        let detail = result.expect("null user ids should deserialize");
        assert!(detail.created_user_id.is_none());
        assert!(detail.updated_user_id.is_none());
    }

    #[tokio::test]
    async fn test_update_document_content_success() {
        let mock_server = MockServer::start().await;
        let doc_api = setup_document_api(&mock_server).await;

        let document_id_str = "00112233445566778899aabbccddeeff";
        let markdown = "# Heading\n\nUpdated document content.";
        let mut response_body = create_mock_document_response_json(document_id_str, 1, "Doc");
        // Update responses omit tags
        response_body.as_object_mut().unwrap().remove("tags");
        response_body["plain"] = serde_json::json!(markdown);
        response_body["code"] = serde_json::Value::Null;
        response_body["markdownIsFallback"] = serde_json::json!(false);

        Mock::given(method("PATCH"))
            .and(path(format!(
                "/api/v2/documents/{}/content",
                document_id_str
            )))
            .and(header("Content-Type", "application/x-www-form-urlencoded"))
            .and(body_string(
                "content=%23+Heading%0A%0AUpdated+document+content.",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
            .mount(&mock_server)
            .await;

        let params = UpdateDocumentContentParams::new(
            DocumentId::unsafe_new(document_id_str.to_string()),
            markdown,
        );

        let result = doc_api.update_document_content(params).await;
        let updated = result.expect("update_document_content should succeed");
        assert_eq!(updated.document.title, "Doc");
        assert_eq!(updated.document.plain.as_deref(), Some(markdown));
        assert!(updated.code.is_none());
        assert!(!updated.markdown_is_fallback);
    }

    #[tokio::test]
    async fn test_update_document_content_no_change() {
        let mock_server = MockServer::start().await;
        let doc_api = setup_document_api(&mock_server).await;

        let document_id_str = "00112233445566778899aabbccddeeff";
        let mut response_body = create_mock_document_response_json(document_id_str, 1, "Doc");
        response_body["code"] = serde_json::json!("NO_CHANGE");
        response_body["markdownIsFallback"] = serde_json::json!(false);

        Mock::given(method("PATCH"))
            .and(path(format!(
                "/api/v2/documents/{}/content",
                document_id_str
            )))
            .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
            .mount(&mock_server)
            .await;

        let params = UpdateDocumentContentParams::new(
            DocumentId::unsafe_new(document_id_str.to_string()),
            "Plain text content",
        );

        let result = doc_api.update_document_content(params).await;
        let updated = result.expect("NO_CHANGE is a successful response");
        assert_eq!(updated.code.as_deref(), Some("NO_CHANGE"));
    }

    #[tokio::test]
    async fn test_update_document_content_conflict() {
        let mock_server = MockServer::start().await;
        let doc_api = setup_document_api(&mock_server).await;

        let document_id_str = "00112233445566778899aabbccddeeff";

        Mock::given(method("PATCH"))
            .and(path(format!(
                "/api/v2/documents/{}/content",
                document_id_str
            )))
            .respond_with(ResponseTemplate::new(409).set_body_json(serde_json::json!({
                "errors": [{
                    "message": "Document has been changed",
                    "code": 7,
                    "moreInfo": "DOCUMENT_CHANGED"
                }]
            })))
            .mount(&mock_server)
            .await;

        let params = UpdateDocumentContentParams::new(
            DocumentId::unsafe_new(document_id_str.to_string()),
            "# New content",
        );

        let err = doc_api
            .update_document_content(params)
            .await
            .expect_err("409 should be an error");
        match err {
            backlog_api_core::Error::HttpStatus { status, errors, .. } => {
                assert_eq!(status, 409);
                assert_eq!(errors[0].more_info.as_deref(), Some("DOCUMENT_CHANGED"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }
}
