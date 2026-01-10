//! Project user and administrator management commands

use crate::commands::common::{parse_project_id_or_key, CliResult};
use backlog_api_client::client::BacklogApiClient;
use backlog_project::{GetProjectUserListParams, api::GetProjectAdministratorListParams};

#[cfg(feature = "project_writable")]
use backlog_project::api::{
    AddProjectAdministratorParams, AddProjectUserParams, DeleteProjectAdministratorParams,
    DeleteProjectUserParams,
};

/// List users for a project
pub async fn list(client: &BacklogApiClient, project_id_or_key: &str) -> CliResult<()> {
    println!("Listing users for project: {project_id_or_key}");

    let proj_id_or_key = parse_project_id_or_key(project_id_or_key)?;
    let params = GetProjectUserListParams::new(proj_id_or_key);
    match client.project().get_project_user_list(params).await {
        Ok(users) => {
            if users.is_empty() {
                println!("No users found in this project");
            } else {
                for user in users {
                    let role_str = match user.role_type {
                        backlog_core::Role::Admin => "Admin",
                        backlog_core::Role::User => "User",
                        backlog_core::Role::Reporter => "Reporter",
                        backlog_core::Role::Viewer => "Viewer",
                        backlog_core::Role::Guest => "Guest",
                    };
                    let last_login = match user.last_login_time {
                        Some(time) => time.format("%Y-%m-%d %H:%M:%S").to_string(),
                        None => "Never".to_string(),
                    };
                    println!(
                        "[{}] {} ({}) - {} - Last login: {}",
                        user.id, user.name, user.mail_address, role_str, last_login
                    );
                }
            }
        }
        Err(e) => {
            eprintln!("Error listing project users: {e}");
        }
    }
    Ok(())
}

/// List administrators for a project
pub async fn admin_list(client: &BacklogApiClient, project_id_or_key: &str) -> CliResult<()> {
    println!("Listing administrators for project: {project_id_or_key}");

    let proj_id_or_key = parse_project_id_or_key(project_id_or_key)?;
    let params = GetProjectAdministratorListParams::new(proj_id_or_key);
    match client
        .project()
        .get_project_administrator_list(params)
        .await
    {
        Ok(admins) => {
            if admins.is_empty() {
                println!("No administrators found in this project");
            } else {
                println!("\nProject Administrators:");
                println!("{:-<80}", "");
                for admin in admins {
                    let last_login = match admin.last_login_time {
                        Some(time) => time.format("%Y-%m-%d %H:%M:%S").to_string(),
                        None => "Never".to_string(),
                    };
                    println!(
                        "[{}] {} ({}) - Last login: {}",
                        admin.id, admin.name, admin.mail_address, last_login
                    );
                }
            }
        }
        Err(e) => {
            eprintln!("Error listing project administrators: {e}");
        }
    }
    Ok(())
}

/// Add a user as a project administrator
#[cfg(feature = "project_writable")]
pub async fn admin_add(
    client: &BacklogApiClient,
    project_id_or_key: &str,
    user_id: u32,
) -> CliResult<()> {
    println!("Adding user {user_id} as administrator to project: {project_id_or_key}");

    let proj_id_or_key = parse_project_id_or_key(project_id_or_key)?;
    let params = AddProjectAdministratorParams::new(proj_id_or_key, user_id);
    match client.project().add_project_administrator(params).await {
        Ok(user) => {
            println!("✅ Successfully added administrator:");
            println!("  User ID: {}", user.id);
            println!("  Name: {}", user.name);
            println!("  Email: {}", user.mail_address);
            let role_str = match user.role_type {
                backlog_core::Role::Admin => "Administrator",
                backlog_core::Role::User => "User",
                backlog_core::Role::Reporter => "Reporter",
                backlog_core::Role::Viewer => "Viewer",
                backlog_core::Role::Guest => "Guest",
            };
            println!("  Role: {role_str}");
        }
        Err(e) => {
            eprintln!("❌ Error adding project administrator: {e}");
            std::process::exit(1);
        }
    }
    Ok(())
}

/// Remove an administrator from a project
#[cfg(feature = "project_writable")]
pub async fn admin_remove(
    client: &BacklogApiClient,
    project_id_or_key: &str,
    user_id: u32,
) -> CliResult<()> {
    println!("Removing administrator {user_id} from project: {project_id_or_key}");

    let proj_id_or_key = parse_project_id_or_key(project_id_or_key)?;
    let params = DeleteProjectAdministratorParams::new(proj_id_or_key, user_id);
    match client.project().delete_project_administrator(params).await {
        Ok(user) => {
            println!("Successfully removed administrator:");
            println!("  User ID: {}", user.id);
            println!("  Name: {}", user.name);
            println!("  Email: {}", user.mail_address);
        }
        Err(e) => {
            eprintln!("Error removing project administrator: {e}");
            std::process::exit(1);
        }
    }
    Ok(())
}

/// Add a user to a project
#[cfg(feature = "project_writable")]
pub async fn user_add(
    client: &BacklogApiClient,
    project_id_or_key: &str,
    user_id: u32,
) -> CliResult<()> {
    println!("Adding user {user_id} to project: {project_id_or_key}");

    let proj_id_or_key = parse_project_id_or_key(project_id_or_key)?;
    let params = AddProjectUserParams::new(proj_id_or_key, user_id);
    match client.project().add_project_user(params).await {
        Ok(user) => {
            println!(
                "Successfully added user: {} ({})",
                user.name, user.mail_address
            );
        }
        Err(e) => {
            eprintln!("Error adding user: {e}");
        }
    }
    Ok(())
}

/// Remove a user from a project
#[cfg(feature = "project_writable")]
pub async fn user_remove(
    client: &BacklogApiClient,
    project_id_or_key: &str,
    user_id: u32,
) -> CliResult<()> {
    println!("Removing user {user_id} from project: {project_id_or_key}");

    let proj_id_or_key = parse_project_id_or_key(project_id_or_key)?;
    let params = DeleteProjectUserParams::new(proj_id_or_key, user_id);
    match client.project().delete_project_user(params).await {
        Ok(user) => {
            println!(
                "Successfully removed user: {} ({})",
                user.name, user.mail_address
            );
        }
        Err(e) => {
            eprintln!("Error removing user: {e}");
        }
    }
    Ok(())
}
