use clap::{Args, Parser};
use std::path::PathBuf;

#[cfg(feature = "user")]
#[derive(Args)]
pub struct UserArgs {
    #[clap(subcommand)]
    pub command: UserCommands,
}

#[cfg(feature = "user")]
#[derive(Parser)]
pub enum UserCommands {
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
