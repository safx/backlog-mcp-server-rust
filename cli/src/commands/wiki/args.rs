use clap::{Args, Parser};
use std::path::PathBuf;

#[cfg(feature = "wiki")]
#[derive(Args)]
pub struct WikiArgs {
    #[clap(subcommand)]
    pub command: WikiCommands,
}

#[cfg(feature = "wiki")]
#[derive(Parser)]
pub enum WikiCommands {
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
pub enum HistoryOrderCli {
    Asc,
    Desc,
}
