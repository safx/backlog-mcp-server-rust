//! List operations for issues
//!
//! This module provides handlers for basic issue listing and viewing:
//! - Show: Display details of a single issue
//! - List: Search and filter issues with various parameters
//! - RecentlyViewed: Get recently viewed issues for the current user

use crate::commands::common::{CliResult, date_to_end_of_day, date_to_start_of_day};
use backlog_api_client::client::BacklogApiClient;
use backlog_api_client::{GetIssueListParamsBuilder, IssueIdOrKey};
use backlog_core::ApiDate;
use backlog_core::identifier::{ProjectId, StatusId, UserId};
use backlog_issue::GetRecentlyViewedIssuesParamsBuilder;
use chrono::NaiveDate;

/// Show details of a specific issue
///
/// Corresponds to `GET /api/v2/issues/:issueIdOrKey`
pub async fn show(client: &BacklogApiClient, issue_id_or_key: &str) -> CliResult<()> {
    println!("Showing issue: {issue_id_or_key}");
    let parsed_issue_id_or_key = issue_id_or_key.parse::<IssueIdOrKey>()?;
    let issue = client
        .issue()
        .get_issue(backlog_issue::GetIssueParams::new(parsed_issue_id_or_key))
        .await?;
    // TODO: Pretty print issue
    println!("{issue:?}");
    Ok(())
}

/// List issues with filters
///
/// Corresponds to `GET /api/v2/issues`
pub async fn list(
    client: &BacklogApiClient,
    params: crate::commands::issue::args::IssueListCliParams,
) -> CliResult<()> {
    println!("Listing issues with params: {params:?}");
    let mut builder = GetIssueListParamsBuilder::default();

    if let Some(p_ids) = params.project_id {
        let parsed_ids: std::result::Result<Vec<ProjectId>, _> = p_ids
            .iter()
            .map(|s| s.parse::<u32>().map(ProjectId::from))
            .collect();
        builder.project_id(parsed_ids?);
    }
    if let Some(a_ids) = params.assignee_id {
        let parsed_ids: std::result::Result<Vec<UserId>, _> = a_ids
            .iter()
            .map(|s| s.parse::<u32>().map(UserId::from))
            .collect();
        builder.assignee_id(parsed_ids?);
    }
    if let Some(s_ids) = params.status_id {
        let parsed_ids: std::result::Result<Vec<StatusId>, _> = s_ids
            .iter()
            .map(|s| s.parse::<u32>().map(StatusId::from))
            .collect();
        builder.status_id(parsed_ids?);
    }
    if let Some(keyword) = params.keyword {
        builder.keyword(keyword);
    }
    builder.count(params.count); // count has a default_value_t

    // Handle date range parameters
    if let Some(start_date_since) = params.start_date_since {
        let date = NaiveDate::parse_from_str(&start_date_since, "%Y-%m-%d")
            .map_err(|_| format!("Invalid start-date-since format: {start_date_since}"))?;
        let datetime = date_to_start_of_day(date);
        builder.start_date_since(ApiDate::from(datetime));
    }
    if let Some(start_date_until) = params.start_date_until {
        let date = NaiveDate::parse_from_str(&start_date_until, "%Y-%m-%d")
            .map_err(|_| format!("Invalid start-date-until format: {start_date_until}"))?;
        let datetime = date_to_end_of_day(date);
        builder.start_date_until(ApiDate::from(datetime));
    }
    if let Some(due_date_since) = params.due_date_since {
        let date = NaiveDate::parse_from_str(&due_date_since, "%Y-%m-%d")
            .map_err(|_| format!("Invalid due-date-since format: {due_date_since}"))?;
        let datetime = date_to_start_of_day(date);
        builder.due_date_since(ApiDate::from(datetime));
    }
    if let Some(due_date_until) = params.due_date_until {
        let date = NaiveDate::parse_from_str(&due_date_until, "%Y-%m-%d")
            .map_err(|_| format!("Invalid due-date-until format: {due_date_until}"))?;
        let datetime = date_to_end_of_day(date);
        builder.due_date_until(ApiDate::from(datetime));
    }

    let list_params = builder.build()?;
    let issues = client.issue().get_issue_list(list_params).await?;
    // TODO: Pretty print issues
    println!("{issues:?}");
    Ok(())
}

/// Get recently viewed issues for the current user
///
/// Corresponds to `GET /api/v2/users/myself/recentlyViewedIssues`
pub async fn recently_viewed(
    client: &BacklogApiClient,
    order: String,
    count: u32,
    offset: Option<u32>,
) -> CliResult<()> {
    println!("Getting recently viewed issues");

    let mut builder = GetRecentlyViewedIssuesParamsBuilder::default();
    builder.order(order);
    builder.count(count);

    if let Some(offset_val) = offset {
        builder.offset(offset_val);
    }

    let params = builder.build()?;

    match client.issue().get_recently_viewed_issues(params).await {
        Ok(issues) => {
            if issues.is_empty() {
                println!("No recently viewed issues found.");
            } else {
                println!("Found {} recently viewed issue(s):", issues.len());
                println!();

                for (index, issue) in issues.iter().enumerate() {
                    println!("{}. {} - {}", index + 1, issue.issue_key, issue.summary);
                    println!("   Project ID: {}", issue.project_id);
                    println!("   Status: {}", issue.status.name);
                    if let Some(priority) = &issue.priority {
                        println!("   Priority: {}", priority.name);
                    }
                    println!("   Issue Type: {}", issue.issue_type.name);
                    if let Some(assignee) = &issue.assignee {
                        println!("   Assignee: {}", assignee.name);
                    }
                    println!("   Updated: {}", issue.updated);
                    println!();
                }
            }
        }
        Err(e) => {
            eprintln!("Error getting recently viewed issues: {e}");
        }
    }
    Ok(())
}

/// Add an issue to recently viewed list
///
/// Corresponds to `POST /api/v2/users/myself/recentlyViewedIssues`
#[cfg(feature = "issue_writable")]
pub async fn add_recently_viewed(
    client: &BacklogApiClient,
    issue_id_or_key: String,
) -> CliResult<()> {
    use backlog_core::IssueKey;
    use backlog_issue::AddRecentlyViewedIssueParams;
    use std::str::FromStr;

    println!("Adding issue {issue_id_or_key} to recently viewed list");

    let issue_id_or_key = if let Ok(id) = issue_id_or_key.parse::<u32>() {
        IssueIdOrKey::Id(id.into())
    } else {
        IssueIdOrKey::Key(IssueKey::from_str(&issue_id_or_key)?)
    };
    let params = AddRecentlyViewedIssueParams { issue_id_or_key };

    match client.issue().add_recently_viewed_issue(params).await {
        Ok(issue) => {
            println!("Successfully added issue to recently viewed list:");
            println!("  Issue Key: {}", issue.issue_key);
            println!("  Summary: {}", issue.summary);
            println!("  Status: {}", issue.status.name);
            if let Some(priority) = &issue.priority {
                println!("  Priority: {}", priority.name);
            }
            println!("  Issue Type: {}", issue.issue_type.name);
            if let Some(assignee) = &issue.assignee {
                println!("  Assignee: {}", assignee.name);
            }
            Ok(())
        }
        Err(e) => {
            eprintln!("Error adding issue to recently viewed list: {e}");
            Err(e.into())
        }
    }
}
