mod common;
use common::*;

use backlog_core::{ProjectKey, identifier::StarId};
use backlog_wiki::WikiCount;
use backlog_wiki::api::{
    DownloadWikiAttachmentParams, GetWikiAttachmentListParams, GetWikiCountParams,
    GetWikiDetailParams, GetWikiHistoryParams, GetWikiListParams, GetWikiSharedFileListParams,
    GetWikiStarsParams, GetWikiTagListParams,
};
use wiremock::MockServer;
use wiremock::matchers::{method, path, query_param};

#[tokio::test]
async fn test_get_wiki_list_empty_params_success() {
    let mock_server = MockServer::start().await;
    let wiki_api = setup_wiki_api(&mock_server).await;

    let expected_wikis = vec![
        create_mock_wiki(112, 103, "Home", 1, "john"),
        create_mock_wiki(113, 103, "Documentation", 2, "alice"),
    ];

    Mock::given(method("GET"))
        .and(path("/api/v2/wikis"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_wikis))
        .mount(&mock_server)
        .await;

    let params = GetWikiListParams::new();
    let result = wiki_api.get_wiki_list(params).await;
    assert!(result.is_ok());
    let wikis = result.unwrap();
    assert_eq!(wikis.len(), 2);
    assert_eq!(wikis[0].name, "Home");
    assert_eq!(wikis[1].name, "Documentation");
}

#[tokio::test]
async fn test_get_wiki_list_with_project_id() {
    let mock_server = MockServer::start().await;
    let wiki_api = setup_wiki_api(&mock_server).await;

    let expected_wikis = vec![create_mock_wiki(112, 123, "Home", 1, "john")];

    Mock::given(method("GET"))
        .and(path("/api/v2/wikis"))
        .and(query_param("projectIdOrKey", "MYPROJECT"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_wikis))
        .mount(&mock_server)
        .await;

    let params =
        GetWikiListParams::new().project_id_or_key("MYPROJECT".parse::<ProjectKey>().unwrap());
    let result = wiki_api.get_wiki_list(params).await;
    assert!(result.is_ok());
    let wikis = result.unwrap();
    assert_eq!(wikis.len(), 1);
    assert_eq!(wikis[0].name, "Home");
}

#[tokio::test]
async fn test_get_wiki_list_with_keyword() {
    let mock_server = MockServer::start().await;
    let wiki_api = setup_wiki_api(&mock_server).await;

    let expected_wikis = vec![create_mock_wiki(113, 103, "Documentation", 2, "alice")];

    Mock::given(method("GET"))
        .and(path("/api/v2/wikis"))
        .and(query_param("keyword", "doc"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_wikis))
        .mount(&mock_server)
        .await;

    let params = GetWikiListParams::new().keyword("doc");
    let result = wiki_api.get_wiki_list(params).await;
    assert!(result.is_ok());
    let wikis = result.unwrap();
    assert_eq!(wikis.len(), 1);
    assert_eq!(wikis[0].name, "Documentation");
}

#[tokio::test]
async fn test_get_wiki_count_without_project() {
    let mock_server = MockServer::start().await;
    let wiki_api = setup_wiki_api(&mock_server).await;

    let expected_count = WikiCount { count: 42 };

    Mock::given(method("GET"))
        .and(path("/api/v2/wikis/count"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_count))
        .mount(&mock_server)
        .await;

    let params = GetWikiCountParams::new();
    let result = wiki_api.get_wiki_count(params).await;
    assert!(result.is_ok());
    let count = result.unwrap();
    assert_eq!(count.count, 42);
}

#[tokio::test]
async fn test_get_wiki_count_with_project() {
    let mock_server = MockServer::start().await;
    let wiki_api = setup_wiki_api(&mock_server).await;

    let expected_count = WikiCount { count: 15 };

    Mock::given(method("GET"))
        .and(path("/api/v2/wikis/count"))
        .and(query_param("projectIdOrKey", "MYPROJECT"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_count))
        .mount(&mock_server)
        .await;

    let params =
        GetWikiCountParams::new().project_id_or_key("MYPROJECT".parse::<ProjectKey>().unwrap());
    let result = wiki_api.get_wiki_count(params).await;
    assert!(result.is_ok());
    let count = result.unwrap();
    assert_eq!(count.count, 15);
}

#[tokio::test]
async fn test_get_wiki_detail_success() {
    let mock_server = MockServer::start().await;
    let wiki_api = setup_wiki_api(&mock_server).await;

    let expected_detail = create_mock_wiki_detail(123, 456, "API Documentation");

    Mock::given(method("GET"))
        .and(path("/api/v2/wikis/123"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_detail))
        .mount(&mock_server)
        .await;

    let result = wiki_api
        .get_wiki_detail(GetWikiDetailParams::new(WikiId::new(123)))
        .await;
    assert!(result.is_ok());
    let detail = result.unwrap();
    assert_eq!(detail.id.value(), 123);
    assert_eq!(detail.project_id.value(), 456);
    assert_eq!(detail.name, "API Documentation");
    assert!(detail.content.contains("API Documentation"));
    assert_eq!(detail.tags.len(), 1);
    assert_eq!(detail.attachments.len(), 1);
    assert_eq!(detail.shared_files.len(), 1);
    assert_eq!(detail.stars.len(), 1);
}

#[tokio::test]
async fn test_get_wiki_detail_with_u32_id() {
    let mock_server = MockServer::start().await;
    let wiki_api = setup_wiki_api(&mock_server).await;

    let expected_detail = create_mock_wiki_detail(789, 101, "User Guide");

    Mock::given(method("GET"))
        .and(path("/api/v2/wikis/789"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_detail))
        .mount(&mock_server)
        .await;

    let result = wiki_api
        .get_wiki_detail(GetWikiDetailParams::new(789u32))
        .await;
    assert!(result.is_ok());
    let detail = result.unwrap();
    assert_eq!(detail.id.value(), 789);
    assert_eq!(detail.name, "User Guide");
}

#[tokio::test]
async fn test_get_wiki_attachment_list_success() {
    let mock_server = MockServer::start().await;
    let wiki_api = setup_wiki_api(&mock_server).await;

    let expected_attachments = vec![
        create_mock_wiki_attachment(1, "document.pdf", 1024, 1, "john"),
        create_mock_wiki_attachment(2, "image.png", 2048, 2, "alice"),
        create_mock_wiki_attachment(3, "spreadsheet.xlsx", 4096, 1, "john"),
    ];

    Mock::given(method("GET"))
        .and(path("/api/v2/wikis/123/attachments"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_attachments))
        .mount(&mock_server)
        .await;

    let result = wiki_api
        .get_wiki_attachment_list(GetWikiAttachmentListParams::new(WikiId::new(123)))
        .await;
    assert!(result.is_ok());
    let attachments = result.unwrap();
    assert_eq!(attachments.len(), 3);
    assert_eq!(attachments[0].name, "document.pdf");
    assert_eq!(attachments[0].size, 1024);
    assert_eq!(attachments[1].name, "image.png");
    assert_eq!(attachments[1].size, 2048);
    assert_eq!(attachments[2].name, "spreadsheet.xlsx");
    assert_eq!(attachments[2].size, 4096);
}

#[tokio::test]
async fn test_download_wiki_attachment_success() {
    let mock_server = MockServer::start().await;
    let wiki_api = setup_wiki_api(&mock_server).await;

    let attachment_content = "This is a test attachment content.";

    Mock::given(method("GET"))
        .and(path("/api/v2/wikis/123/attachments/456"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_bytes(attachment_content)
                .insert_header("Content-Type", "application/octet-stream")
                .insert_header("Content-Disposition", "attachment; filename=\"test.txt\""),
        )
        .mount(&mock_server)
        .await;

    let result = wiki_api
        .download_wiki_attachment(DownloadWikiAttachmentParams::new(
            WikiId::new(123),
            WikiAttachmentId::new(456),
        ))
        .await;
    assert!(result.is_ok());
    let downloaded_file = result.unwrap();
    assert_eq!(downloaded_file.filename, "test.txt");
    assert_eq!(downloaded_file.content_type, "application/octet-stream");
    assert_eq!(downloaded_file.bytes.len(), attachment_content.len());
}

#[tokio::test]
async fn test_download_wiki_attachment_with_u32_ids() {
    let mock_server = MockServer::start().await;
    let wiki_api = setup_wiki_api(&mock_server).await;

    let attachment_content = "Test content for u32 ID test.";

    Mock::given(method("GET"))
        .and(path("/api/v2/wikis/789/attachments/101"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_bytes(attachment_content)
                .insert_header("Content-Type", "image/png")
                .insert_header("Content-Disposition", "attachment; filename=\"image.png\""),
        )
        .mount(&mock_server)
        .await;

    let result = wiki_api
        .download_wiki_attachment(DownloadWikiAttachmentParams::new(789u32, 101u32))
        .await;
    assert!(result.is_ok());
    let downloaded_file = result.unwrap();
    assert_eq!(downloaded_file.filename, "image.png");
    assert_eq!(downloaded_file.content_type, "image/png");
    assert_eq!(downloaded_file.bytes.len(), attachment_content.len());
}

#[tokio::test]
async fn test_get_wiki_tag_list_success() {
    let mock_server = MockServer::start().await;
    let wiki_api = setup_wiki_api(&mock_server).await;

    let expected_tags = vec![
        WikiTag {
            id: WikiTagId::new(1),
            name: "proceedings".to_string(),
        },
        WikiTag {
            id: WikiTagId::new(2),
            name: "meeting".to_string(),
        },
    ];

    Mock::given(method("GET"))
        .and(path("/api/v2/wikis/tags"))
        .and(query_param("projectIdOrKey", "MFP"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_tags))
        .mount(&mock_server)
        .await;

    let params = GetWikiTagListParams::new("MFP".parse::<ProjectKey>().unwrap());
    let result = wiki_api.get_wiki_tag_list(params).await;
    assert!(result.is_ok());
    let tags = result.unwrap();
    assert_eq!(tags.len(), 2);
    assert_eq!(tags[0].id.value(), 1);
    assert_eq!(tags[0].name, "proceedings");
    assert_eq!(tags[1].id.value(), 2);
    assert_eq!(tags[1].name, "meeting");
}

#[tokio::test]
async fn test_get_wiki_tag_list_empty() {
    let mock_server = MockServer::start().await;
    let wiki_api = setup_wiki_api(&mock_server).await;

    let expected_tags: Vec<WikiTag> = vec![];

    Mock::given(method("GET"))
        .and(path("/api/v2/wikis/tags"))
        .and(query_param("projectIdOrKey", "EMPTY"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_tags))
        .mount(&mock_server)
        .await;

    let params = GetWikiTagListParams::new("EMPTY".parse::<ProjectKey>().unwrap());
    let result = wiki_api.get_wiki_tag_list(params).await;
    assert!(result.is_ok());
    let tags = result.unwrap();
    assert_eq!(tags.len(), 0);
}

#[tokio::test]
async fn test_get_wiki_tag_list_project_not_found() {
    let mock_server = MockServer::start().await;
    let wiki_api = setup_wiki_api(&mock_server).await;

    Mock::given(method("GET"))
        .and(path("/api/v2/wikis/tags"))
        .and(query_param("projectIdOrKey", "INVALID"))
        .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
            "errors": [{"message": "Project not found"}]
        })))
        .mount(&mock_server)
        .await;

    let params = GetWikiTagListParams::new("INVALID".parse::<ProjectKey>().unwrap());
    let result = wiki_api.get_wiki_tag_list(params).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_wiki_tag_list_access_forbidden() {
    let mock_server = MockServer::start().await;
    let wiki_api = setup_wiki_api(&mock_server).await;

    Mock::given(method("GET"))
        .and(path("/api/v2/wikis/tags"))
        .and(query_param("projectIdOrKey", "PRIVATE"))
        .respond_with(ResponseTemplate::new(403).set_body_json(serde_json::json!({
            "errors": [{"message": "You do not have permission to access this project"}]
        })))
        .mount(&mock_server)
        .await;

    let params = GetWikiTagListParams::new("PRIVATE".parse::<ProjectKey>().unwrap());
    let result = wiki_api.get_wiki_tag_list(params).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_wiki_tag_list_server_error() {
    let mock_server = MockServer::start().await;
    let wiki_api = setup_wiki_api(&mock_server).await;

    Mock::given(method("GET"))
        .and(path("/api/v2/wikis/tags"))
        .and(query_param("projectIdOrKey", "ERROR"))
        .respond_with(ResponseTemplate::new(500).set_body_json(serde_json::json!({
            "errors": [{"message": "Internal server error"}]
        })))
        .mount(&mock_server)
        .await;

    let params = GetWikiTagListParams::new("ERROR".parse::<ProjectKey>().unwrap());
    let result = wiki_api.get_wiki_tag_list(params).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_wiki_tag_list_single_tag() {
    let mock_server = MockServer::start().await;
    let wiki_api = setup_wiki_api(&mock_server).await;

    let expected_tags = vec![WikiTag {
        id: WikiTagId::new(1),
        name: "important".to_string(),
    }];

    Mock::given(method("GET"))
        .and(path("/api/v2/wikis/tags"))
        .and(query_param("projectIdOrKey", "SINGLE"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_tags))
        .mount(&mock_server)
        .await;

    let params = GetWikiTagListParams::new("SINGLE".parse::<ProjectKey>().unwrap());
    let result = wiki_api.get_wiki_tag_list(params).await;
    assert!(result.is_ok());
    let tags = result.unwrap();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].id.value(), 1);
    assert_eq!(tags[0].name, "important");
}

#[test]
fn test_get_wiki_tag_list_params_to_query() {
    let params = GetWikiTagListParams::new("MFP".parse::<ProjectKey>().unwrap());

    // IntoRequestトレイトのto_query()メソッドをテスト
    use backlog_api_core::IntoRequest;
    let query = params.to_query();

    // シリアライズして確認
    let json = serde_json::to_string(&query).unwrap();
    assert!(json.contains("projectIdOrKey"));
    assert!(json.contains("MFP"));
}

#[tokio::test]
async fn test_get_wiki_history_success() {
    let mock_server = MockServer::start().await;
    let wiki_api = setup_wiki_api(&mock_server).await;

    let expected_history = vec![
        create_mock_wiki_history(123, 2, "Updated Page", "alice"),
        create_mock_wiki_history(123, 1, "Initial Page", "john"),
    ];

    Mock::given(method("GET"))
        .and(path("/api/v2/wikis/123/history"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_history))
        .mount(&mock_server)
        .await;

    let params = GetWikiHistoryParams::new(WikiId::new(123));
    let result = wiki_api.get_wiki_history(params).await;
    assert!(result.is_ok());
    let history = result.unwrap();
    assert_eq!(history.len(), 2);
    assert_eq!(history[0].version, 2);
    assert_eq!(history[0].name, "Updated Page");
    assert_eq!(history[1].version, 1);
    assert_eq!(history[1].name, "Initial Page");
}

#[tokio::test]
async fn test_get_wiki_history_with_parameters() {
    let mock_server = MockServer::start().await;
    let wiki_api = setup_wiki_api(&mock_server).await;

    let expected_history = vec![create_mock_wiki_history(123, 3, "Latest Version", "bob")];

    Mock::given(method("GET"))
        .and(path("/api/v2/wikis/123/history"))
        .and(query_param("minId", "100"))
        .and(query_param("maxId", "200"))
        .and(query_param("count", "10"))
        .and(query_param("order", "asc"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_history))
        .mount(&mock_server)
        .await;

    let params = GetWikiHistoryParams::new(WikiId::new(123))
        .min_id(100)
        .max_id(200)
        .count(10)
        .order(backlog_wiki::HistoryOrder::Asc);

    let result = wiki_api.get_wiki_history(params).await;
    assert!(result.is_ok());
    let history = result.unwrap();
    assert_eq!(history.len(), 1);
    assert_eq!(history[0].version, 3);
    assert_eq!(history[0].name, "Latest Version");
}

#[tokio::test]
async fn test_get_wiki_history_empty() {
    let mock_server = MockServer::start().await;
    let wiki_api = setup_wiki_api(&mock_server).await;

    let expected_history: Vec<WikiHistory> = vec![];

    Mock::given(method("GET"))
        .and(path("/api/v2/wikis/999/history"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_history))
        .mount(&mock_server)
        .await;

    let params = GetWikiHistoryParams::new(WikiId::new(999));
    let result = wiki_api.get_wiki_history(params).await;
    assert!(result.is_ok());
    let history = result.unwrap();
    assert_eq!(history.len(), 0);
}

#[tokio::test]
async fn test_get_wiki_history_not_found() {
    let mock_server = MockServer::start().await;
    let wiki_api = setup_wiki_api(&mock_server).await;

    Mock::given(method("GET"))
        .and(path("/api/v2/wikis/404/history"))
        .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
            "errors": [{"message": "Wiki not found"}]
        })))
        .mount(&mock_server)
        .await;

    let params = GetWikiHistoryParams::new(WikiId::new(404));
    let result = wiki_api.get_wiki_history(params).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_wiki_history_access_forbidden() {
    let mock_server = MockServer::start().await;
    let wiki_api = setup_wiki_api(&mock_server).await;

    Mock::given(method("GET"))
        .and(path("/api/v2/wikis/403/history"))
        .respond_with(ResponseTemplate::new(403).set_body_json(serde_json::json!({
            "errors": [{"message": "You do not have permission to access this wiki"}]
        })))
        .mount(&mock_server)
        .await;

    let params = GetWikiHistoryParams::new(WikiId::new(403));
    let result = wiki_api.get_wiki_history(params).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_wiki_history_server_error() {
    let mock_server = MockServer::start().await;
    let wiki_api = setup_wiki_api(&mock_server).await;

    Mock::given(method("GET"))
        .and(path("/api/v2/wikis/500/history"))
        .respond_with(ResponseTemplate::new(500).set_body_json(serde_json::json!({
            "errors": [{"message": "Internal server error"}]
        })))
        .mount(&mock_server)
        .await;

    let params = GetWikiHistoryParams::new(WikiId::new(500));
    let result = wiki_api.get_wiki_history(params).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_wiki_history_with_u32_id() {
    let mock_server = MockServer::start().await;
    let wiki_api = setup_wiki_api(&mock_server).await;

    let expected_history = vec![create_mock_wiki_history(789, 1, "Test Wiki", "admin")];

    Mock::given(method("GET"))
        .and(path("/api/v2/wikis/789/history"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_history))
        .mount(&mock_server)
        .await;

    let params = GetWikiHistoryParams::new(789u32);
    let result = wiki_api.get_wiki_history(params).await;
    assert!(result.is_ok());
    let history = result.unwrap();
    assert_eq!(history.len(), 1);
    assert_eq!(history[0].page_id.value(), 789);
    assert_eq!(history[0].version, 1);
}

#[tokio::test]
async fn test_get_wiki_shared_file_list_success() {
    let mock_server = MockServer::start().await;
    let wiki_api = setup_wiki_api(&mock_server).await;

    let expected_shared_files = vec![
        create_mock_shared_file(1, 123, "/docs", "document.pdf", 2048, 1, "john"),
        create_mock_shared_file(2, 123, "/images", "logo.png", 1024, 2, "alice"),
        create_mock_shared_file(3, 123, "/data", "data.xlsx", 4096, 1, "john"),
    ];

    Mock::given(method("GET"))
        .and(path("/api/v2/wikis/123/sharedFiles"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_shared_files))
        .mount(&mock_server)
        .await;

    let result = wiki_api
        .get_wiki_shared_file_list(GetWikiSharedFileListParams::new(WikiId::new(123)))
        .await;
    assert!(result.is_ok());
    let shared_files = result.unwrap();
    assert_eq!(shared_files.len(), 3);
    assert_eq!(shared_files[0].name, "document.pdf");
    assert_eq!(shared_files[0].dir, "/docs");
    assert_eq!(shared_files[0].id.value(), 1);
    assert_eq!(shared_files[1].name, "logo.png");
    assert_eq!(shared_files[1].dir, "/images");
    assert_eq!(shared_files[2].name, "data.xlsx");
    assert_eq!(shared_files[2].dir, "/data");
}

#[tokio::test]
async fn test_get_wiki_shared_file_list_empty() {
    let mock_server = MockServer::start().await;
    let wiki_api = setup_wiki_api(&mock_server).await;

    let expected_shared_files: Vec<SharedFile> = vec![];

    Mock::given(method("GET"))
        .and(path("/api/v2/wikis/456/sharedFiles"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_shared_files))
        .mount(&mock_server)
        .await;

    let result = wiki_api
        .get_wiki_shared_file_list(GetWikiSharedFileListParams::new(WikiId::new(456)))
        .await;
    assert!(result.is_ok());
    let shared_files = result.unwrap();
    assert_eq!(shared_files.len(), 0);
}

#[tokio::test]
async fn test_get_wiki_shared_file_list_with_u32_id() {
    let mock_server = MockServer::start().await;
    let wiki_api = setup_wiki_api(&mock_server).await;

    let expected_shared_files = vec![create_mock_shared_file(
        5,
        789,
        "/uploads",
        "report.pdf",
        3072,
        3,
        "bob",
    )];

    Mock::given(method("GET"))
        .and(path("/api/v2/wikis/789/sharedFiles"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_shared_files))
        .mount(&mock_server)
        .await;

    let result = wiki_api
        .get_wiki_shared_file_list(GetWikiSharedFileListParams::new(789u32))
        .await;
    assert!(result.is_ok());
    let shared_files = result.unwrap();
    assert_eq!(shared_files.len(), 1);
    assert_eq!(shared_files[0].name, "report.pdf");
    assert_eq!(shared_files[0].project_id.value(), 789);
}

#[tokio::test]
async fn test_get_wiki_shared_file_list_wiki_not_found() {
    let mock_server = MockServer::start().await;
    let wiki_api = setup_wiki_api(&mock_server).await;

    Mock::given(method("GET"))
        .and(path("/api/v2/wikis/999/sharedFiles"))
        .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
            "errors": [{"message": "Wiki not found"}]
        })))
        .mount(&mock_server)
        .await;

    let result = wiki_api
        .get_wiki_shared_file_list(GetWikiSharedFileListParams::new(WikiId::new(999)))
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_wiki_shared_file_list_access_forbidden() {
    let mock_server = MockServer::start().await;
    let wiki_api = setup_wiki_api(&mock_server).await;

    Mock::given(method("GET"))
        .and(path("/api/v2/wikis/777/sharedFiles"))
        .respond_with(ResponseTemplate::new(403).set_body_json(serde_json::json!({
            "errors": [{"message": "You do not have permission to access this wiki"}]
        })))
        .mount(&mock_server)
        .await;

    let result = wiki_api
        .get_wiki_shared_file_list(GetWikiSharedFileListParams::new(WikiId::new(777)))
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_wiki_shared_file_list_server_error() {
    let mock_server = MockServer::start().await;
    let wiki_api = setup_wiki_api(&mock_server).await;

    Mock::given(method("GET"))
        .and(path("/api/v2/wikis/888/sharedFiles"))
        .respond_with(ResponseTemplate::new(500).set_body_json(serde_json::json!({
            "errors": [{"message": "Internal server error"}]
        })))
        .mount(&mock_server)
        .await;

    let result = wiki_api
        .get_wiki_shared_file_list(GetWikiSharedFileListParams::new(WikiId::new(888)))
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_wiki_stars_success() {
    let mock_server = MockServer::start().await;
    let wiki_api = setup_wiki_api(&mock_server).await;

    let expected_stars = serde_json::json!([
        {
            "id": 75,
            "comment": null,
            "url": "https://xx.backlog.jp/alias/wiki/1",
            "title": "[TEST1] Home | Wiki - Backlog",
            "presenter": {
                "id": 1,
                "userId": "admin",
                "name": "admin",
                "roleType": 1,
                "lang": "ja",
                "mailAddress": "admin@example.com",
                "lastLoginTime": "2024-01-01T00:00:00Z"
            },
            "created": "2014-01-23T10:55:19Z"
        },
        {
            "id": 80,
            "comment": "Great documentation!",
            "url": "https://xx.backlog.jp/alias/wiki/2",
            "title": "[TEST2] API Guide | Wiki - Backlog",
            "presenter": {
                "id": 2,
                "userId": "user1",
                "name": "Test User",
                "roleType": 2,
                "lang": "en",
                "mailAddress": "user@example.com",
                "lastLoginTime": null
            },
            "created": "2014-02-01T15:30:00Z"
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/api/v2/wikis/123/stars"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_stars))
        .mount(&mock_server)
        .await;

    let result = wiki_api
        .get_wiki_stars(GetWikiStarsParams::new(WikiId::new(123)))
        .await;
    assert!(result.is_ok());
    let stars = result.unwrap();
    assert_eq!(stars.len(), 2);
    assert_eq!(stars[0].id, StarId::new(75));
    assert_eq!(stars[0].comment, None);
    assert_eq!(stars[0].url, "https://xx.backlog.jp/alias/wiki/1");
    assert_eq!(stars[0].title, "[TEST1] Home | Wiki - Backlog");
    assert_eq!(stars[0].presenter.name, "admin");
    assert_eq!(stars[1].id, StarId::new(80));
    assert_eq!(stars[1].comment, Some("Great documentation!".to_string()));
}

#[tokio::test]
async fn test_get_wiki_stars_empty() {
    let mock_server = MockServer::start().await;
    let wiki_api = setup_wiki_api(&mock_server).await;

    let expected_stars: Vec<serde_json::Value> = vec![];

    Mock::given(method("GET"))
        .and(path("/api/v2/wikis/456/stars"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_stars))
        .mount(&mock_server)
        .await;

    let result = wiki_api
        .get_wiki_stars(GetWikiStarsParams::new(WikiId::new(456)))
        .await;
    assert!(result.is_ok());
    let stars = result.unwrap();
    assert_eq!(stars.len(), 0);
}

#[tokio::test]
async fn test_get_wiki_stars_with_u32_id() {
    let mock_server = MockServer::start().await;
    let wiki_api = setup_wiki_api(&mock_server).await;

    let expected_stars = serde_json::json!([
        {
            "id": 100,
            "comment": "Excellent!",
            "url": "https://xx.backlog.jp/alias/wiki/789",
            "title": "Technical Specs | Wiki - Backlog",
            "presenter": {
                "id": 3,
                "userId": "reviewer",
                "name": "Reviewer",
                "roleType": 2,
                "lang": "ja",
                "mailAddress": "reviewer@example.com",
                "lastLoginTime": "2024-03-01T09:00:00Z"
            },
            "created": "2024-03-15T14:20:00Z"
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/api/v2/wikis/789/stars"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_stars))
        .mount(&mock_server)
        .await;

    let result = wiki_api
        .get_wiki_stars(GetWikiStarsParams::new(789u32))
        .await;
    assert!(result.is_ok());
    let stars = result.unwrap();
    assert_eq!(stars.len(), 1);
    assert_eq!(stars[0].id, StarId::new(100));
    assert_eq!(stars[0].comment, Some("Excellent!".to_string()));
}

#[tokio::test]
async fn test_get_wiki_stars_not_found() {
    let mock_server = MockServer::start().await;
    let wiki_api = setup_wiki_api(&mock_server).await;

    Mock::given(method("GET"))
        .and(path("/api/v2/wikis/999/stars"))
        .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
            "errors": [{"message": "Wiki not found"}]
        })))
        .mount(&mock_server)
        .await;

    let result = wiki_api
        .get_wiki_stars(GetWikiStarsParams::new(WikiId::new(999)))
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_wiki_stars_access_forbidden() {
    let mock_server = MockServer::start().await;
    let wiki_api = setup_wiki_api(&mock_server).await;

    Mock::given(method("GET"))
        .and(path("/api/v2/wikis/403/stars"))
        .respond_with(ResponseTemplate::new(403).set_body_json(serde_json::json!({
            "errors": [{"message": "You do not have permission to access this wiki"}]
        })))
        .mount(&mock_server)
        .await;

    let result = wiki_api
        .get_wiki_stars(GetWikiStarsParams::new(WikiId::new(403)))
        .await;
    assert!(result.is_err());
}
