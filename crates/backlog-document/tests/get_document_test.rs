mod common;
use common::setup_document_api;

use backlog_core::identifier::{DocumentId, ProjectId};
use backlog_document::GetDocumentParams;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn test_get_document_success() {
    let server = wiremock::MockServer::start().await;
    let doc_api = setup_document_api(&server).await;

    let document_id_str = "00112233445566778899aabbccddeeff";
    let response_body = r#"{
        "id": "00112233445566778899aabbccddeeff",
        "projectId": 1,
        "title": "Test Document",
        "json": {"type": "doc", "content": []},
        "plain": "Plain text content",
        "statusId": 1,
        "emoji": "📄",
        "attachments": [
            {
                "id": 12345,
                "name": "attachment.pdf",
                "size": 435506,
                "createdUser": {
                    "id": 1,
                    "userId": "admin",
                    "name": "AdminUser",
                    "roleType": 1,
                    "lang": "ja",
                    "mailAddress": "admin@example.com",
                    "nulabAccount": null,
                    "keyword": "AdminUser",
                    "lastLoginTime": "2023-11-30T10:00:00Z"
                },
                "created": "2023-12-01T00:00:00Z"
            }
        ],
        "createdUser": {
            "id": 1,
            "userId": "admin",
            "name": "AdminUser",
            "roleType": 1,
            "lang": "ja",
            "mailAddress": "admin@example.com",
            "nulabAccount": null,
            "keyword": "AdminUser",
            "lastLoginTime": "2023-11-30T12:00:00Z"
        },
        "created": "2023-01-01T00:00:00Z",
        "updatedUser": {
            "id": 2,
            "userId": "editor",
            "name": "EditorUser",
            "roleType": 2,
            "lang": "ja",
            "mailAddress": "editor@example.com",
            "nulabAccount": null,
            "keyword": "EditorUser",
            "lastLoginTime": "2023-12-01T09:00:00Z"
        },
        "updated": "2023-12-01T00:00:00Z",
        "tags": [
            {"id": 1, "name": "important"},
            {"id": 2, "name": "review"}
        ]
    }"#;

    Mock::given(method("GET"))
        .and(path(format!("/api/v2/documents/{}", document_id_str)))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(response_body)
                .insert_header("Content-Type", "application/json"),
        )
        .mount(&server)
        .await;

    let document_id = DocumentId::unsafe_new(document_id_str.to_string());
    let params = GetDocumentParams::new(document_id);

    let result = doc_api.get_document(params).await;

    let document = result.expect("get_document should succeed");

    // Verify basic fields
    assert_eq!(
        document.id,
        DocumentId::unsafe_new(document_id_str.to_string())
    );
    assert_eq!(document.project_id, ProjectId::new(1));
    assert_eq!(document.title, "Test Document");
    assert_eq!(document.plain, "Plain text content");
    assert_eq!(document.status_id, 1);
    assert_eq!(document.emoji, Some("📄".to_string()));

    // Verify attachments
    assert_eq!(document.attachments.len(), 1);
    assert_eq!(document.attachments[0].name, "attachment.pdf");
    assert_eq!(document.attachments[0].size, 435506);

    // Verify tags
    assert_eq!(document.tags.len(), 2);
    assert_eq!(document.tags[0].name, "important");
    assert_eq!(document.tags[1].name, "review");

    // Verify users
    assert_eq!(document.created_user.user_id, Some("admin".to_string()));
    assert_eq!(document.updated_user.user_id, Some("editor".to_string()));
}

#[tokio::test]
async fn test_get_document_minimal_response() {
    let server = wiremock::MockServer::start().await;
    let doc_api = setup_document_api(&server).await;

    let document_id_str = "aabbccddeeff00112233445566778899";
    let response_body = r#"{
        "id": "aabbccddeeff00112233445566778899",
        "projectId": 2,
        "title": "Minimal Document",
        "json": {"type": "doc", "content": []},
        "plain": "Minimal content",
        "statusId": 1,
        "emoji": null,
        "attachments": [],
        "createdUser": {
            "id": 1,
            "userId": "user1",
            "name": "UserOne",
            "roleType": 1,
            "lang": "en",
            "mailAddress": "user1@example.com",
            "nulabAccount": null,
            "keyword": null,
            "lastLoginTime": null
        },
        "created": "2023-01-01T00:00:00Z",
        "updatedUser": {
            "id": 1,
            "userId": "user1",
            "name": "UserOne",
            "roleType": 1,
            "lang": "en",
            "mailAddress": "user1@example.com",
            "nulabAccount": null,
            "keyword": null,
            "lastLoginTime": null
        },
        "updated": "2023-01-01T00:00:00Z",
        "tags": []
    }"#;

    Mock::given(method("GET"))
        .and(path(format!("/api/v2/documents/{}", document_id_str)))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(response_body)
                .insert_header("Content-Type", "application/json"),
        )
        .mount(&server)
        .await;

    let document_id = DocumentId::unsafe_new(document_id_str.to_string());
    let params = GetDocumentParams::new(document_id);

    let result = doc_api.get_document(params).await;

    let document = result.expect("get_document should succeed with minimal response");

    // Verify basic fields
    assert_eq!(
        document.id,
        DocumentId::unsafe_new(document_id_str.to_string())
    );
    assert_eq!(document.project_id, ProjectId::new(2));
    assert_eq!(document.title, "Minimal Document");
    assert_eq!(document.emoji, None);

    // Verify empty collections
    assert_eq!(document.attachments.len(), 0);
    assert_eq!(document.tags.len(), 0);
}

#[tokio::test]
async fn test_get_document_not_found() {
    let server = wiremock::MockServer::start().await;
    let doc_api = setup_document_api(&server).await;

    let document_id_str = "fffffffffffffffffffffffffffffff0";

    Mock::given(method("GET"))
        .and(path(format!("/api/v2/documents/{}", document_id_str)))
        .respond_with(
            ResponseTemplate::new(404)
                .set_body_string(
                    r#"{"errors":[{"message":"ドキュメントが見つかりません","code":6,"moreInfo":""}]}"#,
                )
                .insert_header("Content-Type", "application/json"),
        )
        .mount(&server)
        .await;

    let document_id = DocumentId::unsafe_new(document_id_str.to_string());
    let params = GetDocumentParams::new(document_id);

    let result = doc_api.get_document(params).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_document_unauthorized() {
    let server = wiremock::MockServer::start().await;
    let doc_api = setup_document_api(&server).await;

    let document_id_str = "11223344556677889900aabbccddeeff";

    Mock::given(method("GET"))
        .and(path(format!("/api/v2/documents/{}", document_id_str)))
        .respond_with(
            ResponseTemplate::new(403).set_body_string(
                r#"{"errors":[{"message":"このリソースにアクセスする権限がありません","code":11,"moreInfo":""}]}"#,
            ).insert_header("Content-Type", "application/json"),
        )
        .mount(&server)
        .await;

    let document_id = DocumentId::unsafe_new(document_id_str.to_string());
    let params = GetDocumentParams::new(document_id);

    let result = doc_api.get_document(params).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_document_server_error() {
    let server = wiremock::MockServer::start().await;
    let doc_api = setup_document_api(&server).await;

    let document_id_str = "99887766554433221100ffeeddccbbaa";

    Mock::given(method("GET"))
        .and(path(format!("/api/v2/documents/{}", document_id_str)))
        .respond_with(ResponseTemplate::new(500))
        .mount(&server)
        .await;

    let document_id = DocumentId::unsafe_new(document_id_str.to_string());
    let params = GetDocumentParams::new(document_id);

    let result = doc_api.get_document(params).await;

    assert!(result.is_err());
}
