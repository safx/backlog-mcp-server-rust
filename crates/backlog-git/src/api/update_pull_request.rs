use crate::models::PullRequest;
use backlog_api_core::{HttpMethod, IntoRequest};
use backlog_core::{
    ProjectIdOrKey, RepositoryIdOrName,
    identifier::{IssueId, PullRequestNumber, UserId},
};
use serde::Serialize;

use backlog_api_macros::ToFormParams;

pub type UpdatePullRequestResponse = PullRequest;

#[derive(Debug, Clone, ToFormParams)]
pub struct UpdatePullRequestParams {
    #[form(skip)]
    pub project_id_or_key: ProjectIdOrKey,
    #[form(skip)]
    pub repo_id_or_name: RepositoryIdOrName,
    #[form(skip)]
    pub number: PullRequestNumber,
    pub summary: Option<String>,
    pub description: Option<String>,
    #[form(name = "issueId")]
    pub issue_id: Option<IssueId>,
    #[form(name = "assigneeId")]
    pub assignee_id: Option<UserId>,
    #[form(array, name = "notifiedUserId")]
    pub notified_user_ids: Option<Vec<UserId>>,
    pub comment: Option<String>,
}

impl UpdatePullRequestParams {
    pub fn new(
        project_id_or_key: impl Into<ProjectIdOrKey>,
        repo_id_or_name: impl Into<RepositoryIdOrName>,
        number: impl Into<PullRequestNumber>,
    ) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
            repo_id_or_name: repo_id_or_name.into(),
            number: number.into(),
            summary: None,
            description: None,
            issue_id: None,
            assignee_id: None,
            notified_user_ids: None,
            comment: None,
        }
    }

    pub fn summary(mut self, summary: impl Into<String>) -> Self {
        self.summary = Some(summary.into());
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn issue_id(mut self, issue_id: IssueId) -> Self {
        self.issue_id = Some(issue_id);
        self
    }

    pub fn assignee_id(mut self, assignee_id: UserId) -> Self {
        self.assignee_id = Some(assignee_id);
        self
    }

    pub fn notified_user_ids(mut self, notified_user_ids: Vec<UserId>) -> Self {
        self.notified_user_ids = Some(notified_user_ids);
        self
    }

    pub fn comment(mut self, comment: impl Into<String>) -> Self {
        self.comment = Some(comment.into());
        self
    }
}

impl IntoRequest for UpdatePullRequestParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Patch
    }

    fn path(&self) -> String {
        format!(
            "/api/v2/projects/{}/git/repositories/{}/pullRequests/{}",
            self.project_id_or_key, self.repo_id_or_name, self.number
        )
    }

    fn to_form(&self) -> impl Serialize {
        Vec::<(String, String)>::from(self)
    }
}
