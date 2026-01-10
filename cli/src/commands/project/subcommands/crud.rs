//! Project CRUD (Create, Update, Delete) operations

use crate::commands::common::{parse_project_id_or_key, CliResult};
use backlog_api_client::client::BacklogApiClient;

#[cfg(feature = "project_writable")]
use backlog_project::api::{AddProjectParams, DeleteProjectParams, UpdateProjectParams};

/// Add a new project
#[cfg(feature = "project_writable")]
pub async fn add(
    client: &BacklogApiClient,
    name: &str,
    key: &str,
    chart_enabled: Option<bool>,
    use_resolved_for_chart: Option<bool>,
    subtasking_enabled: Option<bool>,
    project_leader_can_edit_project_leader: Option<bool>,
    use_wiki: Option<bool>,
    use_file_sharing: Option<bool>,
    use_wiki_tree_view: Option<bool>,
    use_subversion: Option<bool>,
    use_git: Option<bool>,
    use_original_image_size_at_wiki: Option<bool>,
    text_formatting_rule: Option<&str>,
    use_dev_attributes: Option<bool>,
) -> CliResult<()> {
    println!("Adding new project: {name} ({key})");

    let mut params = AddProjectParams::new(name, key);

    if let Some(enabled) = chart_enabled {
        params = params.chart_enabled(enabled);
    }
    if let Some(enabled) = use_resolved_for_chart {
        params = params.use_resolved_for_chart(enabled);
    }
    if let Some(enabled) = subtasking_enabled {
        params = params.subtasking_enabled(enabled);
    }
    if let Some(enabled) = project_leader_can_edit_project_leader {
        params = params.project_leader_can_edit_project_leader(enabled);
    }
    if let Some(enabled) = use_wiki {
        params = params.use_wiki(enabled);
    }
    if let Some(enabled) = use_file_sharing {
        params = params.use_file_sharing(enabled);
    }
    if let Some(enabled) = use_wiki_tree_view {
        params = params.use_wiki_tree_view(enabled);
    }
    if let Some(enabled) = use_subversion {
        params = params.use_subversion(enabled);
    }
    if let Some(enabled) = use_git {
        params = params.use_git(enabled);
    }
    if let Some(enabled) = use_original_image_size_at_wiki {
        params = params.use_original_image_size_at_wiki(enabled);
    }
    if let Some(rule) = text_formatting_rule {
        params = params.text_formatting_rule(rule.to_string());
    }
    if let Some(enabled) = use_dev_attributes {
        params = params.use_dev_attributes(enabled);
    }

    match client.project().add_project(params).await {
        Ok(project) => {
            println!("✅ Project created successfully:");
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
            println!("Use Dev Attributes: {}", project.use_dev_attributes);
        }
        Err(e) => {
            eprintln!("Error creating project: {e}");
        }
    }
    Ok(())
}

/// Update project settings
#[cfg(feature = "project_writable")]
pub async fn update(
    client: &BacklogApiClient,
    project_id_or_key: &str,
    name: Option<String>,
    key: Option<String>,
    chart_enabled: Option<bool>,
    use_resolved_for_chart: Option<bool>,
    subtasking_enabled: Option<bool>,
    project_leader_can_edit_project_leader: Option<bool>,
    use_wiki: Option<bool>,
    use_file_sharing: Option<bool>,
    use_wiki_tree_view: Option<bool>,
    use_subversion: Option<bool>,
    use_git: Option<bool>,
    use_original_image_size_at_wiki: Option<bool>,
    text_formatting_rule: Option<String>,
    archived: Option<bool>,
    use_dev_attributes: Option<bool>,
) -> CliResult<()> {
    println!("Updating project: {project_id_or_key}");

    let proj_id_or_key = parse_project_id_or_key(project_id_or_key)?;
    let mut params = UpdateProjectParams::new(proj_id_or_key);

    if let Some(name) = name {
        params.name = Some(name);
    }
    if let Some(key) = key {
        params.key = Some(key);
    }
    if let Some(chart_enabled) = chart_enabled {
        params.chart_enabled = Some(chart_enabled);
    }
    if let Some(use_resolved_for_chart) = use_resolved_for_chart {
        params.use_resolved_for_chart = Some(use_resolved_for_chart);
    }
    if let Some(subtasking_enabled) = subtasking_enabled {
        params.subtasking_enabled = Some(subtasking_enabled);
    }
    if let Some(project_leader_can_edit_project_leader) = project_leader_can_edit_project_leader {
        params.project_leader_can_edit_project_leader = Some(project_leader_can_edit_project_leader);
    }
    if let Some(use_wiki) = use_wiki {
        params.use_wiki = Some(use_wiki);
    }
    if let Some(use_file_sharing) = use_file_sharing {
        params.use_file_sharing = Some(use_file_sharing);
    }
    if let Some(use_wiki_tree_view) = use_wiki_tree_view {
        params.use_wiki_tree_view = Some(use_wiki_tree_view);
    }
    if let Some(use_subversion) = use_subversion {
        params.use_subversion = Some(use_subversion);
    }
    if let Some(use_git) = use_git {
        params.use_git = Some(use_git);
    }
    if let Some(use_original_image_size_at_wiki) = use_original_image_size_at_wiki {
        params.use_original_image_size_at_wiki = Some(use_original_image_size_at_wiki);
    }
    if let Some(text_formatting_rule) = text_formatting_rule {
        params.text_formatting_rule = Some(match text_formatting_rule.as_str() {
            "backlog" => backlog_project::api::TextFormattingRule::Backlog,
            "markdown" => backlog_project::api::TextFormattingRule::Markdown,
            _ => {
                eprintln!(
                    "Invalid text formatting rule: {text_formatting_rule}. Use 'backlog' or 'markdown'"
                );
                std::process::exit(1);
            }
        });
    }
    if let Some(archived) = archived {
        params.archived = Some(archived);
    }
    if let Some(use_dev_attributes) = use_dev_attributes {
        params.use_dev_attributes = Some(use_dev_attributes);
    }

    match client.project().update_project(params).await {
        Ok(project) => {
            println!("✅ Project updated successfully:");
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
            println!("Use Dev Attributes: {}", project.use_dev_attributes);
        }
        Err(e) => {
            eprintln!("Error updating project: {e}");
        }
    }
    Ok(())
}

/// Delete a project
#[cfg(feature = "project_writable")]
pub async fn delete(client: &BacklogApiClient, project_id_or_key: &str) -> CliResult<()> {
    println!("Deleting project: {project_id_or_key}");
    println!("⚠️  WARNING: This will permanently delete the project and all associated data!");
    println!("Are you sure you want to continue? Type 'yes' to confirm:");

    let mut confirmation = String::new();
    std::io::stdin()
        .read_line(&mut confirmation)
        .map_err(|e| format!("Failed to read confirmation: {}", e))?;

    if confirmation.trim() != "yes" {
        println!("Project deletion cancelled.");
        return Ok(());
    }

    let proj_id_or_key = parse_project_id_or_key(project_id_or_key)?;
    let params = DeleteProjectParams::new(proj_id_or_key);

    match client.project().delete_project(params).await {
        Ok(project) => {
            println!("✅ Project deleted successfully:");
            println!("Project ID: {}", project.id);
            println!("Project Key: {}", project.project_key);
            println!("Name: {}", project.name);
        }
        Err(e) => {
            eprintln!("Error deleting project: {e}");
        }
    }
    Ok(())
}
