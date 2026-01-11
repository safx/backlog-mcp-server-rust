//! Attachment operations for issues
//!
//! This module provides handlers for managing issue attachments:
//! - Downloading attachments
//! - Deleting attachments

use crate::commands::common::CliResult;
use backlog_api_client::client::BacklogApiClient;
use backlog_api_client::{AttachmentId, IssueIdOrKey};
use backlog_core::IssueKey;
use backlog_issue::{DeleteAttachmentParams, GetAttachmentFileParams};
use std::str::FromStr;
use tokio::fs;

/// Download an issue attachment
///
/// Corresponds to `GET /api/v2/issues/:issueIdOrKey/attachments/:attachmentId`
pub async fn download_attachment(
    client: &BacklogApiClient,
    args: crate::commands::issue::args::DownloadAttachmentArgs,
) -> CliResult<()> {
    println!(
        "Downloading attachment {} for issue {} to {}",
        args.attachment_id,
        args.issue_id_or_key,
        args.output.display()
    );

    let parsed_issue_id_or_key = IssueIdOrKey::from_str(&args.issue_id_or_key).map_err(|e| {
        format!(
            "Failed to parse issue_id_or_key '{}': {}",
            args.issue_id_or_key, e
        )
    })?;

    let parsed_attachment_id = AttachmentId::new(args.attachment_id);

    let params = GetAttachmentFileParams::new(parsed_issue_id_or_key, parsed_attachment_id);
    match client.issue().get_attachment_file(params).await {
        Ok(downloaded_file) => {
            if let Err(e) = fs::write(&args.output, &downloaded_file.bytes).await {
                eprintln!(
                    "Error writing attachment to {}: {}",
                    args.output.display(),
                    e
                );
            } else {
                println!(
                    "Attachment downloaded successfully to: {}",
                    args.output.display()
                );
            }
        }
        Err(e) => {
            eprintln!("Error downloading attachment: {e}");
        }
    }
    Ok(())
}

/// Delete an attachment from an issue
///
/// Corresponds to `DELETE /api/v2/issues/:issueIdOrKey/attachments/:attachmentId`
#[cfg(feature = "issue_writable")]
pub async fn delete_attachment(
    client: &BacklogApiClient,
    issue_id: String,
    attachment_id: u32,
) -> CliResult<()> {
    let params = DeleteAttachmentParams {
        issue_id_or_key: issue_id.parse::<IssueKey>()?.into(),
        attachment_id: AttachmentId::new(attachment_id),
    };

    let attachment = client.issue().delete_attachment(params).await?;
    println!("✅ Attachment deleted successfully");
    println!("Deleted Attachment ID: {}", attachment.id);
    println!("Deleted File Name: {}", attachment.name);
    println!("File Size: {} bytes", attachment.size);
    println!("Originally Created: {}", attachment.created);
    Ok(())
}
