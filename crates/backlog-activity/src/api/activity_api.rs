use backlog_api_core::Result;
use backlog_core::identifier::ActivityId;
use client::Client;

use super::get_activity::{GetActivityParams, GetActivityResponse};

/// API client for Backlog Activity endpoints.
///
/// Provides methods to interact with the Backlog Activity API.
#[derive(Clone, Debug)]
pub struct ActivityApi(Client);

impl ActivityApi {
    /// Creates a new `ActivityApi` instance with the given HTTP client.
    pub fn new(client: Client) -> Self {
        Self(client)
    }

    /// Get an activity by ID.
    /// Corresponds to `GET /api/v2/activities/:activityId`.
    pub async fn get_activity(
        &self,
        activity_id: impl Into<ActivityId>,
    ) -> Result<GetActivityResponse> {
        let params = GetActivityParams {
            activity_id: activity_id.into(),
        };
        self.0.execute(params).await
    }
}
