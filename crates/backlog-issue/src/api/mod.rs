// Main API struct
mod issue_api;
pub use issue_api::IssueApi;

// Read-only API modules
mod count_comment;
mod count_issue;
mod get_attachment_file;
mod get_attachment_list;
mod get_comment;
mod get_comment_list;
mod get_comment_notifications;
mod get_issue;
mod get_issue_list;
mod get_participant_list;
mod get_recently_viewed_issues;
mod get_shared_file_list;

// Write-only API modules (feature-gated)
#[cfg(feature = "writable")]
mod add_comment;
#[cfg(feature = "writable")]
mod add_comment_notification;
#[cfg(feature = "writable")]
mod add_issue;
#[cfg(feature = "writable")]
mod add_issue_with_api_date;
#[cfg(feature = "writable")]
mod add_recently_viewed_issue;
#[cfg(feature = "writable")]
mod custom_field_utils;
#[cfg(feature = "writable")]
mod delete_attachment;
#[cfg(feature = "writable")]
mod delete_comment;
#[cfg(feature = "writable")]
mod delete_issue;
#[cfg(feature = "writable")]
mod link_shared_files;
#[cfg(feature = "writable")]
mod unlink_shared_file;
#[cfg(feature = "writable")]
mod update_comment;
#[cfg(feature = "writable")]
mod update_issue;

// Re-export all parameter types and response types

// Read-only exports (always available)
pub use count_comment::{CountCommentParams, CountCommentResponse};
pub use count_issue::{CountIssueParams, CountIssueParamsBuilder, CountIssueResponse};
pub use get_attachment_file::{GetAttachmentFileParams, GetAttachmentFileParamsBuilder};
pub use get_attachment_list::{GetAttachmentListParams, GetAttachmentListResponse};
pub use get_comment::{GetCommentParams, GetCommentResponse};
pub use get_comment_list::{
    CommentOrder, GetCommentListParams, GetCommentListParamsBuilder, GetCommentListResponse,
};
pub use get_comment_notifications::{
    GetCommentNotificationsParams, GetCommentNotificationsResponse,
};
pub use get_issue::{GetIssueParams, GetIssueResponse};
pub use get_issue_list::{GetIssueListParams, GetIssueListParamsBuilder, GetIssueListResponse};
pub use get_participant_list::{GetParticipantListParams, GetParticipantListResponse};
pub use get_recently_viewed_issues::{
    GetRecentlyViewedIssuesParams, GetRecentlyViewedIssuesParamsBuilder,
    GetRecentlyViewedIssuesResponse,
};
pub use get_shared_file_list::{GetSharedFileListParams, GetSharedFileListResponse};

// Write-only exports (feature-gated)
#[cfg(feature = "writable")]
pub use add_comment::{AddCommentParams, AddCommentParamsBuilder, AddCommentResponse};
#[cfg(feature = "writable")]
pub use add_comment_notification::{AddCommentNotificationParams, AddCommentNotificationResponse};
#[cfg(feature = "writable")]
pub use add_issue::{AddIssueParams, AddIssueParamsBuilder, AddIssueResponse};
#[cfg(feature = "writable")]
pub use add_issue_with_api_date::{
    AddIssueWithApiDateParams, AddIssueWithApiDateParamsBuilder, AddIssueWithApiDateResponse,
};
#[cfg(feature = "writable")]
pub use add_recently_viewed_issue::{AddRecentlyViewedIssueParams, AddRecentlyViewedIssueResponse};
#[cfg(feature = "writable")]
pub use delete_attachment::{DeleteAttachmentParams, DeleteAttachmentResponse};
#[cfg(feature = "writable")]
pub use delete_comment::{DeleteCommentParams, DeleteCommentResponse};
#[cfg(feature = "writable")]
pub use delete_issue::{DeleteIssueParams, DeleteIssueResponse};
#[cfg(feature = "writable")]
pub use link_shared_files::{
    LinkSharedFilesToIssueParams, LinkSharedFilesToIssueParamsBuilder,
    LinkSharedFilesToIssueResponse,
};
#[cfg(feature = "writable")]
pub use unlink_shared_file::{UnlinkSharedFileParams, UnlinkSharedFileResponse};
#[cfg(feature = "writable")]
pub use update_comment::{UpdateCommentParams, UpdateCommentResponse};
#[cfg(feature = "writable")]
pub use update_issue::{UpdateIssueParams, UpdateIssueParamsBuilder, UpdateIssueResponse};
