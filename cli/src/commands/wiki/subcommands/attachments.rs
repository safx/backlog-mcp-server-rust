use crate::commands::common::CliResult;
use backlog_api_client::client::BacklogApiClient;
use backlog_core::identifier::{AttachmentId, Identifier, WikiAttachmentId, WikiId};
use backlog_space::UploadAttachmentParams;
use backlog_wiki::{
    AttachFilesToWikiParams, DeleteWikiAttachmentParams, DownloadWikiAttachmentParams,
    GetWikiAttachmentListParams,
};
use std::path::PathBuf;

/// List attachments for a wiki page
pub(crate) async fn list_attachments(client: &BacklogApiClient, wiki_id: u32) -> CliResult<()> {
    println!("Listing attachments for wiki ID: {wiki_id}");

    let attachments = client
        .wiki()
        .get_wiki_attachment_list(GetWikiAttachmentListParams::new(WikiId::new(wiki_id)))
        .await?;

    if attachments.is_empty() {
        println!("No attachments found for this wiki page");
    } else {
        println!("Found {} attachment(s):", attachments.len());
        for attachment in attachments {
            println!(
                "[{}] {} ({} bytes)",
                attachment.id.value(),
                attachment.name,
                attachment.size
            );
            println!(
                "  Created by: {} at {}",
                attachment.created_user.name,
                attachment.created.format("%Y-%m-%d %H:%M:%S")
            );
        }
    }

    Ok(())
}

/// Download a wiki attachment
pub(crate) async fn download_attachment(
    client: &BacklogApiClient,
    wiki_id: u32,
    attachment_id: u32,
    output: Option<String>,
) -> CliResult<()> {
    println!("Downloading attachment {attachment_id} from wiki ID: {wiki_id}");

    let downloaded_file = client
        .wiki()
        .download_wiki_attachment(DownloadWikiAttachmentParams::new(
            WikiId::new(wiki_id),
            WikiAttachmentId::new(attachment_id),
        ))
        .await?;

    let filename = output.unwrap_or(downloaded_file.filename.clone());
    tokio::fs::write(&filename, &downloaded_file.bytes).await?;

    println!("✅ Successfully downloaded to: {filename}");
    println!("   Content-Type: {}", downloaded_file.content_type);
    println!("   File size: {} bytes", downloaded_file.bytes.len());

    Ok(())
}

/// Attach a file to a wiki page (2-step operation: upload to space, then attach to wiki)
#[cfg(feature = "wiki_writable")]
pub(crate) async fn attach_file(
    client: &BacklogApiClient,
    wiki_id: u32,
    file_path: PathBuf,
) -> CliResult<()> {
    println!("Attaching file to wiki ID: {wiki_id}");

    // Step 1: Upload file to space to get attachment ID
    println!("📤 Uploading file: {}", file_path.display());
    let upload_params = UploadAttachmentParams::new(file_path.clone());

    let attachment = client.space().upload_attachment(upload_params).await?;
    println!("✅ File uploaded successfully");
    println!("   Attachment ID: {}", attachment.id);
    println!("   File name: {}", attachment.name);
    println!("   File size: {} bytes", attachment.size);

    // Step 2: Attach the uploaded file to the wiki page
    println!("🔗 Attaching file to wiki page...");
    let attach_params =
        AttachFilesToWikiParams::new(WikiId::new(wiki_id), vec![AttachmentId::new(attachment.id)]);

    let wiki_attachments = client.wiki().attach_files_to_wiki(attach_params).await?;
    println!("✅ File attached to wiki successfully");
    for attachment in wiki_attachments {
        println!("   Attachment ID: {}", attachment.id.value());
        println!("   File name: {}", attachment.name);
        println!("   File size: {} bytes", attachment.size);
        println!(
            "   Attached by: {} at {}",
            attachment.created_user.name,
            attachment.created.format("%Y-%m-%d %H:%M:%S")
        );
    }

    Ok(())
}

/// Delete an attachment from a wiki page
#[cfg(feature = "wiki_writable")]
pub(crate) async fn delete_attachment(
    client: &BacklogApiClient,
    wiki_id: u32,
    attachment_id: u32,
    force: bool,
) -> CliResult<()> {
    // Get attachment details before deletion for confirmation
    if !force {
        print!(
            "Are you sure you want to delete attachment {attachment_id} from wiki {wiki_id}? [y/N]: "
        );
        use std::io::{self, Write};
        io::stdout()
            .flush()
            .map_err(|e| format!("Failed to flush stdout: {}", e))?;

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|e| format!("Failed to read input: {}", e))?;
        let input = input.trim().to_lowercase();

        if input != "y" && input != "yes" {
            println!("Operation cancelled.");
            return Ok(());
        }
    }

    println!("🗑️ Deleting attachment {attachment_id} from wiki {wiki_id}...");

    let delete_params =
        DeleteWikiAttachmentParams::new(WikiId::new(wiki_id), WikiAttachmentId::new(attachment_id));

    let deleted_attachment = client.wiki().delete_wiki_attachment(delete_params).await?;
    println!("✅ Attachment deleted successfully");
    println!("   Deleted attachment: {}", deleted_attachment.name);
    println!("   File size: {} bytes", deleted_attachment.size);
    println!(
        "   Originally attached by: {} at {}",
        deleted_attachment.created_user.name,
        deleted_attachment.created.format("%Y-%m-%d %H:%M:%S")
    );

    Ok(())
}
