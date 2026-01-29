use clap::{Args, Parser};

#[cfg(feature = "document")]
#[derive(Args)]
pub struct DocumentArgs {
    #[clap(subcommand)]
    pub command: DocumentCommands,
}

#[cfg(feature = "document")]
#[derive(Parser)]
pub enum DocumentCommands {
    /// List documents in a project
    List {
        /// Project ID or Key (required)
        #[clap(short, long)]
        project_id: String,
        /// Search keyword
        #[clap(short, long)]
        keyword: Option<String>,
        /// Sort key (created, updated, title)
        #[clap(short, long)]
        sort: Option<String>,
        /// Sort order (asc or desc)
        #[clap(short, long)]
        order: Option<String>,
        /// Pagination offset
        #[clap(long)]
        offset: Option<u32>,
        /// Number of items to retrieve (default: 20, max: 100)
        #[clap(short, long)]
        count: Option<u32>,
        /// Output in JSON format
        #[clap(long)]
        json: bool,
    },
    /// Get document details
    Get {
        /// Document ID (32-character hex string)
        #[clap(name = "DOCUMENT_ID")]
        document_id: String,
        /// Output in JSON format
        #[clap(long)]
        json: bool,
    },
    /// Get document tree structure
    Tree {
        /// Project ID or Key (required)
        #[clap(short, long)]
        project_id: String,
        /// Output in JSON format
        #[clap(long)]
        json: bool,
    },
    /// Download attachment from a document
    Download {
        /// Document ID
        #[clap(name = "DOCUMENT_ID")]
        document_id: String,
        /// Attachment ID
        #[clap(name = "ATTACHMENT_ID")]
        attachment_id: u32,
        /// Output file path (if not specified, use original filename)
        #[clap(short, long)]
        output: Option<String>,
    },
    #[cfg(feature = "document_writable")]
    /// Create a new document
    Add {
        /// Project ID (required, numeric only)
        #[clap(short, long)]
        project_id: String,
        /// Document title
        #[clap(short, long)]
        title: String,
        /// Document content
        #[clap(short, long)]
        content: Option<String>,
        /// Emoji
        #[clap(short, long)]
        emoji: Option<String>,
        /// Parent document ID (32-char hex)
        #[clap(long)]
        parent_id: Option<String>,
        /// Add as last child of parent
        #[clap(long)]
        add_last: bool,
        /// Output in JSON format
        #[clap(long)]
        json: bool,
    },
    #[cfg(feature = "document_writable")]
    /// Delete a document
    Delete {
        /// Document ID (32-char hex)
        #[clap(name = "DOCUMENT_ID")]
        document_id: String,
        /// Output in JSON format
        #[clap(long)]
        json: bool,
    },
}
