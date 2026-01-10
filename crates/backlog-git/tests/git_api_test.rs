mod common;
use common::*;

use backlog_core::{
    ProjectIdOrKey, RepositoryIdOrName,
    identifier::{Identifier, PullRequestAttachmentId, PullRequestNumber, StatusId, UserId},
};
use backlog_git::api::{
    DownloadPullRequestAttachmentParams, GetPullRequestAttachmentListParams,
    GetPullRequestCommentCountParams, GetPullRequestCommentListParams, GetPullRequestCountParams,
    GetPullRequestListParams, GetPullRequestParams, GetRepositoryListParams, GetRepositoryParams,
};
use backlog_git::models::PrCommentOrder;
use std::str::FromStr;
use wiremock::matchers::query_param;

#[tokio::test]
async fn test_get_repository_list_success() {
    let mock_server = MockServer::start().await;
    let api = setup_git_api(&mock_server).await;

    Mock::given(method("GET"))
        .and(path("/api/v2/projects/TEST/git/repositories"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(
                    r#"[
                {
                    "id": 1,
                    "projectId": 100,
                    "name": "test-repo",
                    "description": "Test repository",
                    "hookUrl": null,
                    "httpUrl": "https://example.backlog.jp/git/TEST/test-repo.git",
                    "sshUrl": "git@example.backlog.jp:TEST/test-repo.git",
                    "displayOrder": 1,
                    "pushedAt": null,
                    "createdUser": null,
                    "created": "2023-01-01T00:00:00Z",
                    "updatedUser": null,
                    "updated": "2023-01-01T00:00:00Z"
                }
            ]"#,
                )
                .insert_header("content-type", "application/json"),
        )
        .mount(&mock_server)
        .await;

    let params = GetRepositoryListParams::new(
        ProjectIdOrKey::from_str("TEST").expect("TEST is a valid project key"),
    );
    let result = api.get_repository_list(params).await;

    let repositories = result.expect("get_repository_list should succeed");
    assert_eq!(repositories.len(), 1);
    assert_eq!(repositories[0].name, "test-repo");
}

#[tokio::test]
async fn test_get_repository_success() {
    let mock_server = MockServer::start().await;
    let api = setup_git_api(&mock_server).await;

    Mock::given(method("GET"))
        .and(path("/api/v2/projects/TEST/git/repositories/test-repo"))
        .respond_with(ResponseTemplate::new(200).set_body_string(
            r#"{
                "id": 1,
                "projectId": 100,
                "name": "test-repo",
                "description": "Test repository",
                "hookUrl": null,
                "httpUrl": "https://example.backlog.jp/git/TEST/test-repo.git",
                "sshUrl": "git@example.backlog.jp:TEST/test-repo.git",
                "displayOrder": 1,
                "pushedAt": null,
                "createdUser": null,
                "created": "2023-01-01T00:00:00Z",
                "updatedUser": null,
                "updated": "2023-01-01T00:00:00Z"
            }"#,
        ))
        .mount(&mock_server)
        .await;

    let params = GetRepositoryParams::new(
        ProjectIdOrKey::from_str("TEST").expect("TEST is a valid project key"),
        RepositoryIdOrName::from_str("test-repo").expect("test-repo is a valid repository name"),
    );
    let result = api.get_repository(params).await;

    let repository = result.expect("get_repository should succeed");
    assert_eq!(repository.name, "test-repo");
}

#[tokio::test]
async fn test_get_pull_request_list_success() {
    let mock_server = MockServer::start().await;
    let api = setup_git_api(&mock_server).await;

    Mock::given(method("GET"))
        .and(path(
            "/api/v2/projects/TEST/git/repositories/test-repo/pullRequests",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_string(
            r#"[
                {
                    "id": 1,
                    "projectId": 100,
                    "repositoryId": 1,
                    "number": 1,
                    "summary": "Test PR",
                    "description": "Test description",
                    "base": "main",
                    "branch": "feature/test",
                    "status": {"id": 1, "name": "Open"},
                    "assignee": null,
                    "issue": null,
                    "baseCommit": null,
                    "branchCommit": null,
                    "closeAt": null,
                    "mergeAt": null,
                    "createdUser": null,
                    "created": "2023-01-01T00:00:00Z",
                    "updatedUser": null,
                    "updated": "2023-01-01T00:00:00Z"
                }
            ]"#,
        ))
        .mount(&mock_server)
        .await;

    let params = GetPullRequestListParams::new(
        ProjectIdOrKey::from_str("TEST").expect("TEST is a valid project key"),
        RepositoryIdOrName::from_str("test-repo").expect("test-repo is a valid repository name"),
    );
    let result = api.get_pull_request_list(params).await;

    let pull_requests = result.expect("get_pull_request_list should succeed");
    assert_eq!(pull_requests.len(), 1);
    assert_eq!(pull_requests[0].summary, "Test PR");
}

#[tokio::test]
async fn test_get_pull_request_success() {
    let mock_server = MockServer::start().await;
    let api = setup_git_api(&mock_server).await;

    Mock::given(method("GET"))
        .and(path(
            "/api/v2/projects/TEST/git/repositories/test-repo/pullRequests/1",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_string(
            r#"{
                "id": 1,
                "projectId": 100,
                "repositoryId": 1,
                "number": 1,
                "summary": "Test PR",
                "description": "Test description",
                "base": "main",
                "branch": "feature/test",
                "status": {"id": 1, "name": "Open"},
                "assignee": null,
                "issue": null,
                "baseCommit": null,
                "branchCommit": null,
                "closeAt": null,
                "mergeAt": null,
                "createdUser": null,
                "created": "2023-01-01T00:00:00Z",
                "updatedUser": null,
                "updated": "2023-01-01T00:00:00Z"
            }"#,
        ))
        .mount(&mock_server)
        .await;

    let params = GetPullRequestParams::new(
        ProjectIdOrKey::from_str("TEST").expect("TEST is a valid project key"),
        RepositoryIdOrName::from_str("test-repo").expect("test-repo is a valid repository name"),
        PullRequestNumber::new(1),
    );
    let result = api.get_pull_request(params).await;

    let pull_request = result.expect("get_pull_request should succeed");
    assert_eq!(pull_request.summary, "Test PR");
    assert_eq!(pull_request.number.value(), 1);
}

#[tokio::test]
async fn test_get_pull_request_count_success() {
    let mock_server = MockServer::start().await;
    let api = setup_git_api(&mock_server).await;

    Mock::given(method("GET"))
        .and(path(
            "/api/v2/projects/TEST/git/repositories/test-repo/pullRequests/count",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_string(r#"{"count": 5}"#))
        .mount(&mock_server)
        .await;

    let params = GetPullRequestCountParams::new(
        ProjectIdOrKey::from_str("TEST").expect("TEST is a valid project key"),
        RepositoryIdOrName::from_str("test-repo").expect("test-repo is a valid repository name"),
    );
    let result = api.get_pull_request_count(params).await;

    let count = result.expect("get_pull_request_count should succeed");
    assert_eq!(count.count, 5);
}

#[tokio::test]
async fn test_get_pull_request_comment_list_success() {
    let mock_server = MockServer::start().await;
    let api = setup_git_api(&mock_server).await;

    Mock::given(method("GET"))
        .and(path(
            "/api/v2/projects/TEST/git/repositories/test-repo/pullRequests/1/comments",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_string(
            r#"[
                {
                    "id": 1,
                    "content": "Test comment",
                    "changeLog": [],
                    "createdUser": {
                        "id": 1,
                        "userId": "admin",
                        "name": "admin",
                        "roleType": 1,
                        "lang": "ja",
                        "mailAddress": "admin@example.com",
                        "lastLoginTime": null
                    },
                    "created": "2023-01-01T00:00:00Z",
                    "updated": "2023-01-01T00:00:00Z",
                    "stars": [],
                    "notifications": []
                }
            ]"#,
        ))
        .mount(&mock_server)
        .await;

    let params = GetPullRequestCommentListParams::new(
        ProjectIdOrKey::from_str("TEST").expect("TEST is a valid project key"),
        RepositoryIdOrName::from_str("test-repo").expect("test-repo is a valid repository name"),
        PullRequestNumber::new(1),
    );
    let result = api.get_pull_request_comment_list(params).await;

    let comments = result.expect("get_pull_request_comment_list should succeed");
    assert_eq!(comments.len(), 1);
    assert_eq!(comments[0].content, "Test comment");
}

#[tokio::test]
async fn test_get_pull_request_comment_count_success() {
    let mock_server = MockServer::start().await;
    let api = setup_git_api(&mock_server).await;

    Mock::given(method("GET"))
        .and(path(
            "/api/v2/projects/TEST/git/repositories/test-repo/pullRequests/1/comments/count",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_string(r#"{"count": 3}"#))
        .mount(&mock_server)
        .await;

    let params = GetPullRequestCommentCountParams::new(
        ProjectIdOrKey::from_str("TEST").expect("TEST is a valid project key"),
        RepositoryIdOrName::from_str("test-repo").expect("test-repo is a valid repository name"),
        PullRequestNumber::new(1),
    );
    let result = api.get_pull_request_comment_count(params).await;

    let count = result.expect("get_pull_request_comment_count should succeed");
    assert_eq!(count.count, 3);
}

#[tokio::test]
async fn test_get_pull_request_attachment_list_success() {
    let mock_server = MockServer::start().await;
    let api = setup_git_api(&mock_server).await;

    Mock::given(method("GET"))
        .and(path(
            "/api/v2/projects/TEST/git/repositories/test-repo/pullRequests/1/attachments",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_string(
            r#"[
                {
                    "id": 1,
                    "name": "test.txt",
                    "size": 1024
                }
            ]"#,
        ))
        .mount(&mock_server)
        .await;

    let params = GetPullRequestAttachmentListParams::new(
        ProjectIdOrKey::from_str("TEST").expect("TEST is a valid project key"),
        RepositoryIdOrName::from_str("test-repo").expect("test-repo is a valid repository name"),
        PullRequestNumber::new(1),
    );
    let result = api.get_pull_request_attachment_list(params).await;

    let attachments = result.expect("get_pull_request_attachment_list should succeed");
    assert_eq!(attachments.len(), 1);
    assert_eq!(attachments[0].name, "test.txt");
    assert_eq!(attachments[0].size, 1024);
}

#[tokio::test]
async fn test_download_pull_request_attachment_success() {
    let mock_server = MockServer::start().await;
    let api = setup_git_api(&mock_server).await;

    Mock::given(method("GET"))
        .and(path(
            "/api/v2/projects/TEST/git/repositories/test-repo/pullRequests/1/attachments/1",
        ))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_bytes(b"test file content")
                .append_header("content-type", "text/plain")
                .append_header("content-disposition", "attachment; filename=\"test.txt\""),
        )
        .mount(&mock_server)
        .await;

    let params = DownloadPullRequestAttachmentParams::new(
        ProjectIdOrKey::from_str("TEST").expect("TEST is a valid project key"),
        RepositoryIdOrName::from_str("test-repo").expect("test-repo is a valid repository name"),
        PullRequestNumber::new(1),
        PullRequestAttachmentId::new(1),
    );
    let result = api.download_pull_request_attachment(params).await;

    let file = result.expect("download_pull_request_attachment should succeed");
    assert_eq!(file.bytes.as_ref(), b"test file content");
    assert_eq!(file.filename, "test.txt".to_string());
}

#[tokio::test]
async fn test_get_repository_not_found() {
    let mock_server = MockServer::start().await;
    let api = setup_git_api(&mock_server).await;

    Mock::given(method("GET"))
        .and(path("/api/v2/projects/TEST/git/repositories/nonexistent"))
        .respond_with(
            ResponseTemplate::new(404)
                .set_body_string(r#"{"errors": [{"message": "Repository not found"}]}"#),
        )
        .mount(&mock_server)
        .await;

    let params = GetRepositoryParams::new(
        ProjectIdOrKey::from_str("TEST").expect("TEST is a valid project key"),
        RepositoryIdOrName::from_str("nonexistent")
            .expect("nonexistent is a valid repository name"),
    );
    let result = api.get_repository(params).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_pull_request_list_with_status_filter() {
    let mock_server = MockServer::start().await;
    let api = setup_git_api(&mock_server).await;

    Mock::given(method("GET"))
        .and(path(
            "/api/v2/projects/TEST/git/repositories/test-repo/pullRequests",
        ))
        .and(query_param("statusId[]", "1"))
        .respond_with(ResponseTemplate::new(200).set_body_string("[]"))
        .mount(&mock_server)
        .await;

    let params = GetPullRequestListParams::new(
        ProjectIdOrKey::from_str("TEST").expect("TEST is a valid project key"),
        RepositoryIdOrName::from_str("test-repo").expect("test-repo is a valid repository name"),
    )
    .status_ids(vec![StatusId::new(1)]);

    let result = api.get_pull_request_list(params).await;
    let pull_requests = result.expect("get_pull_request_list with status filter should succeed");
    assert!(pull_requests.is_empty());
}

#[tokio::test]
async fn test_get_pull_request_list_with_pagination() {
    let mock_server = MockServer::start().await;
    let api = setup_git_api(&mock_server).await;

    Mock::given(method("GET"))
        .and(path(
            "/api/v2/projects/TEST/git/repositories/test-repo/pullRequests",
        ))
        .and(query_param("offset", "10"))
        .and(query_param("count", "20"))
        .respond_with(ResponseTemplate::new(200).set_body_string("[]"))
        .mount(&mock_server)
        .await;

    let params = GetPullRequestListParams::new(
        ProjectIdOrKey::from_str("TEST").expect("TEST is a valid project key"),
        RepositoryIdOrName::from_str("test-repo").expect("test-repo is a valid repository name"),
    )
    .offset(10)
    .count(20);

    let result = api.get_pull_request_list(params).await;
    let pull_requests = result.expect("get_pull_request_list with pagination should succeed");
    assert!(pull_requests.is_empty());
}

#[tokio::test]
async fn test_get_pull_request_count_with_filters() {
    let mock_server = MockServer::start().await;
    let api = setup_git_api(&mock_server).await;

    Mock::given(method("GET"))
        .and(path(
            "/api/v2/projects/TEST/git/repositories/test-repo/pullRequests/count",
        ))
        .and(query_param("statusId[]", "1"))
        .and(query_param("assigneeId[]", "10"))
        .respond_with(ResponseTemplate::new(200).set_body_string(r#"{"count": 2}"#))
        .mount(&mock_server)
        .await;

    let params = GetPullRequestCountParams::new(
        ProjectIdOrKey::from_str("TEST").expect("TEST is a valid project key"),
        RepositoryIdOrName::from_str("test-repo").expect("test-repo is a valid repository name"),
    )
    .status_ids(vec![StatusId::new(1)])
    .assignee_ids(vec![UserId::new(10)]);

    let result = api.get_pull_request_count(params).await;
    let count = result.expect("get_pull_request_count with filters should succeed");
    assert_eq!(count.count, 2);
}

#[tokio::test]
async fn test_get_pull_request_comment_list_with_order() {
    let mock_server = MockServer::start().await;
    let api = setup_git_api(&mock_server).await;

    Mock::given(method("GET"))
        .and(path(
            "/api/v2/projects/TEST/git/repositories/test-repo/pullRequests/1/comments",
        ))
        .and(query_param("order", "desc"))
        .respond_with(ResponseTemplate::new(200).set_body_string("[]"))
        .mount(&mock_server)
        .await;

    let params = GetPullRequestCommentListParams::new(
        ProjectIdOrKey::from_str("TEST").expect("TEST is a valid project key"),
        RepositoryIdOrName::from_str("test-repo").expect("test-repo is a valid repository name"),
        PullRequestNumber::new(1),
    )
    .order(PrCommentOrder::Desc);

    let result = api.get_pull_request_comment_list(params).await;
    let comments = result.expect("get_pull_request_comment_list with order should succeed");
    assert!(comments.is_empty());
}

#[tokio::test]
async fn test_get_pull_request_comment_list_with_pagination() {
    let mock_server = MockServer::start().await;
    let api = setup_git_api(&mock_server).await;

    Mock::given(method("GET"))
        .and(path(
            "/api/v2/projects/TEST/git/repositories/test-repo/pullRequests/1/comments",
        ))
        .and(query_param("minId", "5"))
        .and(query_param("maxId", "15"))
        .and(query_param("count", "10"))
        .respond_with(ResponseTemplate::new(200).set_body_string("[]"))
        .mount(&mock_server)
        .await;

    let params = GetPullRequestCommentListParams::new(
        ProjectIdOrKey::from_str("TEST").expect("TEST is a valid project key"),
        RepositoryIdOrName::from_str("test-repo").expect("test-repo is a valid repository name"),
        PullRequestNumber::new(1),
    )
    .min_id(5)
    .max_id(15)
    .count(10);

    let result = api.get_pull_request_comment_list(params).await;
    let comments = result.expect("get_pull_request_comment_list with pagination should succeed");
    assert!(comments.is_empty());
}
