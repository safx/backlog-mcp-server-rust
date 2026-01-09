#[cfg(feature = "writable")]
mod writable_tests {
    use backlog_api_core::bytes::Bytes;
    use backlog_core::identifier::{AttachmentId, CommentId, SharedFileId, UserId};
    use backlog_core::{IssueIdOrKey, IssueKey, Language, Role, User};
    use backlog_issue::api::IssueApi;
    use backlog_issue::models::{Attachment, Comment, FileContent, SharedFile};
    use backlog_issue::{
        AddCommentParamsBuilder, DeleteAttachmentParams, DeleteCommentParams, DeleteIssueParams,
        GetAttachmentFileParams, LinkSharedFilesToIssueParamsBuilder, UnlinkSharedFileParams,
        UpdateCommentParams,
    };
    use chrono::{TimeZone, Utc};
    use client::test_utils::setup_client;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn create_mock_user(id: u32, name: &str) -> User {
        User {
            id: UserId::new(id),
            user_id: Some(name.to_string()),
            name: name.to_string(),
            role_type: Role::User,
            lang: Some(Language::Japanese),
            mail_address: format!("{name}@example.com"),
            last_login_time: Some(
                chrono::DateTime::parse_from_rfc3339("2024-01-01T00:00:00Z")
                    .unwrap()
                    .with_timezone(&Utc),
            ),
        }
    }

    fn create_mock_comment(id: u32, content: &str, user_id: u32, user_name: &str) -> Comment {
        let created_time = Utc.with_ymd_and_hms(2024, 1, 1, 12, 0, 0).unwrap();
        Comment {
            id: CommentId::new(id),
            content: Some(content.to_string()),
            change_log: vec![],
            created_user: create_mock_user(user_id, user_name),
            created: created_time,
            updated: created_time,
            stars: vec![],
            notifications: vec![],
        }
    }

    fn create_mock_comment_with_updated_time(
        id: u32,
        content: &str,
        user_id: u32,
        user_name: &str,
        updated_time: chrono::DateTime<Utc>,
    ) -> Comment {
        let created_time = Utc.with_ymd_and_hms(2024, 1, 1, 12, 0, 0).unwrap();
        Comment {
            id: CommentId::new(id),
            content: Some(content.to_string()),
            change_log: vec![],
            created_user: create_mock_user(user_id, user_name),
            created: created_time,
            updated: updated_time,
            stars: vec![],
            notifications: vec![],
        }
    }

    fn create_mock_shared_file(
        id: u32,
        dir: &str,
        name: &str,
        size: Option<u64>,
        user_id: u32,
        user_name: &str,
        created_str: &str,
    ) -> SharedFile {
        SharedFile {
            id: SharedFileId::new(id),
            dir: dir.to_string(),
            name: name.to_string(),
            created_user: create_mock_user(user_id, user_name),
            created: chrono::DateTime::parse_from_rfc3339(created_str)
                .unwrap()
                .with_timezone(&Utc),
            updated_user: None,
            updated: None,
            content: match size {
                Some(s) => FileContent::File { size: s },
                None => FileContent::Directory,
            },
        }
    }

    fn create_mock_attachment(
        id: u32,
        name: &str,
        size: u64,
        user_id: u32,
        user_name: &str,
    ) -> Attachment {
        Attachment {
            id: AttachmentId::new(id),
            name: name.to_string(),
            size,
            created_user: create_mock_user(user_id, user_name),
            created: Utc.with_ymd_and_hms(2023, 10, 1, 12, 0, 0).unwrap(),
        }
    }

    #[tokio::test]
    async fn test_add_comment_success() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let issue_api = IssueApi::new(client);
        let issue_key = "TESTKEY-1";

        let expected_comment = create_mock_comment(1001, "This is a test comment", 101, "alice");

        Mock::given(method("POST"))
            .and(path(format!("/api/v2/issues/{issue_key}/comments")))
            .respond_with(ResponseTemplate::new(201).set_body_json(&expected_comment))
            .mount(&mock_server)
            .await;

        let params = AddCommentParamsBuilder::default()
            .issue_id_or_key(IssueIdOrKey::Key(issue_key.parse().unwrap()))
            .content("This is a test comment")
            .build()
            .unwrap();

        let result = issue_api.add_comment(params).await;

        assert!(result.is_ok());
        let comment = result.unwrap();
        assert_eq!(comment.id, CommentId::new(1001));
        assert_eq!(comment.content, Some("This is a test comment".to_string()));
    }

    #[tokio::test]
    async fn test_add_comment_with_notifications_and_attachments() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let issue_api = IssueApi::new(client);
        let issue_key = "TESTKEY-2";

        let expected_comment = create_mock_comment(1002, "Comment with notifications", 102, "bob");

        Mock::given(method("POST"))
            .and(path(format!("/api/v2/issues/{issue_key}/comments")))
            .respond_with(ResponseTemplate::new(201).set_body_json(&expected_comment))
            .mount(&mock_server)
            .await;

        let params = AddCommentParamsBuilder::default()
            .issue_id_or_key(IssueIdOrKey::Key(issue_key.parse().unwrap()))
            .content("Comment with notifications")
            .notified_user_id(vec![UserId::new(123), UserId::new(456)])
            .attachment_id(vec![AttachmentId::new(789)])
            .build()
            .unwrap();

        let result = issue_api.add_comment(params).await;

        if let Err(e) = &result {
            panic!("Expected success, but got error: {e:?}");
        }
        let comment = result.unwrap();
        assert_eq!(comment.id, CommentId::new(1002));
        assert_eq!(
            comment.content,
            Some("Comment with notifications".to_string())
        );
    }

    #[tokio::test]
    async fn test_add_comment_issue_not_found() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let issue_api = IssueApi::new(client);
        let issue_key = "TESTKEY-404";

        Mock::given(method("POST"))
            .and(path(format!("/api/v2/issues/{issue_key}/comments")))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let params = AddCommentParamsBuilder::default()
            .issue_id_or_key(IssueIdOrKey::Key(issue_key.parse().unwrap()))
            .content("Test comment")
            .build()
            .unwrap();

        let result = issue_api.add_comment(params).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_link_shared_files_to_issue_success() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let issue_api = IssueApi::new(client);
        let issue_key = "TESTKEY-1";

        let expected_shared_files = vec![
            create_mock_shared_file(
                123,
                "/shared",
                "document.pdf",
                Some(1024),
                101,
                "alice",
                "2023-01-01T00:00:00Z",
            ),
            create_mock_shared_file(
                456,
                "/files",
                "image.png",
                Some(2048),
                102,
                "bob",
                "2023-01-02T00:00:00Z",
            ),
        ];

        Mock::given(method("POST"))
            .and(path(format!("/api/v2/issues/{issue_key}/sharedFiles")))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_shared_files))
            .mount(&mock_server)
            .await;

        let params = LinkSharedFilesToIssueParamsBuilder::default()
            .issue_id_or_key(IssueIdOrKey::Key(issue_key.parse().unwrap()))
            .shared_file_ids(vec![SharedFileId::new(123), SharedFileId::new(456)])
            .build()
            .unwrap();

        let result = issue_api.link_shared_files_to_issue(params).await;

        assert!(result.is_ok());
        let shared_files = result.unwrap();
        assert_eq!(shared_files.len(), 2);
        assert_eq!(shared_files[0].id, SharedFileId::new(123));
        assert_eq!(shared_files[1].id, SharedFileId::new(456));
    }

    #[tokio::test]
    async fn test_link_shared_files_to_issue_single_file() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let issue_api = IssueApi::new(client);
        let issue_key = "TESTKEY-2";

        let expected_shared_file = vec![create_mock_shared_file(
            789,
            "/documents",
            "report.xlsx",
            Some(4096),
            103,
            "charlie",
            "2023-01-03T00:00:00Z",
        )];

        Mock::given(method("POST"))
            .and(path(format!("/api/v2/issues/{issue_key}/sharedFiles")))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_shared_file))
            .mount(&mock_server)
            .await;

        let params = LinkSharedFilesToIssueParamsBuilder::default()
            .issue_id_or_key(IssueIdOrKey::Key(issue_key.parse().unwrap()))
            .shared_file_ids(vec![SharedFileId::new(789)])
            .build()
            .unwrap();

        let result = issue_api.link_shared_files_to_issue(params).await;

        assert!(result.is_ok());
        let shared_files = result.unwrap();
        assert_eq!(shared_files.len(), 1);
        assert_eq!(shared_files[0].id, SharedFileId::new(789));
        assert_eq!(shared_files[0].name, "report.xlsx");
    }

    #[tokio::test]
    async fn test_get_attachment_file_success() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let issue_api = IssueApi::new(client);

        let issue_key_str = "TESTPROJ-1";
        let attachment_id_val: u32 = 101;
        let issue_id_or_key: IssueIdOrKey = issue_key_str.parse::<IssueKey>().unwrap().into();
        let attachment_id = AttachmentId::new(attachment_id_val);

        let expected_body_bytes = Bytes::from_static(b"sample file content");
        let expected_filename = "test_attachment.dat";
        let expected_content_type = "application/octet-stream";

        Mock::given(method("GET"))
            .and(path(format!(
                "/api/v2/issues/{issue_key_str}/attachments/{attachment_id_val}"
            )))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_bytes(expected_body_bytes.clone())
                    .insert_header("Content-Type", expected_content_type)
                    .insert_header(
                        "Content-Disposition",
                        format!("attachment; filename=\"{expected_filename}\""),
                    ),
            )
            .mount(&mock_server)
            .await;

        let params = GetAttachmentFileParams::new(issue_id_or_key, attachment_id);
        let result = issue_api.get_attachment_file(params).await;

        assert!(result.is_ok());
        let downloaded_file = result.unwrap();
        assert_eq!(downloaded_file.filename, expected_filename);
        assert_eq!(downloaded_file.content_type, expected_content_type);
        assert_eq!(downloaded_file.bytes, expected_body_bytes);
    }

    #[tokio::test]
    async fn test_get_attachment_file_not_found() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let issue_api = IssueApi::new(client);

        let issue_key_str = "TESTPROJ-1";
        let attachment_id_val: u32 = 999;
        let issue_id_or_key: IssueIdOrKey = issue_key_str.parse::<IssueKey>().unwrap().into();
        let attachment_id = AttachmentId::new(attachment_id_val);

        Mock::given(method("GET"))
            .and(path(format!(
                "/api/v2/issues/{issue_key_str}/attachments/{attachment_id_val}",
            )))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let params = GetAttachmentFileParams::new(issue_id_or_key, attachment_id);
        let result = issue_api.get_attachment_file(params).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_comment_success() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let issue_api = IssueApi::new(client);

        let issue_id_or_key = "MFP-2";
        let comment_id = CommentId::new(12345);
        let new_content = "Updated comment content";
        let updated_time = Utc.with_ymd_and_hms(2024, 12, 24, 15, 30, 0).unwrap();

        let expected_comment = create_mock_comment_with_updated_time(
            12345,
            new_content,
            100,
            "testuser",
            updated_time,
        );

        Mock::given(method("PATCH"))
            .and(path(format!(
                "/api/v2/issues/{issue_id_or_key}/comments/{comment_id}",
            )))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_comment))
            .mount(&mock_server)
            .await;

        let params = UpdateCommentParams {
            issue_id_or_key: IssueIdOrKey::Key(issue_id_or_key.parse::<IssueKey>().unwrap()),
            comment_id,
            content: new_content.to_string(),
        };

        let result = issue_api.update_comment(params).await;

        assert!(result.is_ok());
        let comment = result.unwrap();
        assert_eq!(comment.id, CommentId::new(12345));
        assert_eq!(comment.content.unwrap(), new_content);
        assert_eq!(comment.updated, updated_time);
    }

    #[tokio::test]
    async fn test_update_comment_issue_not_found() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let issue_api = IssueApi::new(client);

        let issue_id_or_key = "INVALID-999";
        let comment_id = CommentId::new(12345);

        Mock::given(method("PATCH"))
            .and(path(format!(
                "/api/v2/issues/{issue_id_or_key}/comments/{comment_id}",
            )))
            .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
                "errors": [{"message": "Issue not found"}]
            })))
            .mount(&mock_server)
            .await;

        let params = UpdateCommentParams {
            issue_id_or_key: IssueIdOrKey::Key(issue_id_or_key.parse::<IssueKey>().unwrap()),
            comment_id,
            content: "Updated content".to_string(),
        };

        let result = issue_api.update_comment(params).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_comment_comment_not_found() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let issue_api = IssueApi::new(client);

        let issue_id_or_key = "MFP-2";
        let comment_id = CommentId::new(99999);

        Mock::given(method("PATCH"))
            .and(path(format!(
                "/api/v2/issues/{issue_id_or_key}/comments/{comment_id}",
            )))
            .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
                "errors": [{"message": "Comment not found"}]
            })))
            .mount(&mock_server)
            .await;

        let params = UpdateCommentParams {
            issue_id_or_key: IssueIdOrKey::Key(issue_id_or_key.parse::<IssueKey>().unwrap()),
            comment_id,
            content: "Updated content".to_string(),
        };

        let result = issue_api.update_comment(params).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_comment_forbidden() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let issue_api = IssueApi::new(client);

        let issue_id_or_key = "MFP-2";
        let comment_id = CommentId::new(12345);

        Mock::given(method("PATCH"))
            .and(path(format!(
                "/api/v2/issues/{issue_id_or_key}/comments/{comment_id}",
            )))
            .respond_with(ResponseTemplate::new(403).set_body_json(serde_json::json!({
                "errors": [{"message": "You do not have permission to update this comment"}]
            })))
            .mount(&mock_server)
            .await;

        let params = UpdateCommentParams {
            issue_id_or_key: IssueIdOrKey::Key(issue_id_or_key.parse::<IssueKey>().unwrap()),
            comment_id,
            content: "Updated content".to_string(),
        };

        let result = issue_api.update_comment(params).await;
        assert!(result.is_err());
    }

    #[test]
    fn test_update_comment_params_to_form() {
        let params = UpdateCommentParams {
            issue_id_or_key: IssueIdOrKey::Key("MFP-2".parse::<IssueKey>().unwrap()),
            comment_id: CommentId::new(12345),
            content: "New comment content with 日本語".to_string(),
        };

        let form_data: Vec<(String, String)> = (&params).into();

        assert_eq!(form_data.len(), 1);
        assert_eq!(form_data[0].0, "content");
        assert_eq!(form_data[0].1, "New comment content with 日本語");
    }

    #[tokio::test]
    async fn test_delete_comment_success() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let issue_api = IssueApi::new(client);

        let issue_id_or_key = "MFP-2";
        let comment_id = CommentId::new(12345);

        let expected_comment =
            create_mock_comment(12345, "Deleted comment content", 100, "testuser");

        Mock::given(method("DELETE"))
            .and(path(format!(
                "/api/v2/issues/{issue_id_or_key}/comments/{comment_id}",
            )))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_comment))
            .mount(&mock_server)
            .await;

        let params = DeleteCommentParams {
            issue_id_or_key: IssueIdOrKey::Key(issue_id_or_key.parse::<IssueKey>().unwrap()),
            comment_id,
        };

        let result = issue_api.delete_comment(params).await;

        assert!(result.is_ok());
        let comment = result.unwrap();
        assert_eq!(comment.id, CommentId::new(12345));
        assert_eq!(comment.content.unwrap(), "Deleted comment content");
    }

    #[tokio::test]
    async fn test_delete_comment_issue_not_found() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let issue_api = IssueApi::new(client);

        let issue_id_or_key = "INVALID-999";
        let comment_id = CommentId::new(12345);

        Mock::given(method("DELETE"))
            .and(path(format!(
                "/api/v2/issues/{issue_id_or_key}/comments/{comment_id}",
            )))
            .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
                "errors": [{"message": "Issue not found"}]
            })))
            .mount(&mock_server)
            .await;

        let params = DeleteCommentParams {
            issue_id_or_key: IssueIdOrKey::Key(issue_id_or_key.parse::<IssueKey>().unwrap()),
            comment_id,
        };

        let result = issue_api.delete_comment(params).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_comment_comment_not_found() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let issue_api = IssueApi::new(client);

        let issue_id_or_key = "MFP-2";
        let comment_id = CommentId::new(99999);

        Mock::given(method("DELETE"))
            .and(path(format!(
                "/api/v2/issues/{issue_id_or_key}/comments/{comment_id}",
            )))
            .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
                "errors": [{"message": "Comment not found"}]
            })))
            .mount(&mock_server)
            .await;

        let params = DeleteCommentParams {
            issue_id_or_key: IssueIdOrKey::Key(issue_id_or_key.parse::<IssueKey>().unwrap()),
            comment_id,
        };

        let result = issue_api.delete_comment(params).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_comment_forbidden() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let issue_api = IssueApi::new(client);

        let issue_id_or_key = "MFP-2";
        let comment_id = CommentId::new(12345);

        Mock::given(method("DELETE"))
            .and(path(format!(
                "/api/v2/issues/{issue_id_or_key}/comments/{comment_id}",
            )))
            .respond_with(ResponseTemplate::new(403).set_body_json(serde_json::json!({
                "errors": [{"message": "You do not have permission to delete this comment"}]
            })))
            .mount(&mock_server)
            .await;

        let params = DeleteCommentParams {
            issue_id_or_key: IssueIdOrKey::Key(issue_id_or_key.parse::<IssueKey>().unwrap()),
            comment_id,
        };

        let result = issue_api.delete_comment(params).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_comment_minimal_params() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let issue_api = IssueApi::new(client);

        let issue_key = "ABC-123";
        let comment_id = CommentId::new(1);

        let expected_comment = create_mock_comment(1, "Minimal delete test", 200, "minuser");

        Mock::given(method("DELETE"))
            .and(path(format!(
                "/api/v2/issues/{issue_key}/comments/{comment_id}",
            )))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_comment))
            .mount(&mock_server)
            .await;

        let params = DeleteCommentParams {
            issue_id_or_key: IssueIdOrKey::Key(issue_key.parse::<IssueKey>().unwrap()),
            comment_id,
        };

        let result = issue_api.delete_comment(params).await;
        assert!(result.is_ok());
        let comment = result.unwrap();
        assert_eq!(comment.id, CommentId::new(1));
    }

    #[tokio::test]
    async fn test_unlink_shared_file_success() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let issue_api = IssueApi::new(client);

        let issue_key = "TESTPROJ-1";
        let shared_file_id = SharedFileId::new(456);

        let expected_shared_file = create_mock_shared_file(
            456,
            "/shared/docs",
            "removed_document.pdf",
            Some(2048),
            101,
            "alice",
            "2023-01-01T00:00:00Z",
        );

        Mock::given(method("DELETE"))
            .and(path(format!(
                "/api/v2/issues/{issue_key}/sharedFiles/{shared_file_id}",
            )))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_shared_file))
            .mount(&mock_server)
            .await;

        let params = UnlinkSharedFileParams {
            issue_id_or_key: IssueIdOrKey::Key(issue_key.parse::<IssueKey>().unwrap()),
            shared_file_id,
        };

        let result = issue_api.unlink_shared_file(params).await;

        assert!(result.is_ok());
        let shared_file = result.unwrap();
        assert_eq!(shared_file.id, SharedFileId::new(456));
        assert_eq!(shared_file.name, "removed_document.pdf");
        assert_eq!(shared_file.dir, "/shared/docs");
    }

    #[tokio::test]
    async fn test_unlink_shared_file_issue_not_found() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let issue_api = IssueApi::new(client);

        let issue_key = "INVALID-999";
        let shared_file_id = SharedFileId::new(456);

        Mock::given(method("DELETE"))
            .and(path(format!(
                "/api/v2/issues/{issue_key}/sharedFiles/{shared_file_id}",
            )))
            .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
                "errors": [{"message": "Issue not found"}]
            })))
            .mount(&mock_server)
            .await;

        let params = UnlinkSharedFileParams {
            issue_id_or_key: IssueIdOrKey::Key(issue_key.parse::<IssueKey>().unwrap()),
            shared_file_id,
        };

        let result = issue_api.unlink_shared_file(params).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_unlink_shared_file_file_not_found() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let issue_api = IssueApi::new(client);

        let issue_key = "TESTPROJ-1";
        let shared_file_id = SharedFileId::new(99999);

        Mock::given(method("DELETE"))
            .and(path(format!(
                "/api/v2/issues/{issue_key}/sharedFiles/{shared_file_id}",
            )))
            .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
                "errors": [{"message": "Shared file not found"}]
            })))
            .mount(&mock_server)
            .await;

        let params = UnlinkSharedFileParams {
            issue_id_or_key: IssueIdOrKey::Key(issue_key.parse::<IssueKey>().unwrap()),
            shared_file_id,
        };

        let result = issue_api.unlink_shared_file(params).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_unlink_shared_file_forbidden() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let issue_api = IssueApi::new(client);

        let issue_key = "TESTPROJ-1";
        let shared_file_id = SharedFileId::new(456);

        Mock::given(method("DELETE"))
            .and(path(format!(
                "/api/v2/issues/{issue_key}/sharedFiles/{shared_file_id}",
            )))
            .respond_with(ResponseTemplate::new(403).set_body_json(serde_json::json!({
                "errors": [{"message": "You do not have permission to unlink this shared file"}]
            })))
            .mount(&mock_server)
            .await;

        let params = UnlinkSharedFileParams {
            issue_id_or_key: IssueIdOrKey::Key(issue_key.parse::<IssueKey>().unwrap()),
            shared_file_id,
        };

        let result = issue_api.unlink_shared_file(params).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_attachment_success() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let issue_api = IssueApi::new(client);

        let issue_id_or_key = "MFP-2";
        let attachment_id = AttachmentId::new(12345);

        let expected_attachment =
            create_mock_attachment(12345, "deleted_file.pdf", 1024, 100, "testuser");

        Mock::given(method("DELETE"))
            .and(path(format!(
                "/api/v2/issues/{issue_id_or_key}/attachments/{attachment_id}",
            )))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_attachment))
            .mount(&mock_server)
            .await;

        let params = DeleteAttachmentParams {
            issue_id_or_key: IssueIdOrKey::Key(issue_id_or_key.parse::<IssueKey>().unwrap()),
            attachment_id,
        };

        let result = issue_api.delete_attachment(params).await;
        assert!(result.is_ok());
        let attachment = result.unwrap();
        assert_eq!(attachment.id, AttachmentId::new(12345));
        assert_eq!(attachment.name, "deleted_file.pdf");
        assert_eq!(attachment.size, 1024);
    }

    #[tokio::test]
    async fn test_delete_issue_success() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let issue_api = IssueApi::new(client);

        let issue_key = "TEST-123";

        // 削除される Issue のモックレスポンス
        let expected_issue = serde_json::json!({
            "id": 123,
            "projectId": 1,
            "issueKey": issue_key,
            "keyId": 123,
            "summary": "Deleted Issue",
            "description": "This issue will be deleted",
            "issueType": {
                "id": 1,
                "projectId": 1,
                "name": "Bug",
                "color": "#ff0000",
                "displayOrder": 0
            },
            "priority": {"id": 2, "name": "Normal"},
            "status": {
                "id": 1,
                "projectId": 1,
                "name": "Open",
                "color": "#ff0000",
                "displayOrder": 0
            },
            "assignee": null,
            "category": [],
            "versions": [],
            "milestone": [],
            "startDate": null,
            "dueDate": null,
            "estimatedHours": null,
            "actualHours": null,
            "parentIssueId": null,
            "createdUser": {
                "id": 1,
                "userId": "admin",
                "name": "Admin",
                "roleType": 1,
                "lang": null,
                "mailAddress": "admin@example.com",
                "lastLoginTime": null
            },
            "created": "2024-01-01T00:00:00Z",
            "updatedUser": {
                "id": 1,
                "userId": "admin",
                "name": "Admin",
                "roleType": 1,
                "lang": null,
                "mailAddress": "admin@example.com",
                "lastLoginTime": null
            },
            "updated": "2024-01-01T00:00:00Z",
            "customFields": [],
            "attachments": [],
            "sharedFiles": [],
            "stars": []
        });

        Mock::given(method("DELETE"))
            .and(path(format!("/api/v2/issues/{issue_key}")))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_issue))
            .mount(&mock_server)
            .await;

        let issue_key_parsed = issue_key
            .parse::<IssueKey>()
            .expect("issue_key should parse successfully");
        let params = DeleteIssueParams::new(issue_key_parsed.clone());
        let result = issue_api.delete_issue(params).await;

        assert!(result.is_ok());
        let issue = result.expect("delete_issue should succeed");
        assert_eq!(issue.issue_key, issue_key_parsed);
        assert_eq!(issue.summary, "Deleted Issue");
    }

    #[tokio::test]
    async fn test_delete_issue_not_found() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let issue_api = IssueApi::new(client);

        let issue_key = "INVALID-999";

        Mock::given(method("DELETE"))
            .and(path(format!("/api/v2/issues/{issue_key}")))
            .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
                "errors": [{"message": "No issue for the issueKey."}]
            })))
            .mount(&mock_server)
            .await;

        let params = DeleteIssueParams::new(
            issue_key
                .parse::<IssueKey>()
                .expect("issue_key should parse successfully"),
        );
        let result = issue_api.delete_issue(params).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_issue_forbidden() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let issue_api = IssueApi::new(client);

        let issue_key = "TEST-456";

        Mock::given(method("DELETE"))
            .and(path(format!("/api/v2/issues/{issue_key}")))
            .respond_with(ResponseTemplate::new(403).set_body_json(serde_json::json!({
                "errors": [{"message": "No delete permission for the issue."}]
            })))
            .mount(&mock_server)
            .await;

        let params = DeleteIssueParams::new(
            issue_key
                .parse::<IssueKey>()
                .expect("issue_key should parse successfully"),
        );
        let result = issue_api.delete_issue(params).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_issue_server_error() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let issue_api = IssueApi::new(client);

        let issue_key = "TEST-789";

        Mock::given(method("DELETE"))
            .and(path(format!("/api/v2/issues/{issue_key}")))
            .respond_with(ResponseTemplate::new(500).set_body_json(serde_json::json!({
                "errors": [{"message": "Internal server error"}]
            })))
            .mount(&mock_server)
            .await;

        let params = DeleteIssueParams::new(
            issue_key
                .parse::<IssueKey>()
                .expect("issue_key should parse successfully"),
        );
        let result = issue_api.delete_issue(params).await;

        assert!(result.is_err());
    }
}
