//! Example of using custom fields with the Backlog API
//!
//! This example demonstrates how to:
//! 1. Create an issue with custom fields
//! 2. Update an issue with custom fields
//! 3. Read custom fields from an issue

use backlog_api_client::IssueKey;
use backlog_api_client::client::BacklogApiClient;
use backlog_core::identifier::{
    CustomFieldId, CustomFieldItemId, Identifier, IssueTypeId, PriorityId, ProjectId,
};
use backlog_issue::api::{AddIssueParamsBuilder, UpdateIssueParamsBuilder};
use backlog_issue::models::{CustomFieldInput, CustomFieldTypeId};
use chrono::NaiveDate;
use std::collections::HashMap;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize client with environment variables
    let base_url = env::var("BACKLOG_BASE_URL")?;
    let api_key = env::var("BACKLOG_API_KEY")?;
    let client = BacklogApiClient::new(&base_url)?.with_api_key(&api_key);

    // Example project ID (replace with your actual project ID)
    let project_id = ProjectId::new(1);

    // Example 1: Create an issue with custom fields
    println!("Creating issue with custom fields...");
    let issue = create_issue_with_custom_fields(&client, project_id).await?;
    println!("Created issue: {}", issue.issue_key);

    // Example 2: Update the issue with new custom field values
    println!("\nUpdating issue custom fields...");
    update_issue_custom_fields(&client, &issue.issue_key).await?;
    println!("Updated issue: {}", issue.issue_key);

    // Example 3: Read and display custom fields
    println!("\nReading custom fields from issue...");
    read_custom_fields(&client, &issue.issue_key).await?;

    Ok(())
}

/// Create an issue with various custom field types
async fn create_issue_with_custom_fields(
    client: &BacklogApiClient,
    project_id: ProjectId,
) -> Result<backlog_issue::models::Issue, Box<dyn std::error::Error>> {
    // Prepare custom fields
    let mut custom_fields = HashMap::new();

    // Text field (ID: 1)
    custom_fields.insert(
        CustomFieldId::new(1),
        CustomFieldInput::Text("This is a sample text field".to_string()),
    );

    // TextArea field (ID: 2)
    custom_fields.insert(
        CustomFieldId::new(2),
        CustomFieldInput::TextArea(
            "This is a multi-line\ntext area field\nwith multiple lines".to_string(),
        ),
    );

    // Numeric field (ID: 3)
    custom_fields.insert(CustomFieldId::new(3), CustomFieldInput::Numeric(42.5));

    // Date field (ID: 4)
    custom_fields.insert(
        CustomFieldId::new(4),
        CustomFieldInput::Date(NaiveDate::from_ymd_opt(2024, 6, 24).unwrap()),
    );

    // Single selection list (ID: 5)
    // Assuming option ID 100 exists in your custom field definition
    custom_fields.insert(
        CustomFieldId::new(5),
        CustomFieldInput::SingleList {
            id: CustomFieldItemId::new(100),
            other_value: Some("Additional information".to_string()),
        },
    );

    // Multiple selection list (ID: 6)
    // Assuming option IDs 200, 201, 202 exist in your custom field definition
    custom_fields.insert(
        CustomFieldId::new(6),
        CustomFieldInput::MultipleList {
            ids: vec![
                CustomFieldItemId::new(200),
                CustomFieldItemId::new(201),
                CustomFieldItemId::new(202),
            ],
            other_value: Some("Other notes".to_string()),
        },
    );

    // Checkbox field (ID: 7)
    // Assuming option IDs 300, 301 exist in your custom field definition
    custom_fields.insert(
        CustomFieldId::new(7),
        CustomFieldInput::CheckBox(vec![
            CustomFieldItemId::new(300),
            CustomFieldItemId::new(301),
        ]),
    );

    // Radio button field (ID: 8)
    // Assuming option ID 400 exists in your custom field definition
    custom_fields.insert(
        CustomFieldId::new(8),
        CustomFieldInput::Radio {
            id: CustomFieldItemId::new(400),
            other_value: None,
        },
    );

    // Create issue parameters
    let params = AddIssueParamsBuilder::default()
        .project_id(project_id)
        .summary("Example issue with custom fields".to_string())
        .description("This issue demonstrates all custom field types".to_string())
        .issue_type_id(IssueTypeId::new(1)) // Bug
        .priority_id(PriorityId::new(2)) // Normal
        .custom_fields(custom_fields)
        .build()?;

    // Create the issue
    let issue = client.issue().add_issue(params).await?;

    Ok(issue)
}

/// Update custom fields on an existing issue
async fn update_issue_custom_fields(
    client: &BacklogApiClient,
    issue_key: &IssueKey,
) -> Result<(), Box<dyn std::error::Error>> {
    // Prepare updated custom fields
    let mut custom_fields = HashMap::new();

    // Update text field
    custom_fields.insert(
        CustomFieldId::new(1),
        CustomFieldInput::Text("Updated text value".to_string()),
    );

    // Update numeric field
    custom_fields.insert(CustomFieldId::new(3), CustomFieldInput::Numeric(123.456));

    // Update date field
    custom_fields.insert(
        CustomFieldId::new(4),
        CustomFieldInput::Date(NaiveDate::from_ymd_opt(2024, 12, 31).unwrap()),
    );

    // Update parameters
    let params = UpdateIssueParamsBuilder::default()
        .issue_id_or_key(issue_key.clone())
        .custom_fields(custom_fields)
        .build()?;

    // Update the issue
    client.issue().update_issue(params).await?;

    Ok(())
}

/// Read and display custom fields from an issue
async fn read_custom_fields(
    client: &BacklogApiClient,
    issue_key: &IssueKey,
) -> Result<(), Box<dyn std::error::Error>> {
    // Get the issue
    let issue = client
        .issue()
        .get_issue(backlog_issue::api::GetIssueParams::new(issue_key.clone()))
        .await?;

    // Display custom fields
    println!("Custom fields for issue {}:", issue.issue_key);
    for field in &issue.custom_fields {
        print!(
            "  {} (ID: {}, Type: {:?}): ",
            field.name,
            field.id.value(),
            field.field_type_id
        );

        // Display value based on field type
        match field.field_type_id {
            CustomFieldTypeId::Text | CustomFieldTypeId::TextArea => {
                // Text or TextArea
                if let Some(text) = field.value.as_str() {
                    println!("{text}");
                } else {
                    println!("{:?}", field.value);
                }
            }
            CustomFieldTypeId::Numeric => {
                // Numeric
                if let Some(num) = field.value.as_f64() {
                    println!("{num}");
                } else {
                    println!("{:?}", field.value);
                }
            }
            CustomFieldTypeId::Date => {
                // Date
                if let Some(date) = field.value.as_str() {
                    println!("{date}");
                } else {
                    println!("{:?}", field.value);
                }
            }
            CustomFieldTypeId::SingleList | CustomFieldTypeId::Radio => {
                // SingleList or Radio
                if let Some(obj) = field.value.as_object() {
                    if let Some(name) = obj.get("name").and_then(|v| v.as_str()) {
                        print!("{name}");
                        if let Some(other) = &field.other_value
                            && let Some(other_str) = other.as_str()
                        {
                            print!(" (Other: {other_str})");
                        }
                        println!();
                    } else {
                        println!("{:?}", field.value);
                    }
                } else {
                    println!("{:?}", field.value);
                }
            }
            CustomFieldTypeId::MultipleList | CustomFieldTypeId::CheckBox => {
                // MultipleList or CheckBox
                if let Some(arr) = field.value.as_array() {
                    let names: Vec<String> = arr
                        .iter()
                        .filter_map(|v| {
                            v.as_object()
                                .and_then(|obj| obj.get("name"))
                                .and_then(|name| name.as_str())
                                .map(|s| s.to_string())
                        })
                        .collect();
                    print!("[{}]", names.join(", "));
                    if let Some(other) = &field.other_value
                        && let Some(other_str) = other.as_str()
                    {
                        print!(" (Other: {other_str})");
                    }
                    println!();
                } else {
                    println!("{:?}", field.value);
                }
            }
        }
    }

    Ok(())
}

// Note: To run this example:
// 1. Set your BACKLOG_BASE_URL and BACKLOG_API_KEY environment variables
// 2. Replace the project ID and custom field IDs with your actual values
// 3. Ensure the custom field option IDs match your project's configuration
// 4. Run with: cargo run --example custom_fields_example --features "issue issue_writable"
