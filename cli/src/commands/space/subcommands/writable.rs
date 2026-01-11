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
        eprintln!("Error: File does not exist: {}", file.display());
        std::process::exit(1);
    }

    let params = UploadAttachmentParams::new(file.clone());

    match client.space().upload_attachment(params).await {
        Ok(attachment) => {
            println!("✅ Attachment uploaded successfully");
            println!("Attachment ID: {}", attachment.id);
            println!("Filename: {}", attachment.name);
            println!("Size: {} bytes", attachment.size);
        }
        Err(e) => {
            eprintln!("❌ Failed to upload attachment: {e}");
            std::process::exit(1);
        }
    }
    Ok(())
}

#[cfg(feature = "space_writable")]
pub(crate) async fn update_notification(
    client: &BacklogApiClient,
    content: String,
) -> CliResult<()> {
    println!("Updating space notification...");

    let params = UpdateSpaceNotificationParams::new(content.clone());

    match client.space().update_space_notification(params).await {
        Ok(notification) => {
            println!("✅ Space notification updated successfully");
            println!("Content: {}", notification.content);
            println!(
                "Updated: {}",
                notification.updated.format("%Y-%m-%d %H:%M:%S UTC")
            );
        }
        Err(e) => {
            eprintln!("❌ Failed to update space notification: {e}");
            std::process::exit(1);
        }
    }
    Ok(())
}
