//! User command handler
//!
//! Dispatches user subcommands to their respective implementations.

use crate::commands::common::CliResult;
use crate::commands::user::args::{UserArgs, UserCommands};
use backlog_api_client::client::BacklogApiClient;

use super::subcommands;

/// Execute a user command
pub async fn execute(client: &BacklogApiClient, args: UserArgs) -> CliResult<()> {
    match args.command {
        // Info operations (from subcommands::info)
        UserCommands::List => subcommands::info::list(client).await?,
        UserCommands::Me => subcommands::info::me(client).await?,
        UserCommands::Show { user_id } => subcommands::info::show(client, user_id).await?,
        UserCommands::Icon { user_id, output } => {
            subcommands::info::icon(client, user_id, output).await?
        }

        // Star operations (from subcommands::stars)
        UserCommands::StarCount {
            user_id,
            since,
            until,
        } => subcommands::stars::star_count(client, user_id, since, until).await?,
        UserCommands::Stars {
            user_id,
            min_id,
            max_id,
            count,
            order,
        } => subcommands::stars::stars(client, user_id, min_id, max_id, count, order).await?,

        // Notification operations (from subcommands::notifications)
        UserCommands::NotificationCount {
            already_read,
            resource_already_read,
        } => {
            subcommands::notifications::notification_count(
                client,
                already_read,
                resource_already_read,
            )
            .await?
        }
        UserCommands::Notifications {
            min_id,
            max_id,
            count,
            order,
            sender_id,
        } => {
            subcommands::notifications::notifications(
                client, min_id, max_id, count, order, sender_id,
            )
            .await?
        }
        #[cfg(feature = "user_writable")]
        UserCommands::MarkNotificationRead { notification_id } => {
            subcommands::notifications::mark_notification_read(client, notification_id).await?
        }
        #[cfg(feature = "user_writable")]
        UserCommands::ResetNotifications => {
            subcommands::notifications::reset_notifications(client).await?
        }

        // Watching operations (from subcommands::watchings)
        UserCommands::Watchings {
            user_id,
            order,
            sort,
            count,
            offset,
            resource_already_read,
            issue_ids,
        } => {
            subcommands::watchings::watchings(
                client,
                user_id,
                order,
                sort,
                count,
                offset,
                resource_already_read,
                issue_ids,
            )
            .await?
        }
        UserCommands::WatchingCount {
            user_id,
            resource_already_read,
            already_read,
        } => {
            subcommands::watchings::watching_count(
                client,
                user_id,
                resource_already_read,
                already_read,
            )
            .await?
        }

        // Fallback for disabled write features
        #[cfg(not(feature = "user_writable"))]
        _ => {
            anyhow::bail!(
                "This command requires write access to users and is not available. \
                Please build with the 'user_writable' feature flag:\n\
                cargo build --package blg --features user_writable"
            );
        }
    }
    Ok(())
}
