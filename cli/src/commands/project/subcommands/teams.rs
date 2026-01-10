//! Project team management commands

use crate::commands::common::{parse_project_id_or_key, CliResult};
use backlog_api_client::client::BacklogApiClient;
use backlog_core::identifier::TeamId;
use backlog_project::GetProjectTeamListParams;

#[cfg(feature = "project_writable")]
use backlog_project::api::{AddProjectTeamParams, DeleteProjectTeamParams};

/// List teams for a project
pub async fn list(client: &BacklogApiClient, project_id_or_key: &str) -> CliResult<()> {
    println!("Listing teams for project: {project_id_or_key}");

    let proj_id_or_key = parse_project_id_or_key(project_id_or_key)?;
    let params = GetProjectTeamListParams {
        project_id_or_key: proj_id_or_key,
    };
    match client.project().get_project_team_list(params).await {
        Ok(teams) => {
            if teams.is_empty() {
                println!("No teams found in this project");
            } else {
                println!("Teams in this project:");
                for team in teams {
                    println!("[{}] {}", team.id, team.name);
                    println!("  Members: {} users", team.members.len());
                    println!(
                        "  Created: {} by {}",
                        team.created.format("%Y-%m-%d %H:%M"),
                        team.created_user.name
                    );
                    println!(
                        "  Updated: {} by {}",
                        team.updated.format("%Y-%m-%d %H:%M"),
                        team.updated_user.name
                    );
                    println!();
                }
            }
        }
        Err(e) => {
            eprintln!("Error listing project teams: {e}");
        }
    }
    Ok(())
}

/// Add a team to a project
#[cfg(feature = "project_writable")]
pub async fn add(
    client: &BacklogApiClient,
    project_id_or_key: &str,
    team_id: u32,
) -> CliResult<()> {
    println!("Adding team {team_id} to project: {project_id_or_key}");

    let proj_id_or_key = parse_project_id_or_key(project_id_or_key)?;
    let params = AddProjectTeamParams {
        project_id_or_key: proj_id_or_key,
        team_id: TeamId::new(team_id),
    };

    match client.project().add_project_team(params).await {
        Ok(team) => {
            println!("✅ Team added successfully:");
            println!("ID: {}", team.id);
            println!("Name: {}", team.name);
            println!("Members: {} users", team.members.len());
        }
        Err(e) => {
            eprintln!("❌ Failed to add team to project: {e}");
            std::process::exit(1);
        }
    }
    Ok(())
}

/// Remove a team from a project
#[cfg(feature = "project_writable")]
pub async fn delete(
    client: &BacklogApiClient,
    project_id_or_key: &str,
    team_id: u32,
) -> CliResult<()> {
    println!("Removing team {team_id} from project: {project_id_or_key}");

    let proj_id_or_key = parse_project_id_or_key(project_id_or_key)?;
    let params = DeleteProjectTeamParams {
        project_id_or_key: proj_id_or_key,
        team_id: TeamId::new(team_id),
    };

    match client.project().delete_project_team(params).await {
        Ok(team) => {
            println!("✅ Team removed successfully:");
            println!("ID: {}", team.id);
            println!("Name: {}", team.name);
        }
        Err(e) => {
            eprintln!("❌ Failed to remove team from project: {e}");
            std::process::exit(1);
        }
    }
    Ok(())
}
