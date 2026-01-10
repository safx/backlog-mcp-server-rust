mod activity_commands;
#[cfg(any(
    feature = "project",
    feature = "issue",
    feature = "team",
    feature = "star",
    feature = "rate-limit",
    feature = "watching",
    feature = "webhook",
    feature = "wiki"
))]
mod commands;
#[cfg(feature = "project")]
use activity_commands::{ActivityArgs, ActivityCommands};
#[cfg(feature = "project_writable")]
use commands::common::{date_to_end_of_day, date_to_start_of_day};
#[cfg(feature = "project")]
use commands::project::ProjectArgs;
#[cfg(feature = "rate-limit")]
use commands::rate_limit::{RateLimitCommand, handle_rate_limit_command};
#[cfg(feature = "star")]
use commands::star::{StarArgs, handle_star_command};
#[cfg(feature = "team")]
use commands::team::{TeamArgs, handle_team_command};
#[cfg(feature = "watching")]
use commands::watching::handle_watching_command;
#[cfg(feature = "wiki")]
use commands::wiki::WikiArgs;

#[cfg(feature = "git_writable")]
use backlog_api_client::AddPullRequestParams;
#[cfg(feature = "git_writable")]
#[allow(unused_imports)]
use backlog_api_client::UpdatePullRequestCommentParams;
#[cfg(feature = "git_writable")]
#[allow(unused_imports)]
use backlog_api_client::UpdatePullRequestParams;
use backlog_api_client::{
    GetPullRequestCountParams, ProjectIdOrKey, PullRequestAttachmentId,
    PullRequestCommentId, PullRequestNumber, RepositoryIdOrName, UserId, client::BacklogApiClient,
};
use backlog_core::ApiDate;
#[cfg(feature = "project")]
use backlog_core::identifier::ActivityTypeId;
use backlog_core::identifier::{AttachmentId, Identifier, IssueId, StatusId};
#[cfg(feature = "project")]
use backlog_project::GetProjectRecentUpdatesParams;
use backlog_space::GetLicenceParams;
use backlog_space::GetSpaceDiskUsageParams;
use backlog_space::GetSpaceLogoParams;
#[cfg(feature = "space")]
use backlog_space::GetSpaceRecentUpdatesParams;
#[cfg(feature = "space_writable")]
use backlog_space::{UpdateSpaceNotificationParams, UploadAttachmentParams};
#[cfg(feature = "user")]
use backlog_user::{
    GetNotificationCountParams, GetNotificationsParams, GetOwnUserParams, GetUserIconParams,
    GetUserListParams, GetUserParams, GetUserStarCountParams, GetUserStarsParams,
    GetWatchingCountParams, GetWatchingListParams, NotificationOrder,
    api::{Order as WatchingOrder, StarOrder, WatchingSort},
};
use chrono::NaiveDate;
use clap::{Args, Parser};
use std::env;
use std::path::PathBuf;
use std::str::FromStr;
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
    Repo(RepoArgs),
    /// Manage pull requests
    Pr(PrArgs),
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
struct RepoArgs {
    #[clap(subcommand)]
    command: RepoCommands,
}

#[derive(Parser)]
enum RepoCommands {
    /// List repositories in a project
    List {
        /// Project ID or Key
        #[clap(short, long)]
        project_id: String,
    },
    /// Show details of a specific repository
    Show {
        /// Project ID or Key
        #[clap(short, long)]
        project_id: String,
        /// Repository ID or Name
        #[clap(short, long)]
        repo_id: String,
    },
}

#[derive(Parser)]
struct PrArgs {
    #[clap(subcommand)]
    command: PrCommands,
}

#[derive(Parser)]
enum PrCommands {
    /// List pull requests in a repository
    List {
        /// Project ID or Key
        #[clap(short, long)]
        project_id: String,
        /// Repository ID or Name
        #[clap(short, long)]
        repo_id: String,
    },
    /// Show details of a specific pull request
    Show {
        /// Project ID or Key
        #[clap(short, long)]
        project_id: String,
        /// Repository ID or Name
        #[clap(short, long)]
        repo_id: String,
        /// Pull Request number
        #[clap(short = 'n', long)]
        pr_number: u64,
    },
    /// Download a pull request attachment
    #[command(about = "Download a pull request attachment")]
    DownloadAttachment(DownloadPrAttachmentArgs),
    /// Delete a pull request attachment
    #[cfg(feature = "git_writable")]
    #[command(about = "Delete a pull request attachment")]
    DeleteAttachment(DeletePrAttachmentArgs),
    /// Update a pull request
    #[cfg(feature = "git_writable")]
    Update {
        /// Project ID or Key
        #[clap(short, long)]
        project_id: String,
        /// Repository ID or Name
        #[clap(short, long)]
        repo_id: String,
        /// Pull Request number
        #[clap(long)]
        pr_number: u64,
        /// Update summary (title)
        #[clap(long)]
        summary: Option<String>,
        /// Update description
        #[clap(long)]
        description: Option<String>,
        /// Related issue ID
        #[clap(long)]
        issue_id: Option<u32>,
        /// Assignee user ID
        #[clap(long)]
        assignee_id: Option<u32>,
        /// Notification user IDs (comma-separated)
        #[clap(long, value_delimiter = ',')]
        notify_user_ids: Option<Vec<u32>>,
        /// Comment to add with the update
        #[clap(long)]
        comment: Option<String>,
    },
    /// Update a pull request comment
    #[cfg(feature = "git_writable")]
    CommentUpdate {
        /// Project ID or Key
        #[clap(short, long)]
        project_id: String,
        /// Repository ID or Name
        #[clap(short, long)]
        repo_id: String,
        /// Pull Request number
        #[clap(long)]
        pr_number: u64,
        /// Comment ID to update
        #[clap(long)]
        comment_id: u32,
        /// New content for the comment
        #[clap(short, long)]
        content: String,
    },
    /// Get the number of comments on a pull request
    CommentCount {
        /// Project ID or Key
        #[clap(short, long)]
        project_id: String,
        /// Repository ID or Name
        #[clap(short, long)]
        repo_id: String,
        /// Pull Request number
        #[clap(long)]
        pr_number: u64,
    },
    /// Get the number of pull requests in a repository
    Count {
        /// Project ID or Key
        #[clap(short, long)]
        project_id: String,
        /// Repository ID or Name
        #[clap(short, long)]
        repo_id: String,
        /// Filter by status IDs (comma-separated, e.g., "1,2,3")
        #[clap(long)]
        status_ids: Option<String>,
        /// Filter by assignee user IDs (comma-separated, e.g., "100,200")
        #[clap(long)]
        assignee_ids: Option<String>,
        /// Filter by issue IDs (comma-separated, e.g., "1000,2000")
        #[clap(long)]
        issue_ids: Option<String>,
        /// Filter by created user IDs (comma-separated, e.g., "300,400")
        #[clap(long)]
        created_user_ids: Option<String>,
        /// Offset for pagination
        #[clap(long)]
        offset: Option<u32>,
        /// Number of pull requests to count (1-100, default 20)
        #[clap(long)]
        count: Option<u8>,
    },
    /// Create a new pull request
    #[cfg(feature = "git_writable")]
    Create {
        /// Project ID or Key
        #[clap(short, long)]
        project_id: String,
        /// Repository ID or Name
        #[clap(short, long)]
        repo_id: String,
        /// Pull request title
        #[clap(short, long)]
        summary: String,
        /// Pull request description
        #[clap(short, long)]
        description: String,
        /// Target merge branch
        #[clap(short, long)]
        base: String,
        /// Source branch to be merged
        #[clap(short = 'B', long)]
        branch: String,
        /// Related issue ID
        #[clap(long)]
        issue_id: Option<u32>,
        /// Assignee user ID
        #[clap(long)]
        assignee_id: Option<u32>,
        /// User IDs to notify (comma-separated, e.g., "123,456")
        #[clap(long)]
        notify_user_ids: Option<String>,
        /// Attachment IDs (comma-separated, e.g., "789,101112")
        #[clap(long)]
        attachment_ids: Option<String>,
    },
}

#[derive(Args, Debug)]
struct DownloadPrAttachmentArgs {
    /// Project ID or Key
    #[clap(short = 'p', long)]
    project_id: String,
    /// Repository ID or Name
    #[clap(short = 'r', long)]
    repo_id: String,
    /// Pull Request number
    #[clap(short = 'n', long)]
    pr_number: u64,
    /// The numeric ID of the attachment to download
    #[clap(short = 'a', long)]
    attachment_id: u32,
    /// Output file path to save the attachment
    #[clap(short = 'o', long, value_name = "FILE_PATH")]
    output: PathBuf,
}

#[cfg(feature = "git_writable")]
#[derive(Args, Debug)]
struct DeletePrAttachmentArgs {
    /// Project ID or Key
    #[clap(short = 'p', long)]
    project_id: String,
    /// Repository ID or Name
    #[clap(short = 'r', long)]
    repo_id: String,
    /// Pull Request number
    #[clap(short = 'n', long)]
    pr_number: u64,
    /// The numeric ID of the attachment to delete
    #[clap(short = 'a', long)]
    attachment_id: u32,
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

#[cfg(feature = "user")]
#[derive(Parser)]
struct UserArgs {
    #[clap(subcommand)]
    command: UserCommands,
}

#[cfg(feature = "user")]
#[derive(Parser)]
enum UserCommands {
    /// List all users
    List,
    /// Get current user info
    Me,
    /// Show user details
    Show {
        /// User ID
        #[clap(name = "USER_ID")]
        user_id: u32,
    },
    /// Download user icon
    Icon {
        /// User ID
        #[clap(name = "USER_ID")]
        user_id: u32,
        /// Output file path to save the icon
        #[clap(short, long, value_name = "FILE_PATH")]
        output: PathBuf,
    },
    /// Get user star count
    StarCount {
        /// User ID
        #[clap(name = "USER_ID")]
        user_id: u32,
        /// Count stars from this date (YYYY-MM-DD format)
        #[clap(long)]
        since: Option<String>,
        /// Count stars until this date (YYYY-MM-DD format)
        #[clap(long)]
        until: Option<String>,
    },
    /// Get user stars list
    Stars {
        /// User ID
        #[clap(name = "USER_ID")]
        user_id: u32,
        /// Get stars with ID greater than this value
        #[clap(long)]
        min_id: Option<u64>,
        /// Get stars with ID less than this value
        #[clap(long)]
        max_id: Option<u64>,
        /// Maximum number of results to return (1-100)
        #[clap(long)]
        count: Option<u32>,
        /// Sort order (asc or desc)
        #[clap(long)]
        order: Option<String>,
    },
    /// Get notification count for authenticated user
    NotificationCount {
        /// Include already read notifications
        #[clap(long)]
        already_read: Option<bool>,
        /// Include notifications where resource is already read
        #[clap(long)]
        resource_already_read: Option<bool>,
    },
    /// Get list of notifications for authenticated user
    #[clap(alias = "notif")]
    Notifications {
        /// Show notifications with ID greater than this value
        #[clap(long)]
        min_id: Option<u64>,
        /// Show notifications with ID less than this value
        #[clap(long)]
        max_id: Option<u64>,
        /// Maximum number of results to return (1-100)
        #[clap(long, short = 'n')]
        count: Option<u8>,
        /// Sort order (asc or desc)
        #[clap(long, short = 'o')]
        order: Option<String>,
        /// Filter by sender user ID
        #[clap(long)]
        sender_id: Option<u32>,
    },
    /// Mark a notification as read
    #[cfg(feature = "user_writable")]
    MarkNotificationRead {
        /// Notification ID to mark as read
        notification_id: u32,
    },
    /// Reset all unread notifications (mark all as read)
    #[cfg(feature = "user_writable")]
    ResetNotifications,
    /// Get list of watchings for a user
    Watchings {
        /// User ID
        #[clap(name = "USER_ID")]
        user_id: u32,
        /// Sort order (asc or desc)
        #[clap(long)]
        order: Option<String>,
        /// Sort by (created, updated, issueUpdated)
        #[clap(long)]
        sort: Option<String>,
        /// Maximum number of results to return (1-100)
        #[clap(long)]
        count: Option<u8>,
        /// Offset for pagination
        #[clap(long)]
        offset: Option<u64>,
        /// Filter by resource already read status
        #[clap(long)]
        resource_already_read: Option<bool>,
        /// Filter by issue IDs (comma-separated)
        #[clap(long)]
        issue_ids: Option<String>,
    },
    /// Get count of watchings for a user
    WatchingCount {
        /// User ID
        #[clap(name = "USER_ID")]
        user_id: u32,
        /// Filter by resource already read status
        #[clap(long)]
        resource_already_read: Option<bool>,
        /// Filter by already read status
        #[clap(long)]
        already_read: Option<bool>,
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
        Commands::Repo(repo_args) => match repo_args.command {
            RepoCommands::List { project_id } => {
                println!("Listing repositories for project: {project_id}");
                let proj_id_or_key = project_id.parse::<ProjectIdOrKey>()?;
                // Assumes backlog_git is enabled via features for the client build
                let params = backlog_api_client::GetRepositoryListParams::new(proj_id_or_key);
                let repos = client.git().get_repository_list(params).await?;
                // TODO: Pretty print repositories
                println!("{repos:?}");
            }
            RepoCommands::Show {
                project_id,
                repo_id,
            } => {
                println!("Showing repository {repo_id} in project: {project_id}");
                let proj_id_or_key = project_id.parse::<ProjectIdOrKey>()?;
                let repo_id_or_name = repo_id.parse::<RepositoryIdOrName>()?;
                let params =
                    backlog_api_client::GetRepositoryParams::new(proj_id_or_key, repo_id_or_name);
                let repo = client.git().get_repository(params).await?;
                // TODO: Pretty print repository
                println!("{repo:?}");
            }
        },
        Commands::Pr(pr_args) => match pr_args.command {
            PrCommands::List {
                project_id,
                repo_id,
            } => {
                println!("Listing pull requests for repo {repo_id} in project: {project_id}");
                let proj_id_or_key = project_id.parse::<ProjectIdOrKey>()?;
                let repo_id_or_name = repo_id.parse::<RepositoryIdOrName>()?;
                let params = backlog_api_client::GetPullRequestListParams::new(
                    proj_id_or_key,
                    repo_id_or_name,
                );
                let prs = client.git().get_pull_request_list(params).await?;
                // TODO: Pretty print pull requests
                println!("{prs:?}");
            }
            PrCommands::Show {
                project_id,
                repo_id,
                pr_number,
            } => {
                println!("Showing PR #{pr_number} for repo {repo_id} in project: {project_id}");
                let proj_id_or_key = project_id.parse::<ProjectIdOrKey>()?;
                let repo_id_or_name = repo_id.parse::<RepositoryIdOrName>()?;
                let pr_num = PullRequestNumber::from(pr_number);

                let params = backlog_api_client::GetPullRequestParams::new(
                    proj_id_or_key,
                    repo_id_or_name,
                    pr_num,
                );
                let pr = client.git().get_pull_request(params).await?;
                // TODO: Pretty print pull request
                println!("{pr:?}");
            }
            PrCommands::DownloadAttachment(dl_args) => {
                println!(
                    "Downloading attachment {} for PR #{} in repo {} (project {}) to {}",
                    dl_args.attachment_id,
                    dl_args.pr_number,
                    dl_args.repo_id,
                    dl_args.project_id,
                    dl_args.output.display()
                );

                let parsed_project_id =
                    ProjectIdOrKey::from_str(&dl_args.project_id).map_err(|e| {
                        format!("Failed to parse project_id '{}': {}", dl_args.project_id, e)
                    })?;
                let parsed_repo_id = RepositoryIdOrName::from_str(&dl_args.repo_id)
                    .map_err(|e| format!("Failed to parse repo_id '{}': {}", dl_args.repo_id, e))?;
                let parsed_attachment_id = PullRequestAttachmentId::new(dl_args.attachment_id);

                let parsed_pr_number = PullRequestNumber::from(dl_args.pr_number);

                let params = backlog_api_client::DownloadPullRequestAttachmentParams::new(
                    parsed_project_id,
                    parsed_repo_id,
                    parsed_pr_number,
                    parsed_attachment_id,
                );
                match client.git().download_pull_request_attachment(params).await {
                    Ok(downloaded_file) => {
                        if let Err(e) = fs::write(&dl_args.output, &downloaded_file.bytes).await {
                            eprintln!(
                                "Error writing attachment to {}: {}",
                                dl_args.output.display(),
                                e
                            );
                        } else {
                            println!(
                                "Attachment downloaded successfully to: {}",
                                dl_args.output.display()
                            );
                        }
                    }
                    Err(e) => {
                        eprintln!("Error downloading PR attachment: {e}");
                    }
                }
            }
            #[cfg(feature = "git_writable")]
            PrCommands::DeleteAttachment(del_args) => {
                println!(
                    "Deleting attachment {} from PR #{} in repo {} (project {})",
                    del_args.attachment_id,
                    del_args.pr_number,
                    del_args.repo_id,
                    del_args.project_id
                );

                let parsed_project_id =
                    ProjectIdOrKey::from_str(&del_args.project_id).map_err(|e| {
                        format!(
                            "Failed to parse project_id '{}': {}",
                            del_args.project_id, e
                        )
                    })?;
                let parsed_repo_id =
                    RepositoryIdOrName::from_str(&del_args.repo_id).map_err(|e| {
                        format!("Failed to parse repo_id '{}': {}", del_args.repo_id, e)
                    })?;
                let parsed_attachment_id = PullRequestAttachmentId::new(del_args.attachment_id);
                let parsed_pr_number = PullRequestNumber::from(del_args.pr_number);

                let params = backlog_api_client::DeletePullRequestAttachmentParams::new(
                    parsed_project_id,
                    parsed_repo_id,
                    parsed_pr_number,
                    parsed_attachment_id,
                );
                match client.git().delete_pull_request_attachment(params).await {
                    Ok(deleted_attachment) => {
                        println!("✅ Attachment deleted successfully");
                        println!("Deleted attachment ID: {}", deleted_attachment.id.value());
                        println!("Name: {}", deleted_attachment.name);
                        println!("Size: {} bytes", deleted_attachment.size);
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to delete PR attachment: {e}");
                        std::process::exit(1);
                    }
                }
            }
            #[cfg(feature = "git_writable")]
            PrCommands::Update {
                project_id,
                repo_id,
                pr_number,
                summary,
                description,
                issue_id,
                assignee_id,
                notify_user_ids,
                comment,
            } => {
                println!("Updating PR #{pr_number} in repo {repo_id} (project {project_id})");

                let parsed_project_id = ProjectIdOrKey::from_str(&project_id)
                    .map_err(|e| format!("Failed to parse project_id '{project_id}': {e}"))?;
                let parsed_repo_id = RepositoryIdOrName::from_str(&repo_id)
                    .map_err(|e| format!("Failed to parse repo_id '{repo_id}': {e}"))?;
                let parsed_pr_number = PullRequestNumber::from(pr_number);

                let mut params = UpdatePullRequestParams::new(
                    parsed_project_id,
                    parsed_repo_id,
                    parsed_pr_number,
                );

                if let Some(summary) = summary {
                    params = params.summary(summary.clone());
                }

                if let Some(description) = description {
                    params = params.description(description.clone());
                }

                if let Some(issue_id) = issue_id {
                    params = params.issue_id(IssueId::new(issue_id));
                }

                if let Some(assignee_id) = assignee_id {
                    params = params.assignee_id(UserId::new(assignee_id));
                }

                if let Some(notify_user_ids) = notify_user_ids {
                    let user_ids: Vec<UserId> =
                        notify_user_ids.iter().map(|&id| UserId::new(id)).collect();
                    params = params.notified_user_ids(user_ids);
                }

                if let Some(comment) = comment {
                    params = params.comment(comment.clone());
                }

                match client.git().update_pull_request(params).await {
                    Ok(pull_request) => {
                        println!("✅ Pull request updated successfully");
                        println!("ID: {}", pull_request.id.value());
                        println!("Number: {}", pull_request.number.value());
                        println!("Summary: {}", pull_request.summary);
                        if let Some(description) = &pull_request.description {
                            println!("Description: {description}");
                        }
                        if let Some(assignee) = &pull_request.assignee {
                            println!("Assignee: {} (ID: {})", assignee.name, assignee.id.value());
                        }
                        if let Some(issue) = &pull_request.related_issue {
                            println!("Related Issue ID: {}", issue.id.value());
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to update pull request: {e}");
                        std::process::exit(1);
                    }
                }
            }
            #[cfg(feature = "git_writable")]
            PrCommands::CommentUpdate {
                project_id,
                repo_id,
                pr_number,
                comment_id,
                content,
            } => {
                println!(
                    "Updating comment {comment_id} for PR #{pr_number} in repo {repo_id} (project {project_id})"
                );

                let parsed_project_id = ProjectIdOrKey::from_str(&project_id)
                    .map_err(|e| format!("Failed to parse project_id '{project_id}': {e}"))?;
                let parsed_repo_id = RepositoryIdOrName::from_str(&repo_id)
                    .map_err(|e| format!("Failed to parse repo_id '{repo_id}': {e}"))?;
                let parsed_pr_number = PullRequestNumber::from(pr_number);
                let parsed_comment_id = PullRequestCommentId::new(comment_id);

                let params = backlog_api_client::UpdatePullRequestCommentParams::new(
                    parsed_project_id,
                    parsed_repo_id,
                    parsed_pr_number,
                    parsed_comment_id,
                    &content,
                );

                match client.git().update_pull_request_comment(params).await {
                    Ok(comment) => {
                        println!("✅ Pull request comment updated successfully");
                        println!("Comment ID: {}", comment.id.value());
                        println!("Content: {}", comment.content);
                        println!(
                            "Created by: {} (ID: {})",
                            comment.created_user.name,
                            comment.created_user.id.value()
                        );
                        println!("Created: {}", comment.created);
                        println!("Updated: {}", comment.updated);
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to update pull request comment: {e}");
                        std::process::exit(1);
                    }
                }
            }
            PrCommands::CommentCount {
                project_id,
                repo_id,
                pr_number,
            } => {
                println!(
                    "Getting comment count for PR #{pr_number} in repo {repo_id} (project {project_id})"
                );

                let parsed_project_id = ProjectIdOrKey::from_str(&project_id)
                    .map_err(|e| format!("Failed to parse project_id '{project_id}': {e}"))?;
                let parsed_repo_id = RepositoryIdOrName::from_str(&repo_id)
                    .map_err(|e| format!("Failed to parse repo_id '{repo_id}': {e}"))?;
                let parsed_pr_number = PullRequestNumber::from(pr_number);

                let params = backlog_api_client::GetPullRequestCommentCountParams::new(
                    parsed_project_id,
                    parsed_repo_id,
                    parsed_pr_number,
                );
                match client.git().get_pull_request_comment_count(params).await {
                    Ok(count_response) => {
                        println!("✅ Pull request comment count retrieved successfully");
                        println!("Comment count: {}", count_response.count);
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to get pull request comment count: {e}");
                        std::process::exit(1);
                    }
                }
            }
            PrCommands::Count {
                project_id,
                repo_id,
                status_ids,
                assignee_ids,
                issue_ids,
                created_user_ids,
                offset: _,
                count: _,
            } => {
                println!("Getting pull request count for repo {repo_id} (project {project_id})");

                let parsed_project_id = ProjectIdOrKey::from_str(&project_id)
                    .map_err(|e| format!("Failed to parse project_id '{project_id}': {e}"))?;
                let parsed_repo_id = RepositoryIdOrName::from_str(&repo_id)
                    .map_err(|e| format!("Failed to parse repo_id '{repo_id}': {e}"))?;

                // Parse filter parameters
                let mut params = GetPullRequestCountParams::new(parsed_project_id, parsed_repo_id);

                // Parse status IDs
                if let Some(status_ids_str) = status_ids {
                    let status_ids: Result<Vec<StatusId>, _> = status_ids_str
                        .split(',')
                        .map(|s| s.trim().parse::<u32>().map(StatusId::new))
                        .collect();
                    match status_ids {
                        Ok(ids) => params = params.status_ids(ids),
                        Err(e) => {
                            eprintln!("❌ Failed to parse status_ids: {e}");
                            std::process::exit(1);
                        }
                    };
                }

                // Parse assignee IDs
                if let Some(assignee_ids_str) = assignee_ids {
                    let assignee_ids: Result<Vec<UserId>, _> = assignee_ids_str
                        .split(',')
                        .map(|s| s.trim().parse::<u32>().map(UserId::new))
                        .collect();
                    match assignee_ids {
                        Ok(ids) => params = params.assignee_ids(ids),
                        Err(e) => {
                            eprintln!("❌ Failed to parse assignee_ids: {e}");
                            std::process::exit(1);
                        }
                    };
                }

                // Parse issue IDs
                if let Some(issue_ids_str) = issue_ids {
                    let issue_ids: Result<Vec<IssueId>, _> = issue_ids_str
                        .split(',')
                        .map(|s| s.trim().parse::<u32>().map(IssueId::new))
                        .collect();
                    match issue_ids {
                        Ok(ids) => params = params.issue_ids(ids),
                        Err(e) => {
                            eprintln!("❌ Failed to parse issue_ids: {e}");
                            std::process::exit(1);
                        }
                    };
                }

                // Parse created user IDs
                if let Some(created_user_ids_str) = created_user_ids {
                    let created_user_ids: Result<Vec<UserId>, _> = created_user_ids_str
                        .split(',')
                        .map(|s| s.trim().parse::<u32>().map(UserId::new))
                        .collect();
                    match created_user_ids {
                        Ok(ids) => params = params.created_user_ids(ids),
                        Err(e) => {
                            eprintln!("❌ Failed to parse created_user_ids: {e}");
                            std::process::exit(1);
                        }
                    };
                }

                match client.git().get_pull_request_count(params).await {
                    Ok(count_response) => {
                        println!("✅ Pull request count retrieved successfully");
                        println!("Pull request count: {}", count_response.count);
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to get pull request count: {e}");
                        std::process::exit(1);
                    }
                }
            }
            #[cfg(feature = "git_writable")]
            PrCommands::Create {
                project_id,
                repo_id,
                summary,
                description,
                base,
                branch,
                issue_id,
                assignee_id,
                notify_user_ids,
                attachment_ids,
            } => {
                println!("Creating pull request in repo {repo_id} (project {project_id})");

                let parsed_project_id = ProjectIdOrKey::from_str(&project_id)
                    .map_err(|e| format!("Failed to parse project_id '{project_id}': {e}"))?;
                let parsed_repo_id = RepositoryIdOrName::from_str(&repo_id)
                    .map_err(|e| format!("Failed to parse repo_id '{repo_id}': {e}"))?;

                // Build parameters
                let mut params = AddPullRequestParams::new(
                    parsed_project_id,
                    parsed_repo_id,
                    summary.clone(),
                    description.clone(),
                    base.clone(),
                    branch.clone(),
                );

                // Parse optional issue ID
                if let Some(issue_id) = issue_id {
                    params = params.issue_id(backlog_core::identifier::IssueId::new(issue_id));
                }

                // Parse optional assignee ID
                if let Some(assignee_id) = assignee_id {
                    params = params.assignee_id(UserId::new(assignee_id));
                }

                // Parse notify user IDs
                if let Some(notify_user_ids_str) = notify_user_ids {
                    let notify_user_ids: Result<Vec<UserId>, _> = notify_user_ids_str
                        .split(',')
                        .map(|s| s.trim().parse::<u32>().map(UserId::new))
                        .collect();
                    match notify_user_ids {
                        Ok(ids) => params = params.notified_user_ids(ids),
                        Err(e) => {
                            eprintln!("❌ Failed to parse notify_user_ids: {e}");
                            std::process::exit(1);
                        }
                    };
                }

                // Parse attachment IDs
                if let Some(attachment_ids_str) = attachment_ids {
                    let attachment_ids: Result<Vec<AttachmentId>, _> = attachment_ids_str
                        .split(',')
                        .map(|s| s.trim().parse::<u32>().map(AttachmentId::new))
                        .collect();
                    match attachment_ids {
                        Ok(ids) => params = params.attachment_ids(ids),
                        Err(e) => {
                            eprintln!("❌ Failed to parse attachment_ids: {e}");
                            std::process::exit(1);
                        }
                    };
                }

                match client.git().add_pull_request(params).await {
                    Ok(pull_request) => {
                        println!("✅ Pull request created successfully");
                        println!("ID: {}", pull_request.id.value());
                        println!("Number: {}", pull_request.number.value());
                        println!("Summary: {}", pull_request.summary);
                        if let Some(description) = &pull_request.description {
                            println!("Description: {description}");
                        }
                        println!("Base: {}", pull_request.base);
                        println!("Branch: {}", pull_request.branch);
                        if let Some(assignee) = &pull_request.assignee {
                            println!("Assignee: {} (ID: {})", assignee.name, assignee.id.value());
                        }
                        if let Some(issue) = &pull_request.related_issue {
                            println!("Related Issue ID: {}", issue.id.value());
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to create pull request: {e}");
                        std::process::exit(1);
                    }
                }
            }
        },
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
        Commands::User(user_args) => match user_args.command {
            UserCommands::List => {
                println!("Listing all users:");

                match client.user().get_user_list(GetUserListParams::new()).await {
                    Ok(users) => {
                        if users.is_empty() {
                            println!("No users found");
                        } else {
                            for user in users {
                                let user_id_str = user.user_id.as_deref().unwrap_or("N/A");
                                println!("[{}] {} ({})", user.id, user.name, user_id_str);
                                if !user.mail_address.is_empty() {
                                    println!("  Email: {}", user.mail_address);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error listing users: {e}");
                    }
                }
            }
            UserCommands::Me => {
                println!("Getting current user info:");

                match client.user().get_own_user(GetOwnUserParams::new()).await {
                    Ok(user) => {
                        println!("User ID: {}", user.id);
                        if let Some(login_id) = &user.user_id {
                            println!("Login ID: {login_id}");
                        }
                        println!("Name: {}", user.name);
                        if !user.mail_address.is_empty() {
                            println!("Email: {}", user.mail_address);
                        }
                        if let Some(lang) = &user.lang {
                            println!("Language: {lang}");
                        }
                        if let Some(last_login) = &user.last_login_time {
                            println!("Last Login: {last_login}");
                        }
                    }
                    Err(e) => {
                        eprintln!("Error getting user info: {e}");
                    }
                }
            }
            UserCommands::Show { user_id } => {
                println!("Getting user info for user ID: {user_id}");

                match client.user().get_user(GetUserParams::new(user_id)).await {
                    Ok(user) => {
                        println!("✅ User found");
                        println!("ID: {}", user.id);
                        if let Some(login_id) = &user.user_id {
                            println!("Login ID: {login_id}");
                        }
                        println!("Name: {}", user.name);
                        println!("Role: {}", user.role_type);
                        if !user.mail_address.is_empty() {
                            println!("Email: {}", user.mail_address);
                        }
                        if let Some(lang) = &user.lang {
                            println!("Language: {lang}");
                        }
                        if let Some(last_login) = &user.last_login_time {
                            println!("Last Login: {last_login}");
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to get user: {e}");
                        std::process::exit(1);
                    }
                }
            }
            UserCommands::Icon { user_id, output } => {
                println!("Downloading user icon to {}", output.display());

                match client
                    .user()
                    .get_user_icon(GetUserIconParams::new(user_id))
                    .await
                {
                    Ok(file) => {
                        let icon_bytes = file.bytes;
                        if let Err(e) = fs::write(&output, &icon_bytes).await {
                            eprintln!("Error writing icon to {}: {}", output.display(), e);
                        } else {
                            println!("User icon downloaded successfully to: {}", output.display());
                        }
                    }
                    Err(e) => {
                        eprintln!("Error downloading user icon: {e}");
                    }
                }
            }
            UserCommands::StarCount {
                user_id,
                since,
                until,
            } => {
                println!("Getting star count for user ID: {user_id}");

                let mut params = GetUserStarCountParams::new(user_id);

                if let Some(since_str) = since {
                    match NaiveDate::parse_from_str(&since_str, "%Y-%m-%d") {
                        Ok(date) => {
                            let datetime = date_to_start_of_day(date);
                            params = params.with_since(ApiDate::from(datetime));
                            println!("Counting stars from: {since_str}");
                        }
                        Err(_) => {
                            eprintln!(
                                "Invalid date format for 'since': {since_str}. Expected format: YYYY-MM-DD"
                            );
                            return Ok(());
                        }
                    }
                }

                if let Some(until_str) = until {
                    match NaiveDate::parse_from_str(&until_str, "%Y-%m-%d") {
                        Ok(date) => {
                            let datetime = date_to_end_of_day(date);
                            params = params.with_until(ApiDate::from(datetime));
                            println!("Counting stars until: {until_str}");
                        }
                        Err(_) => {
                            eprintln!(
                                "Invalid date format for 'until': {until_str}. Expected format: YYYY-MM-DD"
                            );
                            return Ok(());
                        }
                    }
                }

                match client.user().get_user_star_count(params).await {
                    Ok(star_count) => {
                        println!("User has received {} star(s)", star_count.count);
                    }
                    Err(e) => {
                        eprintln!("Error getting star count: {e}");
                    }
                }
            }
            UserCommands::Stars {
                user_id,
                min_id,
                max_id,
                count,
                order,
            } => {
                println!("Getting stars for user ID: {user_id}");

                let mut params = GetUserStarsParams::new(user_id);

                if let Some(min_id) = min_id {
                    params = params.with_min_id(min_id);
                }

                if let Some(max_id) = max_id {
                    params = params.with_max_id(max_id);
                }

                if let Some(count) = count {
                    params = params.with_count(count);
                }

                if let Some(order_str) = order {
                    let order_enum = match order_str.to_lowercase().as_str() {
                        "asc" => StarOrder::Asc,
                        "desc" => StarOrder::Desc,
                        _ => {
                            eprintln!("Invalid order: '{order_str}'. Must be 'asc' or 'desc'");
                            return Ok(());
                        }
                    };
                    params = params.with_order(order_enum);
                }

                match client.user().get_user_stars(params).await {
                    Ok(stars) => {
                        if stars.is_empty() {
                            println!("No stars found for this user");
                        } else {
                            println!("Found {} star(s):", stars.len());
                            println!();
                            for star in stars {
                                println!("Star ID: {}", star.id);
                                println!("Title: {}", star.title);
                                println!("URL: {}", star.url);
                                if let Some(comment) = &star.comment {
                                    println!("Comment: {comment}");
                                }
                                println!(
                                    "Presenter: {} (ID: {})",
                                    star.presenter.name, star.presenter.id
                                );
                                println!(
                                    "Created: {}",
                                    star.created.format("%Y-%m-%d %H:%M:%S UTC")
                                );
                                println!("---");
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error getting stars: {e}");
                    }
                }
            }
            UserCommands::NotificationCount {
                already_read,
                resource_already_read,
            } => {
                println!("Getting notification count for authenticated user");

                let mut params = GetNotificationCountParams::new();

                if let Some(already_read) = already_read {
                    params = params.with_already_read(already_read);
                }

                if let Some(resource_already_read) = resource_already_read {
                    params = params.with_resource_already_read(resource_already_read);
                }

                match client.user().get_notification_count(params).await {
                    Ok(notification_count) => {
                        println!("✅ Notification count: {}", notification_count.count);
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to get notification count: {e}");
                        std::process::exit(1);
                    }
                }
            }
            UserCommands::Notifications {
                min_id,
                max_id,
                count,
                order,
                sender_id,
            } => {
                println!("Getting notifications for authenticated user");

                let mut params = GetNotificationsParams::new();

                if let Some(min_id) = min_id {
                    params = params.with_min_id(min_id);
                }

                if let Some(max_id) = max_id {
                    params = params.with_max_id(max_id);
                }

                if let Some(count) = count {
                    params = params.with_count(count);
                }

                if let Some(order_str) = order {
                    let notification_order = match order_str.to_lowercase().as_str() {
                        "asc" => NotificationOrder::Asc,
                        "desc" => NotificationOrder::Desc,
                        _ => {
                            eprintln!("❌ Invalid order. Use 'asc' or 'desc'");
                            std::process::exit(1);
                        }
                    };
                    params = params.with_order(notification_order);
                }

                if let Some(sender_id) = sender_id {
                    params = params.with_sender_id(UserId::new(sender_id));
                }

                match client.user().get_notifications(params).await {
                    Ok(notifications) => {
                        if notifications.is_empty() {
                            println!("No notifications found");
                        } else {
                            println!("Found {} notification(s):", notifications.len());
                            println!();

                            for (index, notification) in notifications.iter().enumerate() {
                                println!(
                                    "{}. Notification #{}",
                                    index + 1,
                                    notification.id.value()
                                );
                                println!(
                                    "   Status: {}",
                                    if notification.already_read {
                                        "Read"
                                    } else {
                                        "Unread"
                                    }
                                );
                                println!("   Reason: {:?}", notification.reason);
                                println!(
                                    "   Project: {} ({})",
                                    notification.project.name, notification.project.project_key
                                );
                                println!(
                                    "   From: {} ({})",
                                    notification.sender.name, notification.sender.mail_address
                                );

                                if let Some(issue) = &notification.issue {
                                    println!("   Issue: {} - {}", issue.issue_key, issue.summary);
                                }

                                if let Some(comment) = &notification.comment
                                    && let Some(content) = &comment.content
                                {
                                    let preview = content.chars().take(100).collect::<String>();
                                    println!("   Comment: {preview}");
                                }

                                println!(
                                    "   Created: {}",
                                    notification.created.format("%Y-%m-%d %H:%M:%S")
                                );
                                println!();
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to get notifications: {e}");
                        std::process::exit(1);
                    }
                }
            }
            #[cfg(feature = "user_writable")]
            UserCommands::MarkNotificationRead { notification_id } => {
                println!("Marking notification {notification_id} as read");

                match client
                    .user()
                    .mark_notification_as_read(notification_id)
                    .await
                {
                    Ok(()) => {
                        println!("✅ Notification marked as read");
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to mark notification as read: {e}");
                        std::process::exit(1);
                    }
                }
            }
            #[cfg(feature = "user_writable")]
            UserCommands::ResetNotifications => {
                println!("Marking all unread notifications as read...");

                match client.user().reset_unread_notification_count().await {
                    Ok(result) => {
                        println!("✅ All unread notifications marked as read");
                        println!("   Previously unread count: {}", result.count);
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to reset notifications: {e}");
                        std::process::exit(1);
                    }
                }
            }
            UserCommands::Watchings {
                user_id,
                order,
                sort,
                count,
                offset,
                resource_already_read,
                issue_ids,
            } => {
                println!("Getting watchings for user {user_id}");

                let mut params = GetWatchingListParams::builder();

                if let Some(order_str) = order {
                    let order_enum = match order_str.to_lowercase().as_str() {
                        "asc" => WatchingOrder::Asc,
                        "desc" => WatchingOrder::Desc,
                        _ => {
                            eprintln!("Invalid order: {order_str}. Use 'asc' or 'desc'");
                            std::process::exit(1);
                        }
                    };
                    params = params.order(order_enum);
                }

                if let Some(sort_str) = sort {
                    let sort_enum = match sort_str.to_lowercase().as_str() {
                        "created" => WatchingSort::Created,
                        "updated" => WatchingSort::Updated,
                        "issueupdated" => WatchingSort::IssueUpdated,
                        _ => {
                            eprintln!(
                                "Invalid sort: {sort_str}. Use 'created', 'updated', or 'issueUpdated'"
                            );
                            std::process::exit(1);
                        }
                    };
                    params = params.sort(sort_enum);
                }

                if let Some(c) = count {
                    params = params.count(c);
                }

                if let Some(o) = offset {
                    params = params.offset(o);
                }

                if let Some(read) = resource_already_read {
                    params = params.resource_already_read(read);
                }

                if let Some(ids_str) = issue_ids {
                    let ids: Vec<IssueId> = ids_str
                        .split(',')
                        .filter_map(|s| s.trim().parse::<u32>().ok())
                        .map(IssueId::from)
                        .collect();
                    if !ids.is_empty() {
                        params = params.issue_ids(ids);
                    }
                }

                let params = params.build()?;

                match client.user().get_watching_list(user_id, params).await {
                    Ok(watchings) => {
                        if watchings.is_empty() {
                            println!("No watchings found");
                        } else {
                            println!("Found {} watching(s):", watchings.len());
                            println!();

                            for (index, watching) in watchings.iter().enumerate() {
                                println!("{}. Watching #{}", index + 1, watching.id.value());
                                println!("   Type: {:?}", watching.watching_type);
                                println!(
                                    "   Status: {}",
                                    if watching.resource_already_read {
                                        "Read"
                                    } else {
                                        "Unread"
                                    }
                                );

                                if let Some(note) = &watching.note {
                                    println!("   Note: {note}");
                                }

                                if let Some(issue) = &watching.issue {
                                    println!("   Issue: {} - {}", issue.issue_key, issue.summary);
                                    println!("   Project ID: {}", issue.project_id.value());
                                    println!("   Status: {}", issue.status.name);
                                    if let Some(assignee) = &issue.assignee {
                                        println!("   Assignee: {}", assignee.name);
                                    }
                                }

                                if let Some(last_updated) = &watching.last_content_updated {
                                    println!(
                                        "   Last Updated: {}",
                                        last_updated.format("%Y-%m-%d %H:%M:%S")
                                    );
                                }

                                println!(
                                    "   Created: {}",
                                    watching.created.format("%Y-%m-%d %H:%M:%S")
                                );
                                println!();
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to get watchings: {e}");
                        std::process::exit(1);
                    }
                }
            }
            UserCommands::WatchingCount {
                user_id,
                resource_already_read,
                already_read,
            } => {
                println!("Getting watching count for user {user_id}");

                let mut params = GetWatchingCountParams::new(UserId::from(user_id));

                if let Some(read) = resource_already_read {
                    params = params.with_resource_already_read(read);
                }

                if let Some(read) = already_read {
                    params = params.with_already_read(read);
                }

                match client.user().get_watching_count(params).await {
                    Ok(response) => {
                        println!("✅ Watching count retrieved successfully");
                        println!("Total watchings: {}", response.count);

                        if resource_already_read.is_some() || already_read.is_some() {
                            println!("\nFilters applied:");
                            if let Some(read) = resource_already_read {
                                println!("  Resource already read: {read}");
                            }
                            if let Some(read) = already_read {
                                println!("  Already read: {read}");
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to get watching count: {e}");
                        std::process::exit(1);
                    }
                }
            }
        },
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
