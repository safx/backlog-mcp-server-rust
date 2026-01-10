#[cfg(feature = "project")]
use super::args::{ActivityArgs, ActivityCommands};
#[cfg(feature = "project")]
use super::subcommands;
#[cfg(feature = "project")]
use crate::commands::common::CliResult;
use backlog_api_client::client::BacklogApiClient;

#[cfg(feature = "project")]
pub async fn execute(client: &BacklogApiClient, activity_args: ActivityArgs) -> CliResult<()> {
    match activity_args.command {
        ActivityCommands::Project {
            project_id,
            type_ids,
            count,
            order,
        } => {
            subcommands::recent::project_recent(client, project_id, type_ids, count, order).await?;
        }
        #[cfg(feature = "space")]
        ActivityCommands::Space {
            type_ids,
            count,
            order,
        } => {
            subcommands::recent::space_recent(client, type_ids, count, order).await?;
        }
    }
    Ok(())
}
