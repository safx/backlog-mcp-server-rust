#[cfg(feature = "space")]
use crate::commands::common::CliResult;
#[cfg(feature = "space")]
use backlog_api_client::client::BacklogApiClient;
#[cfg(feature = "space")]
use backlog_core::identifier::Identifier;
#[cfg(feature = "space")]
use backlog_space::{GetLicenceParams, GetSpaceDiskUsageParams, GetSpaceLogoParams};
#[cfg(feature = "space")]
use std::path::PathBuf;
#[cfg(feature = "space")]
use tokio::fs;

#[cfg(feature = "space")]
pub(crate) async fn logo(client: &BacklogApiClient, output: PathBuf) -> CliResult<()> {
    println!("Downloading space logo to {}", output.display());

    match client
        .space()
        .get_space_logo(GetSpaceLogoParams::new())
        .await
    {
        Ok(downloaded_file) => {
            if let Err(e) = fs::write(&output, &downloaded_file.bytes).await {
                eprintln!("Error writing logo to {}: {}", output.display(), e);
            } else {
                println!(
                    "Space logo downloaded successfully to: {}",
                    output.display()
                );
            }
        }
        Err(e) => {
            eprintln!("Error downloading space logo: {e}");
        }
    }
    Ok(())
}

#[cfg(feature = "space")]
pub(crate) async fn disk_usage(client: &BacklogApiClient, format: String) -> CliResult<()> {
    fn format_bytes(bytes: i64) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
        let abs_bytes = bytes.abs();
        let mut size = abs_bytes as f64;
        let mut unit_index = 0;

        while size >= 1024.0 && unit_index < UNITS.len() - 1 {
            size /= 1024.0;
            unit_index += 1;
        }

        let formatted = if unit_index == 0 {
            format!("{} {}", size as i64, UNITS[unit_index])
        } else {
            format!("{:.2} {}", size, UNITS[unit_index])
        };

        if bytes < 0 {
            format!("-{formatted}")
        } else {
            formatted
        }
    }

    fn calculate_percentage(used: i64, capacity: i64) -> f64 {
        if capacity <= 0 {
            0.0
        } else {
            (used as f64 / capacity as f64) * 100.0
        }
    }

    match client
        .space()
        .get_space_disk_usage(GetSpaceDiskUsageParams::new())
        .await
    {
        Ok(disk_usage) => {
            if format == "json" {
                match serde_json::to_string_pretty(&disk_usage) {
                    Ok(json) => println!("{}", json),
                    Err(e) => eprintln!("Failed to serialize to JSON: {}", e),
                }
            } else {
                // Table format
                let total_used = disk_usage.issue
                    + disk_usage.wiki
                    + disk_usage.file
                    + disk_usage.subversion
                    + disk_usage.git
                    + disk_usage.git_lfs;
                let usage_percentage = calculate_percentage(total_used, disk_usage.capacity);

                println!("Space Disk Usage Summary");
                println!("========================");
                println!("Total Capacity: {}", format_bytes(disk_usage.capacity));
                println!(
                    "Used: {} ({:.1}%)",
                    format_bytes(total_used),
                    usage_percentage
                );
                println!();
                println!("By Feature:");
                println!("- Issues:     {}", format_bytes(disk_usage.issue));
                println!("- Wiki:       {}", format_bytes(disk_usage.wiki));
                println!("- Files:      {}", format_bytes(disk_usage.file));
                println!("- Subversion: {}", format_bytes(disk_usage.subversion));
                println!("- Git:        {}", format_bytes(disk_usage.git));
                println!("- Git LFS:    {}", format_bytes(disk_usage.git_lfs));

                if !disk_usage.details.is_empty() {
                    println!();
                    println!("Top Projects by Usage:");
                    let mut project_usages: Vec<_> = disk_usage
                        .details
                        .iter()
                        .map(|detail| {
                            let total = detail.issue
                                + detail.wiki
                                + detail.document
                                + detail.file
                                + detail.subversion
                                + detail.git
                                + detail.git_lfs;
                            (detail.project_id.value(), total)
                        })
                        .collect();
                    project_usages.sort_by(|a, b| b.1.cmp(&a.1));

                    for (i, (project_id, usage)) in project_usages.iter().take(10).enumerate() {
                        println!(
                            "{}. PROJECT-{}: {}",
                            i + 1,
                            project_id,
                            format_bytes(*usage)
                        );
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error getting space disk usage: {e}");
            if e.to_string().contains("403") {
                eprintln!(
                    "Note: Administrator permissions are required to access disk usage information."
                );
            }
        }
    }
    Ok(())
}

#[cfg(feature = "space")]
pub(crate) async fn licence(client: &BacklogApiClient, format: String) -> CliResult<()> {
    match client.space().get_licence(GetLicenceParams::new()).await {
        Ok(licence) => {
            if format == "json" {
                match serde_json::to_string_pretty(&licence) {
                    Ok(json) => println!("{}", json),
                    Err(e) => eprintln!("Failed to serialize to JSON: {}", e),
                }
            } else {
                // Table format
                println!("Space Licence Information");
                println!("========================");
                println!(
                    "Status: {}",
                    if licence.active { "Active" } else { "Inactive" }
                );
                println!("Licence Type ID: {}", licence.licence_type_id);
                println!();
                println!("Limits:");
                println!("- Users:         {} users", licence.user_limit);
                println!("- Projects:      {} projects", licence.project_limit);
                println!("- Issues:        {} issues", licence.issue_limit);
                println!(
                    "- Storage:       {} GB",
                    licence.storage_limit / 1_073_741_824
                );
                println!();
                println!("Features:");
                println!("- Git:           {}", if licence.git { "✓" } else { "✗" });
                println!(
                    "- Subversion:    {}",
                    if licence.subversion { "✓" } else { "✗" }
                );
                println!("- Gantt Chart:   {}", if licence.gantt { "✓" } else { "✗" });
                println!(
                    "- Burndown:      {}",
                    if licence.burndown { "✓" } else { "✗" }
                );
                println!(
                    "- Wiki:          {}",
                    if licence.wiki_attachment {
                        "✓"
                    } else {
                        "✗"
                    }
                );
                println!(
                    "- File Sharing:  {}",
                    if licence.file_sharing { "✓" } else { "✗" }
                );
                println!();
                if let Some(started_on) = licence.started_on {
                    println!("Started On:  {}", started_on.format("%Y-%m-%d"));
                }
                if let Some(limit_date) = licence.limit_date {
                    println!("Expires On:  {}", limit_date.format("%Y-%m-%d"));
                } else {
                    println!("Expires On:  Unlimited");
                }
            }
        }
        Err(e) => {
            eprintln!("Error getting licence information: {e}");
            if e.to_string().contains("401") {
                eprintln!("Note: Authentication is required to access licence information.");
            }
        }
    }
    Ok(())
}
