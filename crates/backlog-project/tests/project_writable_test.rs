#[cfg(feature = "writable")]
mod writable_tests {

    use backlog_api_core::Error as ApiError;
    use backlog_core::{
        ProjectKey, Role, User,
        identifier::{CategoryId, IssueTypeId, MilestoneId, ProjectId, StatusId, UserId},
    };
    use backlog_project::api::{
        AddCategoryParams, AddIssueTypeParams, AddMilestoneParams, AddProjectAdministratorParams,
        AddProjectParams, AddProjectUserParams, AddStatusParams, DeleteCategoryParams,
        DeleteIssueTypeParams, DeleteProjectAdministratorParams, DeleteProjectParams,
        DeleteProjectUserParams, DeleteStatusParams, DeleteVersionParams, ProjectApi,
        TextFormattingRule, UpdateCategoryParams, UpdateIssueTypeParams, UpdateProjectParams,
        UpdateStatusOrderParams, UpdateStatusParams, UpdateVersionParams,
    };
    use backlog_project::{Category, IssueType, Milestone, Status};
    use chrono::TimeZone;
    use client::test_utils::setup_client;
    use std::str::FromStr;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_add_category_success() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let project_api = ProjectApi::new(client);

        let expected_category = Category {
            id: CategoryId::new(1),
            project_id: ProjectId::new(123),
            name: "Backend".to_string(),
            display_order: 1,
        };

        Mock::given(method("POST"))
            .and(path("/api/v2/projects/TEST_PROJECT/categories"))
            .respond_with(ResponseTemplate::new(201).set_body_json(&expected_category))
            .mount(&mock_server)
            .await;

        let params =
            AddCategoryParams::new(ProjectKey::from_str("TEST_PROJECT").unwrap(), "Backend");
        let result = project_api.add_category(params).await;
        assert!(result.is_ok());
        let category = result.unwrap();
        assert_eq!(category.name, "Backend");
    }

    #[tokio::test]
    async fn test_add_category_duplicate_name_error() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let project_api = ProjectApi::new(client);

        let error_response = serde_json::json!({
            "errors": [
                {
                    "message": "Category name already exists",
                    "code": 10
                }
            ]
        });

        Mock::given(method("POST"))
            .and(path("/api/v2/projects/TEST_PROJECT/categories"))
            .respond_with(ResponseTemplate::new(409).set_body_json(error_response))
            .mount(&mock_server)
            .await;

        let params = AddCategoryParams::new(
            ProjectKey::from_str("TEST_PROJECT").unwrap(),
            "Existing Category",
        );
        let result = project_api.add_category(params).await;
        assert!(result.is_err());
        match result {
            Err(ApiError::HttpStatus { status, .. }) => {
                assert_eq!(status, 409);
            }
            _ => panic!("Expected HttpStatus error"),
        }
    }

    #[tokio::test]
    async fn test_delete_category_success() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let project_api = ProjectApi::new(client);

        let expected_category = Category {
            id: CategoryId::new(1),
            project_id: ProjectId::new(123),
            name: "Backend".to_string(),
            display_order: 1,
        };

        Mock::given(method("DELETE"))
            .and(path("/api/v2/projects/123/categories/1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_category))
            .mount(&mock_server)
            .await;

        let result = project_api
            .delete_category(DeleteCategoryParams::new(
                ProjectId::new(123),
                CategoryId::new(1),
            ))
            .await;
        assert!(result.is_ok());
        let category = result.unwrap();
        assert_eq!(category.name, "Backend");
    }

    #[tokio::test]
    async fn test_update_category_success() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let project_api = ProjectApi::new(client);

        let expected_category = Category {
            id: CategoryId::new(1),
            project_id: ProjectId::new(123),
            name: "Updated Backend".to_string(),
            display_order: 1,
        };

        Mock::given(method("PATCH"))
            .and(path("/api/v2/projects/123/categories/1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_category))
            .mount(&mock_server)
            .await;

        let params =
            UpdateCategoryParams::new(ProjectId::new(123), CategoryId::new(1), "Updated Backend");
        let result = project_api.update_category(params).await;
        assert!(result.is_ok());
        let category = result.unwrap();
        assert_eq!(category.name, "Updated Backend");
    }

    #[tokio::test]
    async fn test_add_issue_type_success() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let project_api = ProjectApi::new(client);

        let expected_issue_type = IssueType {
            id: IssueTypeId::new(1),
            project_id: ProjectId::new(123),
            name: "Bug".to_string(),
            color: "#e30613".to_string(),
            display_order: 1,
            template_summary: None,
            template_description: None,
        };

        Mock::given(method("POST"))
            .and(path("/api/v2/projects/123/issueTypes"))
            .respond_with(ResponseTemplate::new(201).set_body_json(&expected_issue_type))
            .mount(&mock_server)
            .await;

        let params = AddIssueTypeParams::new(
            ProjectId::new(123),
            "Bug",
            backlog_domain_models::IssueTypeColor::Red,
        );
        let result = project_api.add_issue_type(params).await;
        assert!(result.is_ok());
        let issue_type = result.unwrap();
        assert_eq!(issue_type.name, "Bug");
        assert_eq!(issue_type.color, "#e30613");
    }

    #[tokio::test]
    async fn test_add_version_success() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let project_api = ProjectApi::new(client);

        let expected_version = Milestone {
            id: MilestoneId::new(1),
            project_id: ProjectId::new(123),
            name: "Version 1.0".to_string(),
            description: Some("Initial release".to_string()),
            start_date: Some(chrono::Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap()),
            release_due_date: Some(chrono::Utc.with_ymd_and_hms(2023, 1, 31, 0, 0, 0).unwrap()),
            archived: false,
            display_order: Some(1),
        };

        Mock::given(method("POST"))
            .and(path("/api/v2/projects/123/versions"))
            .respond_with(ResponseTemplate::new(201).set_body_json(&expected_version))
            .mount(&mock_server)
            .await;

        let params = AddMilestoneParams::new(ProjectId::new(123), "Version 1.0");
        let result = project_api.add_version(params).await;
        assert!(result.is_ok());
        let version = result.unwrap();
        assert_eq!(version.name, "Version 1.0");
    }

    #[tokio::test]
    async fn test_add_status_success() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let project_api = ProjectApi::new(client);

        let expected_status = Status {
            id: StatusId::new(1),
            project_id: ProjectId::new(123),
            name: "In Review".to_string(),
            color: "#ff9900".to_string(),
            display_order: 3,
        };

        Mock::given(method("POST"))
            .and(path("/api/v2/projects/123/statuses"))
            .respond_with(ResponseTemplate::new(201).set_body_json(&expected_status))
            .mount(&mock_server)
            .await;

        let params = AddStatusParams::new(
            ProjectId::new(123),
            "In Review",
            backlog_domain_models::StatusColor::Orange,
        );
        let result = project_api.add_status(params).await;
        assert!(result.is_ok());
        let status = result.unwrap();
        assert_eq!(status.name, "In Review");
        assert_eq!(status.color, "#ff9900");
    }

    #[tokio::test]
    async fn test_update_status_success() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let project_api = ProjectApi::new(client);

        let expected_status = Status {
            id: StatusId::new(1),
            project_id: ProjectId::new(123),
            name: "Updated Status".to_string(),
            color: "#ff0000".to_string(),
            display_order: 1,
        };

        Mock::given(method("PATCH"))
            .and(path("/api/v2/projects/123/statuses/1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_status))
            .mount(&mock_server)
            .await;

        let params = UpdateStatusParams::new(ProjectId::new(123), StatusId::new(1))
            .name("Updated Status")
            .color(backlog_domain_models::StatusColor::Red);
        let result = project_api.update_status(params).await;
        assert!(result.is_ok());
        let status = result.unwrap();
        assert_eq!(status.name, "Updated Status");
        assert_eq!(status.color, "#ff0000");
    }

    #[tokio::test]
    async fn test_delete_status_success() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let project_api = ProjectApi::new(client);

        let expected_status = Status {
            id: StatusId::new(1),
            project_id: ProjectId::new(123),
            name: "Deleted Status".to_string(),
            color: "#cccccc".to_string(),
            display_order: 5,
        };

        Mock::given(method("DELETE"))
            .and(path("/api/v2/projects/123/statuses/1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_status))
            .mount(&mock_server)
            .await;

        let params =
            DeleteStatusParams::new(ProjectId::new(123), StatusId::new(1), StatusId::new(2));
        let result = project_api.delete_status(params).await;
        assert!(result.is_ok());
        let status = result.unwrap();
        assert_eq!(status.name, "Deleted Status");
    }

    #[tokio::test]
    async fn test_update_status_order_success() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let project_api = ProjectApi::new(client);

        let expected_statuses = vec![
            Status {
                id: StatusId::new(2),
                project_id: ProjectId::new(123),
                name: "In Progress".to_string(),
                color: "#00ff00".to_string(),
                display_order: 1,
            },
            Status {
                id: StatusId::new(1),
                project_id: ProjectId::new(123),
                name: "Open".to_string(),
                color: "#ff0000".to_string(),
                display_order: 2,
            },
        ];

        Mock::given(method("PATCH"))
            .and(path("/api/v2/projects/123/statuses/updateDisplayOrder"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_statuses))
            .mount(&mock_server)
            .await;

        let params = UpdateStatusOrderParams::new(
            ProjectId::new(123),
            vec![StatusId::new(2), StatusId::new(1)],
        );
        let result = project_api.update_status_order(params).await;
        assert!(result.is_ok());
        let statuses = result.unwrap();
        assert_eq!(statuses.len(), 2);
        assert_eq!(statuses[0].display_order, 1);
        assert_eq!(statuses[1].display_order, 2);
    }

    #[tokio::test]
    async fn test_add_project_user_success() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let project_api = ProjectApi::new(client);

        let expected_user = User {
            id: UserId::new(1),
            user_id: Some("john.doe".to_string()),
            name: "John Doe".to_string(),
            role_type: Role::User,
            lang: Some(backlog_core::Language::Japanese),
            mail_address: "john.doe@example.com".to_string(),
            last_login_time: Some(chrono::Utc.with_ymd_and_hms(2023, 12, 1, 10, 0, 0).unwrap()),
        };

        Mock::given(method("POST"))
            .and(path("/api/v2/projects/TEST_PROJECT/users"))
            .and(wiremock::matchers::body_string("userId=1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_user))
            .mount(&mock_server)
            .await;

        let params = AddProjectUserParams::new(ProjectKey::from_str("TEST_PROJECT").unwrap(), 1);
        let result = project_api.add_project_user(params).await;
        assert!(result.is_ok());
        let user = result.unwrap();
        assert_eq!(user.name, "John Doe");
        assert_eq!(user.mail_address, "john.doe@example.com".to_string());
    }

    #[tokio::test]
    async fn test_add_project_user_with_project_id() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let project_api = ProjectApi::new(client);

        let expected_user = User {
            id: UserId::new(2),
            user_id: Some("jane.smith".to_string()),
            name: "Jane Smith".to_string(),
            role_type: Role::Admin,
            lang: Some(backlog_core::Language::English),
            mail_address: "jane.smith@example.com".to_string(),
            last_login_time: None,
        };

        Mock::given(method("POST"))
            .and(path("/api/v2/projects/123/users"))
            .and(wiremock::matchers::body_string("userId=2"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_user))
            .mount(&mock_server)
            .await;

        let params = AddProjectUserParams::new(ProjectId::new(123), 2);
        let result = project_api.add_project_user(params).await;
        assert!(result.is_ok());
        let user = result.unwrap();
        assert_eq!(user.name, "Jane Smith");
        assert_eq!(user.role_type, Role::Admin);
    }

    #[tokio::test]
    async fn test_add_project_user_permission_error() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let project_api = ProjectApi::new(client);

        let error_response = serde_json::json!({
            "errors": [
                {
                    "message": "You do not have permission to add users to this project",
                    "code": 6
                }
            ]
        });

        Mock::given(method("POST"))
            .and(path("/api/v2/projects/TEST_PROJECT/users"))
            .respond_with(ResponseTemplate::new(401).set_body_json(error_response))
            .mount(&mock_server)
            .await;

        let params = AddProjectUserParams::new(ProjectKey::from_str("TEST_PROJECT").unwrap(), 1);
        let result = project_api.add_project_user(params).await;
        assert!(result.is_err());
        match result {
            Err(ApiError::HttpStatus { status, .. }) => {
                assert_eq!(status, 401);
            }
            _ => panic!("Expected HttpStatus error with 401"),
        }
    }

    #[tokio::test]
    async fn test_add_project_user_not_found() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let project_api = ProjectApi::new(client);

        let error_response = serde_json::json!({
            "errors": [
                {
                    "message": "No project found",
                    "code": 7
                }
            ]
        });

        Mock::given(method("POST"))
            .and(path("/api/v2/projects/INVALID_PROJECT/users"))
            .respond_with(ResponseTemplate::new(404).set_body_json(error_response))
            .mount(&mock_server)
            .await;

        let params = AddProjectUserParams::new(ProjectKey::from_str("INVALID_PROJECT").unwrap(), 1);
        let result = project_api.add_project_user(params).await;
        assert!(result.is_err());
        match result {
            Err(ApiError::HttpStatus { status, .. }) => {
                assert_eq!(status, 404);
            }
            _ => panic!("Expected HttpStatus error with 404"),
        }
    }

    #[tokio::test]
    async fn test_add_project_user_already_member() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let project_api = ProjectApi::new(client);

        let error_response = serde_json::json!({
            "errors": [
                {
                    "message": "User is already a member of this project",
                    "code": 11
                }
            ]
        });

        Mock::given(method("POST"))
            .and(path("/api/v2/projects/123/users"))
            .respond_with(ResponseTemplate::new(409).set_body_json(error_response))
            .mount(&mock_server)
            .await;

        let params = AddProjectUserParams::new(ProjectId::new(123), 1);
        let result = project_api.add_project_user(params).await;
        assert!(result.is_err());
        match result {
            Err(ApiError::HttpStatus { status, .. }) => {
                assert_eq!(status, 409);
            }
            _ => panic!("Expected HttpStatus error with 409"),
        }
    }

    #[tokio::test]
    async fn test_delete_project_user_success() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let project_api = ProjectApi::new(client);

        let expected_user = User {
            id: UserId::new(1),
            user_id: Some("john.doe".to_string()),
            name: "John Doe".to_string(),
            role_type: Role::User,
            lang: Some(backlog_core::Language::Japanese),
            mail_address: "john.doe@example.com".to_string(),
            last_login_time: Some(chrono::Utc.with_ymd_and_hms(2023, 12, 1, 10, 0, 0).unwrap()),
        };

        Mock::given(method("DELETE"))
            .and(path("/api/v2/projects/TEST_PROJECT/users"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_user))
            .mount(&mock_server)
            .await;

        let params = DeleteProjectUserParams::new(ProjectKey::from_str("TEST_PROJECT").unwrap(), 1);
        let result = project_api.delete_project_user(params).await;
        assert!(result.is_ok());
        let user = result.unwrap();
        assert_eq!(user.name, "John Doe");
        assert_eq!(user.mail_address, "john.doe@example.com".to_string());
    }

    #[tokio::test]
    async fn test_delete_project_user_with_project_id() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let project_api = ProjectApi::new(client);

        let expected_user = User {
            id: UserId::new(2),
            user_id: Some("jane.smith".to_string()),
            name: "Jane Smith".to_string(),
            role_type: Role::Admin,
            lang: Some(backlog_core::Language::English),
            mail_address: "jane.smith@example.com".to_string(),
            last_login_time: None,
        };

        Mock::given(method("DELETE"))
            .and(path("/api/v2/projects/123/users"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_user))
            .mount(&mock_server)
            .await;

        let params = DeleteProjectUserParams::new(ProjectId::new(123), 2);
        let result = project_api.delete_project_user(params).await;
        assert!(result.is_ok());
        let user = result.unwrap();
        assert_eq!(user.name, "Jane Smith");
        assert_eq!(user.role_type, Role::Admin);
    }

    #[tokio::test]
    async fn test_delete_project_user_permission_error() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let project_api = ProjectApi::new(client);

        let error_response = serde_json::json!({
            "errors": [
                {
                    "message": "You do not have permission to remove users from this project",
                    "code": 6
                }
            ]
        });

        Mock::given(method("DELETE"))
            .and(path("/api/v2/projects/TEST_PROJECT/users"))
            .respond_with(ResponseTemplate::new(401).set_body_json(error_response))
            .mount(&mock_server)
            .await;

        let params = DeleteProjectUserParams::new(ProjectKey::from_str("TEST_PROJECT").unwrap(), 1);
        let result = project_api.delete_project_user(params).await;
        assert!(result.is_err());
        match result {
            Err(ApiError::HttpStatus { status, .. }) => {
                assert_eq!(status, 401);
            }
            _ => panic!("Expected HttpStatus error with 401"),
        }
    }

    #[tokio::test]
    async fn test_delete_project_user_not_found() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let project_api = ProjectApi::new(client);

        let error_response = serde_json::json!({
            "errors": [
                {
                    "message": "No project found",
                    "code": 7
                }
            ]
        });

        Mock::given(method("DELETE"))
            .and(path("/api/v2/projects/INVALID_PROJECT/users"))
            .respond_with(ResponseTemplate::new(404).set_body_json(error_response))
            .mount(&mock_server)
            .await;

        let params =
            DeleteProjectUserParams::new(ProjectKey::from_str("INVALID_PROJECT").unwrap(), 1);
        let result = project_api.delete_project_user(params).await;
        assert!(result.is_err());
        match result {
            Err(ApiError::HttpStatus { status, .. }) => {
                assert_eq!(status, 404);
            }
            _ => panic!("Expected HttpStatus error with 404"),
        }
    }

    #[tokio::test]
    async fn test_delete_project_user_not_member() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let project_api = ProjectApi::new(client);

        let error_response = serde_json::json!({
            "errors": [
                {
                    "message": "User is not a member of this project",
                    "code": 11
                }
            ]
        });

        Mock::given(method("DELETE"))
            .and(path("/api/v2/projects/123/users"))
            .respond_with(ResponseTemplate::new(404).set_body_json(error_response))
            .mount(&mock_server)
            .await;

        let params = DeleteProjectUserParams::new(ProjectId::new(123), 999);
        let result = project_api.delete_project_user(params).await;
        assert!(result.is_err());
        match result {
            Err(ApiError::HttpStatus { status, .. }) => {
                assert_eq!(status, 404);
            }
            _ => panic!("Expected HttpStatus error with 404"),
        }
    }

    #[tokio::test]
    async fn test_add_project_administrator_success() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let project_api = ProjectApi::new(client);

        let expected_user = User {
            id: UserId::new(1),
            user_id: Some("john.doe".to_string()),
            name: "John Doe".to_string(),
            role_type: Role::Admin,
            lang: Some(backlog_core::Language::Japanese),
            mail_address: "john.doe@example.com".to_string(),
            last_login_time: Some(chrono::Utc.with_ymd_and_hms(2023, 12, 1, 10, 0, 0).unwrap()),
        };

        Mock::given(method("POST"))
            .and(path("/api/v2/projects/TEST_PROJECT/administrators"))
            .and(wiremock::matchers::body_string("userId=1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_user))
            .mount(&mock_server)
            .await;

        let params =
            AddProjectAdministratorParams::new(ProjectKey::from_str("TEST_PROJECT").unwrap(), 1);
        let result = project_api.add_project_administrator(params).await;
        assert!(result.is_ok());
        let user = result.unwrap();
        assert_eq!(user.name, "John Doe");
        assert_eq!(user.mail_address, "john.doe@example.com");
        assert_eq!(user.role_type, Role::Admin);
    }

    #[tokio::test]
    async fn test_add_project_administrator_with_project_id() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let project_api = ProjectApi::new(client);

        let expected_user = User {
            id: UserId::new(2),
            user_id: Some("jane.smith".to_string()),
            name: "Jane Smith".to_string(),
            role_type: Role::Admin,
            lang: Some(backlog_core::Language::English),
            mail_address: "jane.smith@example.com".to_string(),
            last_login_time: None,
        };

        Mock::given(method("POST"))
            .and(path("/api/v2/projects/123/administrators"))
            .and(wiremock::matchers::body_string("userId=2"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_user))
            .mount(&mock_server)
            .await;

        let params = AddProjectAdministratorParams::new(ProjectId::new(123), 2);
        let result = project_api.add_project_administrator(params).await;
        assert!(result.is_ok());
        let user = result.unwrap();
        assert_eq!(user.name, "Jane Smith");
        assert_eq!(user.role_type, Role::Admin);
    }

    #[tokio::test]
    async fn test_add_project_administrator_permission_error() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let project_api = ProjectApi::new(client);

        let error_response = serde_json::json!({
            "errors": [
                {
                    "message": "You do not have permission to add administrators to this project",
                    "code": 6
                }
            ]
        });

        Mock::given(method("POST"))
            .and(path("/api/v2/projects/TEST_PROJECT/administrators"))
            .respond_with(ResponseTemplate::new(401).set_body_json(error_response))
            .mount(&mock_server)
            .await;

        let params =
            AddProjectAdministratorParams::new(ProjectKey::from_str("TEST_PROJECT").unwrap(), 1);
        let result = project_api.add_project_administrator(params).await;
        assert!(result.is_err());
        match result {
            Err(ApiError::HttpStatus { status, .. }) => {
                assert_eq!(status, 401);
            }
            _ => panic!("Expected HttpStatus error with 401"),
        }
    }

    #[tokio::test]
    async fn test_add_project_administrator_not_found() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let project_api = ProjectApi::new(client);

        let error_response = serde_json::json!({
            "errors": [
                {
                    "message": "No project found",
                    "code": 7
                }
            ]
        });

        Mock::given(method("POST"))
            .and(path("/api/v2/projects/INVALID_PROJECT/administrators"))
            .respond_with(ResponseTemplate::new(404).set_body_json(error_response))
            .mount(&mock_server)
            .await;

        let params =
            AddProjectAdministratorParams::new(ProjectKey::from_str("INVALID_PROJECT").unwrap(), 1);
        let result = project_api.add_project_administrator(params).await;
        assert!(result.is_err());
        match result {
            Err(ApiError::HttpStatus { status, .. }) => {
                assert_eq!(status, 404);
            }
            _ => panic!("Expected HttpStatus error with 404"),
        }
    }

    #[tokio::test]
    async fn test_add_project_administrator_already_admin() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let project_api = ProjectApi::new(client);

        let error_response = serde_json::json!({
            "errors": [
                {
                    "message": "User is already an administrator of this project",
                    "code": 11
                }
            ]
        });

        Mock::given(method("POST"))
            .and(path("/api/v2/projects/123/administrators"))
            .respond_with(ResponseTemplate::new(409).set_body_json(error_response))
            .mount(&mock_server)
            .await;

        let params = AddProjectAdministratorParams::new(ProjectId::new(123), 1);
        let result = project_api.add_project_administrator(params).await;
        assert!(result.is_err());
        match result {
            Err(ApiError::HttpStatus { status, .. }) => {
                assert_eq!(status, 409);
            }
            _ => panic!("Expected HttpStatus error with 409"),
        }
    }

    #[tokio::test]
    async fn test_add_project_administrator_user_not_found() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let project_api = ProjectApi::new(client);

        let error_response = serde_json::json!({
            "errors": [
                {
                    "message": "User not found",
                    "code": 5
                }
            ]
        });

        Mock::given(method("POST"))
            .and(path("/api/v2/projects/TEST_PROJECT/administrators"))
            .respond_with(ResponseTemplate::new(404).set_body_json(error_response))
            .mount(&mock_server)
            .await;

        let params = AddProjectAdministratorParams::new(
            ProjectKey::from_str("TEST_PROJECT").unwrap(),
            999999,
        );
        let result = project_api.add_project_administrator(params).await;
        assert!(result.is_err());
        match result {
            Err(ApiError::HttpStatus { status, .. }) => {
                assert_eq!(status, 404);
            }
            _ => panic!("Expected HttpStatus error with 404"),
        }
    }

    #[tokio::test]
    async fn test_delete_project_administrator_success() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let project_api = ProjectApi::new(client);

        let response_body = serde_json::json!({
            "id": 1,
            "userId": "testuser",
            "name": "Test User",
            "roleType": 1,
            "lang": null,
            "mailAddress": "test@example.com"
        });

        Mock::given(method("DELETE"))
            .and(path("/api/v2/projects/TEST_PROJECT/administrators"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .mount(&mock_server)
            .await;

        let params =
            DeleteProjectAdministratorParams::new(ProjectKey::from_str("TEST_PROJECT").unwrap(), 1);
        let result = project_api.delete_project_administrator(params).await;
        assert!(result.is_ok());

        let user = result.unwrap();
        assert_eq!(user.id, UserId::new(1));
        assert_eq!(user.user_id, Some("testuser".to_string()));
        assert_eq!(user.name, "Test User");
    }

    #[tokio::test]
    async fn test_delete_project_administrator_unauthorized() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let project_api = ProjectApi::new(client);

        let error_response = serde_json::json!({
            "errors": [
                {
                    "message": "You do not have permission to delete administrators from this project",
                    "code": 6
                }
            ]
        });

        Mock::given(method("DELETE"))
            .and(path("/api/v2/projects/TEST_PROJECT/administrators"))
            .respond_with(ResponseTemplate::new(401).set_body_json(error_response))
            .mount(&mock_server)
            .await;

        let params =
            DeleteProjectAdministratorParams::new(ProjectKey::from_str("TEST_PROJECT").unwrap(), 1);
        let result = project_api.delete_project_administrator(params).await;
        assert!(result.is_err());
        match result {
            Err(ApiError::HttpStatus { status, .. }) => {
                assert_eq!(status, 401);
            }
            _ => panic!("Expected HttpStatus error with 401"),
        }
    }

    #[tokio::test]
    async fn test_delete_project_administrator_not_found() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let project_api = ProjectApi::new(client);

        let error_response = serde_json::json!({
            "errors": [
                {
                    "message": "No project found",
                    "code": 7
                }
            ]
        });

        Mock::given(method("DELETE"))
            .and(path("/api/v2/projects/INVALID_PROJECT/administrators"))
            .respond_with(ResponseTemplate::new(404).set_body_json(error_response))
            .mount(&mock_server)
            .await;

        let params = DeleteProjectAdministratorParams::new(
            ProjectKey::from_str("INVALID_PROJECT").unwrap(),
            1,
        );
        let result = project_api.delete_project_administrator(params).await;
        assert!(result.is_err());
        match result {
            Err(ApiError::HttpStatus { status, .. }) => {
                assert_eq!(status, 404);
            }
            _ => panic!("Expected HttpStatus error with 404"),
        }
    }

    #[tokio::test]
    async fn test_delete_project_administrator_user_not_admin() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let project_api = ProjectApi::new(client);

        let error_response = serde_json::json!({
            "errors": [
                {
                    "message": "User is not an administrator of this project",
                    "code": 11
                }
            ]
        });

        Mock::given(method("DELETE"))
            .and(path("/api/v2/projects/123/administrators"))
            .respond_with(ResponseTemplate::new(409).set_body_json(error_response))
            .mount(&mock_server)
            .await;

        let params = DeleteProjectAdministratorParams::new(ProjectId::new(123), 1);
        let result = project_api.delete_project_administrator(params).await;
        assert!(result.is_err());
        match result {
            Err(ApiError::HttpStatus { status, .. }) => {
                assert_eq!(status, 409);
            }
            _ => panic!("Expected HttpStatus error with 409"),
        }
    }

    #[tokio::test]
    async fn test_delete_project_administrator_user_not_found() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let project_api = ProjectApi::new(client);

        let error_response = serde_json::json!({
            "errors": [
                {
                    "message": "No user found",
                    "code": 5
                }
            ]
        });

        Mock::given(method("DELETE"))
            .and(path("/api/v2/projects/TEST_PROJECT/administrators"))
            .respond_with(ResponseTemplate::new(404).set_body_json(error_response))
            .mount(&mock_server)
            .await;

        let params = DeleteProjectAdministratorParams::new(
            ProjectKey::from_str("TEST_PROJECT").unwrap(),
            999999,
        );
        let result = project_api.delete_project_administrator(params).await;
        assert!(result.is_err());
        match result {
            Err(ApiError::HttpStatus { status, .. }) => {
                assert_eq!(status, 404);
            }
            _ => panic!("Expected HttpStatus error with 404"),
        }
    }

    #[tokio::test]
    async fn test_update_project_success() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let project_api = ProjectApi::new(client);

        let expected_project = serde_json::json!({
            "id": 123,
            "projectKey": "TEST_PROJECT",
            "name": "Updated Test Project",
            "chartEnabled": true,
            "useResolvedForChart": false,
            "subtaskingEnabled": true,
            "projectLeaderCanEditProjectLeader": true,
            "useWiki": true,
            "useFileSharing": true,
            "useWikiTreeView": true,
            "useSubversion": false,
            "useGit": true,
            "useOriginalImageSizeAtWiki": false,
            "textFormattingRule": "markdown",
            "archived": false,
            "displayOrder": 1,
            "useDevAttributes": true
        });

        Mock::given(method("PATCH"))
            .and(path("/api/v2/projects/TEST_PROJECT"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_project))
            .mount(&mock_server)
            .await;

        let params = UpdateProjectParams::new(ProjectKey::from_str("TEST_PROJECT").unwrap())
            .name("Updated Test Project")
            .chart_enabled(true)
            .use_git(true)
            .text_formatting_rule(TextFormattingRule::Markdown);

        let result = project_api.update_project(params).await;
        assert!(result.is_ok());
        let project = result.unwrap();
        assert_eq!(project.name, "Updated Test Project");
        assert_eq!(
            project.project_key,
            ProjectKey::from_str("TEST_PROJECT").unwrap()
        );
    }

    #[tokio::test]
    async fn test_update_project_with_project_id() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let project_api = ProjectApi::new(client);

        let expected_project = serde_json::json!({
            "id": 123,
            "projectKey": "TEST_PROJECT",
            "name": "Updated Project Name",
            "chartEnabled": false,
            "useResolvedForChart": false,
            "subtaskingEnabled": false,
            "projectLeaderCanEditProjectLeader": false,
            "useWiki": false,
            "useFileSharing": false,
            "useWikiTreeView": false,
            "useSubversion": false,
            "useGit": false,
            "useOriginalImageSizeAtWiki": false,
            "textFormattingRule": "backlog",
            "archived": true,
            "displayOrder": 1,
            "useDevAttributes": false
        });

        Mock::given(method("PATCH"))
            .and(path("/api/v2/projects/123"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_project))
            .mount(&mock_server)
            .await;

        let params = UpdateProjectParams::new(ProjectId::new(123))
            .name("Updated Project Name")
            .archived(true)
            .text_formatting_rule(TextFormattingRule::Backlog);

        let result = project_api.update_project(params).await;
        assert!(result.is_ok());
        let project = result.unwrap();
        assert_eq!(project.name, "Updated Project Name");
        assert!(project.archived);
    }

    #[tokio::test]
    async fn test_update_project_all_params() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let project_api = ProjectApi::new(client);

        let expected_project = serde_json::json!({
            "id": 123,
            "projectKey": "NEWKEY",
            "name": "Complete Update Project",
            "chartEnabled": true,
            "useResolvedForChart": true,
            "subtaskingEnabled": true,
            "projectLeaderCanEditProjectLeader": true,
            "useWiki": true,
            "useFileSharing": true,
            "useWikiTreeView": true,
            "useSubversion": true,
            "useGit": true,
            "useOriginalImageSizeAtWiki": true,
            "textFormattingRule": "markdown",
            "archived": false,
            "displayOrder": 1,
            "useDevAttributes": true
        });

        Mock::given(method("PATCH"))
            .and(path("/api/v2/projects/123"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_project))
            .mount(&mock_server)
            .await;

        let params = UpdateProjectParams::new(ProjectId::new(123))
            .name("Complete Update Project")
            .key("NEWKEY")
            .chart_enabled(true)
            .use_resolved_for_chart(true)
            .subtasking_enabled(true)
            .project_leader_can_edit_project_leader(true)
            .use_wiki(true)
            .use_file_sharing(true)
            .use_wiki_tree_view(true)
            .use_subversion(true)
            .use_git(true)
            .use_original_image_size_at_wiki(true)
            .text_formatting_rule(TextFormattingRule::Markdown)
            .archived(false)
            .use_dev_attributes(true);

        let result = project_api.update_project(params).await;
        assert!(result.is_ok());
        let project = result.unwrap();
        assert_eq!(project.name, "Complete Update Project");
        assert_eq!(project.project_key, ProjectKey::from_str("NEWKEY").unwrap());
    }

    #[tokio::test]
    async fn test_update_project_permission_error() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let project_api = ProjectApi::new(client);

        let error_response = serde_json::json!({
            "errors": [
                {
                    "message": "You do not have permission to update this project",
                    "code": 6
                }
            ]
        });

        Mock::given(method("PATCH"))
            .and(path("/api/v2/projects/TEST_PROJECT"))
            .respond_with(ResponseTemplate::new(401).set_body_json(error_response))
            .mount(&mock_server)
            .await;

        let params = UpdateProjectParams::new(ProjectKey::from_str("TEST_PROJECT").unwrap())
            .name("Unauthorized Update");

        let result = project_api.update_project(params).await;
        assert!(result.is_err());
        match result {
            Err(ApiError::HttpStatus { status, .. }) => {
                assert_eq!(status, 401);
            }
            _ => panic!("Expected HttpStatus error with 401"),
        }
    }

    #[tokio::test]
    async fn test_update_project_not_found() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let project_api = ProjectApi::new(client);

        let error_response = serde_json::json!({
            "errors": [
                {
                    "message": "No project found",
                    "code": 7
                }
            ]
        });

        Mock::given(method("PATCH"))
            .and(path("/api/v2/projects/INVALID_PROJECT"))
            .respond_with(ResponseTemplate::new(404).set_body_json(error_response))
            .mount(&mock_server)
            .await;

        let params = UpdateProjectParams::new(ProjectKey::from_str("INVALID_PROJECT").unwrap())
            .name("Not Found Project");

        let result = project_api.update_project(params).await;
        assert!(result.is_err());
        match result {
            Err(ApiError::HttpStatus { status, .. }) => {
                assert_eq!(status, 404);
            }
            _ => panic!("Expected HttpStatus error with 404"),
        }
    }

    #[tokio::test]
    async fn test_update_project_duplicate_key() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let project_api = ProjectApi::new(client);

        let error_response = serde_json::json!({
            "errors": [
                {
                    "message": "Project key already exists",
                    "code": 10
                }
            ]
        });

        Mock::given(method("PATCH"))
            .and(path("/api/v2/projects/123"))
            .respond_with(ResponseTemplate::new(409).set_body_json(error_response))
            .mount(&mock_server)
            .await;

        let params = UpdateProjectParams::new(ProjectId::new(123)).key("EXISTING_KEY");

        let result = project_api.update_project(params).await;
        assert!(result.is_err());
        match result {
            Err(ApiError::HttpStatus { status, .. }) => {
                assert_eq!(status, 409);
            }
            _ => panic!("Expected HttpStatus error with 409"),
        }
    }

    #[tokio::test]
    async fn test_add_project_success() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let project_api = ProjectApi::new(client);

        let expected_project = serde_json::json!({
            "id": 456,
            "projectKey": "NEWPROJECT",
            "name": "New Test Project",
            "chartEnabled": true,
            "subtaskingEnabled": true,
            "projectLeaderCanEditProjectLeader": true,
            "useWiki": true,
            "useFileSharing": true,
            "useWikiTreeView": true,
            "useOriginalImageSizeAtWiki": false,
            "textFormattingRule": "markdown",
            "archived": false,
            "displayOrder": 1,
            "useDevAttributes": true
        });

        Mock::given(method("POST"))
            .and(path("/api/v2/projects"))
            .respond_with(ResponseTemplate::new(201).set_body_json(&expected_project))
            .mount(&mock_server)
            .await;

        let params = AddProjectParams::new("New Test Project", "NEWPROJECT")
            .chart_enabled(true)
            .subtasking_enabled(true)
            .project_leader_can_edit_project_leader(true)
            .use_wiki(true)
            .use_file_sharing(true)
            .use_wiki_tree_view(true)
            .use_git(true)
            .text_formatting_rule("markdown")
            .use_dev_attributes(true);

        let result = project_api.add_project(params).await;
        assert!(result.is_ok());
        let project = result.unwrap();
        assert_eq!(project.name, "New Test Project");
        assert_eq!(
            project.project_key,
            ProjectKey::from_str("NEWPROJECT").unwrap()
        );
        assert_eq!(project.id, ProjectId::new(456));
        assert!(project.chart_enabled);
        assert!(project.use_wiki);
    }

    #[tokio::test]
    async fn test_add_project_minimal_params() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let project_api = ProjectApi::new(client);

        let expected_project = serde_json::json!({
            "id": 789,
            "projectKey": "MINIMAL",
            "name": "Minimal Project",
            "chartEnabled": false,
            "subtaskingEnabled": false,
            "projectLeaderCanEditProjectLeader": false,
            "useWiki": false,
            "useFileSharing": false,
            "useWikiTreeView": false,
            "useOriginalImageSizeAtWiki": false,
            "textFormattingRule": "backlog",
            "archived": false,
            "displayOrder": 1,
            "useDevAttributes": false
        });

        Mock::given(method("POST"))
            .and(path("/api/v2/projects"))
            .respond_with(ResponseTemplate::new(201).set_body_json(&expected_project))
            .mount(&mock_server)
            .await;

        let params = AddProjectParams::new("Minimal Project", "MINIMAL");
        let result = project_api.add_project(params).await;
        assert!(result.is_ok());
        let project = result.unwrap();
        assert_eq!(project.name, "Minimal Project");
        assert_eq!(
            project.project_key,
            ProjectKey::from_str("MINIMAL").unwrap()
        );
        assert_eq!(project.id, ProjectId::new(789));
    }

    #[tokio::test]
    async fn test_add_project_duplicate_key_error() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let project_api = ProjectApi::new(client);

        let error_response = serde_json::json!({
            "errors": [
                {
                    "message": "Project key already exists",
                    "code": 10
                }
            ]
        });

        Mock::given(method("POST"))
            .and(path("/api/v2/projects"))
            .respond_with(ResponseTemplate::new(409).set_body_json(error_response))
            .mount(&mock_server)
            .await;

        let params = AddProjectParams::new("Duplicate Project", "EXISTING");
        let result = project_api.add_project(params).await;
        assert!(result.is_err());
        match result {
            Err(ApiError::HttpStatus { status, .. }) => {
                assert_eq!(status, 409);
            }
            _ => panic!("Expected HttpStatus error with 409"),
        }
    }

    #[tokio::test]
    async fn test_add_project_permission_error() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let project_api = ProjectApi::new(client);

        let error_response = serde_json::json!({
            "errors": [
                {
                    "message": "You do not have permission to add projects",
                    "code": 6
                }
            ]
        });

        Mock::given(method("POST"))
            .and(path("/api/v2/projects"))
            .respond_with(ResponseTemplate::new(401).set_body_json(error_response))
            .mount(&mock_server)
            .await;

        let params = AddProjectParams::new("Unauthorized Project", "UNAUTH");
        let result = project_api.add_project(params).await;
        assert!(result.is_err());
        match result {
            Err(ApiError::HttpStatus { status, .. }) => {
                assert_eq!(status, 401);
            }
            _ => panic!("Expected HttpStatus error with 401"),
        }
    }

    #[tokio::test]
    async fn test_add_project_invalid_key_format() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let project_api = ProjectApi::new(client);

        let error_response = serde_json::json!({
            "errors": [
                {
                    "message": "Invalid project key format",
                    "code": 3
                }
            ]
        });

        Mock::given(method("POST"))
            .and(path("/api/v2/projects"))
            .respond_with(ResponseTemplate::new(400).set_body_json(error_response))
            .mount(&mock_server)
            .await;

        let params = AddProjectParams::new("Invalid Key Project", "invalid-key");
        let result = project_api.add_project(params).await;
        assert!(result.is_err());
        match result {
            Err(ApiError::HttpStatus { status, .. }) => {
                assert_eq!(status, 400);
            }
            _ => panic!("Expected HttpStatus error with 400"),
        }
    }

    #[tokio::test]
    async fn test_delete_project_success() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let project_api = ProjectApi::new(client);

        let deleted_project = serde_json::json!({
            "id": 123,
            "projectKey": "TEST_PROJECT",
            "name": "Test Project",
            "chartEnabled": true,
            "useResolvedForChart": false,
            "subtaskingEnabled": true,
            "projectLeaderCanEditProjectLeader": true,
            "useWiki": true,
            "useFileSharing": true,
            "useWikiTreeView": true,
            "useOriginalImageSizeAtWiki": false,
            "textFormattingRule": "markdown",
            "archived": false,
            "displayOrder": 0,
            "useDevAttributes": true
        });

        Mock::given(method("DELETE"))
            .and(path("/api/v2/projects/TEST_PROJECT"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&deleted_project))
            .mount(&mock_server)
            .await;

        let params = DeleteProjectParams::new(ProjectKey::from_str("TEST_PROJECT").unwrap());
        let result = project_api.delete_project(params).await;
        assert!(result.is_ok());
        let project = result.unwrap();
        assert_eq!(project.id, ProjectId::new(123));
        assert_eq!(project.project_key.to_string(), "TEST_PROJECT");
        assert_eq!(project.name, "Test Project");
    }

    #[tokio::test]
    async fn test_delete_project_unauthorized() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let project_api = ProjectApi::new(client);

        let error_response = serde_json::json!({
            "errors": [
                {
                    "message": "You do not have permission to delete this project",
                    "code": 6
                }
            ]
        });

        Mock::given(method("DELETE"))
            .and(path("/api/v2/projects/TEST_PROJECT"))
            .respond_with(ResponseTemplate::new(401).set_body_json(error_response))
            .mount(&mock_server)
            .await;

        let params = DeleteProjectParams::new(ProjectKey::from_str("TEST_PROJECT").unwrap());
        let result = project_api.delete_project(params).await;
        assert!(result.is_err());
        match result {
            Err(ApiError::HttpStatus { status, .. }) => {
                assert_eq!(status, 401);
            }
            _ => panic!("Expected HttpStatus error with 401"),
        }
    }

    #[tokio::test]
    async fn test_delete_project_not_found() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let project_api = ProjectApi::new(client);

        let error_response = serde_json::json!({
            "errors": [
                {
                    "message": "No project found",
                    "code": 7
                }
            ]
        });

        Mock::given(method("DELETE"))
            .and(path("/api/v2/projects/INVALID_PROJECT"))
            .respond_with(ResponseTemplate::new(404).set_body_json(error_response))
            .mount(&mock_server)
            .await;

        let params = DeleteProjectParams::new(ProjectKey::from_str("INVALID_PROJECT").unwrap());
        let result = project_api.delete_project(params).await;
        assert!(result.is_err());
        match result {
            Err(ApiError::HttpStatus { status, .. }) => {
                assert_eq!(status, 404);
            }
            _ => panic!("Expected HttpStatus error with 404"),
        }
    }

    #[tokio::test]
    async fn test_update_issue_type_success() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let project_api = ProjectApi::new(client);

        let expected_issue_type = IssueType {
            id: IssueTypeId::new(1),
            project_id: ProjectId::new(123),
            name: "Updated Bug".to_string(),
            color: "#e30613".to_string(),
            display_order: 1,
            template_summary: None,
            template_description: None,
        };

        Mock::given(method("PATCH"))
            .and(path("/api/v2/projects/123/issueTypes/1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_issue_type))
            .mount(&mock_server)
            .await;

        let mut params = UpdateIssueTypeParams::new(ProjectId::new(123), IssueTypeId::new(1));
        params.name = Some("Updated Bug".to_string());

        let result = project_api.update_issue_type(params).await;
        assert!(result.is_ok());
        let issue_type = result.unwrap();
        assert_eq!(issue_type.name, "Updated Bug");
        assert_eq!(issue_type.id, IssueTypeId::new(1));
    }

    #[tokio::test]
    async fn test_update_issue_type_with_all_fields() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let project_api = ProjectApi::new(client);

        let expected_issue_type = IssueType {
            id: IssueTypeId::new(1),
            project_id: ProjectId::new(123),
            name: "Updated Feature".to_string(),
            color: "#007e9a".to_string(),
            display_order: 1,
            template_summary: Some("Feature summary template".to_string()),
            template_description: Some("Feature description template".to_string()),
        };

        Mock::given(method("PATCH"))
            .and(path("/api/v2/projects/TEST_PROJECT/issueTypes/1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_issue_type))
            .mount(&mock_server)
            .await;

        let mut params = UpdateIssueTypeParams::new(
            ProjectKey::from_str("TEST_PROJECT").unwrap(),
            IssueTypeId::new(1),
        );
        params.name = Some("Updated Feature".to_string());
        params.color = Some(backlog_domain_models::IssueTypeColor::Blue);
        params.template_summary = Some("Feature summary template".to_string());
        params.template_description = Some("Feature description template".to_string());

        let result = project_api.update_issue_type(params).await;
        assert!(result.is_ok());
        let issue_type = result.unwrap();
        assert_eq!(issue_type.name, "Updated Feature");
        assert_eq!(issue_type.color, "#007e9a");
        assert_eq!(
            issue_type.template_summary,
            Some("Feature summary template".to_string())
        );
        assert_eq!(
            issue_type.template_description,
            Some("Feature description template".to_string())
        );
    }

    #[tokio::test]
    async fn test_update_issue_type_not_found() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let project_api = ProjectApi::new(client);

        let error_response = serde_json::json!({
            "errors": [
                {
                    "message": "Issue type not found",
                    "code": 7
                }
            ]
        });

        Mock::given(method("PATCH"))
            .and(path("/api/v2/projects/123/issueTypes/999"))
            .respond_with(ResponseTemplate::new(404).set_body_json(error_response))
            .mount(&mock_server)
            .await;

        let mut params = UpdateIssueTypeParams::new(ProjectId::new(123), IssueTypeId::new(999));
        params.name = Some("Updated Bug".to_string());

        let result = project_api.update_issue_type(params).await;
        assert!(result.is_err());
        match result {
            Err(ApiError::HttpStatus { status, .. }) => {
                assert_eq!(status, 404);
            }
            _ => panic!("Expected HttpStatus error with 404"),
        }
    }

    #[tokio::test]
    async fn test_delete_issue_type_success() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let project_api = ProjectApi::new(client);

        let expected_issue_type = IssueType {
            id: IssueTypeId::new(1),
            project_id: ProjectId::new(123),
            name: "Deprecated Bug".to_string(),
            color: "#e30613".to_string(),
            display_order: 1,
            template_summary: None,
            template_description: None,
        };

        Mock::given(method("DELETE"))
            .and(path("/api/v2/projects/123/issueTypes/1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_issue_type))
            .mount(&mock_server)
            .await;

        let params = DeleteIssueTypeParams::new(
            ProjectId::new(123),
            IssueTypeId::new(1),
            IssueTypeId::new(2),
        );
        let result = project_api.delete_issue_type(params).await;
        assert!(result.is_ok());
        let issue_type = result.unwrap();
        assert_eq!(issue_type.name, "Deprecated Bug");
        assert_eq!(issue_type.id, IssueTypeId::new(1));
    }

    #[tokio::test]
    async fn test_delete_issue_type_not_found() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let project_api = ProjectApi::new(client);

        let error_response = serde_json::json!({
            "errors": [
                {
                    "message": "Issue type not found",
                    "code": 7
                }
            ]
        });

        Mock::given(method("DELETE"))
            .and(path("/api/v2/projects/TEST_PROJECT/issueTypes/999"))
            .respond_with(ResponseTemplate::new(404).set_body_json(error_response))
            .mount(&mock_server)
            .await;

        let params = DeleteIssueTypeParams::new(
            ProjectKey::from_str("TEST_PROJECT").unwrap(),
            IssueTypeId::new(999),
            IssueTypeId::new(2),
        );
        let result = project_api.delete_issue_type(params).await;
        assert!(result.is_err());
        match result {
            Err(ApiError::HttpStatus { status, .. }) => {
                assert_eq!(status, 404);
            }
            _ => panic!("Expected HttpStatus error with 404"),
        }
    }

    #[tokio::test]
    async fn test_delete_issue_type_invalid_substitute() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let project_api = ProjectApi::new(client);

        let error_response = serde_json::json!({
            "errors": [
                {
                    "message": "Substitute issue type not found",
                    "code": 7
                }
            ]
        });

        Mock::given(method("DELETE"))
            .and(path("/api/v2/projects/123/issueTypes/1"))
            .respond_with(ResponseTemplate::new(400).set_body_json(error_response))
            .mount(&mock_server)
            .await;

        let params = DeleteIssueTypeParams::new(
            ProjectId::new(123),
            IssueTypeId::new(1),
            IssueTypeId::new(999),
        );
        let result = project_api.delete_issue_type(params).await;
        assert!(result.is_err());
        match result {
            Err(ApiError::HttpStatus { status, .. }) => {
                assert_eq!(status, 400);
            }
            _ => panic!("Expected HttpStatus error with 400"),
        }
    }

    #[tokio::test]
    async fn test_update_version_success() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let project_api = ProjectApi::new(client);

        let expected_version = Milestone {
            id: MilestoneId::new(1),
            project_id: ProjectId::new(123),
            name: "Version 2.0".to_string(),
            description: Some("Updated release".to_string()),
            start_date: None,
            release_due_date: None,
            archived: false,
            display_order: Some(1),
        };

        Mock::given(method("PATCH"))
            .and(path("/api/v2/projects/123/versions/1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_version))
            .mount(&mock_server)
            .await;

        let params =
            UpdateVersionParams::new(ProjectId::new(123), MilestoneId::new(1), "Version 2.0");
        let result = project_api.update_version(params).await;
        assert!(result.is_ok());
        let version = result.unwrap();
        assert_eq!(version.name, "Version 2.0");
        assert_eq!(version.id, MilestoneId::new(1));
    }

    #[tokio::test]
    async fn test_update_version_with_dates() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let project_api = ProjectApi::new(client);

        let expected_version = Milestone {
            id: MilestoneId::new(1),
            project_id: ProjectId::new(123),
            name: "Version 3.0".to_string(),
            description: Some("Release with dates".to_string()),
            start_date: Some(chrono::Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap()),
            release_due_date: Some(chrono::Utc.with_ymd_and_hms(2024, 12, 31, 0, 0, 0).unwrap()),
            archived: true,
            display_order: Some(1),
        };

        Mock::given(method("PATCH"))
            .and(path("/api/v2/projects/TEST_PROJECT/versions/1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_version))
            .mount(&mock_server)
            .await;

        let mut params = UpdateVersionParams::new(
            ProjectKey::from_str("TEST_PROJECT").unwrap(),
            MilestoneId::new(1),
            "Version 3.0",
        );
        params.description = Some("Release with dates".to_string());
        params.start_date = Some(backlog_core::ApiDate::new(
            chrono::Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap(),
        ));
        params.release_due_date = Some(backlog_core::ApiDate::new(
            chrono::Utc.with_ymd_and_hms(2024, 12, 31, 0, 0, 0).unwrap(),
        ));
        params.archived = Some(true);

        let result = project_api.update_version(params).await;
        assert!(result.is_ok());
        let version = result.unwrap();
        assert_eq!(version.name, "Version 3.0");
        assert_eq!(version.description, Some("Release with dates".to_string()));
        assert!(version.archived);
    }

    #[tokio::test]
    async fn test_update_version_not_found() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let project_api = ProjectApi::new(client);

        let error_response = serde_json::json!({
            "errors": [
                {
                    "message": "Version not found",
                    "code": 7
                }
            ]
        });

        Mock::given(method("PATCH"))
            .and(path("/api/v2/projects/123/versions/999"))
            .respond_with(ResponseTemplate::new(404).set_body_json(error_response))
            .mount(&mock_server)
            .await;

        let params =
            UpdateVersionParams::new(ProjectId::new(123), MilestoneId::new(999), "Version 4.0");
        let result = project_api.update_version(params).await;
        assert!(result.is_err());
        match result {
            Err(ApiError::HttpStatus { status, .. }) => {
                assert_eq!(status, 404);
            }
            _ => panic!("Expected HttpStatus error with 404"),
        }
    }

    #[tokio::test]
    async fn test_delete_version_success() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let project_api = ProjectApi::new(client);

        let expected_version = Milestone {
            id: MilestoneId::new(1),
            project_id: ProjectId::new(123),
            name: "Version 1.0".to_string(),
            description: Some("Deprecated version".to_string()),
            start_date: None,
            release_due_date: None,
            archived: true,
            display_order: Some(1),
        };

        Mock::given(method("DELETE"))
            .and(path("/api/v2/projects/123/versions/1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_version))
            .mount(&mock_server)
            .await;

        let params = DeleteVersionParams::new(ProjectId::new(123), MilestoneId::new(1));
        let result = project_api.delete_version(params).await;
        assert!(result.is_ok());
        let version = result.unwrap();
        assert_eq!(version.name, "Version 1.0");
        assert_eq!(version.id, MilestoneId::new(1));
    }

    #[tokio::test]
    async fn test_delete_version_not_found() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let project_api = ProjectApi::new(client);

        let error_response = serde_json::json!({
            "errors": [
                {
                    "message": "Version not found",
                    "code": 7
                }
            ]
        });

        Mock::given(method("DELETE"))
            .and(path("/api/v2/projects/TEST_PROJECT/versions/999"))
            .respond_with(ResponseTemplate::new(404).set_body_json(error_response))
            .mount(&mock_server)
            .await;

        let params = DeleteVersionParams::new(
            ProjectKey::from_str("TEST_PROJECT").unwrap(),
            MilestoneId::new(999),
        );
        let result = project_api.delete_version(params).await;
        assert!(result.is_err());
        match result {
            Err(ApiError::HttpStatus { status, .. }) => {
                assert_eq!(status, 404);
            }
            _ => panic!("Expected HttpStatus error with 404"),
        }
    }
}
