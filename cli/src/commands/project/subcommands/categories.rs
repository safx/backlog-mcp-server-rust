//! Project category management commands

use crate::commands::common::{CliResult, parse_project_id_or_key};
use backlog_api_client::client::BacklogApiClient;
use backlog_core::identifier::CategoryId;
use backlog_project::GetCategoryListParams;

#[cfg(feature = "project_writable")]
use backlog_project::api::{AddCategoryParams, DeleteCategoryParams, UpdateCategoryParams};

/// List categories for a project
pub async fn list(client: &BacklogApiClient, project_id_or_key: &str) -> CliResult<()> {
    println!("Listing categories for project: {project_id_or_key}");

    let proj_id_or_key = parse_project_id_or_key(project_id_or_key)?;
    let params = GetCategoryListParams::new(proj_id_or_key);
    match client.project().get_category_list(params).await {
        Ok(categories) => {
            if categories.is_empty() {
                println!("No categories found");
            } else {
                for category in categories {
                    println!(
                        "[{}] {} (Display Order: {})",
                        category.id, category.name, category.display_order
                    );
                }
            }
        }
        Err(e) => {
            eprintln!("Error listing categories: {e}");
        }
    }
    Ok(())
}

/// Add a category to a project
#[cfg(feature = "project_writable")]
pub async fn add(client: &BacklogApiClient, project_id_or_key: &str, name: &str) -> CliResult<()> {
    println!("Adding category '{name}' to project: {project_id_or_key}");

    let proj_id_or_key = parse_project_id_or_key(project_id_or_key)?;
    let params = AddCategoryParams::new(proj_id_or_key, name.to_string());

    match client.project().add_category(params).await {
        Ok(category) => {
            println!("Category added successfully:");
            println!(
                "[{}] {} (Display Order: {})",
                category.id, category.name, category.display_order
            );
        }
        Err(e) => {
            eprintln!("Error adding category: {e}");
        }
    }
    Ok(())
}

/// Update a category in a project
#[cfg(feature = "project_writable")]
pub async fn update(
    client: &BacklogApiClient,
    project_id_or_key: &str,
    category_id: u32,
    name: &str,
) -> CliResult<()> {
    println!("Updating category {category_id} in project {project_id_or_key} to name '{name}'");

    let proj_id_or_key = parse_project_id_or_key(project_id_or_key)?;
    let cat_id = CategoryId::new(category_id);
    let params = UpdateCategoryParams::new(proj_id_or_key, cat_id, name.to_string());

    match client.project().update_category(params).await {
        Ok(category) => {
            println!("Category updated successfully:");
            println!(
                "[{}] {} (Display Order: {})",
                category.id, category.name, category.display_order
            );
        }
        Err(e) => {
            eprintln!("Error updating category: {e}");
        }
    }
    Ok(())
}

/// Delete a category from a project
#[cfg(feature = "project_writable")]
pub async fn delete(
    client: &BacklogApiClient,
    project_id_or_key: &str,
    category_id: u32,
) -> CliResult<()> {
    println!("Deleting category {category_id} from project: {project_id_or_key}");

    let proj_id_or_key = parse_project_id_or_key(project_id_or_key)?;
    let cat_id = CategoryId::new(category_id);

    match client
        .project()
        .delete_category(DeleteCategoryParams::new(proj_id_or_key, cat_id))
        .await
    {
        Ok(category) => {
            println!("Category deleted successfully:");
            println!(
                "[{}] {} (Display Order: {})",
                category.id, category.name, category.display_order
            );
        }
        Err(e) => {
            eprintln!("Error deleting category: {e}");
        }
    }
    Ok(())
}
