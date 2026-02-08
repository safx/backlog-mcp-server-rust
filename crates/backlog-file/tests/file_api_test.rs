mod common;
use common::*;

use backlog_api_core::Error as ApiError;
use backlog_file::{GetFileParams, GetSharedFilesListParams, SharedFile};
use std::str::FromStr;

fn create_mock_user(id: u32, name: &str) -> User {
    User {
        id: UserId::new(id),
        user_id: Some(name.to_string()),
        name: name.to_string(),
        role_type: Role::Admin,
        lang: Some(Language::Japanese),
        mail_address: format!("{name}@example.com"),
        last_login_time: Some(Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap()),
    }
}

#[tokio::test]
async fn test_get_shared_files_list_success() {
    let server = wiremock::MockServer::start().await;
    let file_api = setup_file_api(&server).await;
    let project_id = ProjectId::new(123);
    let dir_path = "documents";

    let user = create_mock_user(1, "testuser");

    let expected_files = vec![SharedFile {
        id: SharedFileId::new(1),
        project_id: ProjectId::new(123),
        dir: "/documents".to_string(),
        name: "test.txt".to_string(),
        created_user: user.clone(),
        created: Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap(),
        updated_user: Some(user.clone()),
        updated: Some(Utc.with_ymd_and_hms(2023, 1, 2, 0, 0, 0).unwrap()),
        content: backlog_file::models::FileContent::File { size: 1024 },
    }];

    Mock::given(method("GET"))
        .and(path(format!(
            "/api/v2/projects/{project_id}/files/metadata/{dir_path}"
        )))
        .and(query_param("order", "desc"))
        .and(query_param("offset", "0"))
        .and(query_param("count", "20"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_files))
        .mount(&server)
        .await;

    let params = GetSharedFilesListParams {
        project_id_or_key: project_id.into(),
        path: dir_path.to_string(),
        order: Some("desc".to_string()),
        offset: Some(0),
        count: Some(20),
    };

    let result = file_api.get_shared_files_list(params).await;
    let files = result.expect("get_shared_files_list should succeed");
    assert_eq!(files.len(), 1);
    assert_eq!(files[0].name, "test.txt");
    match &files[0].content {
        backlog_file::models::FileContent::File { size } => assert_eq!(*size, 1024),
        _ => panic!("Expected file content"),
    }
    assert_eq!(files[0].project_id.value(), 123);
}

#[tokio::test]
async fn test_get_shared_files_list_empty() {
    let server = wiremock::MockServer::start().await;
    let file_api = setup_file_api(&server).await;
    let project_key = "TEST_PROJECT";
    let dir_path = "empty";

    let expected_files: Vec<SharedFile> = Vec::new();

    Mock::given(method("GET"))
        .and(path(format!(
            "/api/v2/projects/{project_key}/files/metadata/{dir_path}"
        )))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_files))
        .mount(&server)
        .await;

    let params = GetSharedFilesListParams {
        project_id_or_key: ProjectIdOrKey::from_str(project_key).unwrap(),
        path: dir_path.to_string(),
        order: None,
        offset: None,
        count: None,
    };
    let result = file_api.get_shared_files_list(params).await;
    let files = result.expect("get_shared_files_list with empty result should succeed");
    assert!(files.is_empty());
}

#[tokio::test]
async fn test_get_shared_files_list_project_not_found() {
    let server = wiremock::MockServer::start().await;
    let file_api = setup_file_api(&server).await;
    let project_id = 999;
    let dir_path = "documents";

    let error_response = serde_json::json!({
        "errors": [
            {
                "message": "No such project.",
                "code": 6,
                "moreInfo": ""
            }
        ]
    });

    Mock::given(method("GET"))
        .and(path(format!(
            "/api/v2/projects/{project_id}/files/metadata/{dir_path}"
        )))
        .respond_with(ResponseTemplate::new(404).set_body_json(&error_response))
        .mount(&server)
        .await;

    let params = GetSharedFilesListParams {
        project_id_or_key: ProjectId::new(project_id).into(),
        path: dir_path.to_string(),
        order: None,
        offset: None,
        count: None,
    };
    let result = file_api.get_shared_files_list(params).await;
    assert!(result.is_err());
    if let Err(ApiError::HttpStatus { status, errors, .. }) = result {
        assert_eq!(status, 404);
        assert_eq!(errors[0].message, "No such project.");
    } else {
        panic!("Expected ApiError::HttpStatus, got {result:?}");
    }
}

#[tokio::test]
async fn test_get_shared_files_list_with_custom_params() {
    let server = wiremock::MockServer::start().await;
    let file_api = setup_file_api(&server).await;
    let project_id = ProjectId::new(456);
    let dir_path = "uploads";

    let expected_files: Vec<SharedFile> = Vec::new();

    Mock::given(method("GET"))
        .and(path(format!(
            "/api/v2/projects/{project_id}/files/metadata/{dir_path}"
        )))
        .and(query_param("order", "asc"))
        .and(query_param("offset", "10"))
        .and(query_param("count", "50"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_files))
        .mount(&server)
        .await;

    let params = GetSharedFilesListParams {
        project_id_or_key: project_id.into(),
        path: dir_path.to_string(),
        order: Some("asc".to_string()),
        offset: Some(10),
        count: Some(50),
    };

    let result = file_api.get_shared_files_list(params).await;
    let files = result.expect("get_shared_files_list with custom params should succeed");
    assert!(files.is_empty());
}

#[tokio::test]
async fn test_get_file_success() {
    let server = wiremock::MockServer::start().await;
    let file_api = setup_file_api(&server).await;
    let project_id = ProjectId::new(123);
    let shared_file_id = SharedFileId::new(456);

    let file_content = b"Hello, World!";
    Mock::given(method("GET"))
        .and(path(format!(
            "/api/v2/projects/{project_id}/files/{}",
            shared_file_id.value()
        )))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_bytes(file_content.as_slice())
                .insert_header("content-type", "text/plain")
                .insert_header("content-disposition", "attachment; filename=\"test.txt\""),
        )
        .mount(&server)
        .await;

    let params = GetFileParams::new(project_id, shared_file_id);
    let result = file_api.get_file(params).await;
    let downloaded_file = result.expect("get_file should succeed");
    assert_eq!(downloaded_file.filename, "test.txt");
    assert_eq!(downloaded_file.content_type, "text/plain");
    assert_eq!(downloaded_file.bytes.as_ref(), file_content);
}

#[tokio::test]
async fn test_get_file_not_found() {
    let server = wiremock::MockServer::start().await;
    let file_api = setup_file_api(&server).await;
    let project_id = ProjectId::new(123);
    let shared_file_id = SharedFileId::new(999);

    let error_response = serde_json::json!({
        "errors": [
            {
                "message": "No such file.",
                "code": 11,
                "moreInfo": ""
            }
        ]
    });

    Mock::given(method("GET"))
        .and(path(format!(
            "/api/v2/projects/{project_id}/files/{}",
            shared_file_id.value()
        )))
        .respond_with(ResponseTemplate::new(404).set_body_json(&error_response))
        .mount(&server)
        .await;

    let params = GetFileParams::new(project_id, shared_file_id);
    let result = file_api.get_file(params).await;
    assert!(result.is_err());
    if let Err(ApiError::HttpStatus { status, errors, .. }) = result {
        assert_eq!(status, 404);
        assert_eq!(errors[0].message, "No such file.");
    } else {
        panic!("Expected ApiError::HttpStatus, got {result:?}");
    }
}

#[tokio::test]
async fn test_get_shared_files_list_with_directory() {
    let server = wiremock::MockServer::start().await;
    let file_api = setup_file_api(&server).await;

    let project_id = ProjectId::new(123);
    let dir_path = "test-dir";

    let user = create_mock_user(1, "testuser");

    // ディレクトリのレスポンス（sizeフィールドなし）
    let expected_files = vec![SharedFile {
        id: SharedFileId::new(1),
        project_id: ProjectId::new(123),
        dir: "/".to_string(),
        name: "subdir".to_string(),
        created_user: user.clone(),
        created: Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap(),
        updated_user: None,
        updated: None,
        content: backlog_file::models::FileContent::Directory,
    }];

    Mock::given(method("GET"))
        .and(path(format!(
            "/api/v2/projects/{}/files/metadata/{}",
            project_id.value(),
            dir_path
        )))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_files))
        .mount(&server)
        .await;

    let params = GetSharedFilesListParams {
        project_id_or_key: project_id.into(),
        path: dir_path.to_string(),
        order: None,
        offset: None,
        count: None,
    };

    let result = file_api.get_shared_files_list(params).await;
    assert!(result.is_ok());
    let files = result.expect("get_shared_files_list with directory should succeed");
    assert_eq!(files.len(), 1);
    assert_eq!(files[0].name, "subdir");

    // ディレクトリバリアントの確認
    match &files[0].content {
        backlog_file::models::FileContent::Directory => {}
        backlog_file::models::FileContent::File { size } => {
            panic!("Expected Directory, got File with size {size}")
        }
    }
}

#[tokio::test]
async fn test_get_shared_files_list_unauthorized() {
    let server = wiremock::MockServer::start().await;
    let file_api = setup_file_api(&server).await;

    let project_id = ProjectId::new(123);
    let dir_path = "test-dir";

    Mock::given(method("GET"))
        .and(path(format!(
            "/api/v2/projects/{}/files/metadata/{}",
            project_id.value(),
            dir_path
        )))
        .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
            "errors": [{"message": "Authentication required.", "code": 11, "moreInfo": ""}]
        })))
        .mount(&server)
        .await;

    let params = GetSharedFilesListParams {
        project_id_or_key: project_id.into(),
        path: dir_path.to_string(),
        order: None,
        offset: None,
        count: None,
    };

    let result = file_api.get_shared_files_list(params).await;
    assert!(result.is_err());
    if let Err(ApiError::HttpStatus { status, .. }) = result {
        assert_eq!(status, 401);
    } else {
        panic!("Expected ApiError::HttpStatus with 401");
    }
}

#[tokio::test]
async fn test_get_shared_files_list_forbidden() {
    let server = wiremock::MockServer::start().await;
    let file_api = setup_file_api(&server).await;

    let project_id = ProjectId::new(123);
    let dir_path = "test-dir";

    Mock::given(method("GET"))
        .and(path(format!(
            "/api/v2/projects/{}/files/metadata/{}",
            project_id.value(),
            dir_path
        )))
        .respond_with(
            ResponseTemplate::new(403).set_body_json(serde_json::json!({
                "errors": [{"message": "No access permission for this resource.", "code": 11, "moreInfo": ""}]
            })),
        )
        .mount(&server)
        .await;

    let params = GetSharedFilesListParams {
        project_id_or_key: project_id.into(),
        path: dir_path.to_string(),
        order: None,
        offset: None,
        count: None,
    };

    let result = file_api.get_shared_files_list(params).await;
    assert!(result.is_err());
    if let Err(ApiError::HttpStatus { status, .. }) = result {
        assert_eq!(status, 403);
    } else {
        panic!("Expected ApiError::HttpStatus with 403");
    }
}

#[tokio::test]
async fn test_get_file_server_error() {
    let server = wiremock::MockServer::start().await;
    let file_api = setup_file_api(&server).await;

    let project_id = ProjectId::new(123);
    let shared_file_id = SharedFileId::new(456);

    Mock::given(method("GET"))
        .and(path(format!(
            "/api/v2/projects/{}/files/{}",
            project_id.value(),
            shared_file_id.value()
        )))
        .respond_with(ResponseTemplate::new(500))
        .mount(&server)
        .await;

    let params = GetFileParams::new(project_id, shared_file_id);
    let result = file_api.get_file(params).await;
    assert!(result.is_err());
}
