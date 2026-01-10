use crate::issue::response_transformer::*;
use backlog_core::identifier::{
    CustomFieldId, IssueId, IssueTypeId, PriorityId, ProjectId, StatusId, UserId,
};
use backlog_core::{IssueKey, Language, ProjectKey, Role, User};
use backlog_issue::models::{CustomField, CustomFieldTypeId, Issue};
use backlog_project::{IssueType, Priority, Status};
use serde_json::json;
use std::str::FromStr;

fn create_test_issue() -> Issue {
    Issue {
        id: IssueId::new(1),
        project_id: ProjectId::new(1),
        key_id: 1u32,
        issue_key: IssueKey::from_str("TEST-1").unwrap(),
        issue_type: Box::new(IssueType {
            id: IssueTypeId::new(1),
            project_id: ProjectId::new(1),
            name: "Bug".to_string(),
            color: "#990000".to_string(),
            display_order: 0,
            template_summary: None,
            template_description: None,
        }),
        summary: "Test Issue".to_string(),
        description: "Test description".to_string(),
        resolution: None,
        priority: Some(Box::new(Priority {
            id: PriorityId::new(2),
            name: "Normal".to_string(),
        })),
        status: Box::new(Status {
            id: StatusId::new(1),
            project_id: ProjectId::new(1),
            name: "Open".to_string(),
            color: "#ed8077".to_string(),
            display_order: 1000,
        }),
        assignee: None,
        category: vec![],
        versions: vec![],
        milestone: vec![],
        start_date: None,
        due_date: None,
        estimated_hours: None,
        actual_hours: None,
        parent_issue_id: None,
        created_user: Box::new(User {
            id: UserId::new(1),
            user_id: Some("admin".to_string()),
            name: "Admin".to_string(),
            role_type: Role::Admin,
            lang: Some(Language::Japanese),
            mail_address: "admin@example.com".to_string(),
            last_login_time: None,
        }),
        created: "2024-01-01T00:00:00Z".to_string(),
        updated_user: None,
        updated: "2024-01-01T00:00:00Z".to_string(),
        custom_fields: vec![],
        attachments: vec![],
        shared_files: vec![],
        external_file_links: vec![],
        stars: vec![],
    }
}

#[test]
fn test_custom_field_type_id_display() {
    assert_eq!(CustomFieldTypeId::Text.to_string(), "text");
    assert_eq!(CustomFieldTypeId::TextArea.to_string(), "textarea");
    assert_eq!(CustomFieldTypeId::Numeric.to_string(), "number");
    assert_eq!(CustomFieldTypeId::Date.to_string(), "date");
    assert_eq!(CustomFieldTypeId::SingleList.to_string(), "single_list");
    assert_eq!(CustomFieldTypeId::MultipleList.to_string(), "multiple_list");
    assert_eq!(CustomFieldTypeId::CheckBox.to_string(), "checkbox");
    assert_eq!(CustomFieldTypeId::Radio.to_string(), "radio");
}

#[test]
fn test_issue_response_from_issue_without_custom_fields() {
    let issue = create_test_issue();
    let response = IssueResponse::from(issue.clone());

    assert_eq!(response.id, issue.id);
    assert_eq!(response.summary, issue.summary);
    assert!(response.custom_fields.is_empty());
}

#[test]
fn test_issue_response_from_issue_with_custom_fields() {
    let mut issue = create_test_issue();
    issue.custom_fields = vec![
        CustomField {
            id: CustomFieldId::new(318302),
            field_type_id: CustomFieldTypeId::Numeric,
            name: "Foobar".to_string(),
            value: json!(null),
            other_value: None,
        },
        CustomField {
            id: CustomFieldId::new(318303),
            field_type_id: CustomFieldTypeId::Text,
            name: "TestText".to_string(),
            value: json!("Hello World"),
            other_value: None,
        },
        CustomField {
            id: CustomFieldId::new(318304),
            field_type_id: CustomFieldTypeId::SingleList,
            name: "Priority".to_string(),
            value: json!("High"),
            other_value: Some(json!("Custom Priority")),
        },
    ];

    let response = IssueResponse::from(issue);

    assert_eq!(response.custom_fields.len(), 3);

    // Check Foobar field
    let foobar = response.custom_fields.get("Foobar").unwrap();
    assert_eq!(foobar.id, 318302);
    assert_eq!(foobar.field_type_id, "number");
    assert_eq!(foobar.value, json!(null));
    assert_eq!(foobar.other_value, None);

    // Check TestText field
    let test_text = response.custom_fields.get("TestText").unwrap();
    assert_eq!(test_text.id, 318303);
    assert_eq!(test_text.field_type_id, "text");
    assert_eq!(test_text.value, json!("Hello World"));
    assert_eq!(test_text.other_value, None);

    // Check Priority field
    let priority = response.custom_fields.get("Priority").unwrap();
    assert_eq!(priority.id, 318304);
    assert_eq!(priority.field_type_id, "single_list");
    assert_eq!(priority.value, json!("High"));
    assert_eq!(priority.other_value, Some(json!("Custom Priority")));
}

#[test]
fn test_vec_issue_response_from_vec_issue() {
    let mut issue1 = create_test_issue();
    issue1.custom_fields = vec![CustomField {
        id: CustomFieldId::new(318302),
        field_type_id: CustomFieldTypeId::Numeric,
        name: "Score".to_string(),
        value: json!(42),
        other_value: None,
    }];

    let mut issue2 = create_test_issue();
    issue2.id = IssueId::new(2);
    issue2.custom_fields = vec![CustomField {
        id: CustomFieldId::new(318302),
        field_type_id: CustomFieldTypeId::Numeric,
        name: "Score".to_string(),
        value: json!(84),
        other_value: None,
    }];

    let issues = vec![issue1, issue2];
    let responses: Vec<IssueResponse> = issues.into_iter().map(IssueResponse::from).collect();

    assert_eq!(responses.len(), 2);
    assert_eq!(responses[0].id, IssueId::new(1));
    assert_eq!(
        responses[0].custom_fields.get("Score").unwrap().value,
        json!(42)
    );
    assert_eq!(responses[1].id, IssueId::new(2));
    assert_eq!(
        responses[1].custom_fields.get("Score").unwrap().value,
        json!(84)
    );
}

#[test]
fn test_custom_field_info_serialization() {
    let field_info = CustomFieldInfo {
        id: 318302,
        field_type_id: "number".to_string(),
        value: json!(42),
        other_value: None,
    };

    let serialized = serde_json::to_value(&field_info).unwrap();
    assert_eq!(serialized["id"], 318302);
    assert_eq!(serialized["fieldTypeId"], "number");
    assert_eq!(serialized["value"], 42);
    assert!(!serialized.as_object().unwrap().contains_key("otherValue"));

    // With otherValue
    let field_info_with_other = CustomFieldInfo {
        id: 318303,
        field_type_id: "single_list".to_string(),
        value: json!("Option1"),
        other_value: Some(json!("Custom")),
    };

    let serialized = serde_json::to_value(&field_info_with_other).unwrap();
    assert_eq!(serialized["otherValue"], "Custom");
}
