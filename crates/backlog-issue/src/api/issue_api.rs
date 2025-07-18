use backlog_api_core::Result;
use client::{Client, DownloadedFile};

#[cfg(feature = "writable")]
use super::{
    AddCommentNotificationParams, AddCommentParams, AddIssueParams, AddRecentlyViewedIssueParams,
    DeleteAttachmentParams, DeleteCommentParams, DeleteIssueParams, LinkSharedFilesToIssueParams,
    UnlinkSharedFileParams, UpdateCommentParams, UpdateIssueParams,
};
#[cfg(feature = "writable")]
use super::{
    AddCommentNotificationResponse, AddCommentResponse, AddIssueResponse,
    AddRecentlyViewedIssueResponse, DeleteAttachmentResponse, DeleteCommentResponse,
    DeleteIssueResponse, LinkSharedFilesToIssueResponse, UnlinkSharedFileResponse,
    UpdateCommentResponse, UpdateIssueResponse,
};
use super::{
    CountCommentParams, CountIssueParams, GetAttachmentFileParams, GetAttachmentListParams,
    GetCommentListParams, GetCommentNotificationsParams, GetCommentParams, GetIssueListParams,
    GetIssueParams, GetParticipantListParams, GetRecentlyViewedIssuesParams,
    GetSharedFileListParams,
};
use super::{
    CountCommentResponse, CountIssueResponse, GetAttachmentListResponse, GetCommentListResponse,
    GetCommentNotificationsResponse, GetCommentResponse, GetIssueListResponse, GetIssueResponse,
    GetParticipantListResponse, GetRecentlyViewedIssuesResponse, GetSharedFileListResponse,
};

pub struct IssueApi(Client);

impl IssueApi {
    pub fn new(client: Client) -> Self {
        Self(client)
    }

    /// Get issue by its ID or key.
    pub async fn get_issue(&self, params: GetIssueParams) -> Result<GetIssueResponse> {
        self.0.execute(params).await
    }

    /// Get a list of issues with optional parameters.
    pub async fn get_issue_list(&self, params: GetIssueListParams) -> Result<GetIssueListResponse> {
        self.0.execute(params).await
    }

    /// Count issues based on the provided parameters.
    pub async fn count_issue(&self, params: CountIssueParams) -> Result<CountIssueResponse> {
        self.0.execute(params).await
    }

    /// Add a new issue with the provided parameters.
    #[cfg(feature = "writable")]
    pub async fn add_issue(&self, params: AddIssueParams) -> Result<AddIssueResponse> {
        self.0.execute(params).await
    }

    /// Delete an issue by its key.
    #[cfg(feature = "writable")]
    pub async fn delete_issue(&self, params: DeleteIssueParams) -> Result<DeleteIssueResponse> {
        self.0.execute(params).await
    }

    /// Update an existing issue by its ID or key.
    #[cfg(feature = "writable")]
    pub async fn update_issue(&self, params: UpdateIssueParams) -> Result<UpdateIssueResponse> {
        self.0.execute(params).await
    }

    /// Add a new comment to an existing issue.
    #[cfg(feature = "writable")]
    pub async fn add_comment(&self, params: AddCommentParams) -> Result<AddCommentResponse> {
        self.0.execute(params).await
    }

    /// Update an existing comment on an issue.
    ///
    /// Corresponds to `PATCH /api/v2/issues/:issueIdOrKey/comments/:commentId`.
    #[cfg(feature = "writable")]
    pub async fn update_comment(
        &self,
        params: UpdateCommentParams,
    ) -> Result<UpdateCommentResponse> {
        self.0.execute(params).await
    }

    /// Delete a comment from an issue.
    ///
    /// Corresponds to `DELETE /api/v2/issues/:issueIdOrKey/comments/:commentId`.
    #[cfg(feature = "writable")]
    pub async fn delete_comment(
        &self,
        params: DeleteCommentParams,
    ) -> Result<DeleteCommentResponse> {
        self.0.execute(params).await
    }

    /// Delete an attachment from an issue.
    ///
    /// Corresponds to `DELETE /api/v2/issues/:issueIdOrKey/attachments/:attachmentId`.
    #[cfg(feature = "writable")]
    pub async fn delete_attachment(
        &self,
        params: DeleteAttachmentParams,
    ) -> Result<DeleteAttachmentResponse> {
        self.0.execute(params).await
    }

    /// Get a list of comments for an issue by its ID or key.
    pub async fn get_comment_list(
        &self,
        params: GetCommentListParams,
    ) -> Result<GetCommentListResponse> {
        self.0.execute(params).await
    }

    /// Count comments for an issue by its ID or key.
    pub async fn count_comment(&self, params: CountCommentParams) -> Result<CountCommentResponse> {
        self.0.execute(params).await
    }

    /// Get a specific comment for an issue by its ID or key and comment ID.
    pub async fn get_comment(&self, params: GetCommentParams) -> Result<GetCommentResponse> {
        self.0.execute(params).await
    }

    /// Get a list of notifications for a specific comment.
    ///
    /// Corresponds to `GET /api/v2/issues/:issueIdOrKey/comments/:commentId/notifications`.
    pub async fn get_comment_notifications(
        &self,
        params: GetCommentNotificationsParams,
    ) -> Result<GetCommentNotificationsResponse> {
        self.0.execute(params).await
    }

    /// Add notifications to a specific comment.
    ///
    /// Corresponds to `POST /api/v2/issues/:issueIdOrKey/comments/:commentId/notifications`.
    #[cfg(feature = "writable")]
    pub async fn add_comment_notification(
        &self,
        params: AddCommentNotificationParams,
    ) -> Result<AddCommentNotificationResponse> {
        self.0.execute(params).await
    }

    /// Get a list of attachments for an issue by its ID or key.
    pub async fn get_attachment_list(
        &self,
        params: GetAttachmentListParams,
    ) -> Result<GetAttachmentListResponse> {
        self.0.execute(params).await
    }

    /// Get a list of participants in an issue.
    ///
    /// Corresponds to `GET /api/v2/issues/:issueIdOrKey/participants`.
    pub async fn get_participant_list(
        &self,
        params: GetParticipantListParams,
    ) -> Result<GetParticipantListResponse> {
        self.0.execute(params).await
    }

    /// Get a list of shared files linked to an issue.
    ///
    /// Corresponds to `GET /api/v2/issues/:issueIdOrKey/sharedFiles`.
    pub async fn get_shared_file_list(
        &self,
        params: GetSharedFileListParams,
    ) -> Result<GetSharedFileListResponse> {
        self.0.execute(params).await
    }

    /// Link shared files to an issue.
    ///
    /// Corresponds to `POST /api/v2/issues/:issueIdOrKey/sharedFiles`.
    #[cfg(feature = "writable")]
    pub async fn link_shared_files_to_issue(
        &self,
        params: LinkSharedFilesToIssueParams,
    ) -> Result<LinkSharedFilesToIssueResponse> {
        self.0.execute(params).await
    }

    /// Unlink a shared file from an issue.
    ///
    /// Corresponds to `DELETE /api/v2/issues/:issueIdOrKey/sharedFiles/:id`.
    #[cfg(feature = "writable")]
    pub async fn unlink_shared_file(
        &self,
        params: UnlinkSharedFileParams,
    ) -> Result<UnlinkSharedFileResponse> {
        self.0.execute(params).await
    }

    /// Get a specific attachment file by issue ID or key and attachment ID.
    pub async fn get_attachment_file(
        &self,
        params: GetAttachmentFileParams,
    ) -> backlog_api_core::Result<DownloadedFile> {
        self.0.download_file(params).await
    }

    /// Get recently viewed issues for the current user.
    ///
    /// Corresponds to `GET /api/v2/users/myself/recentlyViewedIssues`.
    pub async fn get_recently_viewed_issues(
        &self,
        params: GetRecentlyViewedIssuesParams,
    ) -> Result<GetRecentlyViewedIssuesResponse> {
        self.0.execute(params).await
    }

    /// Add an issue to the recently viewed list.
    ///
    /// Corresponds to `POST /api/v2/users/myself/recentlyViewedIssues`.
    #[cfg(feature = "writable")]
    pub async fn add_recently_viewed_issue(
        &self,
        params: AddRecentlyViewedIssueParams,
    ) -> Result<AddRecentlyViewedIssueResponse> {
        self.0.execute(params).await
    }
}
