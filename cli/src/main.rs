use blg::custom_fields;

mod activity_commands;
#[cfg(any(
    feature = "team",
    feature = "star",
    feature = "rate-limit",
    feature = "watching",
    feature = "webhook"
))]
mod commands;
#[cfg(feature = "project")]
use activity_commands::{ActivityArgs, ActivityCommands};
#[cfg(feature = "rate-limit")]
use commands::rate_limit::{RateLimitCommand, handle_rate_limit_command};
#[cfg(feature = "star")]
use commands::star::{StarArgs, handle_star_command};
#[cfg(feature = "team")]
use commands::team::{TeamArgs, handle_team_command};
#[cfg(feature = "watching")]
use commands::watching::handle_watching_command;

#[cfg(feature = "issue_writable")]
use backlog_api_client::AddCommentParamsBuilder;
#[cfg(feature = "git_writable")]
use backlog_api_client::AddPullRequestParams;
#[cfg(feature = "issue_writable")]
use backlog_api_client::LinkSharedFilesToIssueParamsBuilder;
#[cfg(feature = "git_writable")]
#[allow(unused_imports)]
use backlog_api_client::UpdatePullRequestCommentParams;
#[cfg(feature = "git_writable")]
#[allow(unused_imports)]
use backlog_api_client::UpdatePullRequestParams;
use backlog_api_client::{
    AttachmentId, GetCommentNotificationsParams, GetIssueListParamsBuilder,
    GetPullRequestCountParams, IssueIdOrKey, ProjectId, ProjectIdOrKey, PullRequestAttachmentId,
    PullRequestCommentId, PullRequestNumber, RepositoryIdOrName, StatusId, UserId, WikiId,
    backlog_issue, client::BacklogApiClient,
};
use backlog_core::ApiDate;
#[cfg(feature = "project")]
use backlog_core::identifier::ActivityTypeId;
use backlog_core::identifier::IssueId;
#[cfg(feature = "issue_writable")]
use backlog_core::identifier::SharedFileId;
#[cfg(feature = "wiki")]
use backlog_core::identifier::WikiAttachmentId;
use backlog_core::identifier::{CommentId, Identifier};
#[cfg(any(feature = "issue_writable", feature = "project_writable"))]
use backlog_core::{
    IssueKey,
    identifier::{
        CategoryId, CustomFieldId, CustomFieldItemId, IssueTypeId, MilestoneId, PriorityId,
        ResolutionId, TeamId,
    },
};
#[cfg(feature = "project_writable")]
use backlog_domain_models::{IssueTypeColor, StatusColor};
#[cfg(feature = "issue_writable")]
use backlog_issue::AddCommentNotificationParams;
#[cfg(feature = "issue_writable")]
use backlog_issue::DeleteAttachmentParams;
#[cfg(feature = "issue_writable")]
use backlog_issue::DeleteCommentParams;
use backlog_issue::GetRecentlyViewedIssuesParamsBuilder;
#[cfg(feature = "issue_writable")]
use backlog_issue::UnlinkSharedFileParams;
#[cfg(feature = "issue_writable")]
use backlog_issue::{
    AddIssueParamsBuilder, AddRecentlyViewedIssueParams, UpdateIssueParamsBuilder,
};
#[cfg(feature = "project")]
use backlog_project::GetProjectRecentUpdatesParams;
#[cfg(feature = "project_writable")]
use backlog_project::{
    AddCategoryParams, AddCustomFieldParams, AddIssueTypeParams, AddListItemToCustomFieldParams,
    AddMilestoneParams, AddProjectTeamParams, AddStatusParams, DeleteCategoryParams,
    DeleteCustomFieldParams, DeleteIssueTypeParams, DeleteProjectTeamParams, DeleteStatusParams,
    DeleteVersionParams, UpdateCategoryParams, UpdateCustomFieldParams, UpdateIssueTypeParams,
    UpdateListItemToCustomFieldParams, UpdateStatusOrderParams, UpdateStatusParams,
    UpdateVersionParams,
    api::{DeleteListItemFromCustomFieldParams, DeleteProjectParams, UpdateProjectParams},
};
use backlog_project::{
    GetProjectListParams, GetRecentlyViewedProjectsParamsBuilder, api::GetProjectDiskUsageParams,
};
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
#[cfg(feature = "wiki")]
use backlog_wiki::GetRecentlyViewedWikisParamsBuilder;
#[cfg(feature = "wiki_writable")]
use backlog_wiki::{
    AddWikiParams, AttachFilesToWikiParams, DeleteWikiAttachmentParams, DeleteWikiParams,
    UpdateWikiParams,
};
use chrono::NaiveDate;
#[cfg(feature = "project_writable")]
use chrono::{DateTime, Utc};
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
    Issue(IssueArgs),
    /// Manage space
    Space(SpaceArgs),
    /// Manage projects
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
struct IssueArgs {
    #[clap(subcommand)]
    command: IssueCommands,
}

#[derive(Parser)]
enum IssueCommands {
    /// List issues
    List {
        #[clap(flatten)]
        params: IssueListCliParams,
    },
    /// Show details of a specific issue
    Show {
        /// Issue ID or Key (e.g., "PROJECT-123" or "12345")
        #[clap(name = "ISSUE_ID_OR_KEY")]
        issue_id_or_key: String,
    },
    /// Download an issue attachment
    #[command(about = "Download an issue attachment")]
    DownloadAttachment(DownloadAttachmentArgs),
    /// Add a comment to an issue
    #[cfg(feature = "issue_writable")]
    #[command(about = "Add a comment to an issue")]
    AddComment(AddCommentArgs),
    /// Update an existing comment
    #[cfg(feature = "issue_writable")]
    #[command(about = "Update an existing comment")]
    UpdateComment(UpdateCommentArgs),
    /// Delete a comment from an issue
    #[cfg(feature = "issue_writable")]
    #[command(about = "Delete a comment from an issue")]
    DeleteComment(DeleteCommentArgs),
    /// Delete an attachment from an issue
    #[cfg(feature = "issue_writable")]
    #[command(about = "Delete an attachment from an issue")]
    DeleteAttachment(DeleteAttachmentArgs),
    /// Create a new issue
    #[cfg(feature = "issue_writable")]
    #[command(about = "Create a new issue")]
    Create(CreateIssueArgs),
    /// Update an existing issue
    #[cfg(feature = "issue_writable")]
    #[command(about = "Update an existing issue")]
    Update(UpdateIssueArgs),
    /// Delete an issue
    #[cfg(feature = "issue_writable")]
    #[command(about = "Delete an issue")]
    Delete(DeleteIssueArgs),
    /// Count comments for an issue
    #[command(about = "Count comments for an issue")]
    CountComment(CountCommentArgs),
    /// Get a specific comment for an issue
    #[command(about = "Get a specific comment for an issue")]
    GetComment(GetCommentArgs),
    /// Get notifications for a comment
    #[command(about = "Get notifications for a comment")]
    GetCommentNotifications(GetCommentArgs),
    /// Add notifications to a comment
    #[cfg(feature = "issue_writable")]
    #[command(about = "Add notifications to a comment")]
    AddCommentNotification(AddCommentNotificationArgs),
    /// List participants in an issue
    #[command(about = "List participants in an issue")]
    ListParticipants {
        /// Issue ID or Key (e.g., "PROJECT-123" or "12345")
        #[clap(name = "ISSUE_ID_OR_KEY")]
        issue_id_or_key: String,
    },
    /// List shared files linked to an issue
    #[command(about = "List shared files linked to an issue")]
    ListSharedFiles {
        /// Issue ID or Key (e.g., "PROJECT-123" or "12345")
        #[clap(name = "ISSUE_ID_OR_KEY")]
        issue_id_or_key: String,
    },
    /// Link shared files to an issue
    #[cfg(feature = "issue_writable")]
    #[command(about = "Link shared files to an issue")]
    LinkSharedFiles {
        /// Issue ID or Key (e.g., "PROJECT-123" or "12345")
        #[clap(name = "ISSUE_ID_OR_KEY")]
        issue_id_or_key: String,
        /// Shared file IDs to link (comma-separated)
        #[clap(short, long, value_delimiter = ',')]
        file_ids: Vec<u32>,
    },
    /// Unlink a shared file from an issue
    #[cfg(feature = "issue_writable")]
    #[command(about = "Unlink a shared file from an issue")]
    UnlinkSharedFile {
        /// Issue ID or Key (e.g., "PROJECT-123" or "12345")
        #[clap(name = "ISSUE_ID_OR_KEY")]
        issue_id_or_key: String,
        /// Shared file ID to unlink
        #[clap(short, long)]
        file_id: u32,
    },
    /// Get recently viewed issues
    #[command(about = "Get recently viewed issues for the current user")]
    RecentlyViewed {
        /// Sort order (asc or desc, default: desc)
        #[clap(long, default_value = "desc")]
        order: String,
        /// Number of issues to retrieve (1-100, default: 20)
        #[clap(long, default_value_t = 20)]
        count: u32,
        /// Offset for pagination
        #[clap(long)]
        offset: Option<u32>,
    },
    /// Add an issue to recently viewed list
    #[cfg(feature = "issue_writable")]
    #[command(about = "Add an issue to recently viewed list")]
    AddRecentlyViewed {
        /// Issue ID or Key (e.g., "PROJECT-123" or "12345")
        #[clap(name = "ISSUE_ID_OR_KEY")]
        issue_id_or_key: String,
    },
}

#[derive(Args, Debug)]
struct DownloadAttachmentArgs {
    /// The ID or key of the issue (e.g., "PROJECT-123" or "12345")
    issue_id_or_key: String,

    /// The numeric ID of the attachment to download
    attachment_id: u32,

    /// Output file path to save the attachment
    #[arg(short, long, value_name = "FILE_PATH")]
    output: PathBuf,
}

#[derive(Args, Debug)]
struct AddCommentArgs {
    /// The ID or key of the issue (e.g., "PROJECT-123" or "12345")
    issue_id_or_key: String,

    /// The comment content
    #[arg(short, long)]
    content: String,

    /// User IDs to notify (comma-separated, e.g., "123,456")
    #[arg(short, long)]
    notify_users: Option<String>,

    /// Attachment IDs to include (comma-separated, e.g., "789,101112")
    #[arg(short, long)]
    attachments: Option<String>,
}

#[cfg(feature = "issue_writable")]
#[derive(Args, Debug)]
struct UpdateCommentArgs {
    /// Issue ID or key (e.g., 'PROJECT-123')
    #[clap(short, long)]
    issue_id: String,

    /// Comment ID to update
    #[clap(short = 'c', long)]
    comment_id: u32,

    /// New content for the comment
    #[clap(short = 'n', long)]
    content: String,
}

#[cfg(feature = "issue_writable")]
#[derive(Args, Debug)]
struct DeleteCommentArgs {
    /// Issue ID or key (e.g., 'PROJECT-123')
    #[clap(short, long)]
    issue_id: String,

    /// Comment ID to delete
    #[clap(short = 'c', long)]
    comment_id: u32,
}

#[cfg(feature = "issue_writable")]
#[derive(Args, Debug)]
struct DeleteAttachmentArgs {
    /// Issue ID or key (e.g., 'PROJECT-123')
    #[clap(short, long)]
    issue_id: String,

    /// Attachment ID to delete
    #[clap(short = 'a', long)]
    attachment_id: u32,
}

#[derive(Args, Debug)]
struct CreateIssueArgs {
    /// Project ID or Key
    #[arg(short, long)]
    project_id: String,

    /// Issue summary (title)
    #[arg(short, long)]
    summary: String,

    /// Issue type ID
    #[arg(short = 't', long)]
    issue_type_id: u32,

    /// Priority ID
    #[arg(long)]
    priority_id: u32,

    /// Issue description
    #[arg(short, long)]
    description: Option<String>,

    /// Assignee user ID
    #[arg(short, long)]
    assignee_id: Option<u32>,

    /// Due date (YYYY-MM-DD format)
    #[arg(long)]
    due_date: Option<String>,

    /// Category IDs (comma-separated)
    #[arg(short, long)]
    category_ids: Option<String>,

    /// Milestone IDs (comma-separated)
    #[arg(short, long)]
    milestone_ids: Option<String>,

    /// Custom fields (format: "id:type:value[:other]", can be specified multiple times)
    /// Examples:
    /// --custom-field "1:text:Sample text"
    /// --custom-field "2:numeric:123.45"
    /// --custom-field "3:date:2024-06-24"
    /// --custom-field "4:single_list:100:Other description"
    #[arg(long = "custom-field", value_name = "FIELD")]
    custom_fields: Vec<String>,

    /// Custom fields JSON file path
    /// Expected format:
    /// {
    ///   "1": {"type": "text", "value": "Sample text"},
    ///   "2": {"type": "numeric", "value": 123.45}
    /// }
    #[arg(
        long = "custom-fields-json",
        value_name = "FILE",
        conflicts_with = "custom_fields"
    )]
    custom_fields_json: Option<std::path::PathBuf>,
}

#[derive(Args, Debug)]
struct UpdateIssueArgs {
    /// Issue ID or Key
    issue_id_or_key: String,

    /// Issue summary (title)
    #[arg(short, long)]
    summary: Option<String>,

    /// Issue description
    #[arg(short, long)]
    description: Option<String>,

    /// Issue type ID
    #[arg(short = 't', long)]
    issue_type_id: Option<u32>,

    /// Priority ID
    #[arg(long)]
    priority_id: Option<u32>,

    /// Status ID
    #[arg(long)]
    status_id: Option<String>,

    /// Assignee user ID
    #[arg(short, long)]
    assignee_id: Option<u32>,

    /// Resolution ID
    #[arg(short, long)]
    resolution_id: Option<u32>,

    /// Due date (YYYY-MM-DD format)
    #[arg(long)]
    due_date: Option<String>,

    /// Comment to add with the update
    #[arg(short, long)]
    comment: Option<String>,

    /// Custom fields (format: "id:type:value[:other]", can be specified multiple times)
    #[arg(long = "custom-field", value_name = "FIELD")]
    custom_fields: Vec<String>,

    /// Custom fields JSON file path
    #[arg(
        long = "custom-fields-json",
        value_name = "FILE",
        conflicts_with = "custom_fields"
    )]
    custom_fields_json: Option<std::path::PathBuf>,
}

#[derive(Args, Debug)]
struct DeleteIssueArgs {
    /// Issue Key (e.g., "PROJECT-123")
    issue_key: String,
}

#[derive(Args, Debug)]
struct CountCommentArgs {
    /// The ID or key of the issue (e.g., "PROJECT-123" or "12345")
    issue_id_or_key: String,
}

#[derive(Args, Debug)]
struct GetCommentArgs {
    /// The ID or key of the issue (e.g., "PROJECT-123" or "12345")
    issue_id_or_key: String,
    /// The ID of the comment
    comment_id: u32,
}

#[cfg(feature = "issue_writable")]
#[derive(Args, Debug)]
struct AddCommentNotificationArgs {
    /// The ID or key of the issue (e.g., "PROJECT-123" or "12345")
    issue_id_or_key: String,
    /// The ID of the comment
    comment_id: u32,
    /// User IDs to notify (comma-separated, e.g., "123,456")
    #[arg(short, long)]
    users: String,
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

#[derive(Parser)]
struct ProjectArgs {
    #[clap(subcommand)]
    command: ProjectCommands,
}

#[derive(Parser)]
enum ProjectCommands {
    /// List all projects
    List,
    /// Show details of a specific project
    Show {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
    },
    /// Add a new project
    #[cfg(feature = "project_writable")]
    Add {
        /// Project name
        #[clap(short, long)]
        name: String,
        /// Project key (uppercase letters, numbers, underscores only)
        #[clap(short, long)]
        key: String,
        /// Enable/disable charts
        #[clap(long)]
        chart_enabled: Option<bool>,
        /// Use resolved for chart
        #[clap(long)]
        use_resolved_for_chart: Option<bool>,
        /// Enable/disable subtasking
        #[clap(long)]
        subtasking_enabled: Option<bool>,
        /// Project leader can edit project leader
        #[clap(long)]
        project_leader_can_edit_project_leader: Option<bool>,
        /// Enable/disable Wiki
        #[clap(long)]
        use_wiki: Option<bool>,
        /// Enable/disable file sharing
        #[clap(long)]
        use_file_sharing: Option<bool>,
        /// Enable/disable Wiki tree view
        #[clap(long)]
        use_wiki_tree_view: Option<bool>,
        /// Enable/disable Subversion
        #[clap(long)]
        use_subversion: Option<bool>,
        /// Enable/disable Git
        #[clap(long)]
        use_git: Option<bool>,
        /// Use original image size at Wiki
        #[clap(long)]
        use_original_image_size_at_wiki: Option<bool>,
        /// Text formatting rule (backlog or markdown)
        #[clap(long)]
        text_formatting_rule: Option<String>,
        /// Use dev attributes
        #[clap(long)]
        use_dev_attributes: Option<bool>,
    },
    /// Update project settings
    #[cfg(feature = "project_writable")]
    Update {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
        /// Update project name
        #[clap(long)]
        name: Option<String>,
        /// Update project key
        #[clap(long)]
        key: Option<String>,
        /// Enable/disable charts
        #[clap(long)]
        chart_enabled: Option<bool>,
        /// Use resolved for chart
        #[clap(long)]
        use_resolved_for_chart: Option<bool>,
        /// Enable/disable subtasking
        #[clap(long)]
        subtasking_enabled: Option<bool>,
        /// Project leader can edit project leader
        #[clap(long)]
        project_leader_can_edit_project_leader: Option<bool>,
        /// Enable/disable Wiki
        #[clap(long)]
        use_wiki: Option<bool>,
        /// Enable/disable file sharing
        #[clap(long)]
        use_file_sharing: Option<bool>,
        /// Enable/disable Wiki tree view
        #[clap(long)]
        use_wiki_tree_view: Option<bool>,
        /// Enable/disable Subversion
        #[clap(long)]
        use_subversion: Option<bool>,
        /// Enable/disable Git
        #[clap(long)]
        use_git: Option<bool>,
        /// Use original image size at Wiki
        #[clap(long)]
        use_original_image_size_at_wiki: Option<bool>,
        /// Text formatting rule (backlog or markdown)
        #[clap(long)]
        text_formatting_rule: Option<String>,
        /// Archive/unarchive project
        #[clap(long)]
        archived: Option<bool>,
        /// Use dev attributes
        #[clap(long)]
        use_dev_attributes: Option<bool>,
    },
    /// Delete a project
    #[cfg(feature = "project_writable")]
    Delete {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
    },
    /// List recently viewed projects
    RecentlyViewed {
        /// Sort order ("asc" or "desc", default: "desc")
        #[clap(long)]
        order: Option<String>,
        /// Number of results to return (1-100, default: 20)
        #[clap(long)]
        count: Option<u32>,
        /// Offset for pagination
        #[clap(long)]
        offset: Option<u32>,
    },
    /// List statuses for a project
    StatusList {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
    },
    /// List milestones for a project
    MilestoneList {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
    },
    /// List issue types for a project
    IssueTypeList {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
    },
    /// List categories for a project
    CategoryList {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
    },
    /// Get disk usage for a project
    DiskUsage {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
        /// Show sizes in human-readable format (e.g., 1.2GB)
        #[clap(short = 'H', long)]
        human_readable: bool,
    },
    /// List users for a project
    UserList {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
    },
    /// List administrators for a project
    AdminList {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
    },
    /// Add a user as a project administrator
    #[cfg(feature = "project_writable")]
    AdminAdd {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
        /// User ID to add as administrator
        #[clap(short, long)]
        user_id: u32,
    },
    /// Remove an administrator from a project
    #[cfg(feature = "project_writable")]
    AdminRemove {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
        /// User ID to remove as administrator
        #[clap(short, long)]
        user_id: u32,
    },
    /// Add a user to a project
    #[cfg(feature = "project_writable")]
    UserAdd {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
        /// User ID to add
        #[clap(short, long)]
        user_id: u32,
    },
    /// Remove a user from a project
    #[cfg(feature = "project_writable")]
    UserRemove {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
        /// User ID to remove
        #[clap(short, long)]
        user_id: u32,
    },
    /// List custom fields for a project
    CustomFieldList {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
    },
    /// Update a custom field in a project
    #[cfg(feature = "project_writable")]
    CustomFieldUpdate {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
        /// Custom field ID
        #[clap(long)]
        custom_field_id: u32,
        /// Update the name
        #[clap(long)]
        name: Option<String>,
        /// Update the description
        #[clap(long)]
        description: Option<String>,
        /// Update whether field is required
        #[clap(long)]
        required: Option<bool>,
        /// Update applicable issue types (comma-separated IDs)
        #[clap(long)]
        applicable_issue_types: Option<String>,
        // Date field specific parameters
        /// Minimum date (YYYY-MM-DD format)
        #[clap(long)]
        min_date: Option<String>,
        /// Maximum date (YYYY-MM-DD format)
        #[clap(long)]
        max_date: Option<String>,
        /// Initial value type (1-7)
        #[clap(long)]
        initial_value_type: Option<i32>,
        /// Initial date (YYYY-MM-DD format)
        #[clap(long)]
        initial_date: Option<String>,
        /// Initial shift (days from current date)
        #[clap(long)]
        initial_shift: Option<i32>,
    },
    /// Add a custom field to a project
    #[cfg(feature = "project_writable")]
    CustomFieldAdd {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
        /// Field type (text, textarea, numeric, date, single-list, multiple-list, checkbox, radio)
        #[clap(short = 't', long)]
        field_type: String,
        /// Field name
        #[clap(short, long)]
        name: String,
        /// Field description (optional)
        #[clap(short = 'd', long)]
        description: Option<String>,
        /// Make field required (optional)
        #[clap(short = 'r', long)]
        required: Option<bool>,
        /// Applicable issue type IDs (comma-separated)
        #[clap(long)]
        applicable_issue_types: Option<String>,
        // Numeric field parameters
        /// Minimum value (for numeric fields)
        #[clap(long)]
        min: Option<f64>,
        /// Maximum value (for numeric fields)
        #[clap(long)]
        max: Option<f64>,
        /// Initial value (for numeric fields)
        #[clap(long)]
        initial_value: Option<f64>,
        /// Unit (for numeric fields)
        #[clap(long)]
        unit: Option<String>,
        // Date field parameters
        /// Minimum date (YYYY-MM-DD format)
        #[clap(long)]
        min_date: Option<String>,
        /// Maximum date (YYYY-MM-DD format)
        #[clap(long)]
        max_date: Option<String>,
        /// Initial value type (1-3)
        #[clap(long)]
        initial_value_type: Option<i32>,
        /// Initial date (YYYY-MM-DD format)
        #[clap(long)]
        initial_date: Option<String>,
        /// Initial shift (days from current date)
        #[clap(long)]
        initial_shift: Option<i32>,
        // List field parameters
        /// List items (comma-separated)
        #[clap(long)]
        items: Option<String>,
        /// Allow direct input
        #[clap(long)]
        allow_input: Option<bool>,
        /// Allow adding new items
        #[clap(long)]
        allow_add_item: Option<bool>,
    },
    /// Delete a custom field from a project
    #[cfg(feature = "project_writable")]
    CustomFieldDelete {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
        /// Custom Field ID
        #[clap(short, long)]
        custom_field_id: u32,
    },
    /// Add a list item to a list type custom field
    #[cfg(feature = "project_writable")]
    CustomFieldAddItem {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
        /// Custom Field ID
        #[clap(short, long)]
        custom_field_id: u32,
        /// Name of the new list item
        #[clap(short, long)]
        name: String,
    },
    /// Update a list item in a list type custom field
    #[cfg(feature = "project_writable")]
    CustomFieldUpdateItem {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
        /// Custom Field ID
        #[clap(short, long)]
        custom_field_id: u32,
        /// List Item ID
        #[clap(short, long)]
        item_id: u32,
        /// New name for the list item
        #[clap(short, long)]
        name: String,
    },
    /// Delete a list item from a list type custom field
    #[cfg(feature = "project_writable")]
    CustomFieldDeleteItem {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
        /// Custom Field ID
        #[clap(short, long)]
        custom_field_id: u32,
        /// List Item ID to delete
        #[clap(short, long)]
        item_id: u32,
    },
    /// Add a category to a project
    #[cfg(feature = "project_writable")]
    CategoryAdd {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
        /// Category name
        #[clap(short, long)]
        name: String,
    },
    /// Update a category in a project
    #[cfg(feature = "project_writable")]
    CategoryUpdate {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
        /// Category ID
        #[clap(short, long)]
        category_id: u32,
        /// New category name
        #[clap(short, long)]
        name: String,
    },
    /// Delete a category from a project
    #[cfg(feature = "project_writable")]
    CategoryDelete {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
        /// Category ID
        #[clap(short, long)]
        category_id: u32,
    },
    /// Add an issue type to a project
    #[cfg(feature = "project_writable")]
    IssueTypeAdd {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
        /// Issue type name
        #[clap(short, long)]
        name: String,
        /// Issue type color (available: red, dark-red, purple, violet, blue, teal, green, orange, pink, gray)
        #[clap(short, long)]
        color: String,
        /// Template summary (optional)
        #[clap(short = 's', long)]
        template_summary: Option<String>,
        /// Template description (optional)
        #[clap(short = 'd', long)]
        template_description: Option<String>,
    },
    /// Delete an issue type from a project
    #[cfg(feature = "project_writable")]
    IssueTypeDelete {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
        /// Issue type ID to delete
        #[clap(long)]
        issue_type_id: u32,
        /// Substitute issue type ID for existing issues
        #[clap(long)]
        substitute_issue_type_id: u32,
    },
    /// Update an issue type in a project
    #[cfg(feature = "project_writable")]
    IssueTypeUpdate {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
        /// Issue type ID to update
        #[clap(long)]
        issue_type_id: u32,
        /// New issue type name (optional)
        #[clap(short, long)]
        name: Option<String>,
        /// New issue type color (optional: red, dark-red, purple, violet, blue, teal, green, orange, pink, gray)
        #[clap(short, long)]
        color: Option<String>,
        /// New template summary (optional)
        #[clap(short = 's', long)]
        template_summary: Option<String>,
        /// New template description (optional)
        #[clap(short = 'd', long)]
        template_description: Option<String>,
    },
    /// List priorities (space-wide)
    PriorityList,
    /// List resolutions (space-wide)
    ResolutionList,
    /// Add a version/milestone to a project
    #[cfg(feature = "project_writable")]
    VersionAdd {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
        /// Version name
        #[clap(short, long)]
        name: String,
        /// Version description
        #[clap(short, long)]
        description: Option<String>,
        /// Start date (YYYY-MM-DD)
        #[clap(long)]
        start_date: Option<String>,
        /// Release due date (YYYY-MM-DD)
        #[clap(long)]
        release_due_date: Option<String>,
    },
    /// Update a version/milestone in a project
    #[cfg(feature = "project_writable")]
    VersionUpdate {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
        /// Version ID to update
        #[clap(long)]
        version_id: u32,
        /// New version name
        #[clap(short, long)]
        name: String,
        /// New version description
        #[clap(short, long)]
        description: Option<String>,
        /// New start date (YYYY-MM-DD)
        #[clap(long)]
        start_date: Option<String>,
        /// New release due date (YYYY-MM-DD)
        #[clap(long)]
        release_due_date: Option<String>,
        /// Archive the version
        #[clap(long)]
        archived: Option<bool>,
    },
    /// Delete a version/milestone from a project
    #[cfg(feature = "project_writable")]
    VersionDelete {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
        /// Version ID to delete
        #[clap(long)]
        version_id: u32,
    },
    /// Add a status to a project
    #[cfg(feature = "project_writable")]
    StatusAdd {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
        /// Status name
        #[clap(short, long)]
        name: String,
        /// Status color (red, coral, pink, light-purple, blue, green, light-green, orange, magenta, dark-gray)
        #[clap(short, long)]
        color: String,
    },
    /// Update a status in a project
    #[cfg(feature = "project_writable")]
    StatusUpdate {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
        /// Status ID to update
        #[clap(long)]
        status_id: u32,
        /// New status name (optional)
        #[clap(short, long)]
        name: Option<String>,
        /// New status color (optional: red, coral, pink, light-purple, blue, green, light-green, orange, magenta, dark-gray)
        #[clap(short, long)]
        color: Option<String>,
    },
    /// Delete a status from a project
    #[cfg(feature = "project_writable")]
    StatusDelete {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
        /// Status ID to delete
        #[clap(long)]
        status_id: u32,
        /// Substitute status ID for existing issues
        #[clap(long)]
        substitute_status_id: u32,
    },
    /// Update the display order of statuses in a project
    #[cfg(feature = "project_writable")]
    StatusOrderUpdate {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
        /// Status IDs in desired display order (comma-separated)
        #[clap(long)]
        status_ids: String,
    },
    /// Download project icon
    Icon {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
        /// Output file path to save the icon
        #[clap(short, long, value_name = "FILE_PATH")]
        output: PathBuf,
    },
    /// List teams for a project
    TeamList {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
    },
    /// Add a team to a project
    #[cfg(feature = "project_writable")]
    TeamAdd {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
        /// Team ID to add
        #[clap(short, long)]
        team_id: u32,
    },
    /// Remove a team from a project
    #[cfg(feature = "project_writable")]
    TeamDelete {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
        /// Team ID to remove
        #[clap(short, long)]
        team_id: u32,
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

#[cfg(feature = "wiki")]
#[derive(Parser)]
struct WikiArgs {
    #[clap(subcommand)]
    command: WikiCommands,
}

#[cfg(feature = "wiki")]
#[derive(Parser)]
enum WikiCommands {
    /// List recently viewed wikis
    RecentlyViewed {
        /// Sort order (asc or desc)
        #[clap(short, long)]
        order: Option<String>,
        /// Number of items to retrieve (1-100)
        #[clap(short, long)]
        count: Option<u32>,
        /// Offset for pagination
        #[clap(short = 'O', long)]
        offset: Option<u32>,
    },
    /// List attachments for a wiki page
    ListAttachments {
        /// Wiki ID
        #[clap(name = "WIKI_ID")]
        wiki_id: u32,
    },
    /// List shared files linked to a wiki page
    ListSharedFiles {
        /// Wiki ID
        #[clap(name = "WIKI_ID")]
        wiki_id: u32,
    },
    /// List stars for a wiki page
    Stars {
        /// Wiki ID
        #[clap(name = "WIKI_ID")]
        wiki_id: u32,
    },
    /// Link shared files to a wiki page
    #[cfg(feature = "wiki_writable")]
    LinkSharedFiles {
        /// Wiki ID
        #[clap(name = "WIKI_ID")]
        wiki_id: u32,
        /// Shared file IDs (comma-separated)
        #[clap(name = "FILE_IDS", value_delimiter = ',')]
        file_ids: Vec<u32>,
    },
    /// Unlink a shared file from a wiki page
    #[cfg(feature = "wiki_writable")]
    UnlinkSharedFile {
        /// Wiki ID
        #[clap(name = "WIKI_ID")]
        wiki_id: u32,
        /// Shared file ID
        #[clap(name = "FILE_ID")]
        file_id: u32,
    },
    /// Download an attachment from a wiki page
    DownloadAttachment {
        /// Wiki ID
        #[clap(name = "WIKI_ID")]
        wiki_id: u32,
        /// Attachment ID
        #[clap(name = "ATTACHMENT_ID")]
        attachment_id: u32,
        /// Output file path (if not specified, use original filename)
        #[clap(short, long)]
        output: Option<String>,
    },
    /// Create a new wiki page
    #[cfg(feature = "wiki_writable")]
    Create {
        /// Project ID
        #[clap(long)]
        project_id: String,
        /// Wiki page name
        #[clap(long)]
        name: String,
        /// Wiki page content
        #[clap(long)]
        content: String,
        /// Send email notification
        #[clap(long)]
        mail_notify: Option<bool>,
    },
    /// Update a wiki page
    #[cfg(feature = "wiki_writable")]
    Update {
        /// Wiki ID
        #[clap(name = "WIKI_ID")]
        wiki_id: u32,
        /// New wiki page name
        #[clap(long)]
        name: Option<String>,
        /// New wiki page content
        #[clap(long)]
        content: Option<String>,
        /// Send email notification of update
        #[clap(long)]
        mail_notify: Option<bool>,
    },
    /// Delete a wiki page
    #[cfg(feature = "wiki_writable")]
    Delete {
        /// Wiki ID
        #[clap(name = "WIKI_ID")]
        wiki_id: u32,
        /// Send email notification of deletion
        #[clap(long)]
        mail_notify: Option<bool>,
    },
    /// Attach file to a wiki page
    #[cfg(feature = "wiki_writable")]
    AttachFile {
        /// Wiki ID
        #[clap(name = "WIKI_ID")]
        wiki_id: u32,
        /// File path to attach
        #[clap(long)]
        file_path: PathBuf,
    },
    /// Delete an attachment from a wiki page
    #[cfg(feature = "wiki_writable")]
    DeleteAttachment {
        /// Wiki ID
        #[clap(name = "WIKI_ID")]
        wiki_id: u32,
        /// Attachment ID to delete
        #[clap(name = "ATTACHMENT_ID")]
        attachment_id: u32,
        /// Force deletion without confirmation
        #[clap(long, short = 'f')]
        force: bool,
    },
    /// List tags used in wiki pages
    ListTags {
        /// Project ID or Key
        #[clap(short, long)]
        project_id: String,
    },
    /// Get history of a wiki page
    History {
        /// Wiki ID
        #[clap(name = "WIKI_ID")]
        wiki_id: u32,
        /// Minimum ID for history entries
        #[clap(long)]
        min_id: Option<u32>,
        /// Maximum ID for history entries
        #[clap(long)]
        max_id: Option<u32>,
        /// Maximum number of history entries to retrieve (1-100)
        #[clap(long)]
        count: Option<u32>,
        /// Sort order for history entries
        #[clap(long, value_enum)]
        order: Option<HistoryOrderCli>,
    },
}

#[cfg(feature = "wiki")]
#[derive(Clone, clap::ValueEnum)]
enum HistoryOrderCli {
    Asc,
    Desc,
}

#[derive(Parser, Debug, Default)]
struct IssueListCliParams {
    /// Filter by project ID(s)
    #[clap(long)]
    project_id: Option<Vec<String>>,
    /// Filter by assignee ID(s)
    #[clap(long)]
    assignee_id: Option<Vec<String>>,
    /// Filter by status ID(s)
    #[clap(long)]
    status_id: Option<Vec<String>>,
    /// Keyword to search for in summary or description
    #[clap(long)]
    keyword: Option<String>,
    /// Number of issues to retrieve (1-100, default 20)
    #[clap(long, default_value_t = 20)]
    count: u32,
    /// Filter by start date (since). Format: YYYY-MM-DD
    #[clap(long)]
    start_date_since: Option<String>,
    /// Filter by start date (until). Format: YYYY-MM-DD
    #[clap(long)]
    start_date_until: Option<String>,
    /// Filter by due date (since). Format: YYYY-MM-DD
    #[clap(long)]
    due_date_since: Option<String>,
    /// Filter by due date (until). Format: YYYY-MM-DD
    #[clap(long)]
    due_date_until: Option<String>,
    // TODO: Add more filters like sort, order, offset, issue_type_id, etc.
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

fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", size as u64, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
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
        Commands::Issue(issue_args) => match issue_args.command {
            IssueCommands::Show { issue_id_or_key } => {
                println!("Showing issue: {issue_id_or_key}");
                let parsed_issue_id_or_key = issue_id_or_key.parse::<IssueIdOrKey>()?;
                let issue = client
                    .issue()
                    .get_issue(backlog_issue::GetIssueParams::new(parsed_issue_id_or_key))
                    .await?;
                // TODO: Pretty print issue
                println!("{issue:?}");
            }
            IssueCommands::List { params } => {
                println!("Listing issues with params: {params:?}");
                let mut builder = GetIssueListParamsBuilder::default();

                if let Some(p_ids) = params.project_id {
                    let parsed_ids: std::result::Result<Vec<ProjectId>, _> = p_ids
                        .iter()
                        .map(|s| s.parse::<u32>().map(ProjectId::from))
                        .collect();
                    builder.project_id(parsed_ids?);
                }
                if let Some(a_ids) = params.assignee_id {
                    let parsed_ids: std::result::Result<Vec<UserId>, _> = a_ids
                        .iter()
                        .map(|s| s.parse::<u32>().map(UserId::from))
                        .collect();
                    builder.assignee_id(parsed_ids?);
                }
                if let Some(s_ids) = params.status_id {
                    let parsed_ids: std::result::Result<Vec<StatusId>, _> = s_ids
                        .iter()
                        .map(|s| s.parse::<u32>().map(StatusId::from))
                        .collect();
                    builder.status_id(parsed_ids?);
                }
                if let Some(keyword) = params.keyword {
                    builder.keyword(keyword);
                }
                builder.count(params.count); // count has a default_value_t

                // Handle date range parameters
                if let Some(start_date_since) = params.start_date_since {
                    let date =
                        NaiveDate::parse_from_str(&start_date_since, "%Y-%m-%d").map_err(|_| {
                            format!("Invalid start-date-since format: {start_date_since}")
                        })?;
                    let datetime = date.and_hms_opt(0, 0, 0).unwrap().and_utc();
                    builder.start_date_since(ApiDate::from(datetime));
                }
                if let Some(start_date_until) = params.start_date_until {
                    let date =
                        NaiveDate::parse_from_str(&start_date_until, "%Y-%m-%d").map_err(|_| {
                            format!("Invalid start-date-until format: {start_date_until}")
                        })?;
                    let datetime = date.and_hms_opt(23, 59, 59).unwrap().and_utc();
                    builder.start_date_until(ApiDate::from(datetime));
                }
                if let Some(due_date_since) = params.due_date_since {
                    let date = NaiveDate::parse_from_str(&due_date_since, "%Y-%m-%d")
                        .map_err(|_| format!("Invalid due-date-since format: {due_date_since}"))?;
                    let datetime = date.and_hms_opt(0, 0, 0).unwrap().and_utc();
                    builder.due_date_since(ApiDate::from(datetime));
                }
                if let Some(due_date_until) = params.due_date_until {
                    let date = NaiveDate::parse_from_str(&due_date_until, "%Y-%m-%d")
                        .map_err(|_| format!("Invalid due-date-until format: {due_date_until}"))?;
                    let datetime = date.and_hms_opt(23, 59, 59).unwrap().and_utc();
                    builder.due_date_until(ApiDate::from(datetime));
                }

                let list_params = builder.build()?;
                let issues = client.issue().get_issue_list(list_params).await?;
                // TODO: Pretty print issues
                println!("{issues:?}");
            }
            IssueCommands::DownloadAttachment(dl_args) => {
                println!(
                    "Downloading attachment {} for issue {} to {}",
                    dl_args.attachment_id,
                    dl_args.issue_id_or_key,
                    dl_args.output.display()
                );

                let parsed_issue_id_or_key = IssueIdOrKey::from_str(&dl_args.issue_id_or_key)
                    .map_err(|e| {
                        format!(
                            "Failed to parse issue_id_or_key '{}': {}",
                            dl_args.issue_id_or_key, e
                        )
                    })?;

                let parsed_attachment_id = AttachmentId::new(dl_args.attachment_id);

                let params = backlog_issue::GetAttachmentFileParams::new(
                    parsed_issue_id_or_key,
                    parsed_attachment_id,
                );
                match client.issue().get_attachment_file(params).await {
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
                        eprintln!("Error downloading attachment: {e}");
                    }
                }
            }
            #[cfg(feature = "issue_writable")]
            IssueCommands::AddComment(add_args) => {
                println!(
                    "Adding comment to issue {}: {}",
                    add_args.issue_id_or_key, add_args.content
                );

                let parsed_issue_id_or_key = IssueIdOrKey::from_str(&add_args.issue_id_or_key)
                    .map_err(|e| {
                        format!(
                            "Failed to parse issue_id_or_key '{}': {}",
                            add_args.issue_id_or_key, e
                        )
                    })?;

                let mut builder = AddCommentParamsBuilder::default();
                builder.issue_id_or_key(parsed_issue_id_or_key);
                builder.content(&add_args.content);

                // Parse notify_users if provided
                if let Some(notify_str) = &add_args.notify_users {
                    let user_ids: Result<Vec<UserId>, _> = notify_str
                        .split(',')
                        .map(|s| s.trim().parse::<u32>().map(UserId::new))
                        .collect();
                    match user_ids {
                        Ok(ids) => builder.notified_user_id(ids),
                        Err(e) => {
                            eprintln!("Error parsing notify_users '{notify_str}': {e}");
                            return Ok(());
                        }
                    };
                }

                // Parse attachments if provided
                if let Some(attach_str) = &add_args.attachments {
                    let attachment_ids: Result<Vec<AttachmentId>, _> = attach_str
                        .split(',')
                        .map(|s| s.trim().parse::<u32>().map(AttachmentId::new))
                        .collect();
                    match attachment_ids {
                        Ok(ids) => builder.attachment_id(ids),
                        Err(e) => {
                            eprintln!("Error parsing attachments '{attach_str}': {e}");
                            return Ok(());
                        }
                    };
                }

                let params = builder.build()?;

                match client.issue().add_comment(params).await {
                    Ok(comment) => {
                        println!("Comment added successfully!");
                        println!("Comment ID: {}", comment.id);
                        println!("Created by: {}", comment.created_user.name);
                        println!("Created at: {}", comment.created);
                        if let Some(content) = &comment.content {
                            println!("Content: {content}");
                        }
                    }
                    Err(e) => {
                        eprintln!("Error adding comment: {e}");
                    }
                }
            }
            #[cfg(feature = "issue_writable")]
            IssueCommands::UpdateComment(args) => {
                use backlog_core::identifier::CommentId;
                use backlog_issue::UpdateCommentParams;

                let params = UpdateCommentParams {
                    issue_id_or_key: args.issue_id.parse::<IssueKey>()?.into(),
                    comment_id: CommentId::new(args.comment_id),
                    content: args.content,
                };

                match client.issue().update_comment(params).await {
                    Ok(comment) => {
                        println!("✅ Comment updated successfully");
                        println!("Comment ID: {}", comment.id);
                        println!("Content: {}", comment.content.unwrap_or_default());
                        println!("Updated: {}", comment.updated);
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to update comment: {e}");
                        std::process::exit(1);
                    }
                }
            }
            #[cfg(feature = "issue_writable")]
            IssueCommands::DeleteComment(args) => {
                use backlog_core::identifier::CommentId;

                let params = DeleteCommentParams {
                    issue_id_or_key: args.issue_id.parse::<IssueKey>()?.into(),
                    comment_id: CommentId::new(args.comment_id),
                };

                match client.issue().delete_comment(params).await {
                    Ok(comment) => {
                        println!("✅ Comment deleted successfully");
                        println!("Deleted Comment ID: {}", comment.id);
                        println!("Deleted Content: {}", comment.content.unwrap_or_default());
                        println!("Originally Created: {}", comment.created);
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to delete comment: {e}");
                        std::process::exit(1);
                    }
                }
            }
            #[cfg(feature = "issue_writable")]
            IssueCommands::DeleteAttachment(args) => {
                let params = DeleteAttachmentParams {
                    issue_id_or_key: args.issue_id.parse::<IssueKey>()?.into(),
                    attachment_id: AttachmentId::new(args.attachment_id),
                };

                match client.issue().delete_attachment(params).await {
                    Ok(attachment) => {
                        println!("✅ Attachment deleted successfully");
                        println!("Deleted Attachment ID: {}", attachment.id);
                        println!("Deleted File Name: {}", attachment.name);
                        println!("File Size: {} bytes", attachment.size);
                        println!("Originally Created: {}", attachment.created);
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to delete attachment: {e}");
                        std::process::exit(1);
                    }
                }
            }
            #[cfg(feature = "issue_writable")]
            #[cfg(feature = "issue_writable")]
            IssueCommands::Create(create_args) => {
                println!("Creating new issue...");

                let project_id_or_key = create_args.project_id.parse::<ProjectIdOrKey>()?;
                let project_id = match project_id_or_key {
                    ProjectIdOrKey::Id(id) => id,
                    ProjectIdOrKey::Key(_) => {
                        eprintln!(
                            "Error: Project key not supported for issue creation. Please use project ID."
                        );
                        return Ok(());
                    }
                    ProjectIdOrKey::EitherIdOrKey(id, _) => id,
                };

                let mut builder = AddIssueParamsBuilder::default();
                builder
                    .project_id(project_id)
                    .summary(&create_args.summary)
                    .issue_type_id(IssueTypeId::new(create_args.issue_type_id))
                    .priority_id(PriorityId::new(create_args.priority_id));

                if let Some(description) = &create_args.description {
                    builder.description(description);
                }

                if let Some(assignee_id) = create_args.assignee_id {
                    builder.assignee_id(UserId::new(assignee_id));
                }

                if let Some(_due_date) = &create_args.due_date {
                    // Due date parsing would need proper DateTime conversion
                    // For now, skip this implementation detail
                }

                if let Some(category_str) = &create_args.category_ids {
                    let category_ids: Result<Vec<CategoryId>, _> = category_str
                        .split(',')
                        .map(|s| s.trim().parse::<u32>().map(CategoryId::new))
                        .collect();
                    if let Ok(ids) = category_ids {
                        builder.category_id(ids);
                    }
                }

                if let Some(milestone_str) = &create_args.milestone_ids {
                    let milestone_ids: Result<Vec<MilestoneId>, _> = milestone_str
                        .split(',')
                        .map(|s| s.trim().parse::<u32>().map(MilestoneId::new))
                        .collect();
                    if let Ok(ids) = milestone_ids {
                        builder.milestone_id(ids);
                    }
                }

                // Handle custom fields
                let custom_fields_map = if let Some(json_path) = &create_args.custom_fields_json {
                    match custom_fields::parse_custom_fields_json(json_path.to_str().unwrap()) {
                        Ok(fields) => Some(fields),
                        Err(e) => {
                            eprintln!("Error parsing custom fields JSON: {e}");
                            return Ok(());
                        }
                    }
                } else if !create_args.custom_fields.is_empty() {
                    match custom_fields::parse_custom_field_args(&create_args.custom_fields) {
                        Ok(fields) => Some(fields),
                        Err(e) => {
                            eprintln!("Error parsing custom fields: {e}");
                            return Ok(());
                        }
                    }
                } else {
                    None
                };

                if let Some(fields) = custom_fields_map {
                    builder.custom_fields(fields);
                }

                let params = builder.build()?;

                match client.issue().add_issue(params).await {
                    Ok(issue) => {
                        println!("Issue created successfully!");
                        println!("Issue Key: {}", issue.issue_key);
                        println!("Issue ID: {}", issue.id);
                        println!("Summary: {}", issue.summary);
                        println!("Status: {}", issue.status.name);
                    }
                    Err(e) => {
                        eprintln!("Error creating issue: {e}");
                    }
                }
            }
            #[cfg(feature = "issue_writable")]
            #[cfg(feature = "issue_writable")]
            IssueCommands::Update(update_args) => {
                println!("Updating issue: {}", update_args.issue_id_or_key);

                let issue_id_or_key = update_args.issue_id_or_key.parse::<IssueIdOrKey>()?;

                let mut builder = UpdateIssueParamsBuilder::default();
                builder.issue_id_or_key(issue_id_or_key);

                if let Some(summary) = &update_args.summary {
                    builder.summary(summary);
                }

                if let Some(description) = &update_args.description {
                    builder.description(description);
                }

                if let Some(issue_type_id) = update_args.issue_type_id {
                    builder.issue_type_id(IssueTypeId::new(issue_type_id));
                }

                if let Some(priority_id) = update_args.priority_id {
                    builder.priority_id(PriorityId::new(priority_id));
                }

                if let Some(status_id) = &update_args.status_id {
                    builder.status_id(status_id);
                }

                if let Some(assignee_id) = update_args.assignee_id {
                    builder.assignee_id(UserId::new(assignee_id));
                }

                if let Some(resolution_id) = update_args.resolution_id {
                    builder.resolution_id(ResolutionId::new(resolution_id));
                }

                if let Some(comment) = &update_args.comment {
                    builder.comment(comment);
                }

                // Handle custom fields
                let custom_fields_map = if let Some(json_path) = &update_args.custom_fields_json {
                    match custom_fields::parse_custom_fields_json(json_path.to_str().unwrap()) {
                        Ok(fields) => Some(fields),
                        Err(e) => {
                            eprintln!("Error parsing custom fields JSON: {e}");
                            return Ok(());
                        }
                    }
                } else if !update_args.custom_fields.is_empty() {
                    match custom_fields::parse_custom_field_args(&update_args.custom_fields) {
                        Ok(fields) => Some(fields),
                        Err(e) => {
                            eprintln!("Error parsing custom fields: {e}");
                            return Ok(());
                        }
                    }
                } else {
                    None
                };

                if let Some(fields) = custom_fields_map {
                    builder.custom_fields(fields);
                }

                let params = builder.build()?;

                match client.issue().update_issue(params).await {
                    Ok(issue) => {
                        println!("Issue updated successfully!");
                        println!("Issue Key: {}", issue.issue_key);
                        println!("Summary: {}", issue.summary);
                        println!("Status: {}", issue.status.name);
                    }
                    Err(e) => {
                        eprintln!("Error updating issue: {e}");
                    }
                }
            }
            #[cfg(feature = "issue_writable")]
            #[cfg(feature = "issue_writable")]
            IssueCommands::Delete(delete_args) => {
                println!("Deleting issue: {}", delete_args.issue_key);

                let issue_key = delete_args.issue_key.parse::<IssueKey>()?;

                match client
                    .issue()
                    .delete_issue(backlog_issue::DeleteIssueParams::new(issue_key))
                    .await
                {
                    Ok(issue) => {
                        println!("Issue deleted successfully!");
                        println!("Deleted Issue Key: {}", issue.issue_key);
                        println!("Summary: {}", issue.summary);
                    }
                    Err(e) => {
                        eprintln!("Error deleting issue: {e}");
                    }
                }
            }
            IssueCommands::CountComment(count_args) => {
                println!(
                    "Counting comments for issue: {}",
                    count_args.issue_id_or_key
                );

                let parsed_issue_id_or_key = IssueIdOrKey::from_str(&count_args.issue_id_or_key)
                    .map_err(|e| {
                        format!(
                            "Failed to parse issue_id_or_key '{}': {}",
                            count_args.issue_id_or_key, e
                        )
                    })?;

                match client
                    .issue()
                    .count_comment(backlog_issue::CountCommentParams::new(
                        parsed_issue_id_or_key,
                    ))
                    .await
                {
                    Ok(response) => {
                        println!(
                            "Comment count for issue {}: {}",
                            count_args.issue_id_or_key, response.count
                        );
                    }
                    Err(e) => {
                        eprintln!("Error counting comments: {e}");
                    }
                }
            }
            IssueCommands::GetComment(get_args) => {
                println!(
                    "Getting comment {} for issue: {}",
                    get_args.comment_id, get_args.issue_id_or_key
                );

                let parsed_issue_id_or_key = IssueIdOrKey::from_str(&get_args.issue_id_or_key)
                    .map_err(|e| {
                        format!(
                            "Failed to parse issue_id_or_key '{}': {}",
                            get_args.issue_id_or_key, e
                        )
                    })?;

                let comment_id = CommentId::new(get_args.comment_id);

                match client
                    .issue()
                    .get_comment(backlog_issue::GetCommentParams::new(
                        parsed_issue_id_or_key,
                        comment_id,
                    ))
                    .await
                {
                    Ok(comment) => {
                        println!("Comment ID: {}", comment.id);
                        println!("Created by: {}", comment.created_user.name);
                        println!("Created at: {}", comment.created);
                        println!("Updated at: {}", comment.updated);
                        if let Some(content) = &comment.content {
                            println!("Content: {content}");
                        } else {
                            println!("Content: (empty)");
                        }
                        if !comment.change_log.is_empty() {
                            println!("Change log entries: {}", comment.change_log.len());
                        }
                        if !comment.notifications.is_empty() {
                            println!("Notifications: {}", comment.notifications.len());
                        }
                        if !comment.stars.is_empty() {
                            println!("Stars: {}", comment.stars.len());
                        }
                    }
                    Err(e) => {
                        eprintln!("Error getting comment: {e}");
                    }
                }
            }
            IssueCommands::GetCommentNotifications(get_args) => {
                println!(
                    "Getting notifications for comment {} in issue: {}",
                    get_args.comment_id, get_args.issue_id_or_key
                );

                let parsed_issue_id_or_key = IssueIdOrKey::from_str(&get_args.issue_id_or_key)
                    .map_err(|e| {
                        format!(
                            "Failed to parse issue_id_or_key '{}': {}",
                            get_args.issue_id_or_key, e
                        )
                    })?;

                let comment_id = CommentId::new(get_args.comment_id);

                match client
                    .issue()
                    .get_comment_notifications(GetCommentNotificationsParams::new(
                        parsed_issue_id_or_key,
                        comment_id,
                    ))
                    .await
                {
                    Ok(notifications) => {
                        if notifications.is_empty() {
                            println!("No notifications found for this comment.");
                        } else {
                            println!("Found {} notification(s):", notifications.len());
                            for (i, notification) in notifications.iter().enumerate() {
                                println!(
                                    "  {}. Notification ID: {}",
                                    i + 1,
                                    notification.id.value()
                                );
                                println!("     User: {}", notification.user.name);
                                if let Some(user_id) = &notification.user.user_id {
                                    println!("     User ID: {user_id}");
                                }
                                println!("     Already Read: {}", notification.already_read);
                                println!(
                                    "     Resource Already Read: {}",
                                    notification.resource_already_read
                                );
                                println!("     Reason: {:?}", notification.reason);
                                println!();
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error getting comment notifications: {e}");
                    }
                }
            }
            #[cfg(feature = "issue_writable")]
            IssueCommands::AddCommentNotification(add_args) => {
                println!(
                    "Adding notifications to comment {} in issue: {}",
                    add_args.comment_id, add_args.issue_id_or_key
                );

                let parsed_issue_id_or_key = IssueIdOrKey::from_str(&add_args.issue_id_or_key)
                    .map_err(|e| {
                        format!(
                            "Failed to parse issue_id_or_key '{}': {}",
                            add_args.issue_id_or_key, e
                        )
                    })?;

                let comment_id = CommentId::new(add_args.comment_id);

                // Parse user IDs from comma-separated string
                let user_ids: Result<Vec<UserId>, _> = add_args
                    .users
                    .split(',')
                    .map(|id_str| {
                        id_str
                            .trim()
                            .parse::<u32>()
                            .map(UserId::new)
                            .map_err(|e| format!("Invalid user ID '{}': {}", id_str.trim(), e))
                    })
                    .collect();

                let user_ids = match user_ids {
                    Ok(ids) => ids,
                    Err(e) => {
                        eprintln!("Error parsing user IDs: {e}");
                        return Ok(());
                    }
                };

                let params =
                    AddCommentNotificationParams::new(parsed_issue_id_or_key, comment_id, user_ids);

                match client.issue().add_comment_notification(params).await {
                    Ok(comment) => {
                        println!("✅ Comment notifications added successfully!");
                        println!("Comment ID: {}", comment.id.value());
                        println!("Notifications count: {}", comment.notifications.len());
                        if !comment.notifications.is_empty() {
                            println!("Notified users:");
                            for notification in &comment.notifications {
                                println!(
                                    "  - {} (ID: {})",
                                    notification.user.name,
                                    notification.user.id.value()
                                );
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ Error adding comment notifications: {e}");
                    }
                }
            }
            IssueCommands::ListParticipants { issue_id_or_key } => {
                println!("Listing participants for issue: {issue_id_or_key}");

                let parsed_issue_id_or_key =
                    IssueIdOrKey::from_str(&issue_id_or_key).map_err(|e| {
                        format!("Failed to parse issue_id_or_key '{issue_id_or_key}': {e}")
                    })?;

                match client
                    .issue()
                    .get_participant_list(backlog_issue::GetParticipantListParams::new(
                        parsed_issue_id_or_key,
                    ))
                    .await
                {
                    Ok(participants) => {
                        if participants.is_empty() {
                            println!("No participants found for this issue.");
                        } else {
                            println!("Found {} participant(s):", participants.len());
                            for participant in participants {
                                println!("- {} (ID: {})", participant.name, participant.id.value());
                                if let Some(user_id) = &participant.user_id {
                                    println!("  User ID: {user_id}");
                                }
                                println!("  Email: {}", participant.mail_address);
                                println!("  Role: {:?}", participant.role_type);
                                if let Some(last_login) = &participant.last_login_time {
                                    println!(
                                        "  Last Login: {}",
                                        last_login.format("%Y-%m-%d %H:%M:%S")
                                    );
                                }
                                println!();
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error listing participants: {e}");
                    }
                }
            }
            IssueCommands::RecentlyViewed {
                order,
                count,
                offset,
            } => {
                println!("Getting recently viewed issues");

                let mut builder = GetRecentlyViewedIssuesParamsBuilder::default();
                builder.order(order);
                builder.count(count);

                if let Some(offset_val) = offset {
                    builder.offset(offset_val);
                }

                let params = builder.build()?;

                match client.issue().get_recently_viewed_issues(params).await {
                    Ok(issues) => {
                        if issues.is_empty() {
                            println!("No recently viewed issues found.");
                        } else {
                            println!("Found {} recently viewed issue(s):", issues.len());
                            println!();

                            for (index, issue) in issues.iter().enumerate() {
                                println!("{}. {} - {}", index + 1, issue.issue_key, issue.summary);
                                println!("   Project ID: {}", issue.project_id);
                                println!("   Status: {}", issue.status.name);
                                if let Some(priority) = &issue.priority {
                                    println!("   Priority: {}", priority.name);
                                }
                                println!("   Issue Type: {}", issue.issue_type.name);
                                if let Some(assignee) = &issue.assignee {
                                    println!("   Assignee: {}", assignee.name);
                                }
                                println!("   Updated: {}", issue.updated);
                                println!();
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error getting recently viewed issues: {e}");
                    }
                }
            }
            #[cfg(feature = "issue_writable")]
            IssueCommands::AddRecentlyViewed { issue_id_or_key } => {
                println!("Adding issue {issue_id_or_key} to recently viewed list");

                let issue_id_or_key = if let Ok(id) = issue_id_or_key.parse::<u32>() {
                    IssueIdOrKey::Id(id.into())
                } else {
                    IssueIdOrKey::Key(IssueKey::from_str(&issue_id_or_key)?)
                };
                let params = AddRecentlyViewedIssueParams { issue_id_or_key };

                match client.issue().add_recently_viewed_issue(params).await {
                    Ok(issue) => {
                        println!("Successfully added issue to recently viewed list:");
                        println!("  Issue Key: {}", issue.issue_key);
                        println!("  Summary: {}", issue.summary);
                        println!("  Status: {}", issue.status.name);
                        if let Some(priority) = &issue.priority {
                            println!("  Priority: {}", priority.name);
                        }
                        println!("  Issue Type: {}", issue.issue_type.name);
                        if let Some(assignee) = &issue.assignee {
                            println!("  Assignee: {}", assignee.name);
                        }
                    }
                    Err(e) => {
                        eprintln!("Error adding issue to recently viewed list: {e}");
                    }
                }
            }
            IssueCommands::ListSharedFiles { issue_id_or_key } => {
                println!("Listing shared files for issue: {issue_id_or_key}");

                let parsed_issue_id_or_key =
                    IssueIdOrKey::from_str(&issue_id_or_key).map_err(|e| {
                        format!("Failed to parse issue_id_or_key '{issue_id_or_key}': {e}")
                    })?;

                match client
                    .issue()
                    .get_shared_file_list(backlog_issue::GetSharedFileListParams::new(
                        parsed_issue_id_or_key,
                    ))
                    .await
                {
                    Ok(shared_files) => {
                        if shared_files.is_empty() {
                            println!("No shared files found for this issue.");
                        } else {
                            println!("Found {} shared file(s):", shared_files.len());
                            println!();

                            for (index, file) in shared_files.iter().enumerate() {
                                println!("{}. {}", index + 1, file.name);
                                println!("   ID: {}", file.id);
                                println!("   Directory: {}", file.dir);
                                match &file.content {
                                    backlog_issue::models::FileContent::File { size } => {
                                        println!("   Type: File");
                                        println!("   Size: {size} bytes");
                                    }
                                    backlog_issue::models::FileContent::Directory => {
                                        println!("   Type: Directory");
                                    }
                                }
                                println!("   Created by: {}", file.created_user.name);
                                println!("   Created at: {}", file.created);
                                if let Some(updated_user) = &file.updated_user {
                                    println!("   Updated by: {}", updated_user.name);
                                }
                                if let Some(updated) = &file.updated {
                                    println!("   Updated at: {updated}");
                                }
                                println!();
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error listing shared files: {e}");
                    }
                }
            }
            #[cfg(feature = "issue_writable")]
            IssueCommands::LinkSharedFiles {
                issue_id_or_key,
                file_ids,
            } => {
                println!(
                    "Linking {} shared file(s) to issue: {}",
                    file_ids.len(),
                    issue_id_or_key
                );

                let parsed_issue_id_or_key =
                    IssueIdOrKey::from_str(&issue_id_or_key).map_err(|e| {
                        format!("Failed to parse issue_id_or_key '{issue_id_or_key}': {e}")
                    })?;

                let shared_file_ids: Vec<SharedFileId> =
                    file_ids.iter().map(|&id| SharedFileId::new(id)).collect();

                let params = LinkSharedFilesToIssueParamsBuilder::default()
                    .issue_id_or_key(parsed_issue_id_or_key)
                    .shared_file_ids(shared_file_ids)
                    .build()
                    .map_err(|e| format!("Failed to build parameters: {e}"))?;

                match client.issue().link_shared_files_to_issue(params).await {
                    Ok(linked_files) => {
                        println!(
                            "✅ Successfully linked {} shared file(s) to the issue!",
                            linked_files.len()
                        );
                        println!();

                        for (index, file) in linked_files.iter().enumerate() {
                            println!("{}. {}", index + 1, file.name);
                            println!("   ID: {}", file.id);
                            println!("   Directory: {}", file.dir);
                            match &file.content {
                                backlog_issue::models::FileContent::File { size } => {
                                    println!("   Type: File");
                                    println!("   Size: {size} bytes");
                                }
                                backlog_issue::models::FileContent::Directory => {
                                    println!("   Type: Directory");
                                }
                            }
                            println!("   Created by: {}", file.created_user.name);
                            println!("   Created at: {}", file.created);
                            println!();
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to link shared files to issue: {e}");
                        std::process::exit(1);
                    }
                }
            }
            #[cfg(feature = "issue_writable")]
            IssueCommands::UnlinkSharedFile {
                issue_id_or_key,
                file_id,
            } => {
                println!("Unlinking shared file {file_id} from issue: {issue_id_or_key}");

                let parsed_issue_id_or_key =
                    IssueIdOrKey::from_str(&issue_id_or_key).map_err(|e| {
                        format!("Failed to parse issue_id_or_key '{issue_id_or_key}': {e}")
                    })?;

                let params =
                    UnlinkSharedFileParams::new(parsed_issue_id_or_key, SharedFileId::new(file_id));

                match client.issue().unlink_shared_file(params).await {
                    Ok(unlinked_file) => {
                        println!("✅ Successfully unlinked shared file from the issue!");
                        println!("   Name: {}", unlinked_file.name);
                        println!("   ID: {}", unlinked_file.id);
                        println!("   Directory: {}", unlinked_file.dir);
                        match &unlinked_file.content {
                            backlog_issue::models::FileContent::File { size } => {
                                println!("   Type: File");
                                println!("   Size: {size} bytes");
                            }
                            backlog_issue::models::FileContent::Directory => {
                                println!("   Type: Directory");
                            }
                        }
                        println!("   Created by: {}", unlinked_file.created_user.name);
                        println!("   Created at: {}", unlinked_file.created);
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to unlink shared file from issue: {e}");
                        std::process::exit(1);
                    }
                }
            }
            #[cfg(not(feature = "issue_writable"))]
            _ => {
                eprintln!(
                    "This command requires write access to issues and is not available. \
                    Please build with the 'issue_writable' feature flag:\n\
                    cargo build --package blg --features issue_writable"
                );
                std::process::exit(1);
            }
        },
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
                            println!("{}", serde_json::to_string_pretty(&disk_usage).unwrap());
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
                            println!("{}", serde_json::to_string_pretty(&licence).unwrap());
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
        Commands::Project(project_args) => match project_args.command {
            ProjectCommands::List => {
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
            }
            ProjectCommands::Show { project_id_or_key } => {
                println!("Showing project: {project_id_or_key}");

                let proj_id_or_key = project_id_or_key.parse::<ProjectIdOrKey>()?;
                let params = backlog_project::GetProjectDetailParams::new(proj_id_or_key);
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
            }
            #[cfg(feature = "project_writable")]
            ProjectCommands::Add {
                name,
                key,
                chart_enabled,
                use_resolved_for_chart,
                subtasking_enabled,
                project_leader_can_edit_project_leader,
                use_wiki,
                use_file_sharing,
                use_wiki_tree_view,
                use_subversion,
                use_git,
                use_original_image_size_at_wiki,
                text_formatting_rule,
                use_dev_attributes,
            } => {
                println!("Adding new project: {name} ({key})");

                let mut params = backlog_project::api::AddProjectParams::new(&name, &key);

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
                    params = params.text_formatting_rule(rule);
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
            }
            #[cfg(feature = "project_writable")]
            ProjectCommands::Update {
                project_id_or_key,
                name,
                key,
                chart_enabled,
                use_resolved_for_chart,
                subtasking_enabled,
                project_leader_can_edit_project_leader,
                use_wiki,
                use_file_sharing,
                use_wiki_tree_view,
                use_subversion,
                use_git,
                use_original_image_size_at_wiki,
                text_formatting_rule,
                archived,
                use_dev_attributes,
            } => {
                println!("Updating project: {project_id_or_key}");

                let proj_id_or_key = project_id_or_key.parse::<ProjectIdOrKey>()?;
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
                if let Some(project_leader_can_edit_project_leader) =
                    project_leader_can_edit_project_leader
                {
                    params.project_leader_can_edit_project_leader =
                        Some(project_leader_can_edit_project_leader);
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
            }
            #[cfg(feature = "project_writable")]
            ProjectCommands::Delete { project_id_or_key } => {
                println!("Deleting project: {project_id_or_key}");
                println!(
                    "⚠️  WARNING: This will permanently delete the project and all associated data!"
                );
                println!("Are you sure you want to continue? Type 'yes' to confirm:");

                let mut confirmation = String::new();
                std::io::stdin()
                    .read_line(&mut confirmation)
                    .expect("Failed to read confirmation");

                if confirmation.trim() != "yes" {
                    println!("Project deletion cancelled.");
                    return Ok(());
                }

                let proj_id_or_key = project_id_or_key.parse::<ProjectIdOrKey>()?;
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
            }
            ProjectCommands::RecentlyViewed {
                order,
                count,
                offset,
            } => {
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
            }
            ProjectCommands::StatusList { project_id_or_key } => {
                println!("Listing statuses for project: {project_id_or_key}");

                let proj_id_or_key = project_id_or_key.parse::<ProjectIdOrKey>()?;
                let params = backlog_project::GetStatusListParams::new(proj_id_or_key);
                match client.project().get_status_list(params).await {
                    Ok(statuses) => {
                        if statuses.is_empty() {
                            println!("No statuses found");
                        } else {
                            for status in statuses {
                                println!(
                                    "[{}] {} (Color: {})",
                                    status.id, status.name, status.color
                                );
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error listing statuses: {e}");
                    }
                }
            }
            ProjectCommands::MilestoneList { project_id_or_key } => {
                println!("Listing milestones for project: {project_id_or_key}");

                let proj_id_or_key = project_id_or_key.parse::<ProjectIdOrKey>()?;
                match client
                    .project()
                    .get_version_milestone_list(backlog_project::GetMilestoneListParams::new(
                        proj_id_or_key,
                    ))
                    .await
                {
                    Ok(milestones) => {
                        if milestones.is_empty() {
                            println!("No milestones found");
                        } else {
                            for milestone in milestones {
                                print!("[{}] {}", milestone.id, milestone.name);
                                if let Some(description) = &milestone.description {
                                    print!(" - {description}");
                                }
                                println!();

                                if let Some(start_date) = milestone.start_date {
                                    println!("  Start Date: {}", start_date.format("%Y-%m-%d"));
                                }
                                if let Some(release_date) = milestone.release_due_date {
                                    println!("  Release Due: {}", release_date.format("%Y-%m-%d"));
                                }
                                if milestone.archived {
                                    println!("  Status: Archived");
                                }
                                println!();
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error listing milestones: {e}");
                    }
                }
            }
            ProjectCommands::IssueTypeList { project_id_or_key } => {
                println!("Listing issue types for project: {project_id_or_key}");

                let proj_id_or_key = project_id_or_key.parse::<ProjectIdOrKey>()?;
                let params = backlog_project::GetIssueTypeListParams::new(proj_id_or_key);
                match client.project().get_issue_type_list(params).await {
                    Ok(issue_types) => {
                        if issue_types.is_empty() {
                            println!("No issue types found");
                        } else {
                            for issue_type in issue_types {
                                println!(
                                    "[{}] {} (Color: {})",
                                    issue_type.id, issue_type.name, issue_type.color
                                );
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error listing issue types: {e}");
                    }
                }
            }
            ProjectCommands::CategoryList { project_id_or_key } => {
                println!("Listing categories for project: {project_id_or_key}");

                let proj_id_or_key = project_id_or_key.parse::<ProjectIdOrKey>()?;
                let params = backlog_project::GetCategoryListParams::new(proj_id_or_key);
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
            }
            ProjectCommands::DiskUsage {
                project_id_or_key,
                human_readable,
            } => {
                println!("Getting disk usage for project: {project_id_or_key}");

                let proj_id_or_key = project_id_or_key.parse::<ProjectIdOrKey>()?;
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
            }
            ProjectCommands::UserList { project_id_or_key } => {
                println!("Listing users for project: {project_id_or_key}");

                let proj_id_or_key = project_id_or_key.parse::<ProjectIdOrKey>()?;
                let params = backlog_project::GetProjectUserListParams::new(proj_id_or_key);
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
            }
            ProjectCommands::AdminList { project_id_or_key } => {
                println!("Listing administrators for project: {project_id_or_key}");

                let proj_id_or_key = project_id_or_key.parse::<ProjectIdOrKey>()?;
                let params =
                    backlog_project::api::GetProjectAdministratorListParams::new(proj_id_or_key);
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
            }
            #[cfg(feature = "project_writable")]
            ProjectCommands::AdminAdd {
                project_id_or_key,
                user_id,
            } => {
                println!("Adding user {user_id} as administrator to project: {project_id_or_key}");

                let proj_id_or_key = project_id_or_key.parse::<ProjectIdOrKey>()?;
                let params = backlog_project::api::AddProjectAdministratorParams::new(
                    proj_id_or_key,
                    user_id,
                );
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
            }
            #[cfg(feature = "project_writable")]
            ProjectCommands::AdminRemove {
                project_id_or_key,
                user_id,
            } => {
                println!("Removing administrator {user_id} from project: {project_id_or_key}");

                let proj_id_or_key = project_id_or_key.parse::<ProjectIdOrKey>()?;
                let params = backlog_project::api::DeleteProjectAdministratorParams::new(
                    proj_id_or_key,
                    user_id,
                );
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
            }
            #[cfg(feature = "project_writable")]
            ProjectCommands::UserAdd {
                project_id_or_key,
                user_id,
            } => {
                println!("Adding user {user_id} to project: {project_id_or_key}");

                let proj_id_or_key = project_id_or_key.parse::<ProjectIdOrKey>()?;
                let params =
                    backlog_project::api::AddProjectUserParams::new(proj_id_or_key, user_id);
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
            }
            #[cfg(feature = "project_writable")]
            ProjectCommands::UserRemove {
                project_id_or_key,
                user_id,
            } => {
                println!("Removing user {user_id} from project: {project_id_or_key}");

                let proj_id_or_key = project_id_or_key.parse::<ProjectIdOrKey>()?;
                let params =
                    backlog_project::api::DeleteProjectUserParams::new(proj_id_or_key, user_id);
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
            }
            ProjectCommands::CustomFieldList { project_id_or_key } => {
                println!("Listing custom fields for project: {project_id_or_key}");

                let proj_id_or_key = project_id_or_key.parse::<ProjectIdOrKey>()?;
                let params = backlog_project::GetCustomFieldListParams::new(proj_id_or_key);
                match client.project().get_custom_field_list(params).await {
                    Ok(custom_fields) => {
                        if custom_fields.is_empty() {
                            println!("No custom fields found in this project");
                        } else {
                            for field in custom_fields {
                                let field_type = match field.settings {
                                    backlog_domain_models::CustomFieldSettings::Text => "Text",
                                    backlog_domain_models::CustomFieldSettings::TextArea => {
                                        "TextArea"
                                    }
                                    backlog_domain_models::CustomFieldSettings::Numeric(_) => {
                                        "Numeric"
                                    }
                                    backlog_domain_models::CustomFieldSettings::Date(_) => "Date",
                                    backlog_domain_models::CustomFieldSettings::SingleList(_) => {
                                        "SingleList"
                                    }
                                    backlog_domain_models::CustomFieldSettings::MultipleList(_) => {
                                        "MultipleList"
                                    }
                                    backlog_domain_models::CustomFieldSettings::Checkbox(_) => {
                                        "Checkbox"
                                    }
                                    backlog_domain_models::CustomFieldSettings::Radio(_) => "Radio",
                                };
                                let required_str = if field.required {
                                    "Required"
                                } else {
                                    "Optional"
                                };
                                println!(
                                    "[{}] {} ({}) - {} - Display Order: {}",
                                    field.id,
                                    field.name,
                                    field_type,
                                    required_str,
                                    field.display_order
                                );
                                if !field.description.is_empty() {
                                    println!("    Description: {}", field.description);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error listing custom fields: {e}");
                    }
                }
            }
            #[cfg(feature = "project_writable")]
            ProjectCommands::CustomFieldUpdate {
                project_id_or_key,
                custom_field_id,
                name,
                description,
                required,
                applicable_issue_types,
                min_date,
                max_date,
                initial_value_type,
                initial_date,
                initial_shift,
            } => {
                println!("Updating custom field {custom_field_id} in project: {project_id_or_key}");

                let proj_id_or_key = project_id_or_key.parse::<ProjectIdOrKey>()?;
                let field_id = CustomFieldId::new(custom_field_id);
                let mut params = UpdateCustomFieldParams::new(proj_id_or_key, field_id);

                // Set optional parameters
                if let Some(n) = name {
                    params = params.with_name(n);
                }
                if let Some(d) = description {
                    params = params.with_description(d);
                }
                if let Some(r) = required {
                    params = params.with_required(r);
                }
                if let Some(types) = applicable_issue_types {
                    let type_ids: Vec<IssueTypeId> = types
                        .split(',')
                        .filter_map(|s| s.trim().parse::<u32>().ok())
                        .map(IssueTypeId::new)
                        .collect();
                    if !type_ids.is_empty() {
                        params = params.with_applicable_issue_types(type_ids);
                    }
                }

                // Handle date field specific parameters
                if min_date.is_some()
                    || max_date.is_some()
                    || initial_value_type.is_some()
                    || initial_date.is_some()
                    || initial_shift.is_some()
                {
                    let min_date_parsed = min_date
                        .as_ref()
                        .and_then(|d| backlog_core::Date::from_str(d).ok());
                    let max_date_parsed = max_date
                        .as_ref()
                        .and_then(|d| backlog_core::Date::from_str(d).ok());
                    let initial_date_parsed = initial_date
                        .as_ref()
                        .and_then(|d| backlog_core::Date::from_str(d).ok());

                    params = params.with_date_settings(
                        min_date_parsed,
                        max_date_parsed,
                        initial_value_type,
                        initial_date_parsed,
                        initial_shift,
                    );
                }

                match client.project().update_custom_field(params).await {
                    Ok(field) => {
                        println!("Custom field updated successfully:");
                        println!("[{}] {}", field.id, field.name);
                        if !field.description.is_empty() {
                            println!("Description: {}", field.description);
                        }
                        println!("Required: {}", field.required);
                        if let Some(issue_types) = &field.applicable_issue_types {
                            if !issue_types.is_empty() {
                                let ids: Vec<String> =
                                    issue_types.iter().map(|id| id.to_string()).collect();
                                println!("Applicable Issue Types: {}", ids.join(", "));
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error updating custom field: {e}");
                    }
                }
            }
            #[cfg(feature = "project_writable")]
            ProjectCommands::CustomFieldAdd {
                project_id_or_key,
                field_type,
                name,
                description,
                required,
                applicable_issue_types,
                min,
                max,
                initial_value,
                unit,
                min_date,
                max_date,
                initial_value_type,
                initial_date,
                initial_shift,
                items,
                allow_input,
                allow_add_item,
            } => {
                println!("Adding custom field '{name}' to project: {project_id_or_key}");

                let proj_id_or_key = project_id_or_key.parse::<ProjectIdOrKey>()?;

                // Create params based on field type
                let mut params = match field_type.as_str() {
                    "text" => AddCustomFieldParams::text(proj_id_or_key, name.clone()),
                    "textarea" => AddCustomFieldParams::textarea(proj_id_or_key, name.clone()),
                    "numeric" => AddCustomFieldParams::numeric(proj_id_or_key, name.clone()),
                    "date" => AddCustomFieldParams::date(proj_id_or_key, name.clone()),
                    "single-list" => {
                        if let Some(items_str) = items.as_ref() {
                            let items_vec: Vec<String> =
                                items_str.split(',').map(|s| s.trim().to_string()).collect();
                            AddCustomFieldParams::single_list(
                                proj_id_or_key,
                                name.clone(),
                                items_vec,
                            )
                        } else {
                            eprintln!("Error: --items is required for single-list field type");
                            std::process::exit(1);
                        }
                    }
                    "multiple-list" => {
                        if let Some(items_str) = items.as_ref() {
                            let items_vec: Vec<String> =
                                items_str.split(',').map(|s| s.trim().to_string()).collect();
                            AddCustomFieldParams::multiple_list(
                                proj_id_or_key,
                                name.clone(),
                                items_vec,
                            )
                        } else {
                            eprintln!("Error: --items is required for multiple-list field type");
                            std::process::exit(1);
                        }
                    }
                    "checkbox" => AddCustomFieldParams::checkbox(proj_id_or_key, name.clone()),
                    "radio" => AddCustomFieldParams::radio(proj_id_or_key, name.clone()),
                    _ => {
                        eprintln!("Error: Invalid field type '{field_type}'");
                        eprintln!(
                            "Valid types: text, textarea, numeric, date, single-list, multiple-list, checkbox, radio"
                        );
                        std::process::exit(1);
                    }
                };

                // Set common optional parameters
                if let Some(d) = description {
                    params = params.with_description(d);
                }
                if let Some(r) = required {
                    params = params.with_required(r);
                }
                if let Some(types) = applicable_issue_types {
                    let type_ids: Vec<IssueTypeId> = types
                        .split(',')
                        .filter_map(|s| s.trim().parse::<u32>().ok())
                        .map(IssueTypeId::new)
                        .collect();
                    if !type_ids.is_empty() {
                        params = params.with_applicable_issue_types(type_ids);
                    }
                }

                // Set field-type specific parameters
                match field_type.as_str() {
                    "numeric" => {
                        params =
                            params.with_numeric_settings(min, max, initial_value, unit.clone());
                    }
                    "date" => {
                        let min_date_parsed = min_date
                            .as_ref()
                            .and_then(|d| backlog_core::Date::from_str(d).ok());
                        let max_date_parsed = max_date
                            .as_ref()
                            .and_then(|d| backlog_core::Date::from_str(d).ok());
                        let initial_date_parsed = initial_date
                            .as_ref()
                            .and_then(|d| backlog_core::Date::from_str(d).ok());

                        params = params.with_date_settings(
                            min_date_parsed,
                            max_date_parsed,
                            initial_value_type,
                            initial_date_parsed,
                            initial_shift,
                        );
                    }
                    "single-list" | "multiple-list" => {
                        if let Some(allow_input_val) = allow_input {
                            params = params.with_allow_input(allow_input_val);
                        }
                        if let Some(allow_add_item_val) = allow_add_item {
                            params = params.with_allow_add_item(allow_add_item_val);
                        }
                    }
                    _ => {}
                }

                match client.project().add_custom_field(params).await {
                    Ok(field) => {
                        println!("✅ Custom field added successfully:");
                        println!("[{}] {}", field.id, field.name);
                        let field_type = match &field.settings {
                            backlog_domain_models::CustomFieldSettings::Text => "Text",
                            backlog_domain_models::CustomFieldSettings::TextArea => "TextArea",
                            backlog_domain_models::CustomFieldSettings::Numeric(_) => "Numeric",
                            backlog_domain_models::CustomFieldSettings::Date(_) => "Date",
                            backlog_domain_models::CustomFieldSettings::SingleList(_) => {
                                "SingleList"
                            }
                            backlog_domain_models::CustomFieldSettings::MultipleList(_) => {
                                "MultipleList"
                            }
                            backlog_domain_models::CustomFieldSettings::Checkbox(_) => "Checkbox",
                            backlog_domain_models::CustomFieldSettings::Radio(_) => "Radio",
                        };
                        println!("Type: {field_type}");
                        if !field.description.is_empty() {
                            println!("Description: {}", field.description);
                        }
                        println!("Required: {}", field.required);
                        if let Some(issue_types) = &field.applicable_issue_types {
                            if !issue_types.is_empty() {
                                let ids: Vec<String> =
                                    issue_types.iter().map(|id| id.to_string()).collect();
                                println!("Applicable Issue Types: {}", ids.join(", "));
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ Error adding custom field: {e}");
                        std::process::exit(1);
                    }
                }
            }
            #[cfg(feature = "project_writable")]
            ProjectCommands::CustomFieldDelete {
                project_id_or_key,
                custom_field_id,
            } => {
                println!(
                    "Deleting custom field {custom_field_id} from project: {project_id_or_key}"
                );

                let proj_id_or_key = project_id_or_key.parse::<ProjectIdOrKey>()?;
                let field_id = CustomFieldId::new(custom_field_id);
                let params = DeleteCustomFieldParams::new(proj_id_or_key, field_id);

                match client.project().delete_custom_field(params).await {
                    Ok(field) => {
                        println!("✅ Custom field deleted successfully:");
                        println!("[{}] {}", field.id, field.name);
                        let field_type = match &field.settings {
                            backlog_domain_models::CustomFieldSettings::Text => "Text",
                            backlog_domain_models::CustomFieldSettings::TextArea => "TextArea",
                            backlog_domain_models::CustomFieldSettings::Numeric(_) => "Numeric",
                            backlog_domain_models::CustomFieldSettings::Date(_) => "Date",
                            backlog_domain_models::CustomFieldSettings::SingleList(_) => {
                                "SingleList"
                            }
                            backlog_domain_models::CustomFieldSettings::MultipleList(_) => {
                                "MultipleList"
                            }
                            backlog_domain_models::CustomFieldSettings::Checkbox(_) => "Checkbox",
                            backlog_domain_models::CustomFieldSettings::Radio(_) => "Radio",
                        };
                        println!("Type: {field_type}");
                        if !field.description.is_empty() {
                            println!("Description: {}", field.description);
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ Error deleting custom field: {e}");
                        std::process::exit(1);
                    }
                }
            }
            #[cfg(feature = "project_writable")]
            ProjectCommands::CustomFieldAddItem {
                project_id_or_key,
                custom_field_id,
                name,
            } => {
                println!(
                    "Adding list item '{name}' to custom field {custom_field_id} in project: {project_id_or_key}"
                );

                let proj_id_or_key = project_id_or_key.parse::<ProjectIdOrKey>()?;
                let field_id = CustomFieldId::new(custom_field_id);
                let params =
                    AddListItemToCustomFieldParams::new(proj_id_or_key, field_id, name.clone());

                match client.project().add_list_item_to_custom_field(params).await {
                    Ok(field) => {
                        println!("✅ List item added successfully to custom field:");
                        println!("[{}] {}", field.id, field.name);

                        // Display list items if it's a list type field
                        match &field.settings {
                            backlog_domain_models::CustomFieldSettings::SingleList(settings) => {
                                println!("Type: Single Selection List");
                                println!("List items:");
                                for item in &settings.items {
                                    println!(
                                        "  - [{}] {} (order: {})",
                                        item.id, item.name, item.display_order
                                    );
                                }
                                if let Some(allow) = settings.allow_add_item {
                                    println!("Allow add item: {allow}");
                                }
                            }
                            backlog_domain_models::CustomFieldSettings::MultipleList(settings) => {
                                println!("Type: Multiple Selection List");
                                println!("List items:");
                                for item in &settings.items {
                                    println!(
                                        "  - [{}] {} (order: {})",
                                        item.id, item.name, item.display_order
                                    );
                                }
                                if let Some(allow) = settings.allow_add_item {
                                    println!("Allow add item: {allow}");
                                }
                                if let Some(allow) = settings.allow_input {
                                    println!("Allow input: {allow}");
                                }
                            }
                            _ => {
                                eprintln!("⚠️  Warning: Custom field is not a list type");
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ Error adding list item to custom field: {e}");
                        std::process::exit(1);
                    }
                }
            }
            #[cfg(feature = "project_writable")]
            ProjectCommands::CustomFieldUpdateItem {
                project_id_or_key,
                custom_field_id,
                item_id,
                name,
            } => {
                println!(
                    "Updating list item {item_id} in custom field {custom_field_id} in project: {project_id_or_key}"
                );

                let proj_id_or_key = project_id_or_key.parse::<ProjectIdOrKey>()?;
                let field_id = CustomFieldId::new(custom_field_id);
                let params = UpdateListItemToCustomFieldParams::new(
                    proj_id_or_key,
                    field_id,
                    item_id,
                    name.clone(),
                );

                match client
                    .project()
                    .update_list_item_to_custom_field(params)
                    .await
                {
                    Ok(field) => {
                        println!("✅ List item updated successfully in custom field:");
                        println!("[{}] {}", field.id, field.name);

                        // Display list items if it's a list type field
                        match &field.settings {
                            backlog_domain_models::CustomFieldSettings::SingleList(settings) => {
                                println!("Type: Single Selection List");
                                println!("List items:");
                                for item in &settings.items {
                                    if item.id.value() == item_id {
                                        println!(
                                            "  - [{}] {} (order: {}) ← UPDATED",
                                            item.id, item.name, item.display_order
                                        );
                                    } else {
                                        println!(
                                            "  - [{}] {} (order: {})",
                                            item.id, item.name, item.display_order
                                        );
                                    }
                                }
                                if let Some(allow) = settings.allow_add_item {
                                    println!("Allow add item: {allow}");
                                }
                            }
                            backlog_domain_models::CustomFieldSettings::MultipleList(settings) => {
                                println!("Type: Multiple Selection List");
                                println!("List items:");
                                for item in &settings.items {
                                    if item.id.value() == item_id {
                                        println!(
                                            "  - [{}] {} (order: {}) ← UPDATED",
                                            item.id, item.name, item.display_order
                                        );
                                    } else {
                                        println!(
                                            "  - [{}] {} (order: {})",
                                            item.id, item.name, item.display_order
                                        );
                                    }
                                }
                                if let Some(allow) = settings.allow_add_item {
                                    println!("Allow add item: {allow}");
                                }
                                if let Some(allow) = settings.allow_input {
                                    println!("Allow input: {allow}");
                                }
                            }
                            _ => {
                                eprintln!("⚠️  Warning: Custom field is not a list type");
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ Error updating list item in custom field: {e}");
                        std::process::exit(1);
                    }
                }
            }
            #[cfg(feature = "project_writable")]
            ProjectCommands::CustomFieldDeleteItem {
                project_id_or_key,
                custom_field_id,
                item_id,
            } => {
                println!(
                    "Deleting list item {item_id} from custom field {custom_field_id} in project: {project_id_or_key}"
                );

                let proj_id_or_key = project_id_or_key.parse::<ProjectIdOrKey>()?;
                let field_id = CustomFieldId::new(custom_field_id);
                let item_id = CustomFieldItemId::new(item_id);
                let params =
                    DeleteListItemFromCustomFieldParams::new(proj_id_or_key, field_id, item_id);

                match client
                    .project()
                    .delete_list_item_from_custom_field(params)
                    .await
                {
                    Ok(field) => {
                        println!("✅ List item deleted successfully from custom field:");
                        println!("[{}] {}", field.id, field.name);

                        // Display remaining list items
                        match &field.settings {
                            backlog_domain_models::CustomFieldSettings::SingleList(settings) => {
                                println!("Type: Single Selection List");
                                println!("Remaining list items:");
                                for item in &settings.items {
                                    println!(
                                        "  - [{}] {} (order: {})",
                                        item.id, item.name, item.display_order
                                    );
                                }
                                if let Some(allow) = settings.allow_add_item {
                                    println!("Allow add item: {allow}");
                                }
                            }
                            backlog_domain_models::CustomFieldSettings::MultipleList(settings) => {
                                println!("Type: Multiple Selection List");
                                println!("Remaining list items:");
                                for item in &settings.items {
                                    println!(
                                        "  - [{}] {} (order: {})",
                                        item.id, item.name, item.display_order
                                    );
                                }
                                if let Some(allow) = settings.allow_add_item {
                                    println!("Allow add item: {allow}");
                                }
                                if let Some(allow) = settings.allow_input {
                                    println!("Allow input: {allow}");
                                }
                            }
                            _ => {
                                eprintln!("⚠️  Warning: Custom field is not a list type");
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ Error deleting list item from custom field: {e}");
                        std::process::exit(1);
                    }
                }
            }
            #[cfg(feature = "project_writable")]
            ProjectCommands::CategoryAdd {
                project_id_or_key,
                name,
            } => {
                println!("Adding category '{name}' to project: {project_id_or_key}");

                let proj_id_or_key = project_id_or_key.parse::<ProjectIdOrKey>()?;
                let params = AddCategoryParams::new(proj_id_or_key, name.clone());

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
            }
            #[cfg(feature = "project_writable")]
            ProjectCommands::CategoryUpdate {
                project_id_or_key,
                category_id,
                name,
            } => {
                println!(
                    "Updating category {category_id} in project {project_id_or_key} to name '{name}'"
                );

                let proj_id_or_key = project_id_or_key.parse::<ProjectIdOrKey>()?;
                let cat_id = CategoryId::new(category_id);
                let params = UpdateCategoryParams::new(proj_id_or_key, cat_id, name.clone());

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
            }
            #[cfg(feature = "project_writable")]
            ProjectCommands::CategoryDelete {
                project_id_or_key,
                category_id,
            } => {
                println!("Deleting category {category_id} from project: {project_id_or_key}");

                let proj_id_or_key = project_id_or_key.parse::<ProjectIdOrKey>()?;
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
            }
            #[cfg(feature = "project_writable")]
            ProjectCommands::IssueTypeAdd {
                project_id_or_key,
                name,
                color,
                template_summary,
                template_description,
            } => {
                println!("Adding issue type '{name}' to project: {project_id_or_key}");

                let proj_id_or_key = project_id_or_key.parse::<ProjectIdOrKey>()?;

                // Parse and validate the color
                let parsed_color = color.parse::<IssueTypeColor>().map_err(|e| {
                    format!(
                        "Invalid color '{}': {}\nAvailable colors: {}",
                        color,
                        e,
                        IssueTypeColor::all_names().join(", ")
                    )
                })?;

                let mut params = AddIssueTypeParams::new(proj_id_or_key, &name, parsed_color);
                params.template_summary = template_summary.clone();
                params.template_description = template_description.clone();

                match client.project().add_issue_type(params).await {
                    Ok(issue_type) => {
                        println!("Issue type added successfully:");
                        println!(
                            "[{}] {} (Color: {})",
                            issue_type.id, issue_type.name, issue_type.color
                        );
                        if let Some(template_summary) = &issue_type.template_summary {
                            println!("  Template Summary: {template_summary}");
                        }
                        if let Some(template_description) = &issue_type.template_description {
                            println!("  Template Description: {template_description}");
                        }
                        println!("  Display Order: {}", issue_type.display_order);
                    }
                    Err(e) => {
                        eprintln!("Error adding issue type: {e}");
                    }
                }
            }
            #[cfg(feature = "project_writable")]
            ProjectCommands::IssueTypeDelete {
                project_id_or_key,
                issue_type_id,
                substitute_issue_type_id,
            } => {
                println!(
                    "Deleting issue type {issue_type_id} from project: {project_id_or_key} (substitute: {substitute_issue_type_id})"
                );

                let proj_id_or_key = project_id_or_key.parse::<ProjectIdOrKey>()?;
                let issue_type_id_val = IssueTypeId::new(issue_type_id);
                let substitute_id = IssueTypeId::new(substitute_issue_type_id);

                let params =
                    DeleteIssueTypeParams::new(proj_id_or_key, issue_type_id_val, substitute_id);

                match client.project().delete_issue_type(params).await {
                    Ok(issue_type) => {
                        println!("Issue type deleted successfully:");
                        println!(
                            "[{}] {} (Color: {})",
                            issue_type.id, issue_type.name, issue_type.color
                        );
                        if let Some(template_summary) = &issue_type.template_summary {
                            println!("  Template Summary: {template_summary}");
                        }
                        if let Some(template_description) = &issue_type.template_description {
                            println!("  Template Description: {template_description}");
                        }
                        println!("  Display Order: {}", issue_type.display_order);
                    }
                    Err(e) => {
                        eprintln!("Error deleting issue type: {e}");
                    }
                }
            }
            #[cfg(feature = "project_writable")]
            ProjectCommands::IssueTypeUpdate {
                project_id_or_key,
                issue_type_id,
                name,
                color,
                template_summary,
                template_description,
            } => {
                println!("Updating issue type {issue_type_id} in project: {project_id_or_key}");

                let proj_id_or_key = project_id_or_key.parse::<ProjectIdOrKey>()?;
                let issue_type_id_val = IssueTypeId::new(issue_type_id);

                // Parse color if provided
                let parsed_color = if let Some(color_str) = color {
                    Some(color_str.parse::<IssueTypeColor>().map_err(|e| {
                        format!(
                            "Invalid color '{}': {}\nAvailable colors: {}",
                            color_str,
                            e,
                            IssueTypeColor::all_names().join(", ")
                        )
                    })?)
                } else {
                    None
                };

                let mut params = UpdateIssueTypeParams::new(proj_id_or_key, issue_type_id_val);
                params.name = name.clone();
                params.color = parsed_color;
                params.template_summary = template_summary.clone();
                params.template_description = template_description.clone();

                match client.project().update_issue_type(params).await {
                    Ok(issue_type) => {
                        println!("Issue type updated successfully:");
                        println!(
                            "[{}] {} (Color: {})",
                            issue_type.id, issue_type.name, issue_type.color
                        );
                        if let Some(template_summary) = &issue_type.template_summary {
                            println!("  Template Summary: {template_summary}");
                        }
                        if let Some(template_description) = &issue_type.template_description {
                            println!("  Template Description: {template_description}");
                        }
                        println!("  Display Order: {}", issue_type.display_order);
                    }
                    Err(e) => {
                        eprintln!("Error updating issue type: {e}");
                    }
                }
            }
            #[cfg(feature = "project_writable")]
            ProjectCommands::VersionAdd {
                project_id_or_key,
                name,
                description,
                start_date,
                release_due_date,
            } => {
                println!("Adding version/milestone '{name}' to project: {project_id_or_key}");

                let proj_id_or_key = project_id_or_key.parse::<ProjectIdOrKey>()?;
                let mut params = AddMilestoneParams::new(proj_id_or_key, &name);
                params.description = description.clone();
                params.start_date = start_date.as_ref().map(|d| {
                    DateTime::parse_from_str(&format!("{d}T00:00:00Z"), "%Y-%m-%dT%H:%M:%SZ")
                        .map(|dt| ApiDate::from(dt.with_timezone(&Utc)))
                        .unwrap_or_else(|_| panic!("Invalid date format: {d}"))
                });
                params.release_due_date = release_due_date.as_ref().map(|d| {
                    DateTime::parse_from_str(&format!("{d}T00:00:00Z"), "%Y-%m-%dT%H:%M:%SZ")
                        .map(|dt| ApiDate::from(dt.with_timezone(&Utc)))
                        .unwrap_or_else(|_| panic!("Invalid date format: {d}"))
                });

                match client.project().add_version(params).await {
                    Ok(milestone) => {
                        println!("Version/milestone added successfully:");
                        println!("[{}] {}", milestone.id, milestone.name);
                        if let Some(desc) = &milestone.description {
                            println!("  Description: {desc}");
                        }
                        if let Some(start_date) = &milestone.start_date {
                            println!("  Start Date: {}", start_date.format("%Y-%m-%d"));
                        }
                        if let Some(release_due_date) = &milestone.release_due_date {
                            println!(
                                "  Release Due Date: {}",
                                release_due_date.format("%Y-%m-%d")
                            );
                        }
                        println!("  Archived: {}", milestone.archived);
                        if let Some(display_order) = milestone.display_order {
                            println!("  Display Order: {display_order}");
                        }
                    }
                    Err(e) => {
                        eprintln!("Error adding version/milestone: {e}");
                    }
                }
            }
            #[cfg(feature = "project_writable")]
            ProjectCommands::VersionUpdate {
                project_id_or_key,
                version_id,
                name,
                description,
                start_date,
                release_due_date,
                archived,
            } => {
                println!("Updating version/milestone {version_id} in project: {project_id_or_key}");

                let proj_id_or_key = project_id_or_key.parse::<ProjectIdOrKey>()?;
                let version_id_val = MilestoneId::new(version_id);
                let mut params = UpdateVersionParams::new(proj_id_or_key, version_id_val, &name);
                params.description = description.clone();
                params.start_date = start_date.as_ref().map(|d| {
                    DateTime::parse_from_str(&format!("{d}T00:00:00Z"), "%Y-%m-%dT%H:%M:%SZ")
                        .map(|dt| ApiDate::from(dt.with_timezone(&Utc)))
                        .unwrap_or_else(|_| panic!("Invalid date format: {d}"))
                });
                params.release_due_date = release_due_date.as_ref().map(|d| {
                    DateTime::parse_from_str(&format!("{d}T00:00:00Z"), "%Y-%m-%dT%H:%M:%SZ")
                        .map(|dt| ApiDate::from(dt.with_timezone(&Utc)))
                        .unwrap_or_else(|_| panic!("Invalid date format: {d}"))
                });
                params.archived = archived;

                match client.project().update_version(params).await {
                    Ok(milestone) => {
                        println!("Version/milestone updated successfully:");
                        println!("[{}] {}", milestone.id, milestone.name);
                        if let Some(desc) = &milestone.description {
                            println!("  Description: {desc}");
                        }
                        if let Some(start_date) = &milestone.start_date {
                            println!("  Start Date: {}", start_date.format("%Y-%m-%d"));
                        }
                        if let Some(release_due_date) = &milestone.release_due_date {
                            println!(
                                "  Release Due Date: {}",
                                release_due_date.format("%Y-%m-%d")
                            );
                        }
                        println!("  Archived: {}", milestone.archived);
                        if let Some(display_order) = milestone.display_order {
                            println!("  Display Order: {display_order}");
                        }
                    }
                    Err(e) => {
                        eprintln!("Error updating version/milestone: {e}");
                    }
                }
            }
            #[cfg(feature = "project_writable")]
            ProjectCommands::VersionDelete {
                project_id_or_key,
                version_id,
            } => {
                println!(
                    "Deleting version/milestone {version_id} from project: {project_id_or_key}"
                );

                let proj_id_or_key = project_id_or_key.parse::<ProjectIdOrKey>()?;
                let version_id_val = MilestoneId::new(version_id);
                let params = DeleteVersionParams::new(proj_id_or_key, version_id_val);

                match client.project().delete_version(params).await {
                    Ok(milestone) => {
                        println!("Version/milestone deleted successfully:");
                        println!("[{}] {}", milestone.id, milestone.name);
                        if let Some(desc) = &milestone.description {
                            println!("  Description: {desc}");
                        }
                        if let Some(start_date) = &milestone.start_date {
                            println!("  Start Date: {}", start_date.format("%Y-%m-%d"));
                        }
                        if let Some(release_due_date) = &milestone.release_due_date {
                            println!(
                                "  Release Due Date: {}",
                                release_due_date.format("%Y-%m-%d")
                            );
                        }
                        println!("  Archived: {}", milestone.archived);
                        if let Some(display_order) = milestone.display_order {
                            println!("  Display Order: {display_order}");
                        }
                    }
                    Err(e) => {
                        eprintln!("Error deleting version/milestone: {e}");
                    }
                }
            }
            #[cfg(feature = "project_writable")]
            ProjectCommands::StatusAdd {
                project_id_or_key,
                name,
                color,
            } => {
                println!("Adding status '{name}' to project: {project_id_or_key}");

                let proj_id_or_key = project_id_or_key.parse::<ProjectIdOrKey>()?;
                let parsed_color = StatusColor::from_str(&color)?;

                let params = AddStatusParams::new(proj_id_or_key, &name, parsed_color);

                match client.project().add_status(params).await {
                    Ok(status) => {
                        println!("✅ Status added successfully:");
                        println!("ID: {}", status.id);
                        println!("Name: {}", status.name);
                        println!("Color: {}", status.color);
                        println!("Display Order: {}", status.display_order);
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to add status: {e}");
                        std::process::exit(1);
                    }
                }
            }
            #[cfg(feature = "project_writable")]
            ProjectCommands::StatusUpdate {
                project_id_or_key,
                status_id,
                name,
                color,
            } => {
                println!("Updating status {status_id} in project: {project_id_or_key}");

                let proj_id_or_key = project_id_or_key.parse::<ProjectIdOrKey>()?;
                let status_id_val = StatusId::new(status_id);

                let parsed_color = if let Some(color_str) = &color {
                    Some(StatusColor::from_str(color_str)?)
                } else {
                    None
                };

                let mut params = UpdateStatusParams::new(proj_id_or_key, status_id_val);

                if let Some(name) = name {
                    params = params.name(name);
                }

                if let Some(color) = parsed_color {
                    params = params.color(color);
                }

                match client.project().update_status(params).await {
                    Ok(status) => {
                        println!("✅ Status updated successfully:");
                        println!("ID: {}", status.id);
                        println!("Name: {}", status.name);
                        println!("Color: {}", status.color);
                        println!("Display Order: {}", status.display_order);
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to update status: {e}");
                        std::process::exit(1);
                    }
                }
            }
            #[cfg(feature = "project_writable")]
            ProjectCommands::StatusDelete {
                project_id_or_key,
                status_id,
                substitute_status_id,
            } => {
                println!(
                    "Deleting status {status_id} from project: {project_id_or_key} (substitute: {substitute_status_id})"
                );

                let proj_id_or_key = project_id_or_key.parse::<ProjectIdOrKey>()?;
                let status_id_val = StatusId::new(status_id);
                let substitute_id = StatusId::new(substitute_status_id);

                let params = DeleteStatusParams::new(proj_id_or_key, status_id_val, substitute_id);

                match client.project().delete_status(params).await {
                    Ok(status) => {
                        println!("✅ Status deleted successfully:");
                        println!("ID: {}", status.id);
                        println!("Name: {}", status.name);
                        println!("Color: {}", status.color);
                        println!("Display Order: {}", status.display_order);
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to delete status: {e}");
                        std::process::exit(1);
                    }
                }
            }
            #[cfg(feature = "project_writable")]
            ProjectCommands::StatusOrderUpdate {
                project_id_or_key,
                status_ids,
            } => {
                println!(
                    "Updating status order in project: {project_id_or_key} with IDs: {status_ids}"
                );

                let proj_id_or_key = project_id_or_key.parse::<ProjectIdOrKey>()?;

                // Parse comma-separated status IDs
                let parsed_status_ids: Result<Vec<StatusId>, _> = status_ids
                    .split(',')
                    .map(|s| s.trim().parse::<u32>().map(StatusId::new))
                    .collect();

                let status_id_vec = match parsed_status_ids {
                    Ok(ids) => ids,
                    Err(e) => {
                        eprintln!("❌ Error parsing status IDs '{status_ids}': {e}");
                        std::process::exit(1);
                    }
                };

                let params = UpdateStatusOrderParams::new(proj_id_or_key, status_id_vec);

                match client.project().update_status_order(params).await {
                    Ok(statuses) => {
                        println!("✅ Status order updated successfully:");
                        for (index, status) in statuses.iter().enumerate() {
                            println!(
                                "{}. [{}] {} (Color: {})",
                                index + 1,
                                status.id,
                                status.name,
                                status.color
                            );
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to update status order: {e}");
                        std::process::exit(1);
                    }
                }
            }
            #[cfg(not(feature = "project_writable"))]
            _ => {
                eprintln!(
                    "This command requires write access to projects and is not available. \
                    Please build with the 'project_writable' feature flag:\n\
                    cargo build --package blg --features project_writable"
                );
                std::process::exit(1);
            }
            ProjectCommands::PriorityList => {
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
            }
            ProjectCommands::ResolutionList => {
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
            }
            ProjectCommands::Icon {
                project_id_or_key,
                output,
            } => {
                println!("Downloading project icon to {}", output.display());

                let proj_id_or_key = project_id_or_key.parse::<ProjectIdOrKey>()?;
                let params = backlog_project::GetProjectIconParams::new(proj_id_or_key);
                match client.project().get_project_icon(params).await {
                    Ok(icon_bytes) => {
                        if let Err(e) = fs::write(&output, &icon_bytes).await {
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
            }
            ProjectCommands::TeamList { project_id_or_key } => {
                println!("Listing teams for project: {project_id_or_key}");

                let proj_id_or_key = project_id_or_key.parse::<ProjectIdOrKey>()?;
                let params = backlog_project::GetProjectTeamListParams {
                    project_id_or_key: proj_id_or_key,
                };
                match client.project().get_project_team_list(params).await {
                    Ok(teams) => {
                        if teams.is_empty() {
                            println!("No teams found in this project");
                        } else {
                            println!("Teams in this project:");
                            for team in teams {
                                println!("[{}] {}", team.id.value(), team.name);
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
            }
            #[cfg(feature = "project_writable")]
            ProjectCommands::TeamAdd {
                project_id_or_key,
                team_id,
            } => {
                println!("Adding team {team_id} to project: {project_id_or_key}");

                let proj_id_or_key = project_id_or_key.parse::<ProjectIdOrKey>()?;
                let params = AddProjectTeamParams {
                    project_id_or_key: proj_id_or_key,
                    team_id: TeamId::new(team_id),
                };

                match client.project().add_project_team(params).await {
                    Ok(team) => {
                        println!("✅ Team added successfully:");
                        println!("ID: {}", team.id.value());
                        println!("Name: {}", team.name);
                        println!("Members: {} users", team.members.len());
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to add team to project: {e}");
                        std::process::exit(1);
                    }
                }
            }
            #[cfg(feature = "project_writable")]
            ProjectCommands::TeamDelete {
                project_id_or_key,
                team_id,
            } => {
                println!("Removing team {team_id} from project: {project_id_or_key}");

                let proj_id_or_key = project_id_or_key.parse::<ProjectIdOrKey>()?;
                let params = DeleteProjectTeamParams {
                    project_id_or_key: proj_id_or_key,
                    team_id: TeamId::new(team_id),
                };

                match client.project().delete_project_team(params).await {
                    Ok(team) => {
                        println!("✅ Team removed successfully:");
                        println!("ID: {}", team.id.value());
                        println!("Name: {}", team.name);
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to remove team from project: {e}");
                        std::process::exit(1);
                    }
                }
            }
        },
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
                            let datetime = date.and_hms_opt(0, 0, 0).unwrap().and_utc();
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
                            let datetime = date.and_hms_opt(23, 59, 59).unwrap().and_utc();
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

                                if let Some(comment) = &notification.comment {
                                    if let Some(content) = &comment.content {
                                        let preview = content.chars().take(100).collect::<String>();
                                        println!("   Comment: {preview}");
                                    }
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

                let params = params.build().unwrap();

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
        Commands::Wiki(wiki_args) => match wiki_args.command {
            WikiCommands::RecentlyViewed {
                order,
                count,
                offset,
            } => {
                println!("Getting recently viewed wikis");

                let mut params_builder = GetRecentlyViewedWikisParamsBuilder::default();

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

                match client.wiki().get_recently_viewed_wikis(params).await {
                    Ok(wikis) => {
                        if wikis.is_empty() {
                            println!("No recently viewed wikis found");
                        } else {
                            println!("Recently viewed wikis ({} total):", wikis.len());
                            for wiki in wikis {
                                println!("\n[{}] {}", wiki.id.value(), wiki.name);
                                println!("  Project ID: {}", wiki.project_id.value());
                                if !wiki.tags.is_empty() {
                                    let tag_names: Vec<String> =
                                        wiki.tags.iter().map(|t| t.name.clone()).collect();
                                    println!("  Tags: {}", tag_names.join(", "));
                                }
                                println!(
                                    "  Created by: {} at {}",
                                    wiki.created_user.name,
                                    wiki.created.format("%Y-%m-%d %H:%M:%S")
                                );
                                println!(
                                    "  Updated by: {} at {}",
                                    wiki.updated_user.name,
                                    wiki.updated.format("%Y-%m-%d %H:%M:%S")
                                );
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to get recently viewed wikis: {e}");
                        std::process::exit(1);
                    }
                }
            }
            WikiCommands::ListAttachments { wiki_id } => {
                println!("Listing attachments for wiki ID: {wiki_id}");

                match client
                    .wiki()
                    .get_wiki_attachment_list(backlog_wiki::GetWikiAttachmentListParams::new(
                        WikiId::new(wiki_id),
                    ))
                    .await
                {
                    Ok(attachments) => {
                        if attachments.is_empty() {
                            println!("No attachments found for this wiki page");
                        } else {
                            println!("Found {} attachment(s):", attachments.len());
                            for attachment in attachments {
                                println!(
                                    "[{}] {} ({} bytes)",
                                    attachment.id.value(),
                                    attachment.name,
                                    attachment.size
                                );
                                println!(
                                    "  Created by: {} at {}",
                                    attachment.created_user.name,
                                    attachment.created.format("%Y-%m-%d %H:%M:%S")
                                );
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to list wiki attachments: {e}");
                        std::process::exit(1);
                    }
                }
            }
            WikiCommands::ListSharedFiles { wiki_id } => {
                println!("Listing shared files for wiki ID: {wiki_id}");

                match client
                    .wiki()
                    .get_wiki_shared_file_list(backlog_wiki::GetWikiSharedFileListParams::new(
                        WikiId::new(wiki_id),
                    ))
                    .await
                {
                    Ok(shared_files) => {
                        if shared_files.is_empty() {
                            println!("No shared files found linked to this wiki page");
                        } else {
                            println!("Found {} shared file(s):", shared_files.len());
                            for shared_file in shared_files {
                                println!(
                                    "[{}] {} ({} bytes)",
                                    shared_file.id.value(),
                                    shared_file.name,
                                    match &shared_file.content {
                                        backlog_api_client::FileContent::File { size } => *size,
                                        backlog_api_client::FileContent::Directory => 0,
                                    }
                                );
                                println!("  Path: {}", shared_file.dir);
                                println!(
                                    "  Created by: {} at {}",
                                    shared_file.created_user.name,
                                    shared_file.created.format("%Y-%m-%d %H:%M:%S")
                                );
                                if let Some(updated_user) = &shared_file.updated_user {
                                    if let Some(updated) = &shared_file.updated {
                                        println!(
                                            "  Updated by: {} at {}",
                                            updated_user.name,
                                            updated.format("%Y-%m-%d %H:%M:%S")
                                        );
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to list wiki shared files: {e}");
                        std::process::exit(1);
                    }
                }
            }
            WikiCommands::Stars { wiki_id } => {
                println!("Getting stars for wiki ID: {wiki_id}");

                match client
                    .wiki()
                    .get_wiki_stars(backlog_wiki::GetWikiStarsParams::new(WikiId::new(wiki_id)))
                    .await
                {
                    Ok(stars) => {
                        if stars.is_empty() {
                            println!("No stars found for this wiki page");
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
                        eprintln!("❌ Failed to get wiki stars: {e}");
                        std::process::exit(1);
                    }
                }
            }
            #[cfg(feature = "wiki_writable")]
            WikiCommands::LinkSharedFiles { wiki_id, file_ids } => {
                println!(
                    "Linking {} shared file(s) to wiki ID: {}",
                    file_ids.len(),
                    wiki_id
                );

                let shared_file_ids: Vec<backlog_core::identifier::SharedFileId> = file_ids
                    .iter()
                    .map(|&id| backlog_core::identifier::SharedFileId::new(id))
                    .collect();

                let params = backlog_wiki::LinkSharedFilesToWikiParams::new(
                    WikiId::new(wiki_id),
                    shared_file_ids,
                );

                match client.wiki().link_shared_files_to_wiki(params).await {
                    Ok(shared_files) => {
                        println!(
                            "✅ Successfully linked {} shared file(s) to wiki",
                            shared_files.len()
                        );
                        println!();

                        for (index, file) in shared_files.iter().enumerate() {
                            println!("{}. {}", index + 1, file.name);
                            println!("   ID: {}", file.id.value());
                            println!("   Directory: {}", file.dir);
                            match &file.content {
                                backlog_api_client::FileContent::File { size } => {
                                    println!("   Type: File");
                                    println!("   Size: {size} bytes");
                                }
                                backlog_api_client::FileContent::Directory => {
                                    println!("   Type: Directory");
                                }
                            }
                            println!("   Created by: {}", file.created_user.name);
                            println!("   Created at: {}", file.created);
                            if let Some(updated_user) = &file.updated_user {
                                println!("   Updated by: {}", updated_user.name);
                            }
                            if let Some(updated) = &file.updated {
                                println!("   Updated at: {updated}");
                            }
                            println!();
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to link shared files to wiki: {e}");
                        std::process::exit(1);
                    }
                }
            }
            #[cfg(feature = "wiki_writable")]
            WikiCommands::UnlinkSharedFile { wiki_id, file_id } => {
                println!("Unlinking shared file {file_id} from wiki ID: {wiki_id}");

                let params = backlog_wiki::UnlinkSharedFileFromWikiParams::new(
                    WikiId::new(wiki_id),
                    backlog_core::identifier::SharedFileId::new(file_id),
                );

                match client.wiki().unlink_shared_file_from_wiki(params).await {
                    Ok(shared_file) => {
                        println!("✅ Successfully unlinked shared file from wiki:");
                        println!("   Name: {}", shared_file.name);
                        println!("   ID: {}", shared_file.id.value());
                        println!("   Directory: {}", shared_file.dir);
                        match &shared_file.content {
                            backlog_api_client::FileContent::File { size } => {
                                println!("   Type: File");
                                println!("   Size: {size} bytes");
                            }
                            backlog_api_client::FileContent::Directory => {
                                println!("   Type: Directory");
                            }
                        }
                        println!("   Created by: {}", shared_file.created_user.name);
                        println!("   Created at: {}", shared_file.created);
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to unlink shared file from wiki: {e}");
                        std::process::exit(1);
                    }
                }
            }
            WikiCommands::DownloadAttachment {
                wiki_id,
                attachment_id,
                output,
            } => {
                println!("Downloading attachment {attachment_id} from wiki ID: {wiki_id}");

                match client
                    .wiki()
                    .download_wiki_attachment(backlog_wiki::DownloadWikiAttachmentParams::new(
                        WikiId::new(wiki_id),
                        WikiAttachmentId::new(attachment_id),
                    ))
                    .await
                {
                    Ok(downloaded_file) => {
                        let filename = output.unwrap_or(downloaded_file.filename.clone());

                        match tokio::fs::write(&filename, &downloaded_file.bytes).await {
                            Ok(_) => {
                                println!("✅ Successfully downloaded to: {filename}");
                                println!("   Content-Type: {}", downloaded_file.content_type);
                                println!("   File size: {} bytes", downloaded_file.bytes.len());
                            }
                            Err(e) => {
                                eprintln!("❌ Failed to write file '{filename}': {e}");
                                std::process::exit(1);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to download wiki attachment: {e}");
                        std::process::exit(1);
                    }
                }
            }
            #[cfg(feature = "wiki_writable")]
            WikiCommands::Create {
                project_id,
                name,
                content,
                mail_notify,
            } => {
                println!("Creating new wiki page in project: {project_id}");

                let params = AddWikiParams::new(ProjectId::from_str(&project_id)?, name, content);

                let params = if let Some(mail_notify) = mail_notify {
                    params.mail_notify(mail_notify)
                } else {
                    params
                };

                match client.wiki().add_wiki(params).await {
                    Ok(wiki_detail) => {
                        println!("✅ Wiki page created successfully");
                        println!("   ID: {}", wiki_detail.id.value());
                        println!("   Name: {}", wiki_detail.name);
                        println!("   Project ID: {}", wiki_detail.project_id.value());
                        println!(
                            "   Created by: {} at {}",
                            wiki_detail.created_user.name,
                            wiki_detail.created.format("%Y-%m-%d %H:%M:%S")
                        );
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to create wiki page: {e}");
                        std::process::exit(1);
                    }
                }
            }
            #[cfg(feature = "wiki_writable")]
            WikiCommands::Update {
                wiki_id,
                name,
                content,
                mail_notify,
            } => {
                println!("Updating wiki ID: {wiki_id}");

                // Create params with provided options
                let mut params = UpdateWikiParams::new(WikiId::new(wiki_id));

                if let Some(name) = name {
                    params = params.name(name);
                }

                if let Some(content) = content {
                    params = params.content(content);
                }

                if let Some(mail_notify) = mail_notify {
                    params = params.mail_notify(mail_notify);
                }

                match client.wiki().update_wiki(params).await {
                    Ok(wiki_detail) => {
                        println!("✅ Wiki updated successfully");
                        println!("ID: {}", wiki_detail.id.value());
                        println!("Name: {}", wiki_detail.name);
                        println!("Project ID: {}", wiki_detail.project_id.value());
                        println!("Updated by: {}", wiki_detail.updated_user.name);
                        println!(
                            "Updated at: {}",
                            wiki_detail.updated.format("%Y-%m-%d %H:%M:%S")
                        );

                        if !wiki_detail.tags.is_empty() {
                            let tag_names: Vec<String> = wiki_detail
                                .tags
                                .iter()
                                .map(|tag| tag.name.clone())
                                .collect();
                            println!("Tags: {}", tag_names.join(", "));
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to update wiki: {e}");
                        std::process::exit(1);
                    }
                }
            }
            #[cfg(feature = "wiki_writable")]
            WikiCommands::Delete {
                wiki_id,
                mail_notify,
            } => {
                println!("Deleting wiki ID: {wiki_id}");

                let mut params = DeleteWikiParams::new(WikiId::new(wiki_id));

                if let Some(mail_notify) = mail_notify {
                    params = params.mail_notify(mail_notify);
                }

                match client.wiki().delete_wiki(params).await {
                    Ok(wiki_detail) => {
                        println!("✅ Wiki deleted successfully");
                        println!("   ID: {}", wiki_detail.id.value());
                        println!("   Name: {}", wiki_detail.name);
                        println!("   Project ID: {}", wiki_detail.project_id.value());
                        println!(
                            "   Created by: {} at {}",
                            wiki_detail.created_user.name,
                            wiki_detail.created.format("%Y-%m-%d %H:%M:%S")
                        );
                        println!(
                            "   Last updated by: {} at {}",
                            wiki_detail.updated_user.name,
                            wiki_detail.updated.format("%Y-%m-%d %H:%M:%S")
                        );
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to delete wiki: {e}");
                        std::process::exit(1);
                    }
                }
            }
            #[cfg(feature = "wiki_writable")]
            WikiCommands::AttachFile { wiki_id, file_path } => {
                println!("Attaching file to wiki ID: {wiki_id}");

                // Step 1: Upload file to space to get attachment ID
                println!("📤 Uploading file: {}", file_path.display());
                let upload_params = UploadAttachmentParams::new(file_path.clone());

                let attachment = match client.space().upload_attachment(upload_params).await {
                    Ok(attachment) => {
                        println!("✅ File uploaded successfully");
                        println!("   Attachment ID: {}", attachment.id);
                        println!("   File name: {}", attachment.name);
                        println!("   File size: {} bytes", attachment.size);
                        attachment
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to upload file: {e}");
                        std::process::exit(1);
                    }
                };

                // Step 2: Attach the uploaded file to the wiki page
                println!("🔗 Attaching file to wiki page...");
                let attach_params = AttachFilesToWikiParams::new(
                    WikiId::new(wiki_id),
                    vec![AttachmentId::new(attachment.id)],
                );

                match client.wiki().attach_files_to_wiki(attach_params).await {
                    Ok(wiki_attachments) => {
                        println!("✅ File attached to wiki successfully");
                        for attachment in wiki_attachments {
                            println!("   Attachment ID: {}", attachment.id.value());
                            println!("   File name: {}", attachment.name);
                            println!("   File size: {} bytes", attachment.size);
                            println!(
                                "   Attached by: {} at {}",
                                attachment.created_user.name,
                                attachment.created.format("%Y-%m-%d %H:%M:%S")
                            );
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to attach file to wiki: {e}");
                        std::process::exit(1);
                    }
                }
            }
            #[cfg(feature = "wiki_writable")]
            WikiCommands::DeleteAttachment {
                wiki_id,
                attachment_id,
                force,
            } => {
                // Get attachment details before deletion for confirmation
                if !force {
                    print!(
                        "Are you sure you want to delete attachment {attachment_id} from wiki {wiki_id}? [y/N]: "
                    );
                    use std::io::{self, Write};
                    io::stdout().flush().unwrap();

                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();
                    let input = input.trim().to_lowercase();

                    if input != "y" && input != "yes" {
                        println!("Operation cancelled.");
                        return Ok(());
                    }
                }

                println!("🗑️ Deleting attachment {attachment_id} from wiki {wiki_id}...");

                let delete_params = DeleteWikiAttachmentParams::new(
                    WikiId::new(wiki_id),
                    WikiAttachmentId::new(attachment_id),
                );

                match client.wiki().delete_wiki_attachment(delete_params).await {
                    Ok(deleted_attachment) => {
                        println!("✅ Attachment deleted successfully");
                        println!("   Deleted attachment: {}", deleted_attachment.name);
                        println!("   File size: {} bytes", deleted_attachment.size);
                        println!(
                            "   Originally attached by: {} at {}",
                            deleted_attachment.created_user.name,
                            deleted_attachment.created.format("%Y-%m-%d %H:%M:%S")
                        );
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to delete attachment: {e}");
                        std::process::exit(1);
                    }
                }
            }
            WikiCommands::ListTags { project_id } => {
                println!("Listing tags used in wiki pages for project: {project_id}");

                use backlog_wiki::GetWikiTagListParams;
                let params = GetWikiTagListParams::new(project_id.parse::<ProjectIdOrKey>()?);

                match client.wiki().get_wiki_tag_list(params).await {
                    Ok(tags) => {
                        if tags.is_empty() {
                            println!("No tags found in the project");
                        } else {
                            println!("Wiki Tags ({} total):", tags.len());
                            for tag in tags {
                                println!("  {} (ID: {})", tag.name, tag.id.value());
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to get wiki tags: {e}");
                        std::process::exit(1);
                    }
                }
            }
            WikiCommands::History {
                wiki_id,
                min_id,
                max_id,
                count,
                order,
            } => {
                println!("Getting history for wiki ID: {wiki_id}");

                use backlog_wiki::{GetWikiHistoryParams, HistoryOrder};
                let mut params = GetWikiHistoryParams::new(WikiId::new(wiki_id));

                if let Some(min_id) = min_id {
                    params = params.min_id(min_id);
                }
                if let Some(max_id) = max_id {
                    params = params.max_id(max_id);
                }
                if let Some(count) = count {
                    params = params.count(count);
                }
                if let Some(order) = order {
                    let order = match order {
                        HistoryOrderCli::Asc => HistoryOrder::Asc,
                        HistoryOrderCli::Desc => HistoryOrder::Desc,
                    };
                    params = params.order(order);
                }

                match client.wiki().get_wiki_history(params).await {
                    Ok(history) => {
                        if history.is_empty() {
                            println!("No history found for wiki {wiki_id}");
                        } else {
                            println!("Wiki {wiki_id} History ({} entries):", history.len());
                            for entry in &history {
                                println!(
                                    "Version {}: {} (by {} at {})",
                                    entry.version,
                                    entry.name,
                                    entry.created_user.name,
                                    entry.created.format("%Y-%m-%d %H:%M:%S")
                                );
                                if !entry.content.is_empty() {
                                    let preview = truncate_text(&entry.content, 100);
                                    println!("  Content: {preview}");
                                }
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to get wiki history: {e}");
                        std::process::exit(1);
                    }
                }
            }
        },
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
