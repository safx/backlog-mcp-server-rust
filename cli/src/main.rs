mod activity_commands;
#[cfg(any(
    feature = "project",
    feature = "issue",
    feature = "team",
    feature = "star",
    feature = "rate-limit",
    feature = "watching",
    feature = "webhook",
    feature = "user",
    feature = "wiki",
    feature = "git"
))]
mod commands;
#[cfg(feature = "project")]
use activity_commands::{ActivityArgs, ActivityCommands};
#[cfg(feature = "project")]
use commands::project::ProjectArgs;
#[cfg(feature = "rate-limit")]
use commands::rate_limit::{RateLimitCommand, handle_rate_limit_command};
#[cfg(feature = "star")]
use commands::star::{StarArgs, handle_star_command};
#[cfg(feature = "team")]
use commands::team::{TeamArgs, handle_team_command};
#[cfg(feature = "user")]
use commands::user::UserArgs;
#[cfg(feature = "watching")]
use commands::watching::handle_watching_command;
#[cfg(feature = "wiki")]
use commands::wiki::WikiArgs;

use backlog_api_client::{ProjectIdOrKey, client::BacklogApiClient};
#[cfg(feature = "project")]
use backlog_core::identifier::ActivityTypeId;
use backlog_core::identifier::Identifier;
#[cfg(feature = "project")]
use backlog_project::GetProjectRecentUpdatesParams;
use backlog_space::GetLicenceParams;
use backlog_space::GetSpaceDiskUsageParams;
use backlog_space::GetSpaceLogoParams;
#[cfg(feature = "space")]
use backlog_space::GetSpaceRecentUpdatesParams;
#[cfg(feature = "space_writable")]
use backlog_space::{UpdateSpaceNotificationParams, UploadAttachmentParams};
use clap::{Args, Parser};
use std::env;
use std::path::PathBuf;
use tokio::fs;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Parser)]
enum Commands {
    /// Manage repositories
    #[cfg(feature = "git")]
    Repo(commands::git::RepoArgs),
    /// Manage pull requests
    #[cfg(feature = "git")]
    Pr(commands::git::PrArgs),
    /// Manage issues
    #[cfg(feature = "issue")]
    Issue(commands::issue::IssueArgs),
    /// Manage space
    Space(SpaceArgs),
    /// Manage projects
    #[cfg(feature = "project")]
    Project(ProjectArgs),
    /// Manage users
    #[cfg(feature = "user")]
    User(UserArgs),
    /// Manage wikis
    #[cfg(feature = "wiki")]
    Wiki(WikiArgs),
    /// View activities
    #[cfg(feature = "project")]
    Activity(ActivityArgs),
    /// Manage teams
    #[cfg(feature = "team")]
    Team(TeamArgs),
    /// Manage stars
    #[cfg(feature = "star")]
    Star(StarArgs),
    /// View rate limit information
    #[cfg(feature = "rate-limit")]
    RateLimit(RateLimitArgs),
    /// Manage watchings
    #[cfg(feature = "watching")]
    Watching(WatchingArgs),
    /// Manage webhooks
    #[cfg(feature = "webhook")]
    Webhook(commands::webhook::WebhookArgs),
}

#[cfg(feature = "rate-limit")]
#[derive(Args)]
struct RateLimitArgs {
    #[clap(subcommand)]
    command: RateLimitCommand,
}

#[cfg(feature = "watching")]
#[derive(Args)]
struct WatchingArgs {
    #[clap(subcommand)]
    command: commands::watching::WatchingSubcommand,
}

#[derive(Parser)]
struct SpaceArgs {
    #[clap(subcommand)]
    command: SpaceCommands,
}

#[derive(Parser)]
enum SpaceCommands {
    /// Download space logo
    Logo {
        /// Output file path to save the logo
        #[clap(short, long, value_name = "FILE_PATH")]
        output: PathBuf,
    },
    /// Get space disk usage
    DiskUsage {
        /// Output format (table or json)
        #[clap(short, long, default_value = "table")]
        format: String,
    },
    /// Get licence information
    Licence {
        /// Output format (table or json)
        #[clap(short, long, default_value = "table")]
        format: String,
    },
    /// Upload an attachment file
    #[cfg(feature = "space_writable")]
    UploadAttachment {
        /// File path to upload
        #[clap(short, long, value_name = "FILE_PATH")]
        file: PathBuf,
    },
    /// Update space notification
    #[cfg(feature = "space_writable")]
    UpdateNotification {
        /// Notification content
        #[clap(short, long, value_name = "CONTENT")]
        content: String,
    },
}

/// Truncates a string to a maximum length, ensuring UTF-8 character boundary safety
fn truncate_text(text: &str, max_length: usize) -> String {
    if text.len() <= max_length {
        text.to_string()
    } else {
        let mut end = max_length;
        while !text.is_char_boundary(end) && end > 0 {
            end -= 1;
        }
        format!("{}...", &text[..end])
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_url = env::var("BACKLOG_BASE_URL")?;
    let api_key = env::var("BACKLOG_API_KEY")?;

    let client = BacklogApiClient::new(&base_url)?.with_api_key(api_key);

    let cli = Cli::parse();
    match cli.command {
        #[cfg(feature = "git")]
        Commands::Repo(repo_args) => {
            commands::git::execute_repo(&client, repo_args).await?;
        }
        #[cfg(feature = "git")]
        Commands::Pr(pr_args) => {
            commands::git::execute_pr(&client, pr_args).await?;
        }
        #[cfg(feature = "issue")]
        Commands::Issue(issue_args) => {
            commands::issue::execute(&client, issue_args).await?;
        }
        Commands::Space(space_args) => match space_args.command {
            SpaceCommands::Logo { output } => {
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
            }
            SpaceCommands::DiskUsage { format } => {
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

                            let total_used = disk_usage.issue
                                + disk_usage.wiki
                                + disk_usage.file
                                + disk_usage.subversion
                                + disk_usage.git
                                + disk_usage.git_lfs;
                            let usage_percentage =
                                calculate_percentage(total_used, disk_usage.capacity);

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

                                for (i, (project_id, usage)) in
                                    project_usages.iter().take(10).enumerate()
                                {
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
            }
            SpaceCommands::Licence { format } => {
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
                            eprintln!(
                                "Note: Authentication is required to access licence information."
                            );
                        }
                    }
                }
            }
            #[cfg(feature = "space_writable")]
            SpaceCommands::UploadAttachment { file } => {
                println!("Uploading attachment: {}", file.display());

                // Check if file exists
                if !file.exists() {
                    eprintln!("Error: File does not exist: {}", file.display());
                    std::process::exit(1);
                }

                let params = UploadAttachmentParams::new(file.clone());

                match client.space().upload_attachment(params).await {
                    Ok(attachment) => {
                        println!("✅ Attachment uploaded successfully");
                        println!("Attachment ID: {}", attachment.id);
                        println!("Filename: {}", attachment.name);
                        println!("Size: {} bytes", attachment.size);
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to upload attachment: {e}");
                        std::process::exit(1);
                    }
                }
            }
            #[cfg(feature = "space_writable")]
            SpaceCommands::UpdateNotification { content } => {
                println!("Updating space notification...");

                let params = UpdateSpaceNotificationParams::new(content.clone());

                match client.space().update_space_notification(params).await {
                    Ok(notification) => {
                        println!("✅ Space notification updated successfully");
                        println!("Content: {}", notification.content);
                        println!(
                            "Updated: {}",
                            notification.updated.format("%Y-%m-%d %H:%M:%S UTC")
                        );
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to update space notification: {e}");
                        std::process::exit(1);
                    }
                }
            }
            #[cfg(not(feature = "space_writable"))]
            _ => {
                eprintln!(
                    "This command requires write access to space and is not available. \
                    Please build with the 'space_writable' feature flag:\
\
                    cargo build --package blg --features space_writable"
                );
                std::process::exit(1);
            }
        },
        #[cfg(feature = "project")]
        Commands::Project(project_args) => {
            commands::project::execute(&client, project_args).await?;
        }
        #[cfg(feature = "user")]
        Commands::User(user_args) => {
            commands::user::execute(&client, user_args).await?;
        }
        #[cfg(feature = "wiki")]
        Commands::Wiki(wiki_args) => {
            commands::wiki::execute(&client, wiki_args).await?;
        }
        #[cfg(feature = "project")]
        Commands::Activity(activity_args) => match activity_args.command {
            ActivityCommands::Project {
                project_id,
                type_ids,
                count,
                order,
            } => {
                println!("Getting recent activities for project: {project_id}");

                let proj_id_or_key = project_id.parse::<ProjectIdOrKey>()?;
                let mut params = GetProjectRecentUpdatesParams::new(proj_id_or_key);

                // Parse activity type IDs
                if let Some(type_ids_str) = type_ids {
                    let type_ids: Result<Vec<ActivityTypeId>, _> = type_ids_str
                        .split(',')
                        .map(|s| s.trim().parse::<u32>().map(ActivityTypeId::new))
                        .collect();
                    match type_ids {
                        Ok(ids) => params.activity_type_ids = Some(ids),
                        Err(e) => {
                            eprintln!("❌ Failed to parse type_ids: {e}");
                            std::process::exit(1);
                        }
                    };
                }

                if let Some(count) = count {
                    params.count = Some(count);
                }

                if let Some(order) = order {
                    params.order = Some(order);
                }

                match client.project().get_project_recent_updates(params).await {
                    Ok(activities) => {
                        if activities.is_empty() {
                            println!("No activities found.");
                        } else {
                            println!("Found {} activities:", activities.len());
                            for activity in activities {
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
                                    backlog_core::activity::Content::UserManagement {
                                        users,
                                        ..
                                    } => {
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
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to get project activities: {e}");
                        std::process::exit(1);
                    }
                }
            }
            #[cfg(feature = "space")]
            ActivityCommands::Space {
                type_ids,
                count,
                order,
            } => {
                println!("Getting recent activities for space");

                let mut params = GetSpaceRecentUpdatesParams::default();

                // Parse activity type IDs
                if let Some(type_ids_str) = type_ids {
                    let type_ids: Result<Vec<ActivityTypeId>, _> = type_ids_str
                        .split(',')
                        .map(|s| s.trim().parse::<u32>().map(ActivityTypeId::new))
                        .collect();
                    match type_ids {
                        Ok(ids) => params.activity_type_ids = Some(ids),
                        Err(e) => {
                            eprintln!("❌ Failed to parse type_ids: {e}");
                            std::process::exit(1);
                        }
                    };
                }

                if let Some(count) = count {
                    params.count = Some(count);
                }

                if let Some(order) = order {
                    params.order = Some(order);
                }

                match client.space().get_space_recent_updates(params).await {
                    Ok(activities) => {
                        if activities.is_empty() {
                            println!("No activities found.");
                        } else {
                            println!("Found {} activities:", activities.len());
                            for activity in activities {
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
                                    backlog_core::activity::Content::UserManagement {
                                        users,
                                        ..
                                    } => {
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
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to get space activities: {e}");
                        std::process::exit(1);
                    }
                }
            }
        },
        #[cfg(feature = "team")]
        Commands::Team(team_args) => {
            handle_team_command(client.team(), team_args).await;
        }
        #[cfg(feature = "star")]
        Commands::Star(star_args) => {
            handle_star_command(&client.star(), &star_args.command).await?;
        }
        #[cfg(feature = "rate-limit")]
        Commands::RateLimit(rate_limit_args) => {
            handle_rate_limit_command(rate_limit_args.command).await?;
        }
        #[cfg(feature = "watching")]
        Commands::Watching(watching_args) => {
            handle_watching_command(commands::watching::WatchingCommand {
                command: watching_args.command,
            })
            .await?;
        }
        #[cfg(feature = "webhook")]
        Commands::Webhook(webhook_args) => {
            commands::webhook::execute(&client, webhook_args).await?;
        }
    }

    Ok(())
}
