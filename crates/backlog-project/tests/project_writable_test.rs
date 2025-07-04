#[cfg(feature = "writable")]
mod writable_tests {

    use backlog_api_core::Error as ApiError;
    use backlog_core::{
        ProjectKey,
        identifier::{CategoryId, IssueTypeId, MilestoneId, ProjectId, StatusId},
    };
    use backlog_project::api::{
        AddCategoryParams, AddIssueTypeParams, AddMilestoneParams, AddStatusParams,
        DeleteCategoryParams, DeleteStatusParams, ProjectApi, UpdateCategoryParams,
        UpdateStatusOrderParams, UpdateStatusParams,
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
}
