//! Project list and detail viewing commands

use crate::commands::common::CliResult;
use backlog_api_client::client::BacklogApiClient;
use backlog_core::ProjectIdOrKey;
use backlog_project::{
    GetProjectDetailParams, GetProjectListParams, GetRecentlyViewedProjectsParamsBuilder,
};

/// List all projects
pub async fn list(client: &BacklogApiClient) -> CliResult<()> {
    println!("Listing all projects");

    let params = GetProjectListParams {
        archived: None,
        all: true,
    };

    match client.project().get_project_list(params).await {
        Ok(projects) => {
            if projects.is_empty() {
                println!("No projects found");
            } else {
                for project in projects {
                    println!(
                        "[{}] {} (Key: {})",
                        project.id, project.name, project.project_key
                    );
                    println!("  Chart Enabled: {}", project.chart_enabled);
                    println!("  Subtasking Enabled: {}", project.subtasking_enabled);
                    println!("  Archived: {}", project.archived);
                    println!();
                }
            }
        }
        Err(e) => {
            eprintln!("Error listing projects: {e}");
        }
    }
    Ok(())
}

/// Show details of a specific project
pub async fn show(client: &BacklogApiClient, project_id_or_key: &str) -> CliResult<()> {
    println!("Showing project: {project_id_or_key}");

    let proj_id_or_key = project_id_or_key
        .parse::<ProjectIdOrKey>()
        .map_err(|e| format!("Invalid project: {e}"))?;
    let params = GetProjectDetailParams::new(proj_id_or_key);
    match client.project().get_project(params).await {
        Ok(project) => {
            println!("Project ID: {}", project.id);
            println!("Project Key: {}", project.project_key);
            println!("Name: {}", project.name);
            println!("Chart Enabled: {}", project.chart_enabled);
            println!("Subtasking Enabled: {}", project.subtasking_enabled);
            println!(
                "Project Leader Can Edit Project Leader: {}",
                project.project_leader_can_edit_project_leader
            );
            println!("Use Wiki: {}", project.use_wiki);
            println!("Use File Sharing: {}", project.use_file_sharing);
            println!("Use Wiki Tree View: {}", project.use_wiki_tree_view);
            println!(
                "Use Original Image Size at Wiki: {}",
                project.use_original_image_size_at_wiki
            );
            println!("Text Formatting Rule: {:?}", project.text_formatting_rule);
            println!("Archived: {}", project.archived);
            println!("Display Order: {}", project.display_order);
            println!("Use Dev Attributes: {}", project.use_dev_attributes);
        }
        Err(e) => {
            eprintln!("Error getting project: {e}");
        }
    }
    Ok(())
}

/// List recently viewed projects
pub async fn recently_viewed(
    client: &BacklogApiClient,
    order: Option<String>,
    count: Option<u32>,
    offset: Option<u32>,
) -> CliResult<()> {
    println!("Getting recently viewed projects");

    let mut params_builder = GetRecentlyViewedProjectsParamsBuilder::default();

    if let Some(order) = order {
        params_builder.order(order);
    }
    if let Some(count) = count {
        params_builder.count(count);
    }
    if let Some(offset) = offset {
        params_builder.offset(offset);
    }

    let params = params_builder.build()?;
    match client.project().get_recently_viewed_projects(params).await {
        Ok(projects) => {
            if projects.is_empty() {
                println!("No recently viewed projects found");
            } else {
                println!("\nRecently Viewed Projects:");
                println!("{}", "=".repeat(50));
                for (i, project) in projects.iter().enumerate() {
                    println!(
                        "\n{}. [{}] {} ({})",
                        i + 1,
                        project.id,
                        project.name,
                        project.project_key
                    );
                    println!("   Archived: {}", project.archived);
                    if project.use_wiki {
                        println!("   Features: Wiki enabled");
                    }
                    if project.use_file_sharing {
                        println!("   Features: File sharing enabled");
                    }
                }
                println!();
            }
        }
        Err(e) => {
            eprintln!("Error getting recently viewed projects: {e}");
        }
    }
    Ok(())
}
