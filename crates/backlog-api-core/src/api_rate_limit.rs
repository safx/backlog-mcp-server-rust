use serde::{Deserialize, Serialize};

/// Rate limit information for a specific API operation type.
///
/// Contains the maximum allowed requests, remaining requests, and reset timestamp.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiRateLimit {
    /// Maximum number of requests allowed in the current time window
    pub limit: i32,
    /// Number of requests remaining in the current time window
    pub remaining: i32,
    /// Unix timestamp (seconds since epoch) when the rate limit resets
    pub reset: i32,
}
