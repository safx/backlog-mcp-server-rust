use clap::Parser;
use std::path::PathBuf;

#[cfg(feature = "space")]
#[derive(Parser)]
pub struct SpaceArgs {
    #[clap(subcommand)]
    pub command: SpaceCommands,
}

#[cfg(feature = "space")]
#[derive(Parser)]
pub enum SpaceCommands {
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
