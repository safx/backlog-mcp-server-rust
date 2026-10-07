#[cfg(feature = "writable")]
mod writable_tests {
    use backlog_space::api::{SpaceApi, UpdateSpaceNotificationParams, UploadAttachmentParams};
    use client::test_utils::setup_client;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::NamedTempFile;
    use wiremock::matchers::{body_string, header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    async fn setup_space_api(mock_server: &MockServer) -> SpaceApi {
        let client = setup_client(mock_server).await;
        SpaceApi::new(client)
    }

    #[tokio::test]
    async fn test_upload_attachment_success() {
        let server = MockServer::start().await;
        let space_api = setup_space_api(&server).await;

        let temp_dir = tempfile::tempdir().expect("Failed to create temp directory");
        let file_path = temp_dir.path().join("test_attachment.bin");
        let test_content = b"\x00\xfftest\r\nfile content\x80";
        fs::write(&file_path, test_content).expect("Failed to write to temp file");

        let mock_response = serde_json::json!({
            "id": 456,
            "name": "test_attachment.bin",
            "size": test_content.len()
        });

        Mock::given(method("POST"))
            .and(path("/api/v2/space/attachment"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
            .mount(&server)
            .await;

        let params = UploadAttachmentParams::new(file_path);
        let result = space_api.upload_attachment(params).await;

        assert!(result.is_ok());
        let attachment = result.expect("upload_attachment should succeed");
        assert_eq!(attachment.id, 456);
        assert_eq!(attachment.name, "test_attachment.bin");
        assert_eq!(attachment.size, test_content.len() as u64);

        let requests = server
            .received_requests()
            .await
            .expect("request recording should be enabled");
        assert_eq!(requests.len(), 1);
        let request = &requests[0];
        let content_type = request
            .headers
            .get("content-type")
            .expect("upload should have a Content-Type header")
            .to_str()
            .expect("Content-Type should be valid text");
        let boundary = content_type
            .strip_prefix("multipart/form-data; boundary=")
            .expect("upload should use multipart/form-data with a boundary")
            .trim_matches('"');
        assert!(
            request
                .body
                .starts_with(format!("--{boundary}\r\n").as_bytes())
        );

        let headers_end = request
            .body
            .windows(4)
            .position(|bytes| bytes == b"\r\n\r\n")
            .expect("file part should separate its headers and content");
        let part_headers = std::str::from_utf8(&request.body[..headers_end])
            .expect("multipart headers should be valid text");
        assert!(part_headers.lines().any(|line| {
            line == "Content-Disposition: form-data; name=\"file\"; filename=\"test_attachment.bin\""
        }));
        let uploaded_content = request.body[headers_end + 4..]
            .strip_suffix(format!("\r\n--{boundary}--\r\n").as_bytes())
            .expect("file part should end with the advertised closing boundary");
        assert_eq!(uploaded_content, test_content);
    }

    #[tokio::test]
    async fn test_upload_attachment_file_not_found() {
        let server = MockServer::start().await;
        let space_api = setup_space_api(&server).await;

        let non_existent_file = PathBuf::from("/tmp/non_existent_attachment.txt");
        let params = UploadAttachmentParams::new(non_existent_file);

        let result = space_api.upload_attachment(params).await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, backlog_api_core::Error::FileRead { .. }));
    }

    #[tokio::test]
    async fn test_upload_attachment_api_error() {
        let server = MockServer::start().await;
        let space_api = setup_space_api(&server).await;

        // Create a temporary test file
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let test_content = b"large file content";
        fs::write(temp_file.path(), test_content).expect("Failed to write to temp file");

        let error_response = serde_json::json!({
            "errors": [
                {
                    "message": "File size too large",
                    "code": 2,
                    "moreInfo": ""
                }
            ]
        });

        Mock::given(method("POST"))
            .and(path("/api/v2/space/attachment"))
            .respond_with(ResponseTemplate::new(413).set_body_json(&error_response))
            .mount(&server)
            .await;

        let params = UploadAttachmentParams::new(temp_file.path().to_path_buf());
        let result = space_api.upload_attachment(params).await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(
            err,
            backlog_api_core::Error::HttpStatus { status: 413, .. }
        ));
    }

    #[tokio::test]
    async fn test_upload_attachment_unauthorized() {
        let server = MockServer::start().await;
        let space_api = setup_space_api(&server).await;

        // Create a temporary test file
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let test_content = b"test content";
        fs::write(temp_file.path(), test_content).expect("Failed to write to temp file");

        let error_response = serde_json::json!({
            "errors": [
                {
                    "message": "Unauthorized access",
                    "code": 1,
                    "moreInfo": ""
                }
            ]
        });

        Mock::given(method("POST"))
            .and(path("/api/v2/space/attachment"))
            .respond_with(ResponseTemplate::new(401).set_body_json(&error_response))
            .mount(&server)
            .await;

        let params = UploadAttachmentParams::new(temp_file.path().to_path_buf());
        let result = space_api.upload_attachment(params).await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(
            err,
            backlog_api_core::Error::HttpStatus { status: 401, .. }
        ));
    }

    #[tokio::test]
    async fn test_update_space_notification_success() {
        let server = MockServer::start().await;
        let space_api = setup_space_api(&server).await;

        let mock_response = serde_json::json!({
            "content": "Updated space notification content",
            "updated": "2024-01-20T10:30:00Z"
        });

        Mock::given(method("PUT"))
            .and(path("/api/v2/space/notification"))
            .and(header("Content-Type", "application/x-www-form-urlencoded"))
            .and(body_string("content=Updated+space+notification+content"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
            .expect(1)
            .mount(&server)
            .await;

        let params = UpdateSpaceNotificationParams::new("Updated space notification content");
        let result = space_api.update_space_notification(params).await;

        assert!(result.is_ok());
        let notification = result.expect("update_space_notification should succeed");
        assert_eq!(notification.content, "Updated space notification content");
        assert_eq!(
            notification.updated.to_rfc3339(),
            "2024-01-20T10:30:00+00:00"
        );
    }

    #[tokio::test]
    async fn test_update_space_notification_empty_content() {
        let server = MockServer::start().await;
        let space_api = setup_space_api(&server).await;

        let mock_response = serde_json::json!({
            "content": "",
            "updated": "2024-01-20T10:35:00Z"
        });

        Mock::given(method("PUT"))
            .and(path("/api/v2/space/notification"))
            .and(header("Content-Type", "application/x-www-form-urlencoded"))
            .and(body_string("content="))
            .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
            .expect(1)
            .mount(&server)
            .await;

        let params = UpdateSpaceNotificationParams::new("");
        let result = space_api.update_space_notification(params).await;

        assert!(result.is_ok());
        let notification =
            result.expect("update_space_notification should succeed with empty content");
        assert_eq!(notification.content, "");
    }

    #[tokio::test]
    async fn test_update_space_notification_encodes_content() {
        let server = MockServer::start().await;
        let space_api = setup_space_api(&server).await;
        let content = "通知 & = + %\n";

        Mock::given(method("PUT"))
            .and(path("/api/v2/space/notification"))
            .and(header("Content-Type", "application/x-www-form-urlencoded"))
            .and(body_string("content=%E9%80%9A%E7%9F%A5+%26+%3D+%2B+%25%0A"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "content": content,
                "updated": "2024-01-20T10:30:00Z"
            })))
            .expect(1)
            .mount(&server)
            .await;

        let notification = space_api
            .update_space_notification(UpdateSpaceNotificationParams::new(content))
            .await
            .expect("notification content should be form encoded");
        assert_eq!(notification.content, content);
    }

    #[tokio::test]
    async fn test_update_space_notification_unauthorized() {
        let server = MockServer::start().await;
        let space_api = setup_space_api(&server).await;

        let error_response = serde_json::json!({
            "errors": [
                {
                    "message": "Unauthorized: Admin access required",
                    "code": 6,
                    "moreInfo": ""
                }
            ]
        });

        Mock::given(method("PUT"))
            .and(path("/api/v2/space/notification"))
            .respond_with(ResponseTemplate::new(401).set_body_json(&error_response))
            .mount(&server)
            .await;

        let params = UpdateSpaceNotificationParams::new("New notification");
        let result = space_api.update_space_notification(params).await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(
            err,
            backlog_api_core::Error::HttpStatus { status: 401, .. }
        ));
    }

    #[tokio::test]
    async fn test_update_space_notification_bad_request() {
        let server = MockServer::start().await;
        let space_api = setup_space_api(&server).await;

        let error_response = serde_json::json!({
            "errors": [
                {
                    "message": "Invalid parameters",
                    "code": 3,
                    "moreInfo": ""
                }
            ]
        });

        Mock::given(method("PUT"))
            .and(path("/api/v2/space/notification"))
            .respond_with(ResponseTemplate::new(400).set_body_json(&error_response))
            .mount(&server)
            .await;

        let params = UpdateSpaceNotificationParams::new("Invalid notification");
        let result = space_api.update_space_notification(params).await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(
            err,
            backlog_api_core::Error::HttpStatus { status: 400, .. }
        ));
    }
}
