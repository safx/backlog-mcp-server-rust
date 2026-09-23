use crate::commands::common::CliResult;
use backlog_api_client::client::BacklogApiClient;
use backlog_core::ProjectIdOrKey;
use backlog_document::GetDocumentCountParams;

pub(crate) async fn count(
    client: &BacklogApiClient,
    project_id: String,
    json: bool,
) -> CliResult<()> {
    let project: ProjectIdOrKey = project_id.trim().parse()?;
    let result = client
        .document()
        .get_document_count(GetDocumentCountParams::new(project))
        .await?;
    if json {
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        println!("{}", result.count);
    }
    Ok(())
}
