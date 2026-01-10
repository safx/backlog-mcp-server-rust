use crate::commands::common::CliResult;
use backlog_api_client::{
    ProjectIdOrKey, PullRequestCommentId, PullRequestNumber, RepositoryIdOrName,
    client::BacklogApiClient,
};
use backlog_core::identifier::Identifier;
use std::str::FromStr;

pub(crate) async fn comment_count(
    client: &BacklogApiClient,
    project_id: String,
    repo_id: String,
    pr_number: u64,
) -> CliResult<()> {
    println!("Getting comment count for PR #{pr_number} in repo {repo_id} (project {project_id})");

    let parsed_project_id = ProjectIdOrKey::from_str(&project_id)
        .map_err(|e| format!("Failed to parse project_id '{project_id}': {e}"))?;
    let parsed_repo_id = RepositoryIdOrName::from_str(&repo_id)
        .map_err(|e| format!("Failed to parse repo_id '{repo_id}': {e}"))?;
    let parsed_pr_number = PullRequestNumber::from(pr_number);

    let params = backlog_api_client::GetPullRequestCommentCountParams::new(
        parsed_project_id,
        parsed_repo_id,
        parsed_pr_number,
    );
    match client.git().get_pull_request_comment_count(params).await {
        Ok(count_response) => {
            println!("✅ Pull request comment count retrieved successfully");
            println!("Comment count: {}", count_response.count);
        }
        Err(e) => {
            eprintln!("❌ Failed to get pull request comment count: {e}");
            std::process::exit(1);
        }
    }
    Ok(())
}

#[cfg(feature = "git_writable")]
pub(crate) async fn comment_update(
    client: &BacklogApiClient,
    project_id: String,
    repo_id: String,
    pr_number: u64,
    comment_id: u32,
    content: String,
) -> CliResult<()> {
    println!(
        "Updating comment {comment_id} for PR #{pr_number} in repo {repo_id} (project {project_id})"
    );

    let parsed_project_id = ProjectIdOrKey::from_str(&project_id)
        .map_err(|e| format!("Failed to parse project_id '{project_id}': {e}"))?;
    let parsed_repo_id = RepositoryIdOrName::from_str(&repo_id)
        .map_err(|e| format!("Failed to parse repo_id '{repo_id}': {e}"))?;
    let parsed_pr_number = PullRequestNumber::from(pr_number);
    let parsed_comment_id = PullRequestCommentId::new(comment_id);

    let params = backlog_api_client::UpdatePullRequestCommentParams::new(
        parsed_project_id,
        parsed_repo_id,
        parsed_pr_number,
        parsed_comment_id,
        &content,
    );

    match client.git().update_pull_request_comment(params).await {
        Ok(comment) => {
            println!("✅ Pull request comment updated successfully");
            println!("Comment ID: {}", comment.id.value());
            println!("Content: {}", comment.content);
            println!(
                "Created by: {} (ID: {})",
                comment.created_user.name,
                comment.created_user.id.value()
            );
            println!("Created: {}", comment.created);
            println!("Updated: {}", comment.updated);
        }
        Err(e) => {
            eprintln!("❌ Failed to update pull request comment: {e}");
            std::process::exit(1);
        }
    }
    Ok(())
}
