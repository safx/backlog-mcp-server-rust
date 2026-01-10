use backlog_api_core::ApiRateLimit;
use serde::{Deserialize, Serialize};

/// Response from the rate limit API endpoint.
///
/// Contains rate limit information organized by operation type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RateLimitResponse {
    #[serde(rename = "rateLimit")]
    pub rate_limit: RateLimitInfo,
}

/// Rate limit information for different operation types.
///
/// Each field contains limit, remaining, and reset timestamp for:
/// - `read`: Read operations (e.g., GET requests)
/// - `update`: Update operations (e.g., POST, PUT, DELETE requests)
/// - `search`: Search operations
/// - `icon`: Icon-related operations
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RateLimitInfo {
    pub read: ApiRateLimit,
    pub update: ApiRateLimit,
    pub search: ApiRateLimit,
    pub icon: ApiRateLimit,
}

/// Alias for `RateLimitResponse` used by `get_rate_limit` API.
pub type GetRateLimitResponse = RateLimitResponse;
