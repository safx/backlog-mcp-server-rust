use backlog_api_core::{Error as ApiError, IntoRequest};
use backlog_core::{ApiDate, identifier::UserId};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Parameters for the Get User Star Count API.
#[derive(Builder, Debug, Clone, Serialize)]
#[builder(build_fn(error = "ApiError"))]
pub struct GetUserStarCountParams {
    /// The ID of the user whose star count to retrieve.
    #[serde(skip)]
    pub user_id: UserId,
    /// Count stars from this date (inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<ApiDate>,
    /// Count stars until this date (inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<ApiDate>,
}

impl GetUserStarCountParams {
    /// Creates a new instance with the specified user ID.
    pub fn new(user_id: impl Into<UserId>) -> Self {
        Self {
            user_id: user_id.into(),
            since: None,
            until: None,
        }
    }

    /// Sets the start date for filtering stars.
    pub fn with_since(mut self, since: ApiDate) -> Self {
        self.since = Some(since);
        self
    }

    /// Sets the end date for filtering stars.
    pub fn with_until(mut self, until: ApiDate) -> Self {
        self.until = Some(until);
        self
    }
}

impl IntoRequest for GetUserStarCountParams {
    fn path(&self) -> String {
        format!("/api/v2/users/{}/stars/count", self.user_id)
    }

    fn to_query(&self) -> impl Serialize {
        self
    }
}

/// Response from the Get User Star Count API.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct StarCount {
    /// The number of stars received by the user.
    pub count: u32,
}

/// Type alias for the Get User Star Count API response.
pub type GetUserStarCountResponse = StarCount;

#[cfg(test)]
mod tests {
    use super::*;
    use backlog_core::identifier::Identifier;
    use chrono::{DateTime, NaiveDate, Utc};

    #[test]
    fn test_get_user_star_count_params_basic() {
        let params = GetUserStarCountParams::new(12345u32);

        assert_eq!(params.user_id.value(), 12345);
        assert_eq!(params.path(), "/api/v2/users/12345/stars/count");
    }

    #[test]
    fn test_get_user_star_count_params_with_dates() {
        let since_date = NaiveDate::from_ymd_opt(2024, 1, 1).expect("valid date constant");
        let since_datetime = DateTime::<Utc>::from_naive_utc_and_offset(
            since_date
                .and_hms_opt(0, 0, 0)
                .expect("valid time constant"),
            Utc,
        );
        let since = ApiDate::from(since_datetime);

        let until_date = NaiveDate::from_ymd_opt(2024, 12, 31).expect("valid date constant");
        let until_datetime = DateTime::<Utc>::from_naive_utc_and_offset(
            until_date
                .and_hms_opt(0, 0, 0)
                .expect("valid time constant"),
            Utc,
        );
        let until = ApiDate::from(until_datetime);

        let params = GetUserStarCountParams::new(12345u32)
            .with_since(since)
            .with_until(until);

        // Since parameters are part of the struct, we can verify they're set correctly
        assert_eq!(
            params
                .since
                .as_ref()
                .expect("since should be set")
                .to_string(),
            "2024-01-01"
        );
        assert_eq!(
            params
                .until
                .as_ref()
                .expect("until should be set")
                .to_string(),
            "2024-12-31"
        );
    }

    #[test]
    fn test_star_count_response_deserialization() {
        let json = r#"{"count": 54}"#;
        let response: StarCount =
            serde_json::from_str(json).expect("valid JSON should deserialize");

        assert_eq!(response.count, 54);
    }
}
