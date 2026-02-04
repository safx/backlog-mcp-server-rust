//! Wiki command handler
//!
//! Dispatches wiki subcommands to their respective implementations.

use crate::commands::common::CliResult;
use crate::commands::wiki::args::{WikiArgs, WikiCommands};
use backlog_api_client::client::BacklogApiClient;

use super::subcommands;

/// Execute a wiki command
pub async fn execute(client: &BacklogApiClient, args: WikiArgs) -> CliResult<()> {
    match args.command {
        // List operations (from subcommands::list)
        WikiCommands::RecentlyViewed {
            order,
            count,
            offset,
        } => subcommands::list::recently_viewed(client, order, count, offset).await?,
        WikiCommands::ListTags { project_id } => {
            subcommands::list::list_tags(client, project_id).await?
        }
        WikiCommands::Stars { wiki_id } => subcommands::list::stars(client, wiki_id).await?,
        WikiCommands::History {
            wiki_id,
            min_id,
            max_id,
            count,
            order,
        } => subcommands::list::history(client, wiki_id, min_id, max_id, count, order).await?,

        // CRUD operations (from subcommands::crud)
        #[cfg(feature = "wiki_writable")]
        WikiCommands::Create {
            project_id,
            name,
            content,
            mail_notify,
        } => subcommands::crud::create(client, project_id, name, content, mail_notify).await?,
        #[cfg(feature = "wiki_writable")]
        WikiCommands::Update {
            wiki_id,
            name,
            content,
            mail_notify,
        } => subcommands::crud::update(client, wiki_id, name, content, mail_notify).await?,
        #[cfg(feature = "wiki_writable")]
        WikiCommands::Delete {
            wiki_id,
            mail_notify,
        } => subcommands::crud::delete(client, wiki_id, mail_notify).await?,

        // Attachments (from subcommands::attachments)
        WikiCommands::ListAttachments { wiki_id } => {
            subcommands::attachments::list_attachments(client, wiki_id).await?
        }
        WikiCommands::DownloadAttachment {
            wiki_id,
            attachment_id,
            output,
        } => {
            subcommands::attachments::download_attachment(client, wiki_id, attachment_id, output)
                .await?
        }
        #[cfg(feature = "wiki_writable")]
        WikiCommands::AttachFile { wiki_id, file_path } => {
            subcommands::attachments::attach_file(client, wiki_id, file_path).await?
        }
        #[cfg(feature = "wiki_writable")]
        WikiCommands::DeleteAttachment {
            wiki_id,
            attachment_id,
            force,
        } => {
            subcommands::attachments::delete_attachment(client, wiki_id, attachment_id, force)
                .await?
        }

        // Shared Files (from subcommands::shared_files)
        WikiCommands::ListSharedFiles { wiki_id } => {
            subcommands::shared_files::list_shared_files(client, wiki_id).await?
        }
        #[cfg(feature = "wiki_writable")]
        WikiCommands::LinkSharedFiles { wiki_id, file_ids } => {
            subcommands::shared_files::link_shared_files(client, wiki_id, file_ids).await?
        }
        #[cfg(feature = "wiki_writable")]
        WikiCommands::UnlinkSharedFile { wiki_id, file_id } => {
            subcommands::shared_files::unlink_shared_file(client, wiki_id, file_id).await?
        }

        // Fallback for disabled write features
        #[cfg(not(feature = "wiki_writable"))]
        _ => {
            anyhow::bail!(
                "This command requires write access to wikis and is not available. \
                Please build with the 'wiki_writable' feature flag:\n\
                cargo build --package blg --features wiki_writable"
            );
        }
    }
    Ok(())
}
