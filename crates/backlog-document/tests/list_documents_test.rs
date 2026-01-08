mod common;
use common::*;

use backlog_core::identifier::ProjectId;
use backlog_document::{DocumentOrder, DocumentSortKey, ListDocumentsParamsBuilder};
use wiremock::matchers::query_param;

#[tokio::test]
async fn test_list_documents_with_multiple_project_ids() {
    let server = wiremock::MockServer::start().await;
    let doc_api = setup_document_api(&server).await;

    let response_body = r#"[
        {
            "id": "doc1",
            "projectId": 1,
            "title": "First Document",
            "plain": "Plain text content",
            "statusId": 1,
            "emoji": "📄",
            "createdUser": {
                "id": 1,
                "userId": "admin",
                "name": "Admin User",
                "roleType": 1,
                "lang": "ja",
                "mailAddress": "admin@example.com",
                "nulabAccount": null,
                "keyword": null,
                "lastLoginTime": null
            },
            "created": "2023-01-01T00:00:00Z",
            "updatedUser": {
                "id": 1,
                "userId": "admin",
                "name": "Admin User",
                "roleType": 1,
                "lang": "ja",
                "mailAddress": "admin@example.com",
                "nulabAccount": null,
                "keyword": null,
                "lastLoginTime": null
            },
            "updated": "2023-01-01T00:00:00Z",
            "tags": []
        }
    ]"#;

    Mock::given(method("GET"))
        .and(path("/api/v2/documents"))
        .and(query_param("projectId[]", "1"))
        .and(query_param("projectId[]", "2"))
        .and(query_param("sort", "created"))
        .and(query_param("order", "desc"))
        .and(query_param("offset", "0"))
        .and(query_param("count", "20"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(response_body)
                .insert_header("Content-Type", "application/json"),
        )
        .mount(&server)
        .await;

    let params = ListDocumentsParamsBuilder::default()
        .project_ids(vec![ProjectId::new(1), ProjectId::new(2)])
        .sort(DocumentSortKey::Created)
        .order(DocumentOrder::Desc)
        .offset(0)
        .count(20)
        .build()
        .expect("builder should succeed with project_ids, sort, order, offset, count");

    let result = doc_api.list_documents(params).await;

    let documents = result.expect("list_documents should succeed");
    assert_eq!(documents.len(), 1);
    assert_eq!(documents[0].title, "First Document");
}

#[tokio::test]
async fn test_list_documents_minimal_params() {
    let server = wiremock::MockServer::start().await;
    let doc_api = setup_document_api(&server).await;

    let response_body = r#"[]"#;

    Mock::given(method("GET"))
        .and(path("/api/v2/documents"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(response_body)
                .insert_header("Content-Type", "application/json"),
        )
        .mount(&server)
        .await;

    // All parameters are optional now
    let params = ListDocumentsParamsBuilder::default()
        .build()
        .expect("builder should succeed with default params");

    let result = doc_api.list_documents(params).await;

    let documents = result.expect("list_documents with minimal params should succeed");
    assert_eq!(documents.len(), 0);
}

#[tokio::test]
async fn test_list_documents_with_keyword() {
    let server = wiremock::MockServer::start().await;
    let doc_api = setup_document_api(&server).await;

    let response_body = r#"[]"#;

    Mock::given(method("GET"))
        .and(path("/api/v2/documents"))
        .and(query_param("keyword", "test"))
        .and(query_param("sort", "updated"))
        .and(query_param("order", "asc"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(response_body)
                .insert_header("Content-Type", "application/json"),
        )
        .mount(&server)
        .await;

    let params = ListDocumentsParamsBuilder::default()
        .keyword("test".to_string())
        .sort(DocumentSortKey::Updated)
        .order(DocumentOrder::Asc)
        .build()
        .expect("builder should succeed with keyword and sort params");

    let result = doc_api.list_documents(params).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_list_documents_error() {
    let server = wiremock::MockServer::start().await;
    let doc_api = setup_document_api(&server).await;

    Mock::given(method("GET"))
        .and(path("/api/v2/documents"))
        .respond_with(ResponseTemplate::new(500))
        .mount(&server)
        .await;

    let params = ListDocumentsParamsBuilder::default()
        .project_ids(vec![ProjectId::new(1)])
        .build()
        .expect("builder should succeed with project_ids");

    let result = doc_api.list_documents(params).await;

    assert!(result.is_err());
}
