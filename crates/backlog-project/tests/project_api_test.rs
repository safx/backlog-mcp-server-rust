mod common;

use backlog_core::identifier::{
    CategoryId, CustomFieldId, IssueTypeId, MilestoneId, PriorityId, ProjectId, ResolutionId,
    StatusId, UserId,
};
use backlog_core::{Language, ProjectIdOrKey, ProjectKey, Role, TextFormattingRule, User};
pub use backlog_domain_models::Milestone;
use backlog_domain_models::{
    CustomFieldSettings, CustomFieldType, DateSettings, ListItem, ListSettings, NumericSettings,
};
use backlog_project::api::{
    GetCategoryListParams, GetCustomFieldListParams, GetIssueTypeListParams,
    GetMilestoneListParams, GetProjectAdministratorListParams, GetProjectDetailParams,
    GetProjectIconParams, GetProjectListParams, GetProjectUserListParams, GetStatusListParams,
};
use backlog_project::{Category, IssueType, Priority, Project, Resolution, Status};
pub use chrono::TimeZone;
use common::*;
use serde::Serialize;
use std::str::FromStr;
use wiremock::MockServer;
pub use wiremock::matchers::{method, path};
pub use wiremock::{Mock, ResponseTemplate};

// Test cases for get_project_user_list
#[tokio::test]
async fn test_get_project_user_list_success() {
    let mock_server = MockServer::start().await;
    let project_api = setup_project_api(&mock_server).await;
    let project_id = ProjectId::new(123);

    let expected_users = vec![
        User {
            id: UserId::new(1),
            user_id: Some("admin".to_string()),
            name: "Administrator".to_string(),
            role_type: Role::Admin,
            lang: Some(Language::Japanese),
            mail_address: "admin@example.com".to_string(),
            last_login_time: Some("2022-09-01T06:35:39Z".parse().unwrap()),
        },
        User {
            id: UserId::new(2),
            user_id: Some("user1".to_string()),
            name: "User One".to_string(),
            role_type: Role::User,
            lang: Some(Language::English),
            mail_address: "user1@example.com".to_string(),
            last_login_time: Some("2022-09-02T07:30:15Z".parse().unwrap()),
        },
        User {
            id: UserId::new(3),
            user_id: Some("reporter".to_string()),
            name: "Reporter".to_string(),
            role_type: Role::Reporter,
            lang: None,
            mail_address: "reporter@example.com".to_string(),
            last_login_time: None,
        },
    ];

    Mock::given(method("GET"))
        .and(path("/api/v2/projects/123/users"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_users))
        .mount(&mock_server)
        .await;

    let params = GetProjectUserListParams::new(project_id);
    let result = project_api.get_project_user_list(params).await;
    assert!(result.is_ok());
    let users = result.unwrap();
    assert_eq!(users.len(), 3);
    assert_eq!(users[0].name, "Administrator");
    assert_eq!(users[0].role_type, Role::Admin);
    assert_eq!(users[1].name, "User One");
    assert_eq!(users[1].role_type, Role::User);
    assert_eq!(users[2].name, "Reporter");
    assert_eq!(users[2].role_type, Role::Reporter);
}

#[tokio::test]
async fn test_get_project_user_list_error() {
    let mock_server = MockServer::start().await;
    let project_api = setup_project_api(&mock_server).await;
    let project_id = ProjectId::new(999);

    Mock::given(method("GET"))
        .and(path("/api/v2/projects/999/users"))
        .respond_with(ResponseTemplate::new(404))
        .mount(&mock_server)
        .await;

    let params = GetProjectUserListParams::new(project_id);
    let result = project_api.get_project_user_list(params).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_project_user_list_forbidden() {
    let mock_server = MockServer::start().await;
    let project_api = setup_project_api(&mock_server).await;
    let project_id = ProjectId::new(123);

    Mock::given(method("GET"))
        .and(path("/api/v2/projects/123/users"))
        .respond_with(ResponseTemplate::new(403).set_body_json(serde_json::json!({
            "errors": [{"message": "No permission to access this project."}]
        })))
        .mount(&mock_server)
        .await;

    let params = GetProjectUserListParams::new(project_id);
    let result = project_api.get_project_user_list(params).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_project_user_list_by_project_key() {
    let mock_server = MockServer::start().await;
    let project_api = setup_project_api(&mock_server).await;
    let project_key = ProjectKey::from_str("TESTPROJ").unwrap();

    let expected_users = vec![User {
        id: UserId::new(1),
        user_id: Some("projectlead".to_string()),
        name: "Project Leader".to_string(),
        role_type: Role::Admin,
        lang: Some(Language::Japanese),
        mail_address: "lead@example.com".to_string(),
        last_login_time: Some("2022-09-01T08:00:00Z".parse().unwrap()),
    }];

    Mock::given(method("GET"))
        .and(path("/api/v2/projects/TESTPROJ/users"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_users))
        .mount(&mock_server)
        .await;

    let params = GetProjectUserListParams::new(project_key);
    let result = project_api.get_project_user_list(params).await;
    assert!(result.is_ok());
    let users = result.unwrap();
    assert_eq!(users.len(), 1);
    assert_eq!(users[0].name, "Project Leader");
}

#[tokio::test]
async fn test_get_project_user_list_empty_project() {
    let mock_server = MockServer::start().await;
    let project_api = setup_project_api(&mock_server).await;
    let project_id = ProjectId::new(456);

    let expected_users: Vec<User> = vec![];

    Mock::given(method("GET"))
        .and(path("/api/v2/projects/456/users"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_users))
        .mount(&mock_server)
        .await;

    let params = GetProjectUserListParams::new(project_id);
    let result = project_api.get_project_user_list(params).await;
    assert!(result.is_ok());
    let users = result.unwrap();
    assert_eq!(users.len(), 0);
}

#[tokio::test]
async fn test_get_version_milestone_list_success() {
    let mock_server = MockServer::start().await;
    let project_api = setup_project_api(&mock_server).await;
    let project_id_or_key_str = "TEST_PROJECT";
    let project_id_or_key: ProjectIdOrKey = project_id_or_key_str.parse().unwrap();
    let project_id_numeric = ProjectId::new(1);

    let expected_versions: Vec<Milestone> = vec![
        Milestone {
            id: MilestoneId::new(1),
            project_id: project_id_numeric,
            name: "Version 1.0".to_string(),
            description: Some("Initial release".to_string()),
            start_date: Some(chrono::Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap()),
            release_due_date: Some(chrono::Utc.with_ymd_and_hms(2023, 1, 31, 0, 0, 0).unwrap()),
            archived: false,
            display_order: Some(1),
        },
        Milestone {
            id: MilestoneId::new(2),
            project_id: project_id_numeric,
            name: "Version 1.1".to_string(),
            description: None,
            start_date: None,
            release_due_date: None,
            archived: true,
            display_order: Some(2),
        },
    ];

    Mock::given(method("GET"))
        .and(path(format!(
            "/api/v2/projects/{}/versions",
            project_id_or_key.clone()
        )))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_versions))
        .mount(&mock_server)
        .await;
    let params = GetMilestoneListParams::new(project_id_or_key.clone());
    let result = project_api.get_version_milestone_list(params).await;
    assert!(result.is_ok());
    let versions = result.unwrap();
    assert_eq!(versions.len(), 2);
}

#[tokio::test]
async fn test_get_version_milestone_list_error() {
    let mock_server = MockServer::start().await;
    let project_api = setup_project_api(&mock_server).await;
    let project_id_or_key_str = "TEST_PROJECT_ERROR";
    let project_id_or_key: ProjectIdOrKey = project_id_or_key_str.parse().unwrap();

    Mock::given(method("GET"))
        .and(path(format!(
            "/api/v2/projects/{}/versions",
            project_id_or_key.clone()
        )))
        .respond_with(ResponseTemplate::new(500))
        .mount(&mock_server)
        .await;
    let params = GetMilestoneListParams::new(project_id_or_key.clone());
    let result = project_api.get_version_milestone_list(params).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_status_list_success() {
    let mock_server = MockServer::start().await;
    let project_api = setup_project_api(&mock_server).await;
    let project_id = ProjectId::new(123);

    let expected_statuses = vec![
        Status {
            id: StatusId::new(1),
            project_id,
            name: "Open".to_string(),
            color: "#ff0000".to_string(),
            display_order: 1,
        },
        Status {
            id: StatusId::new(2),
            project_id,
            name: "In Progress".to_string(),
            color: "#00ff00".to_string(),
            display_order: 2,
        },
    ];

    Mock::given(method("GET"))
        .and(path("/api/v2/projects/123/statuses"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_statuses))
        .mount(&mock_server)
        .await;

    let params = GetStatusListParams::new(project_id);
    let result = project_api.get_status_list(params).await;
    assert!(result.is_ok());
    let statuses = result.unwrap();
    assert_eq!(statuses.len(), 2);
    assert_eq!(statuses[0].name, "Open");
    assert_eq!(statuses[1].name, "In Progress");
}

#[tokio::test]
async fn test_get_status_list_error() {
    let mock_server = MockServer::start().await;
    let project_api = setup_project_api(&mock_server).await;
    let project_id = ProjectId::new(999);

    Mock::given(method("GET"))
        .and(path("/api/v2/projects/999/statuses"))
        .respond_with(ResponseTemplate::new(404))
        .mount(&mock_server)
        .await;

    let params = GetStatusListParams::new(project_id);
    let result = project_api.get_status_list(params).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_issue_type_list_success() {
    let mock_server = MockServer::start().await;
    let project_api = setup_project_api(&mock_server).await;
    let project_id = ProjectId::new(123);

    let expected_issue_types = vec![
        IssueType {
            id: IssueTypeId::new(1),
            project_id,
            name: "Bug".to_string(),
            color: "#e30613".to_string(),
            display_order: 1,
            template_summary: None,
            template_description: None,
        },
        IssueType {
            id: IssueTypeId::new(2),
            project_id,
            name: "Task".to_string(),
            color: "#7ea800".to_string(),
            display_order: 2,
            template_summary: Some("Task template".to_string()),
            template_description: Some("Task description template".to_string()),
        },
    ];

    Mock::given(method("GET"))
        .and(path("/api/v2/projects/123/issueTypes"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_issue_types))
        .mount(&mock_server)
        .await;

    let params = GetIssueTypeListParams::new(project_id);
    let result = project_api.get_issue_type_list(params).await;
    assert!(result.is_ok());
    let issue_types = result.unwrap();
    assert_eq!(issue_types.len(), 2);
    assert_eq!(issue_types[0].name, "Bug");
    assert_eq!(issue_types[1].name, "Task");
}

#[tokio::test]
async fn test_get_issue_type_list_error() {
    let mock_server = MockServer::start().await;
    let project_api = setup_project_api(&mock_server).await;
    let project_id = ProjectId::new(999);

    Mock::given(method("GET"))
        .and(path("/api/v2/projects/999/issueTypes"))
        .respond_with(ResponseTemplate::new(404))
        .mount(&mock_server)
        .await;

    let params = GetIssueTypeListParams::new(project_id);
    let result = project_api.get_issue_type_list(params).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_category_list_success() {
    let mock_server = MockServer::start().await;
    let project_api = setup_project_api(&mock_server).await;
    let project_id = ProjectId::new(123);

    let expected_categories = vec![
        Category {
            id: CategoryId::new(1),
            project_id,
            name: "Backend".to_string(),
            display_order: 1,
        },
        Category {
            id: CategoryId::new(2),
            project_id,
            name: "Frontend".to_string(),
            display_order: 2,
        },
    ];

    Mock::given(method("GET"))
        .and(path("/api/v2/projects/123/categories"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_categories))
        .mount(&mock_server)
        .await;

    let params = GetCategoryListParams::new(project_id);
    let result = project_api.get_category_list(params).await;
    assert!(result.is_ok());
    let categories = result.unwrap();
    assert_eq!(categories.len(), 2);
    assert_eq!(categories[0].name, "Backend");
    assert_eq!(categories[1].name, "Frontend");
}

#[tokio::test]
async fn test_get_category_list_error() {
    let mock_server = MockServer::start().await;
    let project_api = setup_project_api(&mock_server).await;
    let project_id = ProjectId::new(999);

    Mock::given(method("GET"))
        .and(path("/api/v2/projects/999/categories"))
        .respond_with(ResponseTemplate::new(404))
        .mount(&mock_server)
        .await;

    let params = GetCategoryListParams::new(project_id);
    let result = project_api.get_category_list(params).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_priority_list_success() {
    let mock_server = MockServer::start().await;
    let project_api = setup_project_api(&mock_server).await;

    let expected_priorities = vec![
        Priority {
            id: PriorityId::new(1),
            name: "High".to_string(),
        },
        Priority {
            id: PriorityId::new(2),
            name: "Medium".to_string(),
        },
        Priority {
            id: PriorityId::new(3),
            name: "Low".to_string(),
        },
    ];

    Mock::given(method("GET"))
        .and(path("/api/v2/priorities"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_priorities))
        .mount(&mock_server)
        .await;

    let result = project_api.get_priority_list().await;
    assert!(result.is_ok());
    let priorities = result.unwrap();
    assert_eq!(priorities.len(), 3);
    assert_eq!(priorities[0].name, "High");
    assert_eq!(priorities[1].name, "Medium");
    assert_eq!(priorities[2].name, "Low");
}

#[tokio::test]
async fn test_get_priority_list_error() {
    let mock_server = MockServer::start().await;
    let project_api = setup_project_api(&mock_server).await;

    Mock::given(method("GET"))
        .and(path("/api/v2/priorities"))
        .respond_with(ResponseTemplate::new(500))
        .mount(&mock_server)
        .await;

    let result = project_api.get_priority_list().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_resolution_list_success() {
    let mock_server = MockServer::start().await;
    let project_api = setup_project_api(&mock_server).await;

    let expected_resolutions = vec![
        Resolution {
            id: ResolutionId::new(1),
            name: "Fixed".to_string(),
        },
        Resolution {
            id: ResolutionId::new(2),
            name: "Won't Fix".to_string(),
        },
        Resolution {
            id: ResolutionId::new(3),
            name: "Duplicate".to_string(),
        },
    ];

    Mock::given(method("GET"))
        .and(path("/api/v2/resolutions"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_resolutions))
        .mount(&mock_server)
        .await;

    let result = project_api.get_resolution_list().await;
    assert!(result.is_ok());
    let resolutions = result.unwrap();
    assert_eq!(resolutions.len(), 3);
    assert_eq!(resolutions[0].name, "Fixed");
    assert_eq!(resolutions[1].name, "Won't Fix");
    assert_eq!(resolutions[2].name, "Duplicate");
}

#[tokio::test]
async fn test_get_resolution_list_error() {
    let mock_server = MockServer::start().await;
    let project_api = setup_project_api(&mock_server).await;

    Mock::given(method("GET"))
        .and(path("/api/v2/resolutions"))
        .respond_with(ResponseTemplate::new(500))
        .mount(&mock_server)
        .await;

    let result = project_api.get_resolution_list().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_project_list_success() {
    let mock_server = MockServer::start().await;
    let project_api = setup_project_api(&mock_server).await;

    let expected_projects = vec![Project {
        id: ProjectId::new(1),
        project_key: ProjectKey::from_str("TEST1").unwrap(),
        name: "Test Project 1".to_string(),
        chart_enabled: true,
        subtasking_enabled: false,
        project_leader_can_edit_project_leader: true,
        use_wiki: true,
        use_file_sharing: true,
        use_wiki_tree_view: false,
        use_original_image_size_at_wiki: false,
        text_formatting_rule: TextFormattingRule::Markdown,
        archived: false,
        display_order: 0,
        use_dev_attributes: true,
    }];

    Mock::given(method("GET"))
        .and(path("/api/v2/projects"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_projects))
        .mount(&mock_server)
        .await;

    let params = GetProjectListParams {
        archived: Some(false),
        all: false,
    };
    let result = project_api.get_project_list(params).await;
    assert!(result.is_ok());
    let projects = result.unwrap();
    assert_eq!(projects.len(), 1);
    assert_eq!(projects[0].name, "Test Project 1");
}

#[tokio::test]
async fn test_get_project_list_error() {
    let mock_server = MockServer::start().await;
    let project_api = setup_project_api(&mock_server).await;

    Mock::given(method("GET"))
        .and(path("/api/v2/projects"))
        .respond_with(ResponseTemplate::new(500))
        .mount(&mock_server)
        .await;

    let params = GetProjectListParams {
        archived: None,
        all: true,
    };
    let result = project_api.get_project_list(params).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_project_success() {
    let mock_server = MockServer::start().await;
    let project_api = setup_project_api(&mock_server).await;

    let expected_project = Project {
        id: ProjectId::new(123),
        project_key: ProjectKey::from_str("TESTPROJ").unwrap(),
        name: "Test Project".to_string(),
        chart_enabled: true,
        subtasking_enabled: true,
        project_leader_can_edit_project_leader: false,
        use_wiki: true,
        use_file_sharing: false,
        use_wiki_tree_view: false,
        use_original_image_size_at_wiki: false,
        text_formatting_rule: TextFormattingRule::Backlog,
        archived: false,
        display_order: 0,
        use_dev_attributes: false,
    };

    Mock::given(method("GET"))
        .and(path("/api/v2/projects/TESTPROJ"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_project))
        .mount(&mock_server)
        .await;

    let params = GetProjectDetailParams::new(ProjectKey::from_str("TESTPROJ").unwrap());
    let result = project_api.get_project(params).await;
    assert!(result.is_ok());
    let project = result.unwrap();
    assert_eq!(project.name, "Test Project");
    assert_eq!(
        project.project_key,
        ProjectKey::from_str("TESTPROJ").unwrap()
    );
}

#[tokio::test]
async fn test_get_project_not_found() {
    let mock_server = MockServer::start().await;
    let project_api = setup_project_api(&mock_server).await;

    Mock::given(method("GET"))
        .and(path("/api/v2/projects/NONEXISTENT"))
        .respond_with(ResponseTemplate::new(404))
        .mount(&mock_server)
        .await;

    let params = GetProjectDetailParams::new(ProjectKey::from_str("NONEXISTENT").unwrap());
    let result = project_api.get_project(params).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_project_icon_success() {
    let mock_server = MockServer::start().await;
    let project_api = setup_project_api(&mock_server).await;

    let expected_image_data = b"fake_image_data";

    Mock::given(method("GET"))
        .and(path("/api/v2/projects/TESTPROJ/image"))
        .respond_with(ResponseTemplate::new(200).set_body_bytes(expected_image_data))
        .mount(&mock_server)
        .await;

    let params = GetProjectIconParams::new(ProjectKey::from_str("TESTPROJ").unwrap());
    let result = project_api.get_project_icon(params).await;
    assert!(result.is_ok());
    let image_data = result.unwrap();
    assert_eq!(image_data, expected_image_data);
}

#[tokio::test]
async fn test_get_project_icon_not_found() {
    let mock_server = MockServer::start().await;
    let project_api = setup_project_api(&mock_server).await;

    Mock::given(method("GET"))
        .and(path("/api/v2/projects/NONEXISTENT/image"))
        .respond_with(ResponseTemplate::new(404))
        .mount(&mock_server)
        .await;

    let params = GetProjectIconParams::new(ProjectKey::from_str("NONEXISTENT").unwrap());
    let result = project_api.get_project_icon(params).await;
    assert!(result.is_err());
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RawCustomFieldType {
    pub id: CustomFieldId,
    pub project_id: ProjectId,
    pub type_id: u8,
    pub name: String,
    pub description: String,
    pub required: bool,
    pub use_issue_type: bool,
    pub applicable_issue_types: Vec<IssueTypeId>,
    pub display_order: i64,
    pub settings: CustomFieldSettings,
}

// Test cases for get_custom_field_list
#[tokio::test]
async fn test_get_custom_field_list_success() {
    let mock_server = MockServer::start().await;
    let project_api = setup_project_api(&mock_server).await;
    let project_id = ProjectId::new(123);

    let expected_fields = vec![
        RawCustomFieldType {
            id: CustomFieldId::new(1),
            project_id,
            type_id: 1,
            name: "Text Field".to_string(),
            description: "A simple text field".to_string(),
            required: true,
            use_issue_type: false,
            applicable_issue_types: vec![],
            display_order: 1,
            settings: CustomFieldSettings::Text,
        },
        RawCustomFieldType {
            id: CustomFieldId::new(2),
            project_id,
            type_id: 5,
            name: "Priority Field".to_string(),
            description: "Priority selection field".to_string(),
            required: false,
            use_issue_type: true,
            applicable_issue_types: vec![IssueTypeId::new(1), IssueTypeId::new(2)],
            display_order: 2,
            settings: CustomFieldSettings::SingleList(ListSettings {
                allow_input: Some(false),
                allow_add_item: Some(true),
                items: vec![
                    ListItem {
                        id: backlog_core::identifier::CustomFieldItemId::new(1),
                        name: "High".to_string(),
                        display_order: 1,
                    },
                    ListItem {
                        id: backlog_core::identifier::CustomFieldItemId::new(2),
                        name: "Medium".to_string(),
                        display_order: 2,
                    },
                ],
            }),
        },
    ];

    Mock::given(method("GET"))
        .and(path("/api/v2/projects/123/customFields"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_fields))
        .mount(&mock_server)
        .await;

    let params = GetCustomFieldListParams::new(project_id);
    let result = project_api.get_custom_field_list(params).await;
    dbg!(&result);
    assert!(result.is_ok());
    let fields = result.unwrap();
    assert_eq!(fields.len(), 2);
    assert_eq!(fields[0].name, "Text Field");
    assert!(fields[0].required);
    assert_eq!(fields[1].name, "Priority Field");
    assert!(!fields[1].required);
    assert_eq!(fields[1].applicable_issue_types.as_ref().unwrap().len(), 2);
}

#[tokio::test]
async fn test_get_custom_field_list_multiple_types() {
    let mock_server = MockServer::start().await;
    let project_api = setup_project_api(&mock_server).await;
    let project_id = ProjectId::new(456);

    let expected_fields = vec![
        RawCustomFieldType {
            id: CustomFieldId::new(10),
            project_id,
            type_id: 3,
            name: "Number Field".to_string(),
            description: "Numeric input field".to_string(),
            required: false,
            use_issue_type: false,
            applicable_issue_types: vec![],
            display_order: 1,
            settings: CustomFieldSettings::Numeric(NumericSettings {
                min: Some(0.0),
                max: Some(100.0),
                initial_value: Some(50.0),
                unit: Some("percent".to_string()),
            }),
        },
        RawCustomFieldType {
            id: CustomFieldId::new(11),
            project_id,
            type_id: 4,
            name: "Date Field".to_string(),
            description: "Date selection field".to_string(),
            required: true,
            use_issue_type: false,
            applicable_issue_types: vec![],
            display_order: 2,
            settings: CustomFieldSettings::Date(DateSettings {
                min: None,
                max: None,
                initial_date: None,
                initial_value_type: None,
                initial_shift: None,
            }),
        },
    ];

    Mock::given(method("GET"))
        .and(path("/api/v2/projects/456/customFields"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_fields))
        .mount(&mock_server)
        .await;

    let params = GetCustomFieldListParams::new(project_id);
    let result = project_api.get_custom_field_list(params).await;
    assert!(result.is_ok());
    let fields = result.unwrap();
    assert_eq!(fields.len(), 2);
    assert!(matches!(
        fields[0].settings,
        CustomFieldSettings::Numeric(_)
    ));
    assert!(matches!(fields[1].settings, CustomFieldSettings::Date(_)));
}

#[tokio::test]
async fn test_get_custom_field_list_empty_project() {
    let mock_server = MockServer::start().await;
    let project_api = setup_project_api(&mock_server).await;
    let project_id = ProjectId::new(789);

    let expected_fields: Vec<CustomFieldType> = vec![];

    Mock::given(method("GET"))
        .and(path("/api/v2/projects/789/customFields"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_fields))
        .mount(&mock_server)
        .await;

    let params = GetCustomFieldListParams::new(project_id);
    let result = project_api.get_custom_field_list(params).await;
    assert!(result.is_ok());
    let fields = result.unwrap();
    assert_eq!(fields.len(), 0);
}

#[tokio::test]
async fn test_get_custom_field_list_project_not_found() {
    let mock_server = MockServer::start().await;
    let project_api = setup_project_api(&mock_server).await;
    let project_id = ProjectId::new(999);

    Mock::given(method("GET"))
        .and(path("/api/v2/projects/999/customFields"))
        .respond_with(ResponseTemplate::new(404))
        .mount(&mock_server)
        .await;

    let params = GetCustomFieldListParams::new(project_id);
    let result = project_api.get_custom_field_list(params).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_custom_field_list_forbidden() {
    let mock_server = MockServer::start().await;
    let project_api = setup_project_api(&mock_server).await;
    let project_id = ProjectId::new(123);

    Mock::given(method("GET"))
        .and(path("/api/v2/projects/123/customFields"))
        .respond_with(ResponseTemplate::new(403).set_body_json(serde_json::json!({
            "errors": [{"message": "No permission to access this project."}]
        })))
        .mount(&mock_server)
        .await;

    let params = GetCustomFieldListParams::new(project_id);
    let result = project_api.get_custom_field_list(params).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_custom_field_list_by_project_key() {
    let mock_server = MockServer::start().await;
    let project_api = setup_project_api(&mock_server).await;
    let project_key = ProjectKey::from_str("TESTPROJ").unwrap();

    let expected_fields = vec![RawCustomFieldType {
        id: CustomFieldId::new(5),
        project_id: ProjectId::new(1),
        type_id: 2,
        name: "Description Field".to_string(),
        description: "Multi-line text area".to_string(),
        required: false,
        use_issue_type: false,
        applicable_issue_types: vec![],
        display_order: 1,
        settings: CustomFieldSettings::TextArea,
    }];

    Mock::given(method("GET"))
        .and(path("/api/v2/projects/TESTPROJ/customFields"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_fields))
        .mount(&mock_server)
        .await;

    let params = GetCustomFieldListParams::new(project_key);
    let result = project_api.get_custom_field_list(params).await;
    assert!(result.is_ok());
    let fields = result.unwrap();
    assert_eq!(fields.len(), 1);
    assert_eq!(fields[0].name, "Description Field");
    assert!(matches!(fields[0].settings, CustomFieldSettings::TextArea));
}

// Test cases for get_project_administrator_list
#[tokio::test]
async fn test_get_project_administrator_list_success() {
    let mock_server = MockServer::start().await;
    let project_api = setup_project_api(&mock_server).await;
    let project_id = ProjectId::new(123);

    let expected_admins = vec![
        User {
            id: UserId::new(5686),
            user_id: Some("takada".to_string()),
            name: "takada".to_string(),
            role_type: Role::Admin,
            lang: Some(Language::Japanese),
            mail_address: "takada@example.com".to_string(),
            last_login_time: Some("2023-09-01T10:00:00Z".parse().unwrap()),
        },
        User {
            id: UserId::new(7890),
            user_id: Some("admin2".to_string()),
            name: "Project Admin 2".to_string(),
            role_type: Role::Admin,
            lang: Some(Language::English),
            mail_address: "admin2@example.com".to_string(),
            last_login_time: Some("2023-09-02T11:30:00Z".parse().unwrap()),
        },
    ];

    Mock::given(method("GET"))
        .and(path("/api/v2/projects/123/administrators"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_admins))
        .mount(&mock_server)
        .await;

    let params = GetProjectAdministratorListParams::new(project_id);
    let result = project_api.get_project_administrator_list(params).await;
    assert!(result.is_ok());
    let admins = result.unwrap();
    assert_eq!(admins.len(), 2);
    assert_eq!(admins[0].name, "takada");
    assert_eq!(admins[0].role_type, Role::Admin);
    assert_eq!(admins[1].name, "Project Admin 2");
    assert_eq!(admins[1].role_type, Role::Admin);
}

#[tokio::test]
async fn test_get_project_administrator_list_empty() {
    let mock_server = MockServer::start().await;
    let project_api = setup_project_api(&mock_server).await;
    let project_id = ProjectId::new(456);

    let expected_admins: Vec<User> = vec![];

    Mock::given(method("GET"))
        .and(path("/api/v2/projects/456/administrators"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_admins))
        .mount(&mock_server)
        .await;

    let params = GetProjectAdministratorListParams::new(project_id);
    let result = project_api.get_project_administrator_list(params).await;
    assert!(result.is_ok());
    let admins = result.unwrap();
    assert_eq!(admins.len(), 0);
}

#[tokio::test]
async fn test_get_project_administrator_list_with_project_key() {
    let mock_server = MockServer::start().await;
    let project_api = setup_project_api(&mock_server).await;
    let project_key = ProjectKey::from_str("TEST").unwrap();

    let expected_admin = vec![User {
        id: UserId::new(1234),
        user_id: Some("projectlead".to_string()),
        name: "Project Leader".to_string(),
        role_type: Role::Admin,
        lang: Some(Language::Japanese),
        mail_address: "lead@example.com".to_string(),
        last_login_time: None,
    }];

    Mock::given(method("GET"))
        .and(path("/api/v2/projects/TEST/administrators"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_admin))
        .mount(&mock_server)
        .await;

    let params = GetProjectAdministratorListParams::new(project_key);
    let result = project_api.get_project_administrator_list(params).await;
    assert!(result.is_ok());
    let admins = result.unwrap();
    assert_eq!(admins.len(), 1);
    assert_eq!(admins[0].name, "Project Leader");
}

#[tokio::test]
async fn test_get_project_administrator_list_not_found() {
    let mock_server = MockServer::start().await;
    let project_api = setup_project_api(&mock_server).await;
    let project_id = ProjectId::new(999);

    Mock::given(method("GET"))
        .and(path("/api/v2/projects/999/administrators"))
        .respond_with(ResponseTemplate::new(404))
        .mount(&mock_server)
        .await;

    let params = GetProjectAdministratorListParams::new(project_id);
    let result = project_api.get_project_administrator_list(params).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_project_administrator_list_forbidden() {
    let mock_server = MockServer::start().await;
    let project_api = setup_project_api(&mock_server).await;
    let project_id = ProjectId::new(123);

    Mock::given(method("GET"))
        .and(path("/api/v2/projects/123/administrators"))
        .respond_with(ResponseTemplate::new(403).set_body_json(serde_json::json!({
            "errors": [{"message": "No permission to access this project."}]
        })))
        .mount(&mock_server)
        .await;

    let params = GetProjectAdministratorListParams::new(project_id);
    let result = project_api.get_project_administrator_list(params).await;
    assert!(result.is_err());
}
