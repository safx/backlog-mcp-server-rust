#[cfg(feature = "space_writable")]
use crate::commands::common::CliResult;
#[cfg(feature = "space_writable")]
use backlog_api_client::client::BacklogApiClient;
#[cfg(feature = "space_writable")]
use backlog_space::{UpdateSpaceNotificationParams, UploadAttachmentParams};
#[cfg(feature = "space_writable")]
use std::path::PathBuf;

#[cfg(feature = "space_writable")]
pub(crate) async fn upload_attachment(client: &BacklogApiClient, file: PathBuf) -> CliResult<()> {
    println!("Uploading attachment: {}", file.display());

    // Check if file exists
    if !file.exists() {
        anyhow::bail!("File does not exist: {}", file.display());
    }

    let params = UploadAttachmentParams::new(file.clone());

    let attachment = client.space().upload_attachment(params).await?;
    println!("✅ Attachment uploaded successfully");
    println!("Attachment ID: {}", attachment.id);
    println!("Filename: {}", attachment.name);
    println!("Size: {} bytes", attachment.size);
    Ok(())
}

#[cfg(feature = "space_writable")]
pub(crate) async fn update_notification(
    client: &BacklogApiClient,
    content: String,
) -> CliResult<()> {
    println!("Updating space notification...");

    let params = UpdateSpaceNotificationParams::new(content.clone());

    let notification = client.space().update_space_notification(params).await?;
    println!("✅ Space notification updated successfully");
    println!("Content: {}", notification.content);
    println!(
        "Updated: {}",
        notification.updated.format("%Y-%m-%d %H:%M:%S UTC")
    );
    Ok(())
}
