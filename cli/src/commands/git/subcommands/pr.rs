use crate::commands::common::CliResult;
use backlog_api_client::{
    GetPullRequestCountParams, ProjectIdOrKey, PullRequestNumber, RepositoryIdOrName, UserId,
    client::BacklogApiClient,
};
use backlog_core::identifier::{AttachmentId, Identifier, IssueId, StatusId};
use std::str::FromStr;

#[cfg(feature = "git_writable")]
use backlog_api_client::{AddPullRequestParams, UpdatePullRequestParams};

pub(crate) async fn list(
    client: &BacklogApiClient,
    project_id: String,
    repo_id: String,
) -> CliResult<()> {
    println!("Listing pull requests for repo {repo_id} in project: {project_id}");
    let proj_id_or_key = project_id.parse::<ProjectIdOrKey>()?;
    let repo_id_or_name = repo_id.parse::<RepositoryIdOrName>()?;
    let params = backlog_api_client::GetPullRequestListParams::new(proj_id_or_key, repo_id_or_name);
    let prs = client.git().get_pull_request_list(params).await?;
    // TODO: Pretty print pull requests
    println!("{prs:?}");
    Ok(())
}

pub(crate) async fn show(
    client: &BacklogApiClient,
    project_id: String,
    repo_id: String,
    pr_number: u64,
) -> CliResult<()> {
    println!("Showing PR #{pr_number} for repo {repo_id} in project: {project_id}");
    let proj_id_or_key = project_id.parse::<ProjectIdOrKey>()?;
    let repo_id_or_name = repo_id.parse::<RepositoryIdOrName>()?;
    let pr_num = PullRequestNumber::from(pr_number);

    let params =
        backlog_api_client::GetPullRequestParams::new(proj_id_or_key, repo_id_or_name, pr_num);
    let pr = client.git().get_pull_request(params).await?;
    // TODO: Pretty print pull request
    println!("{pr:?}");
    Ok(())
}

pub(crate) async fn count(
    client: &BacklogApiClient,
    project_id: String,
    repo_id: String,
    status_ids: Option<String>,
    assignee_ids: Option<String>,
    issue_ids: Option<String>,
    created_user_ids: Option<String>,
) -> CliResult<()> {
    println!("Getting pull request count for repo {repo_id} (project {project_id})");

    let parsed_project_id = ProjectIdOrKey::from_str(&project_id)
        .map_err(|e| format!("Failed to parse project_id '{project_id}': {e}"))?;
    let parsed_repo_id = RepositoryIdOrName::from_str(&repo_id)
        .map_err(|e| format!("Failed to parse repo_id '{repo_id}': {e}"))?;

    // Parse filter parameters
    let mut params = GetPullRequestCountParams::new(parsed_project_id, parsed_repo_id);

    // Parse status IDs
    if let Some(status_ids_str) = status_ids {
        let status_ids: Result<Vec<StatusId>, _> = status_ids_str
            .split(',')
            .map(|s| s.trim().parse::<u32>().map(StatusId::new))
            .collect();
        match status_ids {
            Ok(ids) => params = params.status_ids(ids),
            Err(e) => {
                eprintln!("❌ Failed to parse status_ids: {e}");
                std::process::exit(1);
            }
        };
    }

    // Parse assignee IDs
    if let Some(assignee_ids_str) = assignee_ids {
        let assignee_ids: Result<Vec<UserId>, _> = assignee_ids_str
            .split(',')
            .map(|s| s.trim().parse::<u32>().map(UserId::new))
            .collect();
        match assignee_ids {
            Ok(ids) => params = params.assignee_ids(ids),
            Err(e) => {
                eprintln!("❌ Failed to parse assignee_ids: {e}");
                std::process::exit(1);
            }
        };
    }

    // Parse issue IDs
    if let Some(issue_ids_str) = issue_ids {
        let issue_ids: Result<Vec<IssueId>, _> = issue_ids_str
            .split(',')
            .map(|s| s.trim().parse::<u32>().map(IssueId::new))
            .collect();
        match issue_ids {
            Ok(ids) => params = params.issue_ids(ids),
            Err(e) => {
                eprintln!("❌ Failed to parse issue_ids: {e}");
                std::process::exit(1);
            }
        };
    }

    // Parse created user IDs
    if let Some(created_user_ids_str) = created_user_ids {
        let created_user_ids: Result<Vec<UserId>, _> = created_user_ids_str
            .split(',')
            .map(|s| s.trim().parse::<u32>().map(UserId::new))
            .collect();
        match created_user_ids {
            Ok(ids) => params = params.created_user_ids(ids),
            Err(e) => {
                eprintln!("❌ Failed to parse created_user_ids: {e}");
                std::process::exit(1);
            }
        };
    }

    match client.git().get_pull_request_count(params).await {
        Ok(count_response) => {
            println!("✅ Pull request count retrieved successfully");
            println!("Pull request count: {}", count_response.count);
        }
        Err(e) => {
            eprintln!("❌ Failed to get pull request count: {e}");
            std::process::exit(1);
        }
    }
    Ok(())
}

#[cfg(feature = "git_writable")]
pub(crate) async fn create(
    client: &BacklogApiClient,
    project_id: String,
    repo_id: String,
    summary: String,
    description: String,
    base: String,
    branch: String,
    issue_id: Option<u32>,
    assignee_id: Option<u32>,
    notify_user_ids: Option<String>,
    attachment_ids: Option<String>,
) -> CliResult<()> {
    println!("Creating pull request in repo {repo_id} (project {project_id})");

    let parsed_project_id = ProjectIdOrKey::from_str(&project_id)
        .map_err(|e| format!("Failed to parse project_id '{project_id}': {e}"))?;
    let parsed_repo_id = RepositoryIdOrName::from_str(&repo_id)
        .map_err(|e| format!("Failed to parse repo_id '{repo_id}': {e}"))?;

    // Build parameters
    let mut params = AddPullRequestParams::new(
        parsed_project_id,
        parsed_repo_id,
        summary.clone(),
        description.clone(),
        base.clone(),
        branch.clone(),
    );

    // Parse optional issue ID
    if let Some(issue_id) = issue_id {
        params = params.issue_id(backlog_core::identifier::IssueId::new(issue_id));
    }

    // Parse optional assignee ID
    if let Some(assignee_id) = assignee_id {
        params = params.assignee_id(UserId::new(assignee_id));
    }

    // Parse notify user IDs
    if let Some(notify_user_ids_str) = notify_user_ids {
        let notify_user_ids: Result<Vec<UserId>, _> = notify_user_ids_str
            .split(',')
            .map(|s| s.trim().parse::<u32>().map(UserId::new))
            .collect();
        match notify_user_ids {
            Ok(ids) => params = params.notified_user_ids(ids),
            Err(e) => {
                eprintln!("❌ Failed to parse notify_user_ids: {e}");
                std::process::exit(1);
            }
        };
    }

    // Parse attachment IDs
    if let Some(attachment_ids_str) = attachment_ids {
        let attachment_ids: Result<Vec<AttachmentId>, _> = attachment_ids_str
            .split(',')
            .map(|s| s.trim().parse::<u32>().map(AttachmentId::new))
            .collect();
        match attachment_ids {
            Ok(ids) => params = params.attachment_ids(ids),
            Err(e) => {
                eprintln!("❌ Failed to parse attachment_ids: {e}");
                std::process::exit(1);
            }
        };
    }

    match client.git().add_pull_request(params).await {
        Ok(pull_request) => {
            println!("✅ Pull request created successfully");
            println!("ID: {}", pull_request.id.value());
            println!("Number: {}", pull_request.number.value());
            println!("Summary: {}", pull_request.summary);
            if let Some(description) = &pull_request.description {
                println!("Description: {description}");
            }
            println!("Base: {}", pull_request.base);
            println!("Branch: {}", pull_request.branch);
            if let Some(assignee) = &pull_request.assignee {
                println!("Assignee: {} (ID: {})", assignee.name, assignee.id.value());
            }
            if let Some(issue) = &pull_request.related_issue {
                println!("Related Issue ID: {}", issue.id.value());
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to create pull request: {e}");
            std::process::exit(1);
        }
    }
    Ok(())
}

#[cfg(feature = "git_writable")]
pub(crate) async fn update(
    client: &BacklogApiClient,
    project_id: String,
    repo_id: String,
    pr_number: u64,
    summary: Option<String>,
    description: Option<String>,
    issue_id: Option<u32>,
    assignee_id: Option<u32>,
    notify_user_ids: Option<Vec<u32>>,
    comment: Option<String>,
) -> CliResult<()> {
    println!("Updating PR #{pr_number} in repo {repo_id} (project {project_id})");

    let parsed_project_id = ProjectIdOrKey::from_str(&project_id)
        .map_err(|e| format!("Failed to parse project_id '{project_id}': {e}"))?;
    let parsed_repo_id = RepositoryIdOrName::from_str(&repo_id)
        .map_err(|e| format!("Failed to parse repo_id '{repo_id}': {e}"))?;
    let parsed_pr_number = PullRequestNumber::from(pr_number);

    let mut params =
        UpdatePullRequestParams::new(parsed_project_id, parsed_repo_id, parsed_pr_number);

    if let Some(summary) = summary {
        params = params.summary(summary.clone());
    }

    if let Some(description) = description {
        params = params.description(description.clone());
    }

    if let Some(issue_id) = issue_id {
        params = params.issue_id(IssueId::new(issue_id));
    }

    if let Some(assignee_id) = assignee_id {
        params = params.assignee_id(UserId::new(assignee_id));
    }

    if let Some(notify_user_ids) = notify_user_ids {
        let user_ids: Vec<UserId> = notify_user_ids.iter().map(|&id| UserId::new(id)).collect();
        params = params.notified_user_ids(user_ids);
    }

    if let Some(comment) = comment {
        params = params.comment(comment.clone());
    }

    match client.git().update_pull_request(params).await {
        Ok(pull_request) => {
            println!("✅ Pull request updated successfully");
            println!("ID: {}", pull_request.id.value());
            println!("Number: {}", pull_request.number.value());
            println!("Summary: {}", pull_request.summary);
            if let Some(description) = &pull_request.description {
                println!("Description: {description}");
            }
            if let Some(assignee) = &pull_request.assignee {
                println!("Assignee: {} (ID: {})", assignee.name, assignee.id.value());
            }
            if let Some(issue) = &pull_request.related_issue {
                println!("Related Issue ID: {}", issue.id.value());
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to update pull request: {e}");
            std::process::exit(1);
        }
    }
    Ok(())
}
