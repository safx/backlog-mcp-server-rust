mod common;
use common::setup_document_api;

use backlog_api_core::bytes;
use backlog_core::identifier::{DocumentAttachmentId, DocumentId};
use backlog_document::DownloadAttachmentParams;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn test_download_attachment_success() {
    let server = wiremock::MockServer::start().await;
    let doc_api = setup_document_api(&server).await;

    let document_id_val = 12345;
    let attachment_id_val = 67890;
    let attachment_content = "This is a document attachment content.";

    Mock::given(method("GET"))
        .and(path(format!(
            "/api/v2/documents/{document_id_val}/attachments/{attachment_id_val}"
        )))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_bytes(attachment_content)
                .insert_header("Content-Type", "application/octet-stream")
                .insert_header(
                    "Content-Disposition",
                    "attachment; filename=\"doc_attachment.txt\"",
                ), // Example header
        )
        .mount(&server)
        .await;

    let document_id = DocumentId::unsafe_new(document_id_val.to_string());
    let attachment_id = DocumentAttachmentId::new(attachment_id_val);

    let params = DownloadAttachmentParams::new(document_id, attachment_id);
    let result = doc_api.download_attachment(params).await;

    let downloaded_file = result.expect("download_attachment should succeed");
    assert_eq!(downloaded_file.filename, "doc_attachment.txt");
    assert_eq!(downloaded_file.content_type, "application/octet-stream");
    assert_eq!(
        downloaded_file.bytes,
        bytes::Bytes::from(attachment_content)
    );
}

#[tokio::test]
async fn test_download_attachment_error_404() {
    let server = wiremock::MockServer::start().await;
    let doc_api = setup_document_api(&server).await;

    let document_id_val = 12345;
    let attachment_id_val = 67891;

    Mock::given(method("GET"))
        .and(path(format!(
            "/api/v2/documents/{document_id_val}/attachments/{attachment_id_val}"
        )))
        .respond_with(ResponseTemplate::new(404))
        .mount(&server)
        .await;

    let document_id = DocumentId::unsafe_new(document_id_val.to_string());
    let attachment_id = DocumentAttachmentId::new(attachment_id_val);

    let params = DownloadAttachmentParams::new(document_id, attachment_id);
    let result = doc_api.download_attachment(params).await;

    assert!(result.is_err());
    // Optionally, check the specific error type if desired
    // match result.unwrap_err() {
    //     backlog_api_core::Error::HttpStatus { status, .. } => {
    //         assert_eq!(status, reqwest::StatusCode::NOT_FOUND)
    //     }
    //     _ => panic!("Expected HttpStatus error"),
    // }
}
