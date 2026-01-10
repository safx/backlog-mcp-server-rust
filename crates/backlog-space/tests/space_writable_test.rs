#[cfg(feature = "writable")]
mod writable_tests {
    use backlog_space::api::{SpaceApi, UpdateSpaceNotificationParams, UploadAttachmentParams};
    use client::test_utils::setup_client;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::NamedTempFile;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    async fn setup_space_api(mock_server: &MockServer) -> SpaceApi {
        let client = setup_client(mock_server).await;
        SpaceApi::new(client)
    }

    #[tokio::test]
    async fn test_upload_attachment_success() {
        let server = MockServer::start().await;
        let space_api = setup_space_api(&server).await;

        // Create a temporary test file
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let test_content = b"test file content for attachment";
        fs::write(temp_file.path(), test_content).expect("Failed to write to temp file");

        let mock_response = serde_json::json!({
            "id": 456,
            "name": "test_attachment.txt",
            "size": 32
        });

        Mock::given(method("POST"))
            .and(path("/api/v2/space/attachment"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
            .mount(&server)
            .await;

        let params = UploadAttachmentParams::new(temp_file.path().to_path_buf());
        let result = space_api.upload_attachment(params).await;

        assert!(result.is_ok());
        let attachment = result.expect("upload_attachment should succeed");
        assert_eq!(attachment.id, 456);
        assert_eq!(attachment.name, "test_attachment.txt");
        assert_eq!(attachment.size, 32);
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
            .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
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
            .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
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
