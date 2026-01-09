use crate::models::ParentChildCondition;
use backlog_api_core::{Error as ApiError, IntoRequest};
use backlog_api_macros::ToFormParams;
use backlog_core::ApiDate;
use backlog_core::identifier::{
    CategoryId, IssueId, IssueTypeId, MilestoneId, PriorityId, ProjectId, ResolutionId, StatusId,
    UserId,
};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Response type for counting issues
#[derive(Debug, Deserialize)]
pub struct CountIssueResponse {
    pub count: u32,
}

/// Parameters for counting issues.
/// Corresponds to `GET /api/v2/issues/count`.
///
/// Note: This struct excludes pagination parameters (offset, count, sort, order)
/// as they are not relevant for counting operations.
#[derive(Debug, Clone, Builder, ToFormParams)]
#[builder(build_fn(error = "ApiError"))]
pub struct CountIssueParams {
    #[builder(default, setter(into, strip_option))]
    #[form(array, name = "projectId")]
    pub project_id: Option<Vec<ProjectId>>,
    #[builder(default, setter(into, strip_option))]
    #[form(array, name = "issueTypeId")]
    pub issue_type_id: Option<Vec<IssueTypeId>>,
    #[builder(default, setter(into, strip_option))]
    #[form(array, name = "categoryId")]
    pub category_id: Option<Vec<CategoryId>>,
    #[builder(default, setter(into, strip_option))]
    #[form(array, name = "versionId")]
    pub version_id: Option<Vec<MilestoneId>>,
    #[builder(default, setter(into, strip_option))]
    #[form(array, name = "milestoneId")]
    pub milestone_id: Option<Vec<MilestoneId>>,
    #[builder(default, setter(into, strip_option))]
    #[form(array, name = "statusId")]
    pub status_id: Option<Vec<StatusId>>,
    #[builder(default, setter(into, strip_option))]
    #[form(array, name = "priorityId")]
    pub priority_id: Option<Vec<PriorityId>>,
    #[builder(default, setter(into, strip_option))]
    #[form(array, name = "assigneeId")]
    pub assignee_id: Option<Vec<UserId>>,
    #[builder(default, setter(into, strip_option))]
    #[form(array, name = "createdUserId")]
    pub created_user_id: Option<Vec<UserId>>,
    #[builder(default, setter(into, strip_option))]
    #[form(array, name = "resolutionId")]
    pub resolution_id: Option<Vec<ResolutionId>>,
    #[builder(default, setter(into, strip_option))]
    #[form(name = "parentChild")]
    pub parent_child_condition: Option<ParentChildCondition>,
    #[builder(default, setter(into, strip_option))]
    pub attachment: Option<bool>,
    #[builder(default, setter(into, strip_option))]
    #[form(name = "sharedFile")]
    pub shared_file: Option<bool>,
    #[builder(default, setter(into, strip_option))]
    #[form(name = "createdSince")]
    pub created_since: Option<ApiDate>,
    #[builder(default, setter(into, strip_option))]
    #[form(name = "createdUntil")]
    pub created_until: Option<ApiDate>,
    #[builder(default, setter(into, strip_option))]
    #[form(name = "updatedSince")]
    pub updated_since: Option<ApiDate>,
    #[builder(default, setter(into, strip_option))]
    #[form(name = "updatedUntil")]
    pub updated_until: Option<ApiDate>,
    #[builder(default, setter(into, strip_option))]
    #[form(array, name = "parentIssueId")]
    pub parent_issue_id: Option<Vec<IssueId>>,
    #[builder(default, setter(into, strip_option))]
    pub keyword: Option<String>,
    #[builder(default, setter(into, strip_option))]
    #[form(array)]
    pub id: Option<Vec<IssueId>>,
    #[builder(default, setter(into, strip_option))]
    #[form(name = "startDateSince")]
    pub start_date_since: Option<ApiDate>,
    #[builder(default, setter(into, strip_option))]
    #[form(name = "startDateUntil")]
    pub start_date_until: Option<ApiDate>,
    #[builder(default, setter(into, strip_option))]
    #[form(name = "dueDateSince")]
    pub due_date_since: Option<ApiDate>,
    #[builder(default, setter(into, strip_option))]
    #[form(name = "dueDateUntil")]
    pub due_date_until: Option<ApiDate>,
}

impl IntoRequest for CountIssueParams {
    fn path(&self) -> String {
        "/api/v2/issues/count".to_string()
    }

    fn to_query(&self) -> impl Serialize {
        let params: Vec<(String, String)> = self.into();
        params
    }
}

// Convert CountIssueParams to vector of pairs
impl From<CountIssueParams> for Vec<(String, String)> {
    fn from(params: CountIssueParams) -> Self {
        (&params).into()
    }
}
