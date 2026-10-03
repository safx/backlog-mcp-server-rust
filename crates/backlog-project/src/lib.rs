pub mod api;
//pub mod tests;

// re-export domain models
pub use backlog_domain_models::{
    Category, IssueType, Milestone, Priority, Project, Resolution, Status, Team,
};

pub use api::{
    GetCategoryListParams, GetCategoryListResponse, GetCustomFieldListParams,
    GetCustomFieldListResponse, GetIssueTypeListParams, GetIssueTypeListResponse,
    GetMilestoneListParams, GetMilestoneListResponse, GetPriorityListParams,
    GetPriorityListResponse, GetProjectDetailParams, GetProjectDetailResponse,
    GetProjectIconParams, GetProjectListParams, GetProjectListResponse,
    GetProjectRecentUpdatesParams, GetProjectRecentUpdatesResponse, GetProjectTeamListParams,
    GetProjectTeamListResponse, GetProjectUserListParams, GetProjectUserListResponse,
    GetRecentlyViewedProjectsParams, GetRecentlyViewedProjectsParamsBuilder,
    GetRecentlyViewedProjectsResponse, GetResolutionListParams, GetResolutionListResponse,
    GetStatusListParams, GetStatusListResponse, ProjectApi, RecentlyViewedProject,
};

#[cfg(feature = "writable")]
pub use api::{
    AddCategoryParams, AddCategoryResponse, AddCustomFieldParams, AddCustomFieldResponse,
    AddIssueTypeParams, AddIssueTypeResponse, AddListItemToCustomFieldParams,
    AddListItemToCustomFieldResponse, AddMilestoneParams, AddMilestoneResponse,
    AddProjectTeamParams, AddProjectTeamResponse, AddStatusParams, AddStatusResponse,
    DeleteCategoryParams, DeleteCategoryResponse, DeleteCustomFieldParams,
    DeleteCustomFieldResponse, DeleteIssueTypeParams, DeleteIssueTypeResponse,
    DeleteProjectTeamParams, DeleteProjectTeamResponse, DeleteStatusParams, DeleteStatusResponse,
    DeleteVersionParams, DeleteVersionResponse, UpdateCategoryParams, UpdateCategoryResponse,
    UpdateCustomFieldParams, UpdateCustomFieldResponse, UpdateIssueTypeParams,
    UpdateIssueTypeResponse, UpdateListItemToCustomFieldParams,
    UpdateListItemToCustomFieldResponse, UpdateStatusOrderParams, UpdateStatusOrderResponse,
    UpdateStatusParams, UpdateStatusResponse, UpdateVersionParams, UpdateVersionResponse,
};
