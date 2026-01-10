#[cfg(feature = "writable")]
mod writable_tests {
    use backlog_core::identifier::{
        Identifier, IssueId, PullRequestAttachmentId, PullRequestCommentId, PullRequestNumber,
        UserId,
    };
    use backlog_core::{ProjectIdOrKey, RepositoryIdOrName};
    use backlog_git::api::{
        AddPullRequestCommentParams, AddPullRequestParams, DeletePullRequestAttachmentParams,
        GitApi, UpdatePullRequestCommentParams, UpdatePullRequestParams,
    };
    use client::test_utils::setup_client;
    use std::str::FromStr;
    use wiremock::matchers::{body_string_contains, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    async fn setup_git_api(mock_server: &MockServer) -> GitApi {
        let client = setup_client(mock_server).await;
        GitApi::new(client)
    }

    #[tokio::test]
    async fn test_add_pull_request_success() {
        let mock_server = MockServer::start().await;
        let api = setup_git_api(&mock_server).await;

        Mock::given(method("POST"))
            .and(path(
                "/api/v2/projects/TEST/git/repositories/test-repo/pullRequests",
            ))
            .and(body_string_contains("summary=Test+PR"))
            .and(body_string_contains("description=Test+description"))
            .and(body_string_contains("base=main"))
            .and(body_string_contains("branch=feature%2Ftest"))
            .respond_with(ResponseTemplate::new(201).set_body_string(
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

        let params = AddPullRequestParams::new(
            ProjectIdOrKey::from_str("TEST").expect("TEST is a valid project key"),
            RepositoryIdOrName::from_str("test-repo")
                .expect("test-repo is a valid repository name"),
            "Test PR".to_string(),
            "Test description".to_string(),
            "main".to_string(),
            "feature/test".to_string(),
        );

        let result = api.add_pull_request(params).await;
        let pull_request = result.expect("add_pull_request should succeed");
        assert_eq!(pull_request.summary, "Test PR");
        assert_eq!(pull_request.number.value(), 1);
    }

    #[tokio::test]
    async fn test_add_pull_request_with_optional_params() {
        let mock_server = MockServer::start().await;
        let api = setup_git_api(&mock_server).await;

        Mock::given(method("POST"))
            .and(path("/api/v2/projects/TEST/git/repositories/test-repo/pullRequests"))
            .and(body_string_contains("issueId=123"))
            .and(body_string_contains("assigneeId=456"))
            .and(body_string_contains("notifiedUserId%5B%5D=789"))
            .respond_with(ResponseTemplate::new(201).set_body_string(
                r#"{
                    "id": 1,
                    "projectId": 100,
                    "repositoryId": 1,
                    "number": 1,
                    "summary": "Feature PR",
                    "description": "New feature",
                    "base": "main",
                    "branch": "feature/new",
                    "status": {"id": 1, "name": "Open"},
                    "assignee": {"id": 456, "userId": "user456", "name": "User 456", "roleType": 2, "lang": "ja", "mailAddress": "user456@example.com", "lastLoginTime": null},
                    "issue": {"id": 123},
                    "baseCommit": null,
                    "branchCommit": null,
                    "closeAt": null,
                    "mergeAt": null,
                    "createdUser": null,
                    "created": "2023-01-01T00:00:00Z",
                    "updatedUser": null,
                    "updated": "2023-01-01T00:00:00Z"
                }"#
            ))
            .mount(&mock_server)
            .await;

        let params = AddPullRequestParams::new(
            ProjectIdOrKey::from_str("TEST").expect("TEST is a valid project key"),
            RepositoryIdOrName::from_str("test-repo")
                .expect("test-repo is a valid repository name"),
            "Feature PR".to_string(),
            "New feature".to_string(),
            "main".to_string(),
            "feature/new".to_string(),
        )
        .issue_id(IssueId::new(123))
        .assignee_id(UserId::new(456))
        .notified_user_ids(vec![UserId::new(789)]);

        let result = api.add_pull_request(params).await;
        let pull_request = result.expect("add_pull_request with optional params should succeed");
        assert_eq!(pull_request.summary, "Feature PR");
    }

    #[tokio::test]
    async fn test_add_pull_request_comment_success() {
        let mock_server = MockServer::start().await;
        let api = setup_git_api(&mock_server).await;

        Mock::given(method("POST"))
            .and(path(
                "/api/v2/projects/TEST/git/repositories/test-repo/pullRequests/1/comments",
            ))
            .and(body_string_contains("content=Test+comment"))
            .respond_with(ResponseTemplate::new(201).set_body_string(
                r#"{
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
                }"#,
            ))
            .mount(&mock_server)
            .await;

        let params = AddPullRequestCommentParams::new(
            ProjectIdOrKey::from_str("TEST").expect("TEST is a valid project key"),
            RepositoryIdOrName::from_str("test-repo")
                .expect("test-repo is a valid repository name"),
            PullRequestNumber::new(1),
            "Test comment".to_string(),
        );

        let result = api.add_pull_request_comment(params).await;
        let comment = result.expect("add_pull_request_comment should succeed");
        assert_eq!(comment.content, "Test comment");
    }

    #[tokio::test]
    async fn test_update_pull_request_success() {
        let mock_server = MockServer::start().await;
        let api = setup_git_api(&mock_server).await;

        Mock::given(method("PATCH"))
            .and(path(
                "/api/v2/projects/TEST/git/repositories/test-repo/pullRequests/1",
            ))
            .and(body_string_contains("summary=Updated+PR"))
            .respond_with(ResponseTemplate::new(200).set_body_string(
                r#"{
                    "id": 1,
                    "projectId": 100,
                    "repositoryId": 1,
                    "number": 1,
                    "summary": "Updated PR",
                    "description": "Updated description",
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

        let params = UpdatePullRequestParams::new(
            ProjectIdOrKey::from_str("TEST").expect("TEST is a valid project key"),
            RepositoryIdOrName::from_str("test-repo")
                .expect("test-repo is a valid repository name"),
            PullRequestNumber::new(1),
        )
        .summary("Updated PR".to_string())
        .description("Updated description".to_string());

        let result = api.update_pull_request(params).await;
        let pull_request = result.expect("update_pull_request should succeed");
        assert_eq!(pull_request.summary, "Updated PR");
    }

    #[tokio::test]
    async fn test_update_pull_request_comment_success() {
        let mock_server = MockServer::start().await;
        let api = setup_git_api(&mock_server).await;

        Mock::given(method("PATCH"))
            .and(path(
                "/api/v2/projects/TEST/git/repositories/test-repo/pullRequests/1/comments/1",
            ))
            .and(body_string_contains("content=Updated+comment"))
            .respond_with(ResponseTemplate::new(200).set_body_string(
                r#"{
                    "id": 1,
                    "content": "Updated comment",
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
                }"#,
            ))
            .mount(&mock_server)
            .await;

        let params = UpdatePullRequestCommentParams::new(
            ProjectIdOrKey::from_str("TEST").expect("TEST is a valid project key"),
            RepositoryIdOrName::from_str("test-repo")
                .expect("test-repo is a valid repository name"),
            PullRequestNumber::new(1),
            PullRequestCommentId::new(1),
            "Updated comment".to_string(),
        );

        let result = api.update_pull_request_comment(params).await;
        let comment = result.expect("update_pull_request_comment should succeed");
        assert_eq!(comment.content, "Updated comment");
    }

    #[tokio::test]
    async fn test_delete_pull_request_attachment_success() {
        let mock_server = MockServer::start().await;
        let api = setup_git_api(&mock_server).await;

        Mock::given(method("DELETE"))
            .and(path(
                "/api/v2/projects/TEST/git/repositories/test-repo/pullRequests/1/attachments/1",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_string(
                r#"{
                    "id": 1,
                    "name": "test.txt",
                    "size": 1024
                }"#,
            ))
            .mount(&mock_server)
            .await;

        let params = DeletePullRequestAttachmentParams::new(
            ProjectIdOrKey::from_str("TEST").expect("TEST is a valid project key"),
            RepositoryIdOrName::from_str("test-repo")
                .expect("test-repo is a valid repository name"),
            PullRequestNumber::new(1),
            PullRequestAttachmentId::new(1),
        );

        let result = api.delete_pull_request_attachment(params).await;
        let attachment = result.expect("delete_pull_request_attachment should succeed");
        assert_eq!(attachment.name, "test.txt");
    }

    #[tokio::test]
    async fn test_add_pull_request_validation_error() {
        let mock_server = MockServer::start().await;
        let api = setup_git_api(&mock_server).await;

        Mock::given(method("POST"))
            .and(path(
                "/api/v2/projects/TEST/git/repositories/test-repo/pullRequests",
            ))
            .respond_with(
                ResponseTemplate::new(400)
                    .set_body_string(r#"{"errors": [{"message": "Branch does not exist"}]}"#),
            )
            .mount(&mock_server)
            .await;

        let params = AddPullRequestParams::new(
            ProjectIdOrKey::from_str("TEST").expect("TEST is a valid project key"),
            RepositoryIdOrName::from_str("test-repo")
                .expect("test-repo is a valid repository name"),
            "Test PR".to_string(),
            "Test description".to_string(),
            "main".to_string(),
            "nonexistent-branch".to_string(),
        );

        let result = api.add_pull_request(params).await;
        assert!(result.is_err());
    }
}
