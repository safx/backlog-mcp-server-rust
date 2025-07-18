use crate::models::SpaceNotification;
use backlog_api_core::IntoRequest;
use serde::Serialize;

/// Response type for getting space notification
pub type GetSpaceNotificationResponse = SpaceNotification;

/// Parameters for getting space notification.
///
/// Corresponds to `GET /api/v2/space/notification`.
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetSpaceNotificationParams;

impl GetSpaceNotificationParams {
    /// Creates a new instance.
    pub fn new() -> Self {
        Self
    }
}

impl IntoRequest for GetSpaceNotificationParams {
    fn path(&self) -> String {
        "/api/v2/space/notification".to_string()
    }

    fn to_query(&self) -> impl Serialize {
        self
    }
}
