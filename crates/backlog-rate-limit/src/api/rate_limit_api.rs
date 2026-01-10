use crate::{GetRateLimitParams, GetRateLimitResponse};
use backlog_api_core::Result;
use client::Client;

/// API client for Backlog Rate Limit operations.
///
/// Provides methods to retrieve rate limit information for the current API key.
/// Rate limits are organized by operation type (read, update, search, icon).
#[derive(Debug, Clone)]
pub struct RateLimitApi {
    client: Client,
}

impl RateLimitApi {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// Gets the rate limit information for the API key.
    ///
    /// Corresponds to `GET /api/v2/rateLimit`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use backlog_rate_limit::RateLimitApi;
    /// use client::Client;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::new("https://example.backlog.jp")?
    ///     .with_api_key("your_api_key");
    /// let api = RateLimitApi::new(client);
    ///
    /// let rate_limit = api.get_rate_limit().await?;
    /// println!("Remaining read operations: {}", rate_limit.rate_limit.read.remaining);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_rate_limit(&self) -> Result<GetRateLimitResponse> {
        self.client.execute(GetRateLimitParams::new()).await
    }
}
