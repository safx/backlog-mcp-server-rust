//! Project custom field management commands

use crate::commands::common::{parse_project_id_or_key, CliResult};
use backlog_api_client::client::BacklogApiClient;
use backlog_core::identifier::{CustomFieldId, CustomFieldItemId, IssueTypeId};
use backlog_project::GetCustomFieldListParams;

#[cfg(feature = "project_writable")]
use backlog_project::api::{
    AddCustomFieldParams, AddListItemToCustomFieldParams, DeleteCustomFieldParams,
    DeleteListItemFromCustomFieldParams, UpdateCustomFieldParams,
    UpdateListItemToCustomFieldParams,
};

/// List custom fields for a project
pub async fn list(client: &BacklogApiClient, project_id_or_key: &str) -> CliResult<()> {
    println!("Listing custom fields for project: {project_id_or_key}");

    let proj_id_or_key = parse_project_id_or_key(project_id_or_key)?;
    let params = GetCustomFieldListParams::new(proj_id_or_key);
    match client.project().get_custom_field_list(params).await {
        Ok(custom_fields) => {
            if custom_fields.is_empty() {
                println!("No custom fields found in this project");
            } else {
                for field in custom_fields {
                    let field_type = match field.settings {
                        backlog_domain_models::CustomFieldSettings::Text => "Text",
                        backlog_domain_models::CustomFieldSettings::TextArea => "TextArea",
                        backlog_domain_models::CustomFieldSettings::Numeric(_) => "Numeric",
                        backlog_domain_models::CustomFieldSettings::Date(_) => "Date",
                        backlog_domain_models::CustomFieldSettings::SingleList(_) => "SingleList",
                        backlog_domain_models::CustomFieldSettings::MultipleList(_) => {
                            "MultipleList"
                        }
                        backlog_domain_models::CustomFieldSettings::Checkbox(_) => "Checkbox",
                        backlog_domain_models::CustomFieldSettings::Radio(_) => "Radio",
                    };
                    let required_str = if field.required {
                        "Required"
                    } else {
                        "Optional"
                    };
                    println!(
                        "[{}] {} ({}) - {} - Display Order: {}",
                        field.id, field.name, field_type, required_str, field.display_order
                    );
                    if !field.description.is_empty() {
                        println!("    Description: {}", field.description);
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error listing custom fields: {e}");
        }
    }
    Ok(())
}

/// Add a custom field to a project
#[cfg(feature = "project_writable")]
#[allow(clippy::too_many_arguments)]
pub async fn add(
    client: &BacklogApiClient,
    project_id_or_key: &str,
    field_type: &str,
    name: &str,
    description: Option<String>,
    required: Option<bool>,
    applicable_issue_types: Option<String>,
    min: Option<f64>,
    max: Option<f64>,
    initial_value: Option<f64>,
    unit: Option<String>,
    min_date: Option<String>,
    max_date: Option<String>,
    initial_value_type: Option<i32>,
    initial_date: Option<String>,
    initial_shift: Option<i32>,
    items: Option<String>,
    allow_input: Option<bool>,
    allow_add_item: Option<bool>,
) -> CliResult<()> {
    println!("Adding custom field '{name}' to project: {project_id_or_key}");

    let proj_id_or_key = parse_project_id_or_key(project_id_or_key)?;

    // Create params based on field type
    let mut params = match field_type {
        "text" => AddCustomFieldParams::text(proj_id_or_key, name.to_string()),
        "textarea" => AddCustomFieldParams::textarea(proj_id_or_key, name.to_string()),
        "numeric" => AddCustomFieldParams::numeric(proj_id_or_key, name.to_string()),
        "date" => AddCustomFieldParams::date(proj_id_or_key, name.to_string()),
        "single-list" => {
            if let Some(items_str) = items.as_ref() {
                let items_vec: Vec<String> =
                    items_str.split(',').map(|s| s.trim().to_string()).collect();
                AddCustomFieldParams::single_list(proj_id_or_key, name.to_string(), items_vec)
            } else {
                eprintln!("Error: --items is required for single-list field type");
                std::process::exit(1);
            }
        }
        "multiple-list" => {
            if let Some(items_str) = items.as_ref() {
                let items_vec: Vec<String> =
                    items_str.split(',').map(|s| s.trim().to_string()).collect();
                AddCustomFieldParams::multiple_list(proj_id_or_key, name.to_string(), items_vec)
            } else {
                eprintln!("Error: --items is required for multiple-list field type");
                std::process::exit(1);
            }
        }
        "checkbox" => AddCustomFieldParams::checkbox(proj_id_or_key, name.to_string()),
        "radio" => AddCustomFieldParams::radio(proj_id_or_key, name.to_string()),
        _ => {
            eprintln!("Error: Invalid field type '{field_type}'");
            eprintln!(
                "Valid types: text, textarea, numeric, date, single-list, multiple-list, checkbox, radio"
            );
            std::process::exit(1);
        }
    };

    // Set common optional parameters
    if let Some(d) = description {
        params = params.with_description(d);
    }
    if let Some(r) = required {
        params = params.with_required(r);
    }
    if let Some(types) = applicable_issue_types {
        let type_ids: Vec<IssueTypeId> = types
            .split(',')
            .filter_map(|s| s.trim().parse::<u32>().ok())
            .map(IssueTypeId::new)
            .collect();
        if !type_ids.is_empty() {
            params = params.with_applicable_issue_types(type_ids);
        }
    }

    // Set field-type specific parameters
    match field_type {
        "numeric" => {
            params = params.with_numeric_settings(min, max, initial_value, unit.clone());
        }
        "date" => {
            let min_date_parsed = min_date
                .as_ref()
                .and_then(|d| str::parse::< backlog_core::Date>(d).ok());
            let max_date_parsed = max_date
                .as_ref()
                .and_then(|d| str::parse::< backlog_core::Date>(d).ok());
            let initial_date_parsed = initial_date
                .as_ref()
                .and_then(|d| str::parse::< backlog_core::Date>(d).ok());

            params = params.with_date_settings(
                min_date_parsed,
                max_date_parsed,
                initial_value_type,
                initial_date_parsed,
                initial_shift,
            );
        }
        "single-list" | "multiple-list" => {
            if let Some(allow_input_val) = allow_input {
                params = params.with_allow_input(allow_input_val);
            }
            if let Some(allow_add_item_val) = allow_add_item {
                params = params.with_allow_add_item(allow_add_item_val);
            }
        }
        _ => {}
    }

    match client.project().add_custom_field(params).await {
        Ok(field) => {
            println!("✅ Custom field added successfully:");
            println!("[{}] {}", field.id, field.name);
            let field_type = match &field.settings {
                backlog_domain_models::CustomFieldSettings::Text => "Text",
                backlog_domain_models::CustomFieldSettings::TextArea => "TextArea",
                backlog_domain_models::CustomFieldSettings::Numeric(_) => "Numeric",
                backlog_domain_models::CustomFieldSettings::Date(_) => "Date",
                backlog_domain_models::CustomFieldSettings::SingleList(_) => "SingleList",
                backlog_domain_models::CustomFieldSettings::MultipleList(_) => "MultipleList",
                backlog_domain_models::CustomFieldSettings::Checkbox(_) => "Checkbox",
                backlog_domain_models::CustomFieldSettings::Radio(_) => "Radio",
            };
            println!("Type: {field_type}");
            if !field.description.is_empty() {
                println!("Description: {}", field.description);
            }
            println!("Required: {}", field.required);
            if let Some(issue_types) = &field.applicable_issue_types
                && !issue_types.is_empty()
            {
                let ids: Vec<String> = issue_types.iter().map(|id| id.to_string()).collect();
                println!("Applicable Issue Types: {}", ids.join(", "));
            }
        }
        Err(e) => {
            eprintln!("❌ Error adding custom field: {e}");
            std::process::exit(1);
        }
    }
    Ok(())
}

/// Update a custom field in a project
#[cfg(feature = "project_writable")]
#[allow(clippy::too_many_arguments)]
pub async fn update(
    client: &BacklogApiClient,
    project_id_or_key: &str,
    custom_field_id: u32,
    name: Option<String>,
    description: Option<String>,
    required: Option<bool>,
    applicable_issue_types: Option<String>,
    min_date: Option<String>,
    max_date: Option<String>,
    initial_value_type: Option<i32>,
    initial_date: Option<String>,
    initial_shift: Option<i32>,
) -> CliResult<()> {
    println!("Updating custom field {custom_field_id} in project: {project_id_or_key}");

    let proj_id_or_key = parse_project_id_or_key(project_id_or_key)?;
    let field_id = CustomFieldId::new(custom_field_id);
    let mut params = UpdateCustomFieldParams::new(proj_id_or_key, field_id);

    // Set optional parameters
    if let Some(n) = name {
        params = params.with_name(n);
    }
    if let Some(d) = description {
        params = params.with_description(d);
    }
    if let Some(r) = required {
        params = params.with_required(r);
    }
    if let Some(types) = applicable_issue_types {
        let type_ids: Vec<IssueTypeId> = types
            .split(',')
            .filter_map(|s| s.trim().parse::<u32>().ok())
            .map(IssueTypeId::new)
            .collect();
        if !type_ids.is_empty() {
            params = params.with_applicable_issue_types(type_ids);
        }
    }

    // Handle date field specific parameters
    if min_date.is_some()
        || max_date.is_some()
        || initial_value_type.is_some()
        || initial_date.is_some()
        || initial_shift.is_some()
    {
        let min_date_parsed = min_date
            .as_ref()
            .and_then(|d| str::parse::< backlog_core::Date>(d).ok());
        let max_date_parsed = max_date
            .as_ref()
            .and_then(|d| str::parse::< backlog_core::Date>(d).ok());
        let initial_date_parsed = initial_date
            .as_ref()
            .and_then(|d| str::parse::< backlog_core::Date>(d).ok());

        params = params.with_date_settings(
            min_date_parsed,
            max_date_parsed,
            initial_value_type,
            initial_date_parsed,
            initial_shift,
        );
    }

    match client.project().update_custom_field(params).await {
        Ok(field) => {
            println!("Custom field updated successfully:");
            println!("[{}] {}", field.id, field.name);
            if !field.description.is_empty() {
                println!("Description: {}", field.description);
            }
            println!("Required: {}", field.required);
            if let Some(issue_types) = &field.applicable_issue_types
                && !issue_types.is_empty()
            {
                let ids: Vec<String> = issue_types.iter().map(|id| id.to_string()).collect();
                println!("Applicable Issue Types: {}", ids.join(", "));
            }
        }
        Err(e) => {
            eprintln!("Error updating custom field: {e}");
        }
    }
    Ok(())
}

/// Delete a custom field from a project
#[cfg(feature = "project_writable")]
pub async fn delete(
    client: &BacklogApiClient,
    project_id_or_key: &str,
    custom_field_id: u32,
) -> CliResult<()> {
    println!("Deleting custom field {custom_field_id} from project: {project_id_or_key}");

    let proj_id_or_key = parse_project_id_or_key(project_id_or_key)?;
    let field_id = CustomFieldId::new(custom_field_id);
    let params = DeleteCustomFieldParams::new(proj_id_or_key, field_id);

    match client.project().delete_custom_field(params).await {
        Ok(field) => {
            println!("✅ Custom field deleted successfully:");
            println!("[{}] {}", field.id, field.name);
            let field_type = match &field.settings {
                backlog_domain_models::CustomFieldSettings::Text => "Text",
                backlog_domain_models::CustomFieldSettings::TextArea => "TextArea",
                backlog_domain_models::CustomFieldSettings::Numeric(_) => "Numeric",
                backlog_domain_models::CustomFieldSettings::Date(_) => "Date",
                backlog_domain_models::CustomFieldSettings::SingleList(_) => "SingleList",
                backlog_domain_models::CustomFieldSettings::MultipleList(_) => "MultipleList",
                backlog_domain_models::CustomFieldSettings::Checkbox(_) => "Checkbox",
                backlog_domain_models::CustomFieldSettings::Radio(_) => "Radio",
            };
            println!("Type: {field_type}");
            if !field.description.is_empty() {
                println!("Description: {}", field.description);
            }
        }
        Err(e) => {
            eprintln!("❌ Error deleting custom field: {e}");
            std::process::exit(1);
        }
    }
    Ok(())
}

/// Add a list item to a list type custom field
#[cfg(feature = "project_writable")]
pub async fn add_item(
    client: &BacklogApiClient,
    project_id_or_key: &str,
    custom_field_id: u32,
    name: &str,
) -> CliResult<()> {
    println!(
        "Adding list item '{name}' to custom field {custom_field_id} in project: {project_id_or_key}"
    );

    let proj_id_or_key = parse_project_id_or_key(project_id_or_key)?;
    let field_id = CustomFieldId::new(custom_field_id);
    let params = AddListItemToCustomFieldParams::new(proj_id_or_key, field_id, name.to_string());

    match client.project().add_list_item_to_custom_field(params).await {
        Ok(field) => {
            println!("✅ List item added successfully to custom field:");
            println!("[{}] {}", field.id, field.name);

            // Display list items if it's a list type field
            match &field.settings {
                backlog_domain_models::CustomFieldSettings::SingleList(settings) => {
                    println!("Type: Single Selection List");
                    println!("List items:");
                    for item in &settings.items {
                        println!(
                            "  - [{}] {} (order: {})",
                            item.id, item.name, item.display_order
                        );
                    }
                    if let Some(allow) = settings.allow_add_item {
                        println!("Allow add item: {allow}");
                    }
                }
                backlog_domain_models::CustomFieldSettings::MultipleList(settings) => {
                    println!("Type: Multiple Selection List");
                    println!("List items:");
                    for item in &settings.items {
                        println!(
                            "  - [{}] {} (order: {})",
                            item.id, item.name, item.display_order
                        );
                    }
                    if let Some(allow) = settings.allow_add_item {
                        println!("Allow add item: {allow}");
                    }
                    if let Some(allow) = settings.allow_input {
                        println!("Allow input: {allow}");
                    }
                }
                _ => {
                    eprintln!("⚠️  Warning: Custom field is not a list type");
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Error adding list item to custom field: {e}");
            std::process::exit(1);
        }
    }
    Ok(())
}

/// Update a list item in a list type custom field
#[cfg(feature = "project_writable")]
pub async fn update_item(
    client: &BacklogApiClient,
    project_id_or_key: &str,
    custom_field_id: u32,
    item_id: u32,
    name: &str,
) -> CliResult<()> {
    println!(
        "Updating list item {item_id} in custom field {custom_field_id} in project: {project_id_or_key}"
    );

    let proj_id_or_key = parse_project_id_or_key(project_id_or_key)?;
    let field_id = CustomFieldId::new(custom_field_id);
    let params = UpdateListItemToCustomFieldParams::new(
        proj_id_or_key,
        field_id,
        item_id,
        name.to_string(),
    );

    match client
        .project()
        .update_list_item_to_custom_field(params)
        .await
    {
        Ok(field) => {
            println!("✅ List item updated successfully in custom field:");
            println!("[{}] {}", field.id, field.name);

            // Display list items if it's a list type field
            match &field.settings {
                backlog_domain_models::CustomFieldSettings::SingleList(settings) => {
                    println!("Type: Single Selection List");
                    println!("List items:");
                    for item in &settings.items {
                        if item.id == CustomFieldItemId::new(item_id) {
                            println!(
                                "  - [{}] {} (order: {}) ← UPDATED",
                                item.id, item.name, item.display_order
                            );
                        } else {
                            println!(
                                "  - [{}] {} (order: {})",
                                item.id, item.name, item.display_order
                            );
                        }
                    }
                    if let Some(allow) = settings.allow_add_item {
                        println!("Allow add item: {allow}");
                    }
                }
                backlog_domain_models::CustomFieldSettings::MultipleList(settings) => {
                    println!("Type: Multiple Selection List");
                    println!("List items:");
                    for item in &settings.items {
                        if item.id == CustomFieldItemId::new(item_id) {
                            println!(
                                "  - [{}] {} (order: {}) ← UPDATED",
                                item.id, item.name, item.display_order
                            );
                        } else {
                            println!(
                                "  - [{}] {} (order: {})",
                                item.id, item.name, item.display_order
                            );
                        }
                    }
                    if let Some(allow) = settings.allow_add_item {
                        println!("Allow add item: {allow}");
                    }
                    if let Some(allow) = settings.allow_input {
                        println!("Allow input: {allow}");
                    }
                }
                _ => {
                    eprintln!("⚠️  Warning: Custom field is not a list type");
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Error updating list item in custom field: {e}");
            std::process::exit(1);
        }
    }
    Ok(())
}

/// Delete a list item from a list type custom field
#[cfg(feature = "project_writable")]
pub async fn delete_item(
    client: &BacklogApiClient,
    project_id_or_key: &str,
    custom_field_id: u32,
    item_id: u32,
) -> CliResult<()> {
    println!(
        "Deleting list item {item_id} from custom field {custom_field_id} in project: {project_id_or_key}"
    );

    let proj_id_or_key = parse_project_id_or_key(project_id_or_key)?;
    let field_id = CustomFieldId::new(custom_field_id);
    let item_id_val = CustomFieldItemId::new(item_id);
    let params = DeleteListItemFromCustomFieldParams::new(proj_id_or_key, field_id, item_id_val);

    match client
        .project()
        .delete_list_item_from_custom_field(params)
        .await
    {
        Ok(field) => {
            println!("✅ List item deleted successfully from custom field:");
            println!("[{}] {}", field.id, field.name);

            // Display remaining list items
            match &field.settings {
                backlog_domain_models::CustomFieldSettings::SingleList(settings) => {
                    println!("Type: Single Selection List");
                    println!("Remaining list items:");
                    for item in &settings.items {
                        println!(
                            "  - [{}] {} (order: {})",
                            item.id, item.name, item.display_order
                        );
                    }
                    if let Some(allow) = settings.allow_add_item {
                        println!("Allow add item: {allow}");
                    }
                }
                backlog_domain_models::CustomFieldSettings::MultipleList(settings) => {
                    println!("Type: Multiple Selection List");
                    println!("Remaining list items:");
                    for item in &settings.items {
                        println!(
                            "  - [{}] {} (order: {})",
                            item.id, item.name, item.display_order
                        );
                    }
                    if let Some(allow) = settings.allow_add_item {
                        println!("Allow add item: {allow}");
                    }
                    if let Some(allow) = settings.allow_input {
                        println!("Allow input: {allow}");
                    }
                }
                _ => {
                    eprintln!("⚠️  Warning: Custom field is not a list type");
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Error deleting list item from custom field: {e}");
            std::process::exit(1);
        }
    }
    Ok(())
}
