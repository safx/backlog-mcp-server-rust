#[cfg(feature = "space")]
use super::args::{SpaceArgs, SpaceCommands};
#[cfg(feature = "space")]
use super::subcommands;
#[cfg(feature = "space")]
use crate::commands::common::CliResult;
#[cfg(feature = "space")]
use backlog_api_client::client::BacklogApiClient;

#[cfg(feature = "space")]
pub async fn execute(client: &BacklogApiClient, space_args: SpaceArgs) -> CliResult<()> {
    match space_args.command {
        SpaceCommands::Logo { output } => {
            subcommands::info::logo(client, output).await?;
        }
        SpaceCommands::DiskUsage { format } => {
            subcommands::info::disk_usage(client, format).await?;
        }
        SpaceCommands::Licence { format } => {
            subcommands::info::licence(client, format).await?;
        }
        #[cfg(feature = "space_writable")]
        SpaceCommands::UploadAttachment { file } => {
            subcommands::writable::upload_attachment(client, file).await?;
        }
        #[cfg(feature = "space_writable")]
        SpaceCommands::UpdateNotification { content } => {
            subcommands::writable::update_notification(client, content).await?;
        }
        #[cfg(not(feature = "space_writable"))]
        _ => {
            return Err(
                "This command requires write access to space and is not available. \
                Please build with the 'space_writable' feature flag:\
\
                cargo build --package blg --features space_writable"
                    .into(),
            );
        }
    }

    Ok(())
}
