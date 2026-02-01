mod common;

#[cfg(feature = "writable")]
mod writable_tests {
    use super::common::setup_document_api;
    use backlog_core::identifier::{Identifier, ProjectId};
    use backlog_document::api::{AddDocumentParams, DeleteDocumentParams};
    use wiremock::matchers::{body_string_contains, header, method, path};
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
            .parent_id(parent_id_str.to_string());

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

        let params = DeleteDocumentParams::new(document_id_str.to_string());

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

        let params = DeleteDocumentParams::new(document_id_str.to_string());

        let result = doc_api.delete_document(params).await;
        assert!(result.is_err());
    }
}
