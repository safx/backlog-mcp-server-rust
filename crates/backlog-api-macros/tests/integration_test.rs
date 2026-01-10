use backlog_api_macros::ToFormParams;

#[derive(ToFormParams)]
struct SimpleParams {
    content: String,
    issue_id: u32,
}

#[derive(ToFormParams)]
struct OptionalParams {
    required_field: String,
    optional_field: Option<String>,
    optional_number: Option<u32>,
}

#[derive(ToFormParams)]
struct ArrayParams {
    content: String,
    #[form(array, name = "notifiedUserId")]
    notified_user_ids: Option<Vec<u32>>,
    #[form(array)]
    tag_ids: Vec<u32>,
}

#[derive(ToFormParams)]
struct CustomNameParams {
    #[form(name = "customName")]
    field_with_custom_name: String,
    normal_field: String,
}

#[derive(ToFormParams)]
struct SkipFieldParams {
    included_field: String,
    #[form(skip)]
    #[allow(dead_code)]
    skipped_field: String,
}

#[test]
fn test_simple_params() {
    let params = SimpleParams {
        content: "Hello".to_string(),
        issue_id: 123,
    };

    let form_params: Vec<(String, String)> = (&params).into();

    assert_eq!(form_params.len(), 2);
    assert!(form_params.contains(&("content".to_string(), "Hello".to_string())));
    assert!(form_params.contains(&("issueId".to_string(), "123".to_string())));
}

#[test]
fn test_optional_params() {
    let params = OptionalParams {
        required_field: "required".to_string(),
        optional_field: Some("optional".to_string()),
        optional_number: None,
    };

    let form_params: Vec<(String, String)> = (&params).into();

    assert_eq!(form_params.len(), 2);
    assert!(form_params.contains(&("requiredField".to_string(), "required".to_string())));
    assert!(form_params.contains(&("optionalField".to_string(), "optional".to_string())));

    // None values should not be included
    assert!(!form_params.iter().any(|(key, _)| key == "optionalNumber"));
}

#[test]
fn test_array_params() {
    let params = ArrayParams {
        content: "Hello".to_string(),
        notified_user_ids: Some(vec![1, 2, 3]),
        tag_ids: vec![10, 20],
    };

    let form_params: Vec<(String, String)> = (&params).into();

    assert_eq!(form_params.len(), 6); // content + 3 notified users + 2 tags

    // Regular field
    assert!(form_params.contains(&("content".to_string(), "Hello".to_string())));

    // Optional array with custom name
    assert!(form_params.contains(&("notifiedUserId[]".to_string(), "1".to_string())));
    assert!(form_params.contains(&("notifiedUserId[]".to_string(), "2".to_string())));
    assert!(form_params.contains(&("notifiedUserId[]".to_string(), "3".to_string())));

    // Required array
    assert!(form_params.contains(&("tagIds[]".to_string(), "10".to_string())));
    assert!(form_params.contains(&("tagIds[]".to_string(), "20".to_string())));
}

#[test]
fn test_empty_optional_array() {
    let params = ArrayParams {
        content: "Hello".to_string(),
        notified_user_ids: None,
        tag_ids: vec![],
    };

    let form_params: Vec<(String, String)> = (&params).into();

    assert_eq!(form_params.len(), 1); // Only content
    assert!(form_params.contains(&("content".to_string(), "Hello".to_string())));

    // No array parameters should be present
    assert!(!form_params.iter().any(|(key, _)| key.contains("[]")));
}

#[test]
fn test_custom_name() {
    let params = CustomNameParams {
        field_with_custom_name: "custom".to_string(),
        normal_field: "normal".to_string(),
    };

    let form_params: Vec<(String, String)> = (&params).into();

    assert_eq!(form_params.len(), 2);
    assert!(form_params.contains(&("customName".to_string(), "custom".to_string())));
    assert!(form_params.contains(&("normalField".to_string(), "normal".to_string())));
}

#[test]
fn test_skip_field() {
    let params = SkipFieldParams {
        included_field: "included".to_string(),
        skipped_field: "should not appear".to_string(),
    };

    let form_params: Vec<(String, String)> = (&params).into();

    assert_eq!(form_params.len(), 1);
    assert!(form_params.contains(&("includedField".to_string(), "included".to_string())));

    // Skipped field should not be present
    assert!(
        !form_params
            .iter()
            .any(|(_, value)| value == "should not appear")
    );
}

#[test]
fn test_snake_case_to_camel_case_conversion() {
    #[derive(ToFormParams)]
    struct CaseTestParams {
        snake_case_field: String,
        already_camel: String,
        multi_word_snake_case: String,
    }

    let params = CaseTestParams {
        snake_case_field: "value1".to_string(),
        already_camel: "value2".to_string(),
        multi_word_snake_case: "value3".to_string(),
    };

    let form_params: Vec<(String, String)> = (&params).into();

    assert_eq!(form_params.len(), 3);
    assert!(form_params.contains(&("snakeCaseField".to_string(), "value1".to_string())));
    assert!(form_params.contains(&("alreadyCamel".to_string(), "value2".to_string())));
    assert!(form_params.contains(&("multiWordSnakeCase".to_string(), "value3".to_string())));
}

#[test]
fn test_date_format_attribute() {
    use chrono::{DateTime, TimeZone, Utc};

    #[derive(ToFormParams)]
    struct DateFormatParams {
        content: String,
        #[form(date_format = "%Y-%m-%d")]
        start_date: DateTime<Utc>,
        #[form(name = "dueDate", date_format = "%Y-%m-%d")]
        due_date: Option<DateTime<Utc>>,
    }

    let start = Utc.with_ymd_and_hms(2024, 1, 15, 10, 30, 0).unwrap();
    let due = Utc.with_ymd_and_hms(2024, 2, 20, 0, 0, 0).unwrap();

    let params = DateFormatParams {
        content: "test".to_string(),
        start_date: start,
        due_date: Some(due),
    };

    let form_params: Vec<(String, String)> = (&params).into();

    assert_eq!(form_params.len(), 3);
    assert!(form_params.contains(&("content".to_string(), "test".to_string())));
    assert!(form_params.contains(&("startDate".to_string(), "2024-01-15".to_string())));
    assert!(form_params.contains(&("dueDate".to_string(), "2024-02-20".to_string())));
}
