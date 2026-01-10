//! Issue command handler
//!
//! Dispatches issue subcommands to their respective implementations.

use crate::commands::common::CliResult;
use crate::commands::issue::args::{IssueArgs, IssueCommands};
use backlog_api_client::client::BacklogApiClient;

use super::subcommands;

/// Execute an issue command
pub async fn execute(client: &BacklogApiClient, args: IssueArgs) -> CliResult<()> {
    match args.command {
        // List, Show, RecentlyViewed (from subcommands::list)
        IssueCommands::List { params } => subcommands::list::list(client, params).await,
        IssueCommands::Show { issue_id_or_key } => {
            subcommands::list::show(client, &issue_id_or_key).await
        }
        IssueCommands::RecentlyViewed {
            order,
            count,
            offset,
        } => subcommands::list::recently_viewed(client, order, count, offset).await,

        // CRUD operations (from subcommands::crud)
        #[cfg(feature = "issue_writable")]
        IssueCommands::Create(create_args) => subcommands::crud::create(client, create_args).await,
        #[cfg(feature = "issue_writable")]
        IssueCommands::Update(update_args) => subcommands::crud::update(client, update_args).await,
        #[cfg(feature = "issue_writable")]
        IssueCommands::Delete(delete_args) => {
            subcommands::crud::delete(client, delete_args.issue_key).await
        }

        // Comments (from subcommands::comments)
        #[cfg(feature = "issue_writable")]
        IssueCommands::AddComment(add_args) => {
            subcommands::comments::add_comment(client, add_args).await
        }
        #[cfg(feature = "issue_writable")]
        IssueCommands::UpdateComment(args) => {
            subcommands::comments::update_comment(client, args).await
        }
        #[cfg(feature = "issue_writable")]
        IssueCommands::DeleteComment(args) => {
            subcommands::comments::delete_comment(client, args).await
        }
        IssueCommands::CountComment(count_args) => {
            subcommands::comments::count_comment(client, count_args.issue_id_or_key).await
        }
        IssueCommands::GetComment(get_args) => {
            subcommands::comments::get_comment(
                client,
                get_args.issue_id_or_key,
                get_args.comment_id,
            )
            .await
        }
        IssueCommands::GetCommentNotifications(get_args) => {
            subcommands::comments::get_comment_notifications(
                client,
                get_args.issue_id_or_key,
                get_args.comment_id,
            )
            .await
        }
        #[cfg(feature = "issue_writable")]
        IssueCommands::AddCommentNotification(add_args) => {
            subcommands::comments::add_comment_notification(
                client,
                add_args.issue_id_or_key,
                add_args.comment_id,
                add_args.users,
            )
            .await
        }

        // Attachments (from subcommands::attachments)
        IssueCommands::DownloadAttachment(dl_args) => {
            subcommands::attachments::download_attachment(client, dl_args).await
        }
        #[cfg(feature = "issue_writable")]
        IssueCommands::DeleteAttachment(args) => {
            subcommands::attachments::delete_attachment(client, args.issue_id, args.attachment_id)
                .await
        }

        // Shared Files (from subcommands::shared_files)
        IssueCommands::ListSharedFiles { issue_id_or_key } => {
            subcommands::shared_files::list_shared_files(client, issue_id_or_key).await
        }
        #[cfg(feature = "issue_writable")]
        IssueCommands::LinkSharedFiles {
            issue_id_or_key,
            file_ids,
        } => subcommands::shared_files::link_shared_files(client, issue_id_or_key, file_ids).await,
        #[cfg(feature = "issue_writable")]
        IssueCommands::UnlinkSharedFile {
            issue_id_or_key,
            file_id,
        } => subcommands::shared_files::unlink_shared_file(client, issue_id_or_key, file_id).await,

        // Participants (from subcommands::participants)
        IssueCommands::ListParticipants { issue_id_or_key } => {
            subcommands::participants::list_participants(client, issue_id_or_key).await
        }

        // Recently Viewed Write (from subcommands::list)
        #[cfg(feature = "issue_writable")]
        IssueCommands::AddRecentlyViewed { issue_id_or_key } => {
            subcommands::list::add_recently_viewed(client, issue_id_or_key).await
        }

        // Fallback for disabled write features
        #[cfg(not(feature = "issue_writable"))]
        _ => {
            eprintln!(
                "This command requires write access to issues and is not available. \
                Please build with the 'issue_writable' feature flag:\n\
                cargo build --package blg --features issue_writable"
            );
            std::process::exit(1);
        }
    }
}
