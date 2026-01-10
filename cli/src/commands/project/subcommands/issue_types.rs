//! Project issue type management commands

use crate::commands::common::{CliResult, parse_project_id_or_key};
use backlog_api_client::client::BacklogApiClient;
use backlog_core::identifier::IssueTypeId;
use backlog_project::GetIssueTypeListParams;

#[cfg(feature = "project_writable")]
use backlog_domain_models::IssueTypeColor;
#[cfg(feature = "project_writable")]
use backlog_project::api::{AddIssueTypeParams, DeleteIssueTypeParams, UpdateIssueTypeParams};

/// List issue types for a project
pub async fn list(client: &BacklogApiClient, project_id_or_key: &str) -> CliResult<()> {
    println!("Listing issue types for project: {project_id_or_key}");

    let proj_id_or_key = parse_project_id_or_key(project_id_or_key)?;
    let params = GetIssueTypeListParams::new(proj_id_or_key);
    match client.project().get_issue_type_list(params).await {
        Ok(issue_types) => {
            if issue_types.is_empty() {
                println!("No issue types found");
            } else {
                for issue_type in issue_types {
                    println!(
                        "[{}] {} (Color: {})",
                        issue_type.id, issue_type.name, issue_type.color
                    );
                }
            }
        }
        Err(e) => {
            eprintln!("Error listing issue types: {e}");
        }
    }
    Ok(())
}

/// Add an issue type to a project
#[cfg(feature = "project_writable")]
pub async fn add(
    client: &BacklogApiClient,
    project_id_or_key: &str,
    name: &str,
    color: &str,
    template_summary: Option<String>,
    template_description: Option<String>,
) -> CliResult<()> {
    println!("Adding issue type '{name}' to project: {project_id_or_key}");

    let proj_id_or_key = parse_project_id_or_key(project_id_or_key)?;

    // Parse and validate the color
    let parsed_color = color.parse::<IssueTypeColor>().map_err(|e| {
        format!(
            "Invalid color '{}': {}\nAvailable colors: {}",
            color,
            e,
            IssueTypeColor::all_names().join(", ")
        )
    })?;

    let mut params = AddIssueTypeParams::new(proj_id_or_key, name, parsed_color);
    params.template_summary = template_summary.clone();
    params.template_description = template_description.clone();

    match client.project().add_issue_type(params).await {
        Ok(issue_type) => {
            println!("Issue type added successfully:");
            println!(
                "[{}] {} (Color: {})",
                issue_type.id, issue_type.name, issue_type.color
            );
            if let Some(template_summary) = &issue_type.template_summary {
                println!("  Template Summary: {template_summary}");
            }
            if let Some(template_description) = &issue_type.template_description {
                println!("  Template Description: {template_description}");
            }
            println!("  Display Order: {}", issue_type.display_order);
        }
        Err(e) => {
            eprintln!("Error adding issue type: {e}");
        }
    }
    Ok(())
}

/// Update an issue type in a project
#[cfg(feature = "project_writable")]
pub async fn update(
    client: &BacklogApiClient,
    project_id_or_key: &str,
    issue_type_id: u32,
    name: Option<String>,
    color: Option<String>,
    template_summary: Option<String>,
    template_description: Option<String>,
) -> CliResult<()> {
    println!("Updating issue type {issue_type_id} in project: {project_id_or_key}");

    let proj_id_or_key = parse_project_id_or_key(project_id_or_key)?;
    let issue_type_id_val = IssueTypeId::new(issue_type_id);

    // Parse color if provided
    let parsed_color = if let Some(color_str) = color {
        Some(color_str.parse::<IssueTypeColor>().map_err(|e| {
            format!(
                "Invalid color '{}': {}\nAvailable colors: {}",
                color_str,
                e,
                IssueTypeColor::all_names().join(", ")
            )
        })?)
    } else {
        None
    };

    let mut params = UpdateIssueTypeParams::new(proj_id_or_key, issue_type_id_val);
    params.name = name.clone();
    params.color = parsed_color;
    params.template_summary = template_summary.clone();
    params.template_description = template_description.clone();

    match client.project().update_issue_type(params).await {
        Ok(issue_type) => {
            println!("Issue type updated successfully:");
            println!(
                "[{}] {} (Color: {})",
                issue_type.id, issue_type.name, issue_type.color
            );
            if let Some(template_summary) = &issue_type.template_summary {
                println!("  Template Summary: {template_summary}");
            }
            if let Some(template_description) = &issue_type.template_description {
                println!("  Template Description: {template_description}");
            }
            println!("  Display Order: {}", issue_type.display_order);
        }
        Err(e) => {
            eprintln!("Error updating issue type: {e}");
        }
    }
    Ok(())
}

/// Delete an issue type from a project
#[cfg(feature = "project_writable")]
pub async fn delete(
    client: &BacklogApiClient,
    project_id_or_key: &str,
    issue_type_id: u32,
    substitute_issue_type_id: u32,
) -> CliResult<()> {
    println!(
        "Deleting issue type {issue_type_id} from project: {project_id_or_key} (substitute: {substitute_issue_type_id})"
    );

    let proj_id_or_key = parse_project_id_or_key(project_id_or_key)?;
    let issue_type_id_val = IssueTypeId::new(issue_type_id);
    let substitute_id = IssueTypeId::new(substitute_issue_type_id);

    let params = DeleteIssueTypeParams::new(proj_id_or_key, issue_type_id_val, substitute_id);

    match client.project().delete_issue_type(params).await {
        Ok(issue_type) => {
            println!("Issue type deleted successfully:");
            println!(
                "[{}] {} (Color: {})",
                issue_type.id, issue_type.name, issue_type.color
            );
            if let Some(template_summary) = &issue_type.template_summary {
                println!("  Template Summary: {template_summary}");
            }
            if let Some(template_description) = &issue_type.template_description {
                println!("  Template Description: {template_description}");
            }
            println!("  Display Order: {}", issue_type.display_order);
        }
        Err(e) => {
            eprintln!("Error deleting issue type: {e}");
        }
    }
    Ok(())
}
