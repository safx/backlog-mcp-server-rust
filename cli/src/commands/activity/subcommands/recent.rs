#[cfg(feature = "project")]
use crate::commands::common::{CliResult, truncate_text};
#[cfg(feature = "project")]
use backlog_api_client::ProjectIdOrKey;
use backlog_api_client::client::BacklogApiClient;
use backlog_core::activity::Activity;
#[cfg(any(feature = "project", feature = "space"))]
use backlog_core::identifier::{ActivityTypeId, Identifier};
#[cfg(feature = "project")]
use backlog_project::GetProjectRecentUpdatesParams;
#[cfg(feature = "space")]
use backlog_space::GetSpaceRecentUpdatesParams;

/// Helper function to print a single activity
fn print_activity(activity: &Activity) {
    println!("---");
    println!("ID: {}", activity.id.value());
    println!("Type: {}", activity.type_id);
    // Use helper method to access project name
    let project_name = activity.project_name().unwrap_or("Unknown");
    println!("Project: {project_name}");
    println!("Created by: {}", activity.created_user.name);
    println!(
        "Created at: {}",
        activity.created.format("%Y-%m-%d %H:%M:%S")
    );

    // Display content based on type
    match &activity.content {
        backlog_core::activity::Content::Standard {
            summary,
            description,
            ..
        } => {
            if let Some(summary) = summary {
                println!("Summary: {summary}");
            }
            if let Some(description) = description {
                let preview = truncate_text(description, 100);
                println!("Description: {preview}");
            }
        }
        backlog_core::activity::Content::UserManagement { users, .. } => {
            if let Some(users) = users {
                println!("Users involved: {}", users.len());
                for user in users.iter().take(3) {
                    println!("  - {}", user.name);
                }
                if users.len() > 3 {
                    println!("  ... and {} more", users.len() - 3);
                }
            }
        }
        _ => {
            // Other content types not yet implemented in CLI
            println!("Activity type: {:?}", activity.type_id);
        }
    }
}

/// Helper function to print a list of activities
fn print_activities(activities: &[Activity]) {
    if activities.is_empty() {
        println!("No activities found.");
    } else {
        println!("Found {} activities:", activities.len());
        for activity in activities {
            print_activity(activity);
        }
    }
}

/// Helper function to parse comma-separated activity type IDs
#[cfg(any(feature = "project", feature = "space"))]
fn parse_type_ids(type_ids_str: &str) -> anyhow::Result<Vec<ActivityTypeId>> {
    use anyhow::Context;

    type_ids_str
        .split(',')
        .map(|s| {
            s.trim()
                .parse::<u32>()
                .map(ActivityTypeId::new)
                .with_context(|| format!("Failed to parse type_id '{}'", s.trim()))
        })
        .collect()
}

/// Get recent activities in a project
#[cfg(feature = "project")]
pub(crate) async fn project_recent(
    client: &BacklogApiClient,
    project_id: String,
    type_ids: Option<String>,
    count: Option<u32>,
    order: Option<String>,
) -> CliResult<()> {
    println!("Getting recent activities for project: {project_id}");

    let proj_id_or_key = project_id.parse::<ProjectIdOrKey>()?;
    let mut params = GetProjectRecentUpdatesParams::new(proj_id_or_key);

    // Parse activity type IDs
    if let Some(type_ids_str) = type_ids {
        params.activity_type_ids = Some(parse_type_ids(&type_ids_str)?);
    }

    if let Some(count) = count {
        params.count = Some(count);
    }

    if let Some(order) = order {
        params.order = Some(order);
    }

    let activities = client.project().get_project_recent_updates(params).await?;
    print_activities(&activities);
    Ok(())
}

/// Get recent activities in the space
#[cfg(feature = "space")]
pub(crate) async fn space_recent(
    client: &BacklogApiClient,
    type_ids: Option<String>,
    count: Option<u32>,
    order: Option<String>,
) -> CliResult<()> {
    println!("Getting recent activities for space");

    let mut params = GetSpaceRecentUpdatesParams::default();

    // Parse activity type IDs
    if let Some(type_ids_str) = type_ids {
        params.activity_type_ids = Some(parse_type_ids(&type_ids_str)?);
    }

    if let Some(count) = count {
        params.count = Some(count);
    }

    if let Some(order) = order {
        params.order = Some(order);
    }

    let activities = client.space().get_space_recent_updates(params).await?;
    print_activities(&activities);
    Ok(())
}
