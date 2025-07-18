use backlog_api_core::Result;
use client::Client;

use crate::api::{
    GetNotificationCountParams, GetNotificationCountResponse, GetNotificationsParams,
    GetNotificationsResponse, GetOwnUserParams, GetOwnUserResponse, GetUserIconParams,
    GetUserIconResponse, GetUserListParams, GetUserListResponse, GetUserParams,
    GetUserRecentUpdatesParams, GetUserRecentUpdatesResponse, GetUserResponse,
    GetUserStarCountParams, GetUserStarCountResponse, GetUserStarsParams, GetUserStarsResponse,
    GetWatchingCountParams, GetWatchingListParams, GetWatchingListRequest,
};

pub struct UserApi(Client);

impl UserApi {
    pub fn new(client: Client) -> Self {
        Self(client)
    }

    /// Get the list of users in the space using IntoRequest pattern.
    /// Corresponds to `GET /api/v2/users`.
    pub async fn get_user_list(&self, params: GetUserListParams) -> Result<GetUserListResponse> {
        self.0.execute(params).await
    }

    /// Gets information about a specific user using IntoRequest pattern.
    ///
    /// Corresponds to `GET /api/v2/users/:userId`.
    pub async fn get_user(&self, params: GetUserParams) -> Result<GetUserResponse> {
        self.0.execute(params).await
    }

    /// Get the details of the authenticated user using IntoRequest pattern.
    pub async fn get_own_user(&self, params: GetOwnUserParams) -> Result<GetOwnUserResponse> {
        self.0.execute(params).await
    }

    /// Gets the user icon image data using IntoDownloadRequest pattern.
    ///
    /// Corresponds to `GET /api/v2/users/:userId/icon`.
    pub async fn get_user_icon(&self, params: GetUserIconParams) -> Result<GetUserIconResponse> {
        self.0.download_file(params).await
    }

    /// Gets recent activities for a specific user.
    ///
    /// Corresponds to `GET /api/v2/users/:userId/activities`.
    pub async fn get_user_recent_updates(
        &self,
        params: GetUserRecentUpdatesParams,
    ) -> Result<GetUserRecentUpdatesResponse> {
        self.0.execute(params).await
    }

    /// Gets the count of stars received by a specific user.
    ///
    /// Corresponds to `GET /api/v2/users/:userId/stars/count`.
    pub async fn get_user_star_count(
        &self,
        params: GetUserStarCountParams,
    ) -> Result<GetUserStarCountResponse> {
        self.0.execute(params).await
    }

    /// Gets the list of stars received by a specific user.
    ///
    /// Corresponds to `GET /api/v2/users/:userId/stars`.
    pub async fn get_user_stars(&self, params: GetUserStarsParams) -> Result<GetUserStarsResponse> {
        self.0.execute(params).await
    }

    /// Gets the count of notifications for the authenticated user.
    ///
    /// Corresponds to `GET /api/v2/notifications/count`.
    pub async fn get_notification_count(
        &self,
        params: GetNotificationCountParams,
    ) -> Result<GetNotificationCountResponse> {
        self.0.execute(params).await
    }

    /// Gets the list of notifications for the authenticated user.
    ///
    /// Corresponds to `GET /api/v2/notifications`.
    pub async fn get_notifications(
        &self,
        params: GetNotificationsParams,
    ) -> Result<GetNotificationsResponse> {
        self.0.execute(params).await
    }

    /// Gets the list of watchings for a specific user.
    ///
    /// Corresponds to `GET /api/v2/users/:userId/watchings`.
    pub async fn get_watching_list(
        &self,
        user_id: impl Into<backlog_core::identifier::UserId>,
        params: GetWatchingListParams,
    ) -> Result<crate::models::GetWatchingListResponse> {
        let request = GetWatchingListRequest {
            user_id: user_id.into(),
            params,
        };
        self.0.execute(request).await
    }

    /// Gets the count of watchings for a specific user.
    ///
    /// Corresponds to `GET /api/v2/users/:userId/watchings/count`.
    pub async fn get_watching_count(
        &self,
        params: GetWatchingCountParams,
    ) -> Result<crate::models::GetWatchingCountResponse> {
        self.0.execute(params).await
    }

    /// Mark a notification as read.
    ///
    /// Corresponds to `POST /api/v2/notifications/:id/markAsRead`.
    #[cfg(feature = "writable")]
    pub async fn mark_notification_as_read(
        &self,
        notification_id: impl Into<backlog_core::identifier::NotificationId>,
    ) -> Result<()> {
        let params = super::MarkNotificationAsReadParams::new(notification_id);
        self.0.execute_no_content(params).await?;
        Ok(())
    }

    /// Reset unread notification count by marking all notifications as read.
    ///
    /// Corresponds to `POST /api/v2/notifications/markAsRead`.
    #[cfg(feature = "writable")]
    pub async fn reset_unread_notification_count(
        &self,
    ) -> Result<crate::models::NotificationCount> {
        let params = super::ResetUnreadNotificationCountParams::new();
        self.0.execute(params).await
    }
}
