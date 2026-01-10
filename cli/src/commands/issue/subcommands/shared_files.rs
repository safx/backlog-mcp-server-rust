//! Shared file operations for issues
//!
//! This module provides handlers for managing shared files linked to issues:
//! - Listing shared files
//! - Linking shared files to issues
//! - Unlinking shared files from issues

use crate::commands::common::CliResult;
use backlog_api_client::client::BacklogApiClient;
use backlog_api_client::{IssueIdOrKey, LinkSharedFilesToIssueParamsBuilder};
use backlog_core::identifier::SharedFileId;
use backlog_issue::{GetSharedFileListParams, UnlinkSharedFileParams};
use std::str::FromStr;

/// List shared files linked to an issue
///
/// Corresponds to `GET /api/v2/issues/:issueIdOrKey/sharedFiles`
pub async fn list_shared_files(
    client: &BacklogApiClient,
    issue_id_or_key: String,
) -> CliResult<()> {
    println!("Listing shared files for issue: {issue_id_or_key}");

    let parsed_issue_id_or_key = IssueIdOrKey::from_str(&issue_id_or_key)
        .map_err(|e| format!("Failed to parse issue_id_or_key '{issue_id_or_key}': {e}"))?;

    match client
        .issue()
        .get_shared_file_list(GetSharedFileListParams::new(parsed_issue_id_or_key))
        .await
    {
        Ok(shared_files) => {
            if shared_files.is_empty() {
                println!("No shared files found for this issue.");
            } else {
                println!("Found {} shared file(s):", shared_files.len());
                println!();

                for (index, file) in shared_files.iter().enumerate() {
                    println!("{}. {}", index + 1, file.name);
                    println!("   ID: {}", file.id);
                    println!("   Directory: {}", file.dir);
                    match &file.content {
                        backlog_issue::models::FileContent::File { size } => {
                            println!("   Type: File");
                            println!("   Size: {size} bytes");
                        }
                        backlog_issue::models::FileContent::Directory => {
                            println!("   Type: Directory");
                        }
                    }
                    println!("   Created by: {}", file.created_user.name);
                    println!("   Created at: {}", file.created);
                    if let Some(updated_user) = &file.updated_user {
                        println!("   Updated by: {}", updated_user.name);
                    }
                    if let Some(updated) = &file.updated {
                        println!("   Updated at: {updated}");
                    }
                    println!();
                }
            }
        }
        Err(e) => {
            eprintln!("Error listing shared files: {e}");
        }
    }
    Ok(())
}

/// Link shared files to an issue
///
/// Corresponds to `POST /api/v2/issues/:issueIdOrKey/sharedFiles`
#[cfg(feature = "issue_writable")]
pub async fn link_shared_files(
    client: &BacklogApiClient,
    issue_id_or_key: String,
    file_ids: Vec<u32>,
) -> CliResult<()> {
    println!(
        "Linking {} shared file(s) to issue: {}",
        file_ids.len(),
        issue_id_or_key
    );

    let parsed_issue_id_or_key = IssueIdOrKey::from_str(&issue_id_or_key)
        .map_err(|e| format!("Failed to parse issue_id_or_key '{issue_id_or_key}': {e}"))?;

    let shared_file_ids: Vec<SharedFileId> =
        file_ids.iter().map(|&id| SharedFileId::new(id)).collect();

    let params = LinkSharedFilesToIssueParamsBuilder::default()
        .issue_id_or_key(parsed_issue_id_or_key)
        .shared_file_ids(shared_file_ids)
        .build()
        .map_err(|e| format!("Failed to build parameters: {e}"))?;

    match client.issue().link_shared_files_to_issue(params).await {
        Ok(linked_files) => {
            println!(
                "✅ Successfully linked {} shared file(s) to the issue!",
                linked_files.len()
            );
            println!();

            for (index, file) in linked_files.iter().enumerate() {
                println!("{}. {}", index + 1, file.name);
                println!("   ID: {}", file.id);
                println!("   Directory: {}", file.dir);
                match &file.content {
                    backlog_issue::models::FileContent::File { size } => {
                        println!("   Type: File");
                        println!("   Size: {size} bytes");
                    }
                    backlog_issue::models::FileContent::Directory => {
                        println!("   Type: Directory");
                    }
                }
                println!("   Created by: {}", file.created_user.name);
                println!("   Created at: {}", file.created);
                println!();
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to link shared files to issue: {e}");
            std::process::exit(1);
        }
    }
    Ok(())
}

/// Unlink a shared file from an issue
///
/// Corresponds to `DELETE /api/v2/issues/:issueIdOrKey/sharedFiles/:id`
#[cfg(feature = "issue_writable")]
pub async fn unlink_shared_file(
    client: &BacklogApiClient,
    issue_id_or_key: String,
    file_id: u32,
) -> CliResult<()> {
    println!("Unlinking shared file {file_id} from issue: {issue_id_or_key}");

    let parsed_issue_id_or_key = IssueIdOrKey::from_str(&issue_id_or_key)
        .map_err(|e| format!("Failed to parse issue_id_or_key '{issue_id_or_key}': {e}"))?;

    let params = UnlinkSharedFileParams::new(parsed_issue_id_or_key, SharedFileId::new(file_id));

    match client.issue().unlink_shared_file(params).await {
        Ok(unlinked_file) => {
            println!("✅ Successfully unlinked shared file from the issue!");
            println!("   Name: {}", unlinked_file.name);
            println!("   ID: {}", unlinked_file.id);
            println!("   Directory: {}", unlinked_file.dir);
            match &unlinked_file.content {
                backlog_issue::models::FileContent::File { size } => {
                    println!("   Type: File");
                    println!("   Size: {size} bytes");
                }
                backlog_issue::models::FileContent::Directory => {
                    println!("   Type: Directory");
                }
            }
            println!("   Created by: {}", unlinked_file.created_user.name);
            println!("   Created at: {}", unlinked_file.created);
        }
        Err(e) => {
            eprintln!("❌ Failed to unlink shared file from issue: {e}");
            std::process::exit(1);
        }
    }
    Ok(())
}
