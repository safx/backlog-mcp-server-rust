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
    feature = "git",
    feature = "space"
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
#[cfg(feature = "space")]
use backlog_space::GetSpaceRecentUpdatesParams;
use clap::{Args, Parser};
use std::env;

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
    #[cfg(feature = "space")]
    Space(commands::space::SpaceArgs),
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
        #[cfg(feature = "space")]
        Commands::Space(space_args) => {
            commands::space::execute(&client, space_args).await?;
        }
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
