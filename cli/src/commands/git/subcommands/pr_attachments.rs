use crate::commands::common::CliResult;
use crate::commands::git::args::DownloadPrAttachmentArgs;
use backlog_api_client::{
    ProjectIdOrKey, PullRequestAttachmentId, PullRequestNumber, RepositoryIdOrName,
    client::BacklogApiClient,
};
use backlog_core::identifier::Identifier;
use std::str::FromStr;
use tokio::fs;

#[cfg(feature = "git_writable")]
use crate::commands::git::args::DeletePrAttachmentArgs;

pub(crate) async fn download_attachment(
    client: &BacklogApiClient,
    dl_args: DownloadPrAttachmentArgs,
) -> CliResult<()> {
    println!(
        "Downloading attachment {} for PR #{} in repo {} (project {}) to {}",
        dl_args.attachment_id,
        dl_args.pr_number,
        dl_args.repo_id,
        dl_args.project_id,
        dl_args.output.display()
    );

    let parsed_project_id = ProjectIdOrKey::from_str(&dl_args.project_id)
        .map_err(|e| format!("Failed to parse project_id '{}': {}", dl_args.project_id, e))?;
    let parsed_repo_id = RepositoryIdOrName::from_str(&dl_args.repo_id)
        .map_err(|e| format!("Failed to parse repo_id '{}': {}", dl_args.repo_id, e))?;
    let parsed_attachment_id = PullRequestAttachmentId::new(dl_args.attachment_id);

    let parsed_pr_number = PullRequestNumber::from(dl_args.pr_number);

    let params = backlog_api_client::DownloadPullRequestAttachmentParams::new(
        parsed_project_id,
        parsed_repo_id,
        parsed_pr_number,
        parsed_attachment_id,
    );
    match client.git().download_pull_request_attachment(params).await {
        Ok(downloaded_file) => {
            if let Err(e) = fs::write(&dl_args.output, &downloaded_file.bytes).await {
                eprintln!(
                    "Error writing attachment to {}: {}",
                    dl_args.output.display(),
                    e
                );
            } else {
                println!(
                    "Attachment downloaded successfully to: {}",
                    dl_args.output.display()
                );
            }
        }
        Err(e) => {
            eprintln!("Error downloading PR attachment: {e}");
        }
    }
    Ok(())
}

#[cfg(feature = "git_writable")]
pub(crate) async fn delete_attachment(
    client: &BacklogApiClient,
    del_args: DeletePrAttachmentArgs,
) -> CliResult<()> {
    println!(
        "Deleting attachment {} from PR #{} in repo {} (project {})",
        del_args.attachment_id, del_args.pr_number, del_args.repo_id, del_args.project_id
    );

    let parsed_project_id = ProjectIdOrKey::from_str(&del_args.project_id).map_err(|e| {
        format!(
            "Failed to parse project_id '{}': {}",
            del_args.project_id, e
        )
    })?;
    let parsed_repo_id = RepositoryIdOrName::from_str(&del_args.repo_id)
        .map_err(|e| format!("Failed to parse repo_id '{}': {}", del_args.repo_id, e))?;
    let parsed_attachment_id = PullRequestAttachmentId::new(del_args.attachment_id);
    let parsed_pr_number = PullRequestNumber::from(del_args.pr_number);

    let params = backlog_api_client::DeletePullRequestAttachmentParams::new(
        parsed_project_id,
        parsed_repo_id,
        parsed_pr_number,
        parsed_attachment_id,
    );
    match client.git().delete_pull_request_attachment(params).await {
        Ok(deleted_attachment) => {
            println!("✅ Attachment deleted successfully");
            println!("Deleted attachment ID: {}", deleted_attachment.id.value());
            println!("Name: {}", deleted_attachment.name);
            println!("Size: {} bytes", deleted_attachment.size);
        }
        Err(e) => {
            eprintln!("❌ Failed to delete PR attachment: {e}");
            std::process::exit(1);
        }
    }
    Ok(())
}
