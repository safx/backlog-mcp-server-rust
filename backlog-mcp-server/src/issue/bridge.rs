use super::request::{
    AddCommentRequest, DownloadAttachmentRequest, GetAttachmentListRequest,
    GetIssueCommentsRequest, GetIssueDetailsRequest, GetIssueSharedFilesRequest,
    GetIssuesByMilestoneNameRequest, GetVersionMilestoneListRequest, UpdateIssueRequest,
};
#[cfg(feature = "issue_writable")]
use super::request::{AddIssueRequest, UpdateCommentRequest};
use crate::access_control::AccessControl;
use crate::error::{Error as McpError, Result};
use crate::util::{MatchResult, find_by_name_from_array};
#[cfg(feature = "issue_writable")]
use backlog_api_client::backlog_issue::AddIssueParamsBuilder;
use backlog_api_client::client::BacklogApiClient;
use backlog_api_client::{
    AddCommentParams, Attachment, AttachmentId, Comment, DownloadedFile, GetCommentListParams,
    GetIssueListParamsBuilder, Issue, IssueIdOrKey, IssueKey, IssueSharedFile, Milestone,
    ProjectIdOrKey, UpdateIssueParams, backlog_issue, backlog_project,
};
#[cfg(feature = "issue_writable")]
use backlog_core::identifier::{IssueTypeId, PriorityId, ProjectId};
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::Mutex;

pub(crate) async fn get_issue_details(
    client: Arc<Mutex<BacklogApiClient>>,
    req: GetIssueDetailsRequest,
    access_control: &AccessControl,
) -> Result<Issue> {
    let client_guard = client.lock().await;
    let parsed_issue_key = IssueKey::from_str(req.issue_key.trim())?;
    let issue = client_guard
        .issue()
        .get_issue(backlog_issue::GetIssueParams::new(parsed_issue_key.clone()))
        .await?;

    // Check project access from the response
    access_control
        .check_project_access_by_id_async(&issue.project_id, &client_guard)
        .await?;

    Ok(issue)
}

pub(crate) async fn get_version_milestone_list(
    client: Arc<Mutex<BacklogApiClient>>,
    req: GetVersionMilestoneListRequest,
    access_control: &AccessControl,
) -> Result<Vec<Milestone>> {
    let client_guard = client.lock().await;
    let proj_id_or_key = ProjectIdOrKey::from_str(req.project_id_or_key.trim())?;

    // Check project access with parsed type
    access_control
        .check_project_access_id_or_key_async(&proj_id_or_key, &client_guard)
        .await?;
    let versions = client_guard
        .project()
        .get_version_milestone_list(backlog_project::GetMilestoneListParams::new(proj_id_or_key))
        .await?;
    Ok(versions)
}

pub(crate) async fn get_issues_by_milestone_name(
    client: Arc<Mutex<BacklogApiClient>>,
    req: GetIssuesByMilestoneNameRequest,
    access_control: &AccessControl,
) -> Result<Vec<Issue>> {
    let proj_id_or_key = ProjectIdOrKey::from_str(req.project_id_or_key.trim())?;

    let client_guard = client.lock().await;

    // Check project access with parsed type
    access_control
        .check_project_access_id_or_key_async(&proj_id_or_key, &client_guard)
        .await?;

    let all_project_milestones = client_guard
        .project()
        .get_version_milestone_list(backlog_project::GetMilestoneListParams::new(
            proj_id_or_key.clone(),
        ))
        .await?;

    let milestone =
        find_milestone_by_name(&all_project_milestones, &req.milestone_name, proj_id_or_key)?;
    let params = GetIssueListParamsBuilder::default()
        .project_id(vec![milestone.project_id])
        .milestone_id(vec![milestone.id])
        .build()?;

    let issues = client_guard.issue().get_issue_list(params).await?;
    Ok(issues)
}

fn find_milestone_by_name(
    milestones: &[Milestone],
    milestone_name: &str,
    project_id_or_key: ProjectIdOrKey,
) -> Result<Milestone> {
    match find_by_name_from_array(milestones, milestone_name, |m| &m.name) {
        MatchResult::Exact(milestone) => Ok(milestone),
        MatchResult::Suggestion(suggestions) => Err(McpError::MilestoneNotFoundByName {
            project_id_or_key,
            original_name: milestone_name.to_string(),
            suggestions: Some(suggestions),
        }),
        MatchResult::None => Err(McpError::MilestoneNotFoundByName {
            project_id_or_key,
            original_name: milestone_name.to_string(),
            suggestions: None,
        }),
    }
}

#[cfg(feature = "issue_writable")]
pub(crate) async fn update_issue_impl(
    client: Arc<Mutex<BacklogApiClient>>,
    req: UpdateIssueRequest,
    access_control: &AccessControl,
) -> Result<Issue> {
    if req.summary.is_none() && req.description.is_none() && req.custom_fields.is_none() {
        return Err(McpError::NothingToUpdate);
    }

    let client_guard = client.lock().await;

    // First get issue details to check project access
    let parsed_issue_id_or_key = IssueIdOrKey::from_str(req.issue_id_or_key.trim())?;
    let issue = client_guard
        .issue()
        .get_issue(backlog_issue::GetIssueParams::new(
            parsed_issue_id_or_key.clone(),
        ))
        .await?;

    access_control
        .check_project_access_by_id_async(&issue.project_id, &client_guard)
        .await?;

    // Convert base params
    let mut update_params = UpdateIssueParams::try_from(req.clone())?;

    // Handle custom fields if provided
    if let Some(custom_fields_by_name) = req.custom_fields {
        let project_id_or_key = ProjectIdOrKey::from(issue.project_id);
        let custom_fields = crate::issue::custom_field_converter::resolve_custom_fields(
            &client_guard,
            &project_id_or_key,
            custom_fields_by_name,
        )
        .await?;

        // Set custom fields on params
        update_params.custom_fields = Some(custom_fields);
    }

    let updated_issue = client_guard.issue().update_issue(update_params).await?;
    Ok(updated_issue)
}

pub(crate) async fn get_issue_comments_impl(
    client: Arc<Mutex<BacklogApiClient>>,
    req: GetIssueCommentsRequest,
    access_control: &AccessControl,
) -> Result<Vec<Comment>> {
    let comment_params = GetCommentListParams::try_from(req.clone())?;

    let client_guard = client.lock().await;

    // First get issue details to check project access
    let parsed_issue_id_or_key = IssueIdOrKey::from_str(req.issue_id_or_key.trim())?;
    let issue = client_guard
        .issue()
        .get_issue(backlog_issue::GetIssueParams::new(parsed_issue_id_or_key))
        .await?;

    access_control
        .check_project_access_by_id_async(&issue.project_id, &client_guard)
        .await?;

    let comments = client_guard
        .issue()
        .get_comment_list(comment_params)
        .await?;
    Ok(comments)
}

pub(crate) async fn get_attachment_list_impl(
    client: Arc<Mutex<BacklogApiClient>>,
    req: GetAttachmentListRequest,
    access_control: &AccessControl,
) -> Result<Vec<Attachment>> {
    let parsed_issue_id_or_key =
        IssueIdOrKey::from_str(req.issue_id_or_key.trim()).map_err(|e| {
            McpError::Parameter(format!(
                "Invalid issueIdOrKey: {}. Error: {}",
                req.issue_id_or_key, e
            ))
        })?;

    let client_guard = client.lock().await;

    // First get issue details to check project access
    let issue = client_guard
        .issue()
        .get_issue(backlog_issue::GetIssueParams::new(
            parsed_issue_id_or_key.clone(),
        ))
        .await?;

    access_control
        .check_project_access_by_id_async(&issue.project_id, &client_guard)
        .await?;

    let attachments = client_guard
        .issue()
        .get_attachment_list(backlog_issue::GetAttachmentListParams::new(
            parsed_issue_id_or_key.clone(),
        ))
        .await?;
    Ok(attachments)
}

pub(crate) async fn download_issue_attachment_file(
    client: Arc<Mutex<BacklogApiClient>>,
    req: DownloadAttachmentRequest,
    access_control: &AccessControl,
) -> Result<DownloadedFile> {
    let parsed_issue_id_or_key = IssueIdOrKey::from_str(&req.issue_id_or_key)?;
    let parsed_attachment_id = AttachmentId::new(req.attachment_id);

    let client_guard = client.lock().await;

    // First get issue details to check project access
    let issue = client_guard
        .issue()
        .get_issue(backlog_issue::GetIssueParams::new(
            parsed_issue_id_or_key.clone(),
        ))
        .await?;

    access_control
        .check_project_access_by_id_async(&issue.project_id, &client_guard)
        .await?;

    let params =
        backlog_issue::GetAttachmentFileParams::new(parsed_issue_id_or_key, parsed_attachment_id);
    let attachment = client_guard.issue().get_attachment_file(params).await?;
    Ok(attachment)
}

#[cfg(feature = "issue_writable")]
pub(crate) async fn add_comment_impl(
    client: Arc<Mutex<BacklogApiClient>>,
    req: AddCommentRequest,
    access_control: &AccessControl,
) -> Result<Comment> {
    let add_comment_params = AddCommentParams::try_from(req.clone())?;

    let client_guard = client.lock().await;

    // First get issue details to check project access
    let parsed_issue_id_or_key = IssueIdOrKey::from_str(req.issue_id_or_key.trim())?;
    let issue = client_guard
        .issue()
        .get_issue(backlog_issue::GetIssueParams::new(parsed_issue_id_or_key))
        .await?;

    access_control
        .check_project_access_by_id_async(&issue.project_id, &client_guard)
        .await?;

    let comment = client_guard.issue().add_comment(add_comment_params).await?;
    Ok(comment)
}

#[cfg(feature = "issue_writable")]
pub(crate) async fn update_comment_impl(
    client: Arc<Mutex<BacklogApiClient>>,
    req: UpdateCommentRequest,
    access_control: &AccessControl,
) -> Result<Comment> {
    use backlog_api_client::backlog_issue::UpdateCommentParams;
    use backlog_core::identifier::CommentId;

    let parsed_issue_id_or_key = IssueIdOrKey::from_str(req.issue_id_or_key.trim())?;
    let comment_id = CommentId::new(req.comment_id);

    let client_guard = client.lock().await;

    // Phase 3: First get issue details to check project access
    let issue = client_guard
        .issue()
        .get_issue(backlog_issue::GetIssueParams::new(
            parsed_issue_id_or_key.clone(),
        ))
        .await?;

    access_control
        .check_project_access_by_id_async(&issue.project_id, &client_guard)
        .await?;

    let params = UpdateCommentParams {
        issue_id_or_key: parsed_issue_id_or_key,
        comment_id,
        content: req.content,
    };

    let comment = client_guard.issue().update_comment(params).await?;
    Ok(comment)
}

pub(crate) async fn get_issue_shared_files_impl(
    client: Arc<Mutex<BacklogApiClient>>,
    req: GetIssueSharedFilesRequest,
    access_control: &AccessControl,
) -> Result<Vec<IssueSharedFile>> {
    let parsed_issue_id_or_key = IssueIdOrKey::from_str(req.issue_id_or_key.trim())?;

    let client_guard = client.lock().await;

    // First get issue details to check project access
    let issue = client_guard
        .issue()
        .get_issue(backlog_issue::GetIssueParams::new(
            parsed_issue_id_or_key.clone(),
        ))
        .await?;

    access_control
        .check_project_access_by_id_async(&issue.project_id, &client_guard)
        .await?;

    let shared_files = client_guard
        .issue()
        .get_shared_file_list(backlog_issue::GetSharedFileListParams::new(
            parsed_issue_id_or_key,
        ))
        .await?;
    Ok(shared_files)
}

#[cfg(feature = "issue_writable")]
pub(crate) async fn add_issue_impl(
    client: Arc<Mutex<BacklogApiClient>>,
    req: AddIssueRequest,
    access_control: &AccessControl,
) -> Result<Issue> {
    let project_id_or_key = ProjectIdOrKey::from_str(req.project_id_or_key.trim())?;
    let issue_type_id = IssueTypeId::new(req.issue_type_id);
    let priority_id = PriorityId::new(req.priority_id);

    let client_guard = client.lock().await;

    let project_id = match &project_id_or_key {
        ProjectIdOrKey::Id(id) => *id,
        ProjectIdOrKey::Key(key) => {
            let project = access_control
                .project_cache()
                .get_by_key(key, &client_guard)
                .await?;
            project.id
        }
        ProjectIdOrKey::EitherIdOrKey(id, _) => *id,
    };

    // Check project access with resolved ID
    access_control
        .check_project_access_by_id_async(&project_id, &client_guard)
        .await?;

    let mut builder = AddIssueParamsBuilder::default();
    builder
        .project_id(project_id)
        .summary(req.summary)
        .issue_type_id(issue_type_id)
        .priority_id(priority_id);

    if let Some(description) = req.description {
        builder.description(description);
    }

    // Handle custom fields if provided
    if let Some(custom_fields_by_name) = req.custom_fields {
        let project_id_or_key = ProjectIdOrKey::from(project_id);
        let custom_fields = crate::issue::custom_field_converter::resolve_custom_fields(
            &client_guard,
            &project_id_or_key,
            custom_fields_by_name,
        )
        .await?;

        builder.custom_fields(custom_fields);
    }

    let params = builder.build()?;

    let issue = client_guard.issue().add_issue(params).await?;
    Ok(issue)
}
