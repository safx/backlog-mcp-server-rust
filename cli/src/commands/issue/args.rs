use clap::{Args, Parser};
use std::path::PathBuf;

#[derive(Parser)]
pub struct IssueArgs {
    #[clap(subcommand)]
    pub command: IssueCommands,
}

#[derive(Parser)]
pub enum IssueCommands {
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
pub struct DownloadAttachmentArgs {
    /// The ID or key of the issue (e.g., "PROJECT-123" or "12345")
    pub issue_id_or_key: String,

    /// The numeric ID of the attachment to download
    pub attachment_id: u32,

    /// Output file path to save the attachment
    #[arg(short, long, value_name = "FILE_PATH")]
    pub output: PathBuf,
}

#[derive(Args, Debug)]
pub struct AddCommentArgs {
    /// The ID or key of the issue (e.g., "PROJECT-123" or "12345")
    pub issue_id_or_key: String,

    /// The comment content
    #[arg(short, long)]
    pub content: String,

    /// User IDs to notify (comma-separated, e.g., "123,456")
    #[arg(short, long)]
    pub notify_users: Option<String>,

    /// Attachment IDs to include (comma-separated, e.g., "789,101112")
    #[arg(short, long)]
    pub attachments: Option<String>,
}

#[cfg(feature = "issue_writable")]
#[derive(Args, Debug)]
pub struct UpdateCommentArgs {
    /// Issue ID or key (e.g., 'PROJECT-123')
    #[clap(short, long)]
    pub issue_id: String,

    /// Comment ID to update
    #[clap(short = 'c', long)]
    pub comment_id: u32,

    /// New content for the comment
    #[clap(short = 'n', long)]
    pub content: String,
}

#[cfg(feature = "issue_writable")]
#[derive(Args, Debug)]
pub struct DeleteCommentArgs {
    /// Issue ID or key (e.g., 'PROJECT-123')
    #[clap(short, long)]
    pub issue_id: String,

    /// Comment ID to delete
    #[clap(short = 'c', long)]
    pub comment_id: u32,
}

#[cfg(feature = "issue_writable")]
#[derive(Args, Debug)]
pub struct DeleteAttachmentArgs {
    /// Issue ID or key (e.g., 'PROJECT-123')
    #[clap(short, long)]
    pub issue_id: String,

    /// Attachment ID to delete
    #[clap(short = 'a', long)]
    pub attachment_id: u32,
}

#[derive(Args, Debug)]
pub struct CreateIssueArgs {
    /// Project ID or Key
    #[arg(short, long)]
    pub project_id: String,

    /// Issue summary (title)
    #[arg(short, long)]
    pub summary: String,

    /// Issue type ID
    #[arg(short = 't', long)]
    pub issue_type_id: u32,

    /// Priority ID
    #[arg(long)]
    pub priority_id: u32,

    /// Issue description
    #[arg(short, long)]
    pub description: Option<String>,

    /// Assignee user ID
    #[arg(short, long)]
    pub assignee_id: Option<u32>,

    /// Due date (YYYY-MM-DD format)
    #[arg(long)]
    pub due_date: Option<String>,

    /// Category IDs (comma-separated)
    #[arg(short, long)]
    pub category_ids: Option<String>,

    /// Milestone IDs (comma-separated)
    #[arg(short, long)]
    pub milestone_ids: Option<String>,

    /// Custom fields (format: "id:type:value[:other]", can be specified multiple times)
    /// Examples:
    /// --custom-field "1:text:Sample text"
    /// --custom-field "2:numeric:123.45"
    /// --custom-field "3:date:2024-06-24"
    /// --custom-field "4:single_list:100:Other description"
    #[arg(long = "custom-field", value_name = "FIELD")]
    pub custom_fields: Vec<String>,

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
    pub custom_fields_json: Option<std::path::PathBuf>,
}

#[derive(Args, Debug)]
pub struct UpdateIssueArgs {
    /// Issue ID or Key
    pub issue_id_or_key: String,

    /// Issue summary (title)
    #[arg(short, long)]
    pub summary: Option<String>,

    /// Issue description
    #[arg(short, long)]
    pub description: Option<String>,

    /// Issue type ID
    #[arg(short = 't', long)]
    pub issue_type_id: Option<u32>,

    /// Priority ID
    #[arg(long)]
    pub priority_id: Option<u32>,

    /// Status ID
    #[arg(long)]
    pub status_id: Option<String>,

    /// Assignee user ID
    #[arg(short, long)]
    pub assignee_id: Option<u32>,

    /// Resolution ID
    #[arg(short, long)]
    pub resolution_id: Option<u32>,

    /// Due date (YYYY-MM-DD format)
    #[arg(long)]
    pub due_date: Option<String>,

    /// Comment to add with the update
    #[arg(short, long)]
    pub comment: Option<String>,

    /// Custom fields (format: "id:type:value[:other]", can be specified multiple times)
    #[arg(long = "custom-field", value_name = "FIELD")]
    pub custom_fields: Vec<String>,

    /// Custom fields JSON file path
    #[arg(
        long = "custom-fields-json",
        value_name = "FILE",
        conflicts_with = "custom_fields"
    )]
    pub custom_fields_json: Option<std::path::PathBuf>,
}

#[derive(Args, Debug)]
pub struct DeleteIssueArgs {
    /// Issue Key (e.g., "PROJECT-123")
    pub issue_key: String,
}

#[derive(Args, Debug)]
pub struct CountCommentArgs {
    /// The ID or key of the issue (e.g., "PROJECT-123" or "12345")
    pub issue_id_or_key: String,
}

#[derive(Args, Debug)]
pub struct GetCommentArgs {
    /// The ID or key of the issue (e.g., "PROJECT-123" or "12345")
    pub issue_id_or_key: String,
    /// The ID of the comment
    pub comment_id: u32,
}

#[cfg(feature = "issue_writable")]
#[derive(Args, Debug)]
pub struct AddCommentNotificationArgs {
    /// The ID or key of the issue (e.g., "PROJECT-123" or "12345")
    pub issue_id_or_key: String,
    /// The ID of the comment
    pub comment_id: u32,
    /// User IDs to notify (comma-separated, e.g., "123,456")
    #[arg(short, long)]
    pub users: String,
}

#[derive(Parser, Debug, Default)]
pub struct IssueListCliParams {
    /// Filter by project ID(s)
    #[clap(long)]
    pub project_id: Option<Vec<String>>,
    /// Filter by assignee ID(s)
    #[clap(long)]
    pub assignee_id: Option<Vec<String>>,
    /// Filter by status ID(s)
    #[clap(long)]
    pub status_id: Option<Vec<String>>,
    /// Keyword to search for in summary or description
    #[clap(long)]
    pub keyword: Option<String>,
    /// Number of issues to retrieve (1-100, default 20)
    #[clap(long, default_value_t = 20)]
    pub count: u32,
    /// Filter by start date (since). Format: YYYY-MM-DD
    #[clap(long)]
    pub start_date_since: Option<String>,
    /// Filter by start date (until). Format: YYYY-MM-DD
    #[clap(long)]
    pub start_date_until: Option<String>,
    /// Filter by due date (since). Format: YYYY-MM-DD
    #[clap(long)]
    pub due_date_since: Option<String>,
    /// Filter by due date (until). Format: YYYY-MM-DD
    #[clap(long)]
    pub due_date_until: Option<String>,
    // TODO: Add more filters like sort, order, offset, issue_type_id, etc.
}
