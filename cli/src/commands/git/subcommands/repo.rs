use crate::commands::common::CliResult;
use backlog_api_client::{ProjectIdOrKey, RepositoryIdOrName, client::BacklogApiClient};

pub(crate) async fn list(client: &BacklogApiClient, project_id: String) -> CliResult<()> {
    println!("Listing repositories for project: {project_id}");
    let proj_id_or_key = project_id.parse::<ProjectIdOrKey>()?;
    // Assumes backlog_git is enabled via features for the client build
    let params = backlog_api_client::GetRepositoryListParams::new(proj_id_or_key);
    let repos = client.git().get_repository_list(params).await?;
    // TODO: Pretty print repositories
    println!("{repos:?}");
    Ok(())
}

pub(crate) async fn show(
    client: &BacklogApiClient,
    project_id: String,
    repo_id: String,
) -> CliResult<()> {
    println!("Showing repository {repo_id} in project: {project_id}");
    let proj_id_or_key = project_id.parse::<ProjectIdOrKey>()?;
    let repo_id_or_name = repo_id.parse::<RepositoryIdOrName>()?;
    let params = backlog_api_client::GetRepositoryParams::new(proj_id_or_key, repo_id_or_name);
    let repo = client.git().get_repository(params).await?;
    // TODO: Pretty print repository
    println!("{repo:?}");
    Ok(())
}
