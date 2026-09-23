mod common;
use common::setup_document_api;

use backlog_core::identifier::{DocumentId, Identifier};
use backlog_document::GetDocumentCommentsParams;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

const DOC_ID: &str = "019d46599c2d7e19adfa562d5b9019a6";

fn comment_json(id: &str, replies: &str) -> String {
    format!(
        r#"{{
            "id": "{id}",
            "documentId": "{DOC_ID}",
            "statusId": 0,
            "content": "{{\"type\":\"doc\",\"content\":[]}}",
            "plain": "What's this?",
            "commentType": "comment",
            "createdUserId": 2,
            "created": "2026-04-01T00:03:47Z",
            "updatedUserId": 2,
            "updated": "2026-04-01T00:03:47Z",
            "createdUser": {{
                "id": 2,
                "userId": "admin",
                "uniqueId": null,
                "name": "管理者",
                "mailAddress": "admin@example.com",
                "roleType": 1,
                "lang": null,
                "icon": "icons/person_168.gif"
            }},
            "replies": [{replies}]
        }}"#
    )
}

#[tokio::test]
async fn test_get_document_comments_success() {
    let server = MockServer::start().await;
    let api = setup_document_api(&server).await;

    let reply = comment_json("bbbb", "");
    let body = format!("[{}]", comment_json("aaaa", &reply));

    Mock::given(method("GET"))
        .and(path(format!("/api/v2/documents/{DOC_ID}/comments")))
        .respond_with(ResponseTemplate::new(200).set_body_string(body))
        .expect(1)
        .mount(&server)
        .await;

    let comments = api
        .get_document_comments(GetDocumentCommentsParams::new(DocumentId::unsafe_new(
            DOC_ID.to_string(),
        )))
        .await
        .expect("should succeed");

    assert_eq!(comments.len(), 1);
    let c = &comments[0];
    assert_eq!(c.id, "aaaa");
    assert_eq!(c.document_id.value(), DOC_ID);
    assert_eq!(c.plain, "What's this?");
    assert_eq!(c.content, r#"{"type":"doc","content":[]}"#);
    assert_eq!(c.comment_type, "comment");
    assert_eq!(c.created_user.name, "管理者");
    assert_eq!(c.replies.len(), 1);
    assert_eq!(c.replies[0].id, "bbbb");
    assert!(c.replies[0].replies.is_empty());
}

#[tokio::test]
async fn test_get_document_comments_empty() {
    let server = MockServer::start().await;
    let api = setup_document_api(&server).await;

    Mock::given(method("GET"))
        .and(path(format!("/api/v2/documents/{DOC_ID}/comments")))
        .respond_with(ResponseTemplate::new(200).set_body_string("[]"))
        .mount(&server)
        .await;

    let comments = api
        .get_document_comments(GetDocumentCommentsParams::new(DocumentId::unsafe_new(
            DOC_ID.to_string(),
        )))
        .await
        .expect("should succeed");
    assert!(comments.is_empty());
}

#[tokio::test]
async fn test_get_document_comments_error_statuses() {
    for status in [400u16, 401, 403, 404, 500] {
        let server = MockServer::start().await;
        let api = setup_document_api(&server).await;

        Mock::given(method("GET"))
            .and(path(format!("/api/v2/documents/{DOC_ID}/comments")))
            .respond_with(
                ResponseTemplate::new(status)
                    .set_body_string(r#"{"errors":[{"message":"error","code":6,"moreInfo":""}]}"#),
            )
            .mount(&server)
            .await;

        let err = api
            .get_document_comments(GetDocumentCommentsParams::new(DocumentId::unsafe_new(
                DOC_ID.to_string(),
            )))
            .await
            .expect_err("should fail");
        assert!(
            matches!(err, backlog_api_core::Error::HttpStatus { status: s, .. } if s == status),
            "unexpected error for {status}: {err:?}"
        );
    }
}
