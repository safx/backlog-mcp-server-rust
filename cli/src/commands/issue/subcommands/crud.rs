//! Create, Update, Delete operations for issues
//!
//! This module provides handlers for issue modification operations.
//! All functions require the `issue_writable` feature flag.

use crate::commands::common::CliResult;
use backlog_api_client::ProjectIdOrKey;
use backlog_api_client::client::BacklogApiClient;
use backlog_core::IssueKey;
use backlog_core::identifier::{
    CategoryId, IssueTypeId, MilestoneId, PriorityId, ResolutionId, UserId,
};
use backlog_issue::{AddIssueParamsBuilder, DeleteIssueParams, UpdateIssueParamsBuilder};
use blg::custom_fields;

/// Create a new issue
///
/// Corresponds to `POST /api/v2/issues`
#[cfg(feature = "issue_writable")]
pub async fn create(
    client: &BacklogApiClient,
    args: crate::commands::issue::args::CreateIssueArgs,
) -> CliResult<()> {
    println!("Creating new issue...");

    let project_id_or_key = args.project_id.parse::<ProjectIdOrKey>()?;
    let project_id = match project_id_or_key {
        ProjectIdOrKey::Id(id) => id,
        ProjectIdOrKey::Key(_) => {
            return Err(
                "Project key not supported for issue creation. Please use project ID.".into(),
            );
        }
        ProjectIdOrKey::EitherIdOrKey(id, _) => id,
    };

    let mut builder = AddIssueParamsBuilder::default();
    builder
        .project_id(project_id)
        .summary(&args.summary)
        .issue_type_id(IssueTypeId::new(args.issue_type_id))
        .priority_id(PriorityId::new(args.priority_id));

    if let Some(description) = &args.description {
        builder.description(description);
    }

    if let Some(assignee_id) = args.assignee_id {
        builder.assignee_id(UserId::new(assignee_id));
    }

    if let Some(_due_date) = &args.due_date {
        // Due date parsing would need proper DateTime conversion
        // For now, skip this implementation detail
    }

    if let Some(category_str) = &args.category_ids {
        let category_ids: Result<Vec<CategoryId>, _> = category_str
            .split(',')
            .map(|s| s.trim().parse::<u32>().map(CategoryId::new))
            .collect();
        if let Ok(ids) = category_ids {
            builder.category_id(ids);
        }
    }

    if let Some(milestone_str) = &args.milestone_ids {
        let milestone_ids: Result<Vec<MilestoneId>, _> = milestone_str
            .split(',')
            .map(|s| s.trim().parse::<u32>().map(MilestoneId::new))
            .collect();
        if let Ok(ids) = milestone_ids {
            builder.milestone_id(ids);
        }
    }

    // Handle custom fields
    let custom_fields_map = if let Some(json_path) = &args.custom_fields_json {
        let path_str = json_path
            .to_str()
            .ok_or_else(|| format!("Invalid UTF-8 in file path: {:?}", json_path))?;
        match custom_fields::parse_custom_fields_json(path_str) {
            Ok(fields) => Some(fields),
            Err(e) => {
                return Err(format!("Error parsing custom fields JSON: {e}").into());
            }
        }
    } else if !args.custom_fields.is_empty() {
        match custom_fields::parse_custom_field_args(&args.custom_fields) {
            Ok(fields) => Some(fields),
            Err(e) => {
                return Err(format!("Error parsing custom fields: {e}").into());
            }
        }
    } else {
        None
    };

    if let Some(fields) = custom_fields_map {
        builder.custom_fields(fields);
    }

    let params = builder.build()?;

    let issue = client.issue().add_issue(params).await?;
    println!("Issue created successfully!");
    println!("Issue Key: {}", issue.issue_key);
    println!("Issue ID: {}", issue.id);
    println!("Summary: {}", issue.summary);
    println!("Status: {}", issue.status.name);
    Ok(())
}

/// Update an existing issue
///
/// Corresponds to `PATCH /api/v2/issues/:issueIdOrKey`
#[cfg(feature = "issue_writable")]
pub async fn update(
    client: &BacklogApiClient,
    args: crate::commands::issue::args::UpdateIssueArgs,
) -> CliResult<()> {
    println!("Updating issue: {}", args.issue_id_or_key);

    let issue_id_or_key = args
        .issue_id_or_key
        .parse::<backlog_api_client::IssueIdOrKey>()?;

    let mut builder = UpdateIssueParamsBuilder::default();
    builder.issue_id_or_key(issue_id_or_key);

    if let Some(summary) = &args.summary {
        builder.summary(summary);
    }

    if let Some(description) = &args.description {
        builder.description(description);
    }

    if let Some(issue_type_id) = args.issue_type_id {
        builder.issue_type_id(IssueTypeId::new(issue_type_id));
    }

    if let Some(priority_id) = args.priority_id {
        builder.priority_id(PriorityId::new(priority_id));
    }

    if let Some(status_id) = &args.status_id {
        builder.status_id(status_id);
    }

    if let Some(assignee_id) = args.assignee_id {
        builder.assignee_id(UserId::new(assignee_id));
    }

    if let Some(resolution_id) = args.resolution_id {
        builder.resolution_id(ResolutionId::new(resolution_id));
    }

    if let Some(comment) = &args.comment {
        builder.comment(comment);
    }

    // Handle custom fields
    let custom_fields_map = if let Some(json_path) = &args.custom_fields_json {
        let path_str = json_path
            .to_str()
            .ok_or_else(|| format!("Invalid UTF-8 in file path: {:?}", json_path))?;
        match custom_fields::parse_custom_fields_json(path_str) {
            Ok(fields) => Some(fields),
            Err(e) => {
                return Err(format!("Error parsing custom fields JSON: {e}").into());
            }
        }
    } else if !args.custom_fields.is_empty() {
        match custom_fields::parse_custom_field_args(&args.custom_fields) {
            Ok(fields) => Some(fields),
            Err(e) => {
                return Err(format!("Error parsing custom fields: {e}").into());
            }
        }
    } else {
        None
    };

    if let Some(fields) = custom_fields_map {
        builder.custom_fields(fields);
    }

    let params = builder.build()?;

    let issue = client.issue().update_issue(params).await?;
    println!("Issue updated successfully!");
    println!("Issue Key: {}", issue.issue_key);
    println!("Summary: {}", issue.summary);
    println!("Status: {}", issue.status.name);
    Ok(())
}

/// Delete an issue
///
/// Corresponds to `DELETE /api/v2/issues/:issueKey`
#[cfg(feature = "issue_writable")]
pub async fn delete(client: &BacklogApiClient, issue_key: String) -> CliResult<()> {
    println!("Deleting issue: {issue_key}");

    let issue_key = issue_key.parse::<IssueKey>()?;

    let issue = client
        .issue()
        .delete_issue(DeleteIssueParams::new(issue_key))
        .await?;
    println!("Issue deleted successfully!");
    println!("Deleted Issue Key: {}", issue.issue_key);
    println!("Summary: {}", issue.summary);
    Ok(())
}
