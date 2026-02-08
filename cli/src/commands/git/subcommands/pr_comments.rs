use crate::commands::common::CliResult;
use anyhow::Context;
use backlog_api_client::{
    ProjectIdOrKey, PullRequestCommentId, PullRequestNumber, RepositoryIdOrName,
    client::BacklogApiClient,
};
use backlog_core::identifier::Identifier;

pub(crate) async fn comment_count(
    client: &BacklogApiClient,
    project_id: String,
    repo_id: String,
    pr_number: u64,
) -> CliResult<()> {
    println!("Getting comment count for PR #{pr_number} in repo {repo_id} (project {project_id})");

    let parsed_project_id: ProjectIdOrKey = project_id
        .parse()
        .with_context(|| format!("Failed to parse project_id '{project_id}'"))?;
    let parsed_repo_id: RepositoryIdOrName = repo_id
        .parse()
        .with_context(|| format!("Failed to parse repo_id '{repo_id}'"))?;
    let parsed_pr_number = PullRequestNumber::from(pr_number);

    let params = backlog_api_client::GetPullRequestCommentCountParams::new(
        parsed_project_id,
        parsed_repo_id,
        parsed_pr_number,
    );
    let count_response = client.git().get_pull_request_comment_count(params).await?;
    println!("✅ Pull request comment count retrieved successfully");
    println!("Comment count: {}", count_response.count);
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

    let parsed_project_id: ProjectIdOrKey = project_id
        .parse()
        .with_context(|| format!("Failed to parse project_id '{project_id}'"))?;
    let parsed_repo_id: RepositoryIdOrName = repo_id
        .parse()
        .with_context(|| format!("Failed to parse repo_id '{repo_id}'"))?;
    let parsed_pr_number = PullRequestNumber::from(pr_number);
    let parsed_comment_id = PullRequestCommentId::new(comment_id);

    let params = backlog_api_client::UpdatePullRequestCommentParams::new(
        parsed_project_id,
        parsed_repo_id,
        parsed_pr_number,
        parsed_comment_id,
        &content,
    );

    let comment = client.git().update_pull_request_comment(params).await?;
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
    Ok(())
}
