//! Project status management commands

use crate::commands::common::CliResult;
use backlog_api_client::client::BacklogApiClient;
use backlog_core::{ProjectIdOrKey, identifier::StatusId};
use backlog_project::GetStatusListParams;

#[cfg(feature = "project_writable")]
use backlog_domain_models::StatusColor;
#[cfg(feature = "project_writable")]
use backlog_project::api::{
    AddStatusParams, DeleteStatusParams, UpdateStatusOrderParams, UpdateStatusParams,
};
#[cfg(feature = "project_writable")]
use std::str::FromStr;

/// List statuses for a project
pub async fn list(client: &BacklogApiClient, project_id_or_key: &str) -> CliResult<()> {
    println!("Listing statuses for project: {project_id_or_key}");

    let proj_id_or_key = project_id_or_key
        .parse::<ProjectIdOrKey>()
        .map_err(|e| format!("Invalid project: {e}"))?;
    let params = GetStatusListParams::new(proj_id_or_key);
    match client.project().get_status_list(params).await {
        Ok(statuses) => {
            if statuses.is_empty() {
                println!("No statuses found");
            } else {
                for status in statuses {
                    println!("[{}] {} (Color: {})", status.id, status.name, status.color);
                }
            }
        }
        Err(e) => {
            eprintln!("Error listing statuses: {e}");
        }
    }
    Ok(())
}

/// Add a status to a project
#[cfg(feature = "project_writable")]
pub async fn add(
    client: &BacklogApiClient,
    project_id_or_key: &str,
    name: &str,
    color: &str,
) -> CliResult<()> {
    println!("Adding status '{name}' to project: {project_id_or_key}");

    let proj_id_or_key = project_id_or_key
        .parse::<ProjectIdOrKey>()
        .map_err(|e| format!("Invalid project: {e}"))?;
    let parsed_color = StatusColor::from_str(color)?;

    let params = AddStatusParams::new(proj_id_or_key, name, parsed_color);

    let status = client.project().add_status(params).await?;
    println!("✅ Status added successfully:");
    println!("ID: {}", status.id);
    println!("Name: {}", status.name);
    println!("Color: {}", status.color);
    println!("Display Order: {}", status.display_order);
    Ok(())
}

/// Update a status in a project
#[cfg(feature = "project_writable")]
pub async fn update(
    client: &BacklogApiClient,
    project_id_or_key: &str,
    status_id: u32,
    name: Option<String>,
    color: Option<String>,
) -> CliResult<()> {
    println!("Updating status {status_id} in project: {project_id_or_key}");

    let proj_id_or_key = project_id_or_key
        .parse::<ProjectIdOrKey>()
        .map_err(|e| format!("Invalid project: {e}"))?;
    let status_id_val = StatusId::new(status_id);

    let parsed_color = if let Some(color_str) = &color {
        Some(StatusColor::from_str(color_str)?)
    } else {
        None
    };

    let mut params = UpdateStatusParams::new(proj_id_or_key, status_id_val);

    if let Some(name) = name {
        params = params.name(name);
    }

    if let Some(color) = parsed_color {
        params = params.color(color);
    }

    let status = client.project().update_status(params).await?;
    println!("✅ Status updated successfully:");
    println!("ID: {}", status.id);
    println!("Name: {}", status.name);
    println!("Color: {}", status.color);
    println!("Display Order: {}", status.display_order);
    Ok(())
}

/// Delete a status from a project
#[cfg(feature = "project_writable")]
pub async fn delete(
    client: &BacklogApiClient,
    project_id_or_key: &str,
    status_id: u32,
    substitute_status_id: u32,
) -> CliResult<()> {
    println!(
        "Deleting status {status_id} from project: {project_id_or_key} (substitute: {substitute_status_id})"
    );

    let proj_id_or_key = project_id_or_key
        .parse::<ProjectIdOrKey>()
        .map_err(|e| format!("Invalid project: {e}"))?;
    let status_id_val = StatusId::new(status_id);
    let substitute_id = StatusId::new(substitute_status_id);

    let params = DeleteStatusParams::new(proj_id_or_key, status_id_val, substitute_id);

    let status = client.project().delete_status(params).await?;
    println!("✅ Status deleted successfully:");
    println!("ID: {}", status.id);
    println!("Name: {}", status.name);
    println!("Color: {}", status.color);
    println!("Display Order: {}", status.display_order);
    Ok(())
}

/// Update the display order of statuses in a project
#[cfg(feature = "project_writable")]
pub async fn update_order(
    client: &BacklogApiClient,
    project_id_or_key: &str,
    status_ids: &str,
) -> CliResult<()> {
    println!("Updating status order in project: {project_id_or_key} with IDs: {status_ids}");

    let proj_id_or_key = project_id_or_key
        .parse::<ProjectIdOrKey>()
        .map_err(|e| format!("Invalid project: {e}"))?;

    // Parse comma-separated status IDs
    let status_id_vec: Vec<StatusId> = status_ids
        .split(',')
        .map(|s| s.trim().parse::<u32>().map(StatusId::new))
        .collect::<Result<_, _>>()?;

    let params = UpdateStatusOrderParams::new(proj_id_or_key, status_id_vec);

    let statuses = client.project().update_status_order(params).await?;
    println!("✅ Status order updated successfully:");
    for (index, status) in statuses.iter().enumerate() {
        println!(
            "{}. [{}] {} (Color: {})",
            index + 1,
            status.id,
            status.name,
            status.color
        );
    }
    Ok(())
}
