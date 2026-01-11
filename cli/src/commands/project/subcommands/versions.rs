//! Project version/milestone management commands

use crate::commands::common::{CliResult, parse_project_id_or_key};
use backlog_api_client::client::BacklogApiClient;
use backlog_core::identifier::MilestoneId;
use backlog_project::GetMilestoneListParams;

#[cfg(feature = "project_writable")]
use backlog_core::ApiDate;
#[cfg(feature = "project_writable")]
use backlog_project::api::{AddMilestoneParams, DeleteVersionParams, UpdateVersionParams};

/// Parse date string and convert to ApiDate
/// Returns an error if the date format is invalid (expected YYYY-MM-DD)
#[cfg(feature = "project_writable")]
fn parse_date_to_api_date(date_str: &str) -> CliResult<ApiDate> {
    use crate::commands::common::date_to_start_of_day;
    use chrono::NaiveDate;

    let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
        .map_err(|_| format!("Invalid date format: {date_str}. Expected YYYY-MM-DD"))?;
    Ok(ApiDate::from(date_to_start_of_day(date)))
}

/// List milestones for a project
pub async fn list(client: &BacklogApiClient, project_id_or_key: &str) -> CliResult<()> {
    println!("Listing milestones for project: {project_id_or_key}");

    let proj_id_or_key = parse_project_id_or_key(project_id_or_key)?;
    match client
        .project()
        .get_version_milestone_list(GetMilestoneListParams::new(proj_id_or_key))
        .await
    {
        Ok(milestones) => {
            if milestones.is_empty() {
                println!("No milestones found");
            } else {
                for milestone in milestones {
                    print!("[{}] {}", milestone.id, milestone.name);
                    if let Some(description) = &milestone.description {
                        print!(" - {description}");
                    }
                    println!();

                    if let Some(start_date) = milestone.start_date {
                        println!("  Start Date: {}", start_date.format("%Y-%m-%d"));
                    }
                    if let Some(release_date) = milestone.release_due_date {
                        println!("  Release Due: {}", release_date.format("%Y-%m-%d"));
                    }
                    if milestone.archived {
                        println!("  Status: Archived");
                    }
                    println!();
                }
            }
        }
        Err(e) => {
            eprintln!("Error listing milestones: {e}");
        }
    }
    Ok(())
}

/// Add a version/milestone to a project
#[cfg(feature = "project_writable")]
pub async fn add(
    client: &BacklogApiClient,
    project_id_or_key: &str,
    name: &str,
    description: Option<String>,
    start_date: Option<String>,
    release_due_date: Option<String>,
) -> CliResult<()> {
    println!("Adding version/milestone '{name}' to project: {project_id_or_key}");

    let proj_id_or_key = parse_project_id_or_key(project_id_or_key)?;
    let mut params = AddMilestoneParams::new(proj_id_or_key, name);
    params.description = description.clone();
    params.start_date = start_date
        .as_ref()
        .map(|d| parse_date_to_api_date(d))
        .transpose()?;
    params.release_due_date = release_due_date
        .as_ref()
        .map(|d| parse_date_to_api_date(d))
        .transpose()?;

    match client.project().add_version(params).await {
        Ok(milestone) => {
            println!("Version/milestone added successfully:");
            println!("[{}] {}", milestone.id, milestone.name);
            if let Some(desc) = &milestone.description {
                println!("  Description: {desc}");
            }
            if let Some(start_date) = &milestone.start_date {
                println!("  Start Date: {}", start_date.format("%Y-%m-%d"));
            }
            if let Some(release_due_date) = &milestone.release_due_date {
                println!(
                    "  Release Due Date: {}",
                    release_due_date.format("%Y-%m-%d")
                );
            }
            println!("  Archived: {}", milestone.archived);
            if let Some(display_order) = milestone.display_order {
                println!("  Display Order: {display_order}");
            }
        }
        Err(e) => {
            eprintln!("Error adding version/milestone: {e}");
        }
    }
    Ok(())
}

/// Update a version/milestone in a project
#[cfg(feature = "project_writable")]
pub async fn update(
    client: &BacklogApiClient,
    project_id_or_key: &str,
    version_id: u32,
    name: &str,
    description: Option<String>,
    start_date: Option<String>,
    release_due_date: Option<String>,
    archived: Option<bool>,
) -> CliResult<()> {
    println!("Updating version/milestone {version_id} in project: {project_id_or_key}");

    let proj_id_or_key = parse_project_id_or_key(project_id_or_key)?;
    let version_id_val = MilestoneId::new(version_id);
    let mut params = UpdateVersionParams::new(proj_id_or_key, version_id_val, name);
    params.description = description.clone();
    params.start_date = start_date
        .as_ref()
        .map(|d| parse_date_to_api_date(d))
        .transpose()?;
    params.release_due_date = release_due_date
        .as_ref()
        .map(|d| parse_date_to_api_date(d))
        .transpose()?;
    params.archived = archived;

    match client.project().update_version(params).await {
        Ok(milestone) => {
            println!("Version/milestone updated successfully:");
            println!("[{}] {}", milestone.id, milestone.name);
            if let Some(desc) = &milestone.description {
                println!("  Description: {desc}");
            }
            if let Some(start_date) = &milestone.start_date {
                println!("  Start Date: {}", start_date.format("%Y-%m-%d"));
            }
            if let Some(release_due_date) = &milestone.release_due_date {
                println!(
                    "  Release Due Date: {}",
                    release_due_date.format("%Y-%m-%d")
                );
            }
            println!("  Archived: {}", milestone.archived);
            if let Some(display_order) = milestone.display_order {
                println!("  Display Order: {display_order}");
            }
        }
        Err(e) => {
            eprintln!("Error updating version/milestone: {e}");
        }
    }
    Ok(())
}

/// Delete a version/milestone from a project
#[cfg(feature = "project_writable")]
pub async fn delete(
    client: &BacklogApiClient,
    project_id_or_key: &str,
    version_id: u32,
) -> CliResult<()> {
    println!("Deleting version/milestone {version_id} from project: {project_id_or_key}");

    let proj_id_or_key = parse_project_id_or_key(project_id_or_key)?;
    let version_id_val = MilestoneId::new(version_id);
    let params = DeleteVersionParams::new(proj_id_or_key, version_id_val);

    match client.project().delete_version(params).await {
        Ok(milestone) => {
            println!("Version/milestone deleted successfully:");
            println!("[{}] {}", milestone.id, milestone.name);
            if let Some(desc) = &milestone.description {
                println!("  Description: {desc}");
            }
            if let Some(start_date) = &milestone.start_date {
                println!("  Start Date: {}", start_date.format("%Y-%m-%d"));
            }
            if let Some(release_due_date) = &milestone.release_due_date {
                println!(
                    "  Release Due Date: {}",
                    release_due_date.format("%Y-%m-%d")
                );
            }
            println!("  Archived: {}", milestone.archived);
            if let Some(display_order) = milestone.display_order {
                println!("  Display Order: {display_order}");
            }
        }
        Err(e) => {
            eprintln!("Error deleting version/milestone: {e}");
        }
    }
    Ok(())
}
