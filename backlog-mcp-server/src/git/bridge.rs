use crate::access_control::AccessControl;
use crate::error::{Error, Result};
#[cfg(feature = "git_writable")]
use crate::git::request::AddPullRequestCommentRequest;
use crate::git::request::{
    DownloadPullRequestAttachmentRequest, GetPullRequestAttachmentListRequest,
    GetPullRequestCommentListRequest, GetPullRequestDetailsRequest, GetRepositoryDetailsRequest,
    GetRepositoryListRequest, ListPullRequestsRequest,
};
#[cfg(feature = "git_writable")]
use backlog_api_client::AddPullRequestCommentParams;
use backlog_api_client::client::BacklogApiClient;
use backlog_api_client::{
    DownloadPullRequestAttachmentParams, DownloadedFile, GetPullRequestAttachmentListParams,
    GetPullRequestCommentListParams, GetPullRequestListParams, GetPullRequestParams,
    GetRepositoryListParams, GetRepositoryParams, ProjectIdOrKey, PullRequest,
    PullRequestAttachment, PullRequestAttachmentId, PullRequestComment, PullRequestNumber,
    Repository, RepositoryIdOrName,
};
use std::{str::FromStr, sync::Arc};
use tokio::sync::Mutex;

pub(crate) async fn get_repository_list(
    client: Arc<Mutex<BacklogApiClient>>,
    req: GetRepositoryListRequest,
    access_control: &AccessControl,
) -> Result<Vec<Repository>> {
    let project_id = req.project_id_or_key.parse::<ProjectIdOrKey>()?;

    let client_guard = client.lock().await;

    access_control
        .check_project_access_id_or_key_async(&project_id, &client_guard)
        .await?;

    let params = GetRepositoryListParams::new(project_id);
    let repositories = client_guard.git().get_repository_list(params).await?;
    Ok(repositories)
}

pub(crate) async fn get_repository(
    client: Arc<Mutex<BacklogApiClient>>,
    req: GetRepositoryDetailsRequest,
    access_control: &AccessControl,
) -> Result<Repository> {
    let proj_id_or_key = req.project_id_or_key.parse::<ProjectIdOrKey>()?;
    let repo_id_or_name = RepositoryIdOrName::from_str(req.repo_id_or_name.trim())?;

    let client_guard = client.lock().await;

    access_control
        .check_project_access_id_or_key_async(&proj_id_or_key, &client_guard)
        .await?;

    let params = GetRepositoryParams::new(proj_id_or_key, repo_id_or_name);
    let repository = client_guard.git().get_repository(params).await?;
    Ok(repository)
}

pub(crate) async fn get_pull_request_list(
    client: Arc<Mutex<BacklogApiClient>>,
    req: ListPullRequestsRequest,
    access_control: &AccessControl,
) -> Result<Vec<PullRequest>> {
    let proj_id_or_key = req.project_id_or_key.parse::<ProjectIdOrKey>()?;
    let repo_id_or_name = RepositoryIdOrName::from_str(req.repo_id_or_name.trim())?;

    let client_guard = client.lock().await;

    access_control
        .check_project_access_id_or_key_async(&proj_id_or_key, &client_guard)
        .await?;

    let params = GetPullRequestListParams::new(proj_id_or_key, repo_id_or_name);
    let pull_requests = client_guard.git().get_pull_request_list(params).await?;
    Ok(pull_requests)
}

pub(crate) async fn get_pull_request(
    client: Arc<Mutex<BacklogApiClient>>,
    req: GetPullRequestDetailsRequest,
    access_control: &AccessControl,
) -> Result<PullRequest> {
    let proj_id_or_key = req.project_id_or_key.parse::<ProjectIdOrKey>()?;
    let repo_id_or_name = RepositoryIdOrName::from_str(req.repo_id_or_name.trim())?;
    let pr_number = PullRequestNumber::from(req.pr_number);

    let client_guard = client.lock().await;

    access_control
        .check_project_access_id_or_key_async(&proj_id_or_key, &client_guard)
        .await?;

    let params = GetPullRequestParams::new(proj_id_or_key, repo_id_or_name, pr_number);
    let pull_request = client_guard.git().get_pull_request(params).await?;
    Ok(pull_request)
}

pub(crate) async fn get_pull_request_attachment_list_tool(
    client: Arc<Mutex<BacklogApiClient>>,
    req: GetPullRequestAttachmentListRequest,
    access_control: &AccessControl,
) -> Result<Vec<PullRequestAttachment>> {
    let project_id_or_key = req.project_id_or_key.parse::<ProjectIdOrKey>()?;
    let repo_id_or_name = RepositoryIdOrName::from_str(req.repo_id_or_name.trim())?;
    let pr_number = PullRequestNumber::from(req.pr_number);

    let client_guard = client.lock().await;

    access_control
        .check_project_access_id_or_key_async(&project_id_or_key, &client_guard)
        .await?;

    let params =
        GetPullRequestAttachmentListParams::new(project_id_or_key, repo_id_or_name, pr_number);
    Ok(client_guard
        .git()
        .get_pull_request_attachment_list(params)
        .await?)
}

pub(crate) async fn download_pr_attachment_bridge(
    client: Arc<Mutex<BacklogApiClient>>,
    req: DownloadPullRequestAttachmentRequest,
    access_control: &AccessControl,
) -> Result<DownloadedFile> {
    let project_id_or_key = req.project_id_or_key.parse::<ProjectIdOrKey>()?;
    let repo_id_or_name = RepositoryIdOrName::from_str(req.repo_id_or_name.trim())?;
    let pr_number = PullRequestNumber::from(req.pr_number);
    let attachment_id_for_download = PullRequestAttachmentId::new(req.attachment_id);

    let client_guard = client.lock().await;

    access_control
        .check_project_access_id_or_key_async(&project_id_or_key, &client_guard)
        .await?;

    let params = DownloadPullRequestAttachmentParams::new(
        project_id_or_key,
        repo_id_or_name,
        pr_number,
        attachment_id_for_download,
    );

    client_guard
        .git()
        .download_pull_request_attachment(params)
        .await
        .map_err(Error::from)
}

pub(crate) async fn get_pull_request_comment_list_tool(
    client: Arc<Mutex<BacklogApiClient>>,
    req: GetPullRequestCommentListRequest,
    access_control: &AccessControl,
) -> Result<Vec<PullRequestComment>> {
    let project_id_or_key = req.project_id_or_key.parse::<ProjectIdOrKey>()?;
    let params = GetPullRequestCommentListParams::try_from(req)?;

    let client_guard = client.lock().await;

    access_control
        .check_project_access_id_or_key_async(&project_id_or_key, &client_guard)
        .await?;

    Ok(client_guard
        .git()
        .get_pull_request_comment_list(params)
        .await?)
}

#[cfg(feature = "git_writable")]
pub(crate) async fn add_pull_request_comment_bridge(
    client: Arc<Mutex<BacklogApiClient>>,
    req: AddPullRequestCommentRequest,
    access_control: &AccessControl,
) -> Result<PullRequestComment> {
    let project_id_or_key = req.project_id_or_key.parse::<ProjectIdOrKey>()?;
    let params = AddPullRequestCommentParams::try_from(req)?;

    let client_guard = client.lock().await;

    access_control
        .check_project_access_id_or_key_async(&project_id_or_key, &client_guard)
        .await?;

    Ok(client_guard.git().add_pull_request_comment(params).await?)
}
