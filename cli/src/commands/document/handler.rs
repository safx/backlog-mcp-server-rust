//! Document command handler
//!
//! Dispatches document subcommands to their respective implementations.

use crate::commands::common::CliResult;
use crate::commands::document::args::{DocumentArgs, DocumentCommands};
use backlog_api_client::client::BacklogApiClient;

use super::subcommands;

/// Execute a document command
pub async fn execute(client: &BacklogApiClient, args: DocumentArgs) -> CliResult<()> {
    match args.command {
        DocumentCommands::List {
            project_id,
            keyword,
            sort,
            order,
            offset,
            count,
            json,
        } => {
            subcommands::list::list(
                client,
                subcommands::list::ListOptions {
                    project_id,
                    keyword,
                    sort,
                    order,
                    offset,
                    count,
                    json,
                },
            )
            .await?
        }
        DocumentCommands::Get { document_id, json } => {
            subcommands::list::get(client, document_id, json).await?
        }
        DocumentCommands::Tree { project_id, json } => {
            subcommands::list::tree(client, project_id, json).await?
        }
        DocumentCommands::Download {
            document_id,
            attachment_id,
            output,
        } => subcommands::list::download(client, document_id, attachment_id, output).await?,
    }
    Ok(())
}
