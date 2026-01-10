use clap::{Args, Parser};
use std::path::PathBuf;

#[cfg(feature = "git")]
#[derive(Parser)]
pub struct RepoArgs {
    #[clap(subcommand)]
    pub command: RepoCommands,
}

#[cfg(feature = "git")]
#[derive(Parser)]
pub enum RepoCommands {
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

#[cfg(feature = "git")]
#[derive(Parser)]
pub struct PrArgs {
    #[clap(subcommand)]
    pub command: PrCommands,
}

#[cfg(feature = "git")]
#[derive(Parser)]
pub enum PrCommands {
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

#[cfg(feature = "git")]
#[derive(Args, Debug)]
pub struct DownloadPrAttachmentArgs {
    /// Project ID or Key
    #[clap(short = 'p', long)]
    pub project_id: String,
    /// Repository ID or Name
    #[clap(short = 'r', long)]
    pub repo_id: String,
    /// Pull Request number
    #[clap(short = 'n', long)]
    pub pr_number: u64,
    /// The numeric ID of the attachment to download
    #[clap(short = 'a', long)]
    pub attachment_id: u32,
    /// Output file path to save the attachment
    #[clap(short = 'o', long, value_name = "FILE_PATH")]
    pub output: PathBuf,
}

#[cfg(feature = "git_writable")]
#[derive(Args, Debug)]
pub struct DeletePrAttachmentArgs {
    /// Project ID or Key
    #[clap(short = 'p', long)]
    pub project_id: String,
    /// Repository ID or Name
    #[clap(short = 'r', long)]
    pub repo_id: String,
    /// Pull Request number
    #[clap(short = 'n', long)]
    pub pr_number: u64,
    /// The numeric ID of the attachment to delete
    #[clap(short = 'a', long)]
    pub attachment_id: u32,
}
