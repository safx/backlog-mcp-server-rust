//! Miscellaneous project commands (priorities, resolutions, icon, disk usage)

use crate::commands::common::{CliResult, format_bytes};
use anyhow::Context;
use backlog_api_client::client::BacklogApiClient;
use backlog_core::ProjectIdOrKey;
use backlog_project::api::{GetProjectDiskUsageParams, GetProjectIconParams};
use std::path::Path;
use tokio::fs;

/// List priorities (space-wide)
pub async fn priority_list(client: &BacklogApiClient) -> CliResult<()> {
    println!("Listing priorities (space-wide):");

    match client.project().get_priority_list().await {
        Ok(priorities) => {
            if priorities.is_empty() {
                println!("No priorities found");
            } else {
                for priority in priorities {
                    println!("[{}] {}", priority.id, priority.name);
                }
            }
        }
        Err(e) => {
            eprintln!("Error listing priorities: {e}");
        }
    }
    Ok(())
}

/// List resolutions (space-wide)
pub async fn resolution_list(client: &BacklogApiClient) -> CliResult<()> {
    println!("Listing resolutions (space-wide):");

    match client.project().get_resolution_list().await {
        Ok(resolutions) => {
            if resolutions.is_empty() {
                println!("No resolutions found");
            } else {
                for resolution in resolutions {
                    println!("[{}] {}", resolution.id, resolution.name);
                }
            }
        }
        Err(e) => {
            eprintln!("Error listing resolutions: {e}");
        }
    }
    Ok(())
}

/// Download project icon
pub async fn download_icon(
    client: &BacklogApiClient,
    project_id_or_key: &str,
    output: &Path,
) -> CliResult<()> {
    println!("Downloading project icon to {}", output.display());

    let proj_id_or_key = project_id_or_key
        .parse::<ProjectIdOrKey>()
        .with_context(|| "Invalid project")?;
    let params = GetProjectIconParams::new(proj_id_or_key);
    match client.project().get_project_icon(params).await {
        Ok(icon_bytes) => {
            if let Err(e) = fs::write(output, &icon_bytes).await {
                eprintln!("Error writing icon to {}: {}", output.display(), e);
            } else {
                println!(
                    "Project icon downloaded successfully to: {}",
                    output.display()
                );
            }
        }
        Err(e) => {
            eprintln!("Error downloading project icon: {e}");
        }
    }
    Ok(())
}

/// Get disk usage for a project
pub async fn disk_usage(
    client: &BacklogApiClient,
    project_id_or_key: &str,
    human_readable: bool,
) -> CliResult<()> {
    println!("Getting disk usage for project: {project_id_or_key}");

    let proj_id_or_key = project_id_or_key
        .parse::<ProjectIdOrKey>()
        .with_context(|| "Invalid project")?;
    let params = GetProjectDiskUsageParams::new(proj_id_or_key);
    match client.project().get_disk_usage(params).await {
        Ok(disk_usage) => {
            let total = disk_usage.issue
                + disk_usage.wiki
                + disk_usage.document
                + disk_usage.file
                + disk_usage.subversion
                + disk_usage.git
                + disk_usage.git_lfs;

            println!("\nProject Disk Usage (ID: {})", disk_usage.project_id);
            println!("┌─────────────┬──────────────┬────────────┐");
            println!("│ Component   │ Size         │ Percentage │");
            println!("├─────────────┼──────────────┼────────────┤");

            let components = [
                ("Issues", disk_usage.issue),
                ("Wiki", disk_usage.wiki),
                ("Documents", disk_usage.document),
                ("Files", disk_usage.file),
                ("Subversion", disk_usage.subversion),
                ("Git", disk_usage.git),
                ("Git LFS", disk_usage.git_lfs),
            ];

            for (name, size) in components {
                let size_str = if human_readable {
                    format_bytes(size as u64)
                } else {
                    format!("{size} bytes")
                };
                let percentage = if total > 0 {
                    (size as f64 / total as f64) * 100.0
                } else {
                    0.0
                };
                println!("│ {name:<11} │ {size_str:<12} │ {percentage:>9.1}% │");
            }

            println!("├─────────────┼──────────────┼────────────┤");
            let total_str = if human_readable {
                format_bytes(total as u64)
            } else {
                format!("{total} bytes")
            };
            println!("│ Total       │ {total_str:<12} │     100.0% │");
            println!("└─────────────┴──────────────┴────────────┘");
        }
        Err(e) => {
            eprintln!("Error getting disk usage: {e}");
        }
    }
    Ok(())
}
