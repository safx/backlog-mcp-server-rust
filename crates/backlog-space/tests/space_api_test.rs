mod common;

use backlog_core::identifier::Identifier;
use backlog_space::api::{
    GetLicenceParams, GetSpaceDiskUsageParams, GetSpaceLogoParams, GetSpaceNotificationParams,
    GetSpaceParams,
};
use common::*;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_get_space_success() {
    let server = MockServer::start().await;
    let space_api = setup_space_api(&server).await;

    let mock_response = serde_json::json!({
        "spaceKey": "MYSPACE",
        "name": "My Space",
        "ownerId": 1,
        "lang": "ja",
        "timezone": "Asia/Tokyo",
        "reportSendTime": "09:00",
        "textFormattingRule": "markdown",
        "created": "2024-01-01T00:00:00Z",
        "updated": "2024-01-01T00:00:00Z"
    });

    Mock::given(method("GET"))
        .and(path("/api/v2/space"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
        .mount(&server)
        .await;

    let result = space_api.get_space(GetSpaceParams::new()).await;
    assert!(result.is_ok());
    let space = result.expect("get_space should succeed");
    assert_eq!(space.name, "My Space");
}

#[tokio::test]
async fn test_get_space_logo_success() {
    let server = MockServer::start().await;
    let space_api = setup_space_api(&server).await;

    let logo_content = b"fake_logo_content";

    Mock::given(method("GET"))
        .and(path("/api/v2/space/image"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_bytes(logo_content)
                .insert_header("Content-Type", "image/png")
                .insert_header("Content-Disposition", "attachment; filename=\"logo.png\""),
        )
        .mount(&server)
        .await;

    let result = space_api.get_space_logo(GetSpaceLogoParams::new()).await;
    assert!(result.is_ok());
    let downloaded_file = result.expect("get_space_logo should succeed");
    assert_eq!(downloaded_file.filename, "logo.png");
    assert_eq!(downloaded_file.content_type, "image/png");
    assert_eq!(
        downloaded_file.bytes,
        backlog_api_core::bytes::Bytes::from(logo_content.as_slice())
    );
}

#[tokio::test]
async fn test_get_space_disk_usage_success() {
    let server = MockServer::start().await;
    let space_api = setup_space_api(&server).await;

    let mock_response = serde_json::json!({
        "capacity": 10737418240i64, // 10GB
        "issue": 1073741824i64, // 1GB
        "wiki": 536870912i64, // 512MB
        "file": 268435456i64, // 256MB
        "subversion": 0i64,
        "git": 134217728i64, // 128MB
        "gitLFS": 67108864i64, // 64MB
        "details": [
            {
                "projectId": 1,
                "issue": 536870912i64, // 512MB
                "wiki": 268435456i64, // 256MB
                "document": 134217728i64, // 128MB
                "file": 134217728i64, // 128MB
                "subversion": 0i64,
                "git": 67108864i64, // 64MB
                "gitLFS": 33554432i64 // 32MB
            },
            {
                "projectId": 2,
                "issue": 536870912i64, // 512MB
                "wiki": 268435456i64, // 256MB
                "document": 0i64,
                "file": 134217728i64, // 128MB
                "subversion": 0i64,
                "git": 67108864i64, // 64MB
                "gitLFS": 33554432i64 // 32MB
            }
        ]
    });

    Mock::given(method("GET"))
        .and(path("/api/v2/space/diskUsage"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
        .mount(&server)
        .await;

    let result = space_api
        .get_space_disk_usage(GetSpaceDiskUsageParams::new())
        .await;
    assert!(result.is_ok());
    let disk_usage = result.expect("get_space_disk_usage should succeed");
    assert_eq!(disk_usage.capacity, 10737418240);
    assert_eq!(disk_usage.issue, 1073741824);
    assert_eq!(disk_usage.details.len(), 2);
    assert_eq!(disk_usage.details[0].project_id.value(), 1);
    assert_eq!(disk_usage.details[0].issue, 536870912);
}

#[tokio::test]
async fn test_get_space_notification_success() {
    let server = MockServer::start().await;
    let space_api = setup_space_api(&server).await;

    let mock_response = serde_json::json!({
        "content": "This is a space notification",
        "updated": "2024-01-15T10:30:00Z"
    });

    Mock::given(method("GET"))
        .and(path("/api/v2/space/notification"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
        .mount(&server)
        .await;

    let result = space_api
        .get_space_notification(GetSpaceNotificationParams::new())
        .await;
    assert!(result.is_ok());
    let notification = result.expect("get_space_notification should succeed");
    assert_eq!(notification.content, "This is a space notification");
    assert_eq!(
        notification.updated.to_rfc3339(),
        "2024-01-15T10:30:00+00:00"
    );
}

#[tokio::test]
async fn test_get_space_disk_usage_forbidden() {
    let server = MockServer::start().await;
    let space_api = setup_space_api(&server).await;

    let error_response = serde_json::json!({
        "errors": [
            {
                "message": "Administrator permissions required",
                "code": 11,
                "moreInfo": ""
            }
        ]
    });

    Mock::given(method("GET"))
        .and(path("/api/v2/space/diskUsage"))
        .respond_with(ResponseTemplate::new(403).set_body_json(&error_response))
        .mount(&server)
        .await;

    let result = space_api
        .get_space_disk_usage(GetSpaceDiskUsageParams::new())
        .await;
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(matches!(
        err,
        backlog_api_core::Error::HttpStatus { status: 403, .. }
    ));
}

#[tokio::test]
async fn test_get_space_notification_error() {
    let server = MockServer::start().await;
    let space_api = setup_space_api(&server).await;

    let error_response = serde_json::json!({
        "errors": [
            {
                "message": "No permission to access this resource",
                "code": 3,
                "moreInfo": ""
            }
        ]
    });

    Mock::given(method("GET"))
        .and(path("/api/v2/space/notification"))
        .respond_with(ResponseTemplate::new(403).set_body_json(&error_response))
        .mount(&server)
        .await;

    let result = space_api
        .get_space_notification(GetSpaceNotificationParams::new())
        .await;
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(matches!(
        err,
        backlog_api_core::Error::HttpStatus { status: 403, .. }
    ));
}

#[tokio::test]
async fn test_get_space_disk_usage_with_negative_values() {
    let server = MockServer::start().await;
    let space_api = setup_space_api(&server).await;

    let mock_response = serde_json::json!({
        "capacity": 10737418240i64, // 10GB
        "issue": -2610477i64, // Negative value as reported in the issue
        "wiki": 536870912i64, // 512MB
        "file": 268435456i64, // 256MB
        "subversion": 0i64,
        "git": 134217728i64, // 128MB
        "gitLFS": 67108864i64, // 64MB
        "details": [
            {
                "projectId": 1,
                "issue": -1000000i64, // Negative value
                "wiki": 268435456i64,
                "document": 134217728i64,
                "file": 134217728i64,
                "subversion": 0i64,
                "git": 67108864i64,
                "gitLFS": 33554432i64
            }
        ]
    });

    Mock::given(method("GET"))
        .and(path("/api/v2/space/diskUsage"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
        .mount(&server)
        .await;

    let result = space_api
        .get_space_disk_usage(GetSpaceDiskUsageParams::new())
        .await;
    assert!(result.is_ok());
    let disk_usage = result.expect("get_space_disk_usage should handle negative values");
    assert_eq!(disk_usage.issue, -2610477);
    assert_eq!(disk_usage.details[0].issue, -1000000);
}

#[tokio::test]
async fn test_get_licence_success() {
    let server = MockServer::start().await;
    let space_api = setup_space_api(&server).await;

    let mock_response = serde_json::json!({
        "active": true,
        "attachmentLimit": 1073741824i64,
        "attachmentLimitPerFile": 268435456i64,
        "attachmentNumLimit": 1000,
        "attribute": true,
        "attributeLimit": 100,
        "burndown": true,
        "commentLimit": 100000,
        "componentLimit": 1000,
        "fileSharing": true,
        "gantt": true,
        "git": true,
        "issueLimit": 1000000,
        "licenceTypeId": 1,
        "limitDate": "2025-12-31T23:59:59Z",
        "nulabAccount": true,
        "parentChildIssue": true,
        "postIssueByMail": false,
        "projectLimit": 100,
        "pullRequestAttachmentLimitPerFile": 268435456i64,
        "pullRequestAttachmentNumLimit": 100,
        "remoteAddress": true,
        "remoteAddressLimit": 10,
        "startedOn": "2024-01-01T00:00:00Z",
        "storageLimit": 10737418240i64,
        "subversion": false,
        "subversionExternal": false,
        "userLimit": 500,
        "versionLimit": 100,
        "wikiAttachment": true,
        "wikiAttachmentLimitPerFile": 268435456i64,
        "wikiAttachmentNumLimit": 100
    });

    Mock::given(method("GET"))
        .and(path("/api/v2/space/licence"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
        .mount(&server)
        .await;

    let result = space_api.get_licence(GetLicenceParams::new()).await;
    assert!(result.is_ok());
    let licence = result.expect("get_licence should succeed");
    assert!(licence.active);
    assert_eq!(licence.licence_type_id, 1);
    assert_eq!(licence.user_limit, 500);
    assert_eq!(licence.project_limit, 100);
    assert!(licence.git);
    assert!(!licence.subversion);
}

#[tokio::test]
async fn test_get_licence_minimal_response() {
    let server = MockServer::start().await;
    let space_api = setup_space_api(&server).await;

    let mock_response = serde_json::json!({
        "active": false,
        "attachmentLimit": 0,
        "attachmentLimitPerFile": 0,
        "attachmentNumLimit": 0,
        "attribute": false,
        "attributeLimit": 0,
        "burndown": false,
        "commentLimit": 0,
        "componentLimit": 0,
        "fileSharing": false,
        "gantt": false,
        "git": false,
        "issueLimit": 0,
        "licenceTypeId": 0,
        "limitDate": "2024-01-01T00:00:00Z",
        "nulabAccount": false,
        "parentChildIssue": false,
        "postIssueByMail": false,
        "projectLimit": 0,
        "pullRequestAttachmentLimitPerFile": 0,
        "pullRequestAttachmentNumLimit": 0,
        "remoteAddress": false,
        "remoteAddressLimit": 0,
        "startedOn": "2024-01-01T00:00:00Z",
        "storageLimit": 0,
        "subversion": false,
        "subversionExternal": false,
        "userLimit": 0,
        "versionLimit": 0,
        "wikiAttachment": false,
        "wikiAttachmentLimitPerFile": 0,
        "wikiAttachmentNumLimit": 0
    });

    Mock::given(method("GET"))
        .and(path("/api/v2/space/licence"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
        .mount(&server)
        .await;

    let result = space_api.get_licence(GetLicenceParams::new()).await;
    assert!(result.is_ok());
    let licence = result.expect("get_licence should succeed with minimal response");
    assert!(!licence.active);
    assert_eq!(licence.licence_type_id, 0);
    assert_eq!(licence.user_limit, 0);
}

#[tokio::test]
async fn test_get_licence_unauthorized() {
    let server = MockServer::start().await;
    let space_api = setup_space_api(&server).await;

    let error_response = serde_json::json!({
        "errors": [
            {
                "message": "Authenticate error",
                "code": 6,
                "moreInfo": ""
            }
        ]
    });

    Mock::given(method("GET"))
        .and(path("/api/v2/space/licence"))
        .respond_with(ResponseTemplate::new(401).set_body_json(&error_response))
        .mount(&server)
        .await;

    let result = space_api.get_licence(GetLicenceParams::new()).await;
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(matches!(
        err,
        backlog_api_core::Error::HttpStatus { status: 401, .. }
    ));
}

#[tokio::test]
async fn test_get_space_unauthorized() {
    let server = MockServer::start().await;
    let space_api = setup_space_api(&server).await;

    let error_response = serde_json::json!({
        "errors": [
            {
                "message": "Authenticate error",
                "code": 6,
                "moreInfo": ""
            }
        ]
    });

    Mock::given(method("GET"))
        .and(path("/api/v2/space"))
        .respond_with(ResponseTemplate::new(401).set_body_json(&error_response))
        .mount(&server)
        .await;

    let result = space_api.get_space(GetSpaceParams::new()).await;
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(matches!(
        err,
        backlog_api_core::Error::HttpStatus { status: 401, .. }
    ));
}

#[tokio::test]
async fn test_get_space_internal_server_error() {
    let server = MockServer::start().await;
    let space_api = setup_space_api(&server).await;

    let error_response = serde_json::json!({
        "errors": [
            {
                "message": "Internal server error",
                "code": 1,
                "moreInfo": ""
            }
        ]
    });

    Mock::given(method("GET"))
        .and(path("/api/v2/space"))
        .respond_with(ResponseTemplate::new(500).set_body_json(&error_response))
        .mount(&server)
        .await;

    let result = space_api.get_space(GetSpaceParams::new()).await;
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(matches!(
        err,
        backlog_api_core::Error::HttpStatus { status: 500, .. }
    ));
}

#[tokio::test]
async fn test_get_space_logo_not_found() {
    let server = MockServer::start().await;
    let space_api = setup_space_api(&server).await;

    let error_response = serde_json::json!({
        "errors": [
            {
                "message": "No image",
                "code": 9,
                "moreInfo": ""
            }
        ]
    });

    Mock::given(method("GET"))
        .and(path("/api/v2/space/image"))
        .respond_with(ResponseTemplate::new(404).set_body_json(&error_response))
        .mount(&server)
        .await;

    let result = space_api.get_space_logo(GetSpaceLogoParams::new()).await;
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(matches!(
        err,
        backlog_api_core::Error::HttpStatus { status: 404, .. }
    ));
}
