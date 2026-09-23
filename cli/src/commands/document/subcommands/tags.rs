use crate::commands::common::CliResult;
use backlog_api_client::client::BacklogApiClient;
use backlog_document::{AddDocumentTagParams, RemoveDocumentTagParams};

pub(crate) async fn add(
    client: &BacklogApiClient,
    document_id: String,
    tag_names: Vec<String>,
    json: bool,
) -> CliResult<()> {
    let params = AddDocumentTagParams::new(document_id.trim().parse()?, tag_names);
    params.validate()?;
    let tags = client.document().add_document_tag(params).await?;
    if json {
        println!("{}", serde_json::to_string_pretty(&tags)?);
    } else {
        for tag in tags {
            println!("{}: {}", tag.id, tag.name);
        }
    }
    Ok(())
}

pub(crate) async fn remove(
    client: &BacklogApiClient,
    document_id: String,
    tag_names: Vec<String>,
    json: bool,
) -> CliResult<()> {
    let params = RemoveDocumentTagParams::new(document_id.trim().parse()?, tag_names);
    params.validate()?;
    client.document().remove_document_tag(params).await?;
    if json {
        println!("{}", serde_json::json!({"success": true}));
    } else {
        println!("Tags removed from document {}", document_id.trim());
    }
    Ok(())
}
