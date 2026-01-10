use backlog_api_core::IntoRequest;
use backlog_api_macros::ToFormParams;
use backlog_core::identifier::UserId;
use backlog_domain_models::Star;
use serde::Serialize;
use std::fmt;

/// Order for sorting stars
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum StarOrder {
    /// Ascending order (oldest first)
    Asc,
    /// Descending order (newest first)
    Desc,
}

impl fmt::Display for StarOrder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StarOrder::Asc => write!(f, "asc"),
            StarOrder::Desc => write!(f, "desc"),
        }
    }
}

/// Parameters for getting user stars.
///
/// # Example
/// ```no_run
/// # use backlog_user::api::{GetUserStarsParams, StarOrder};
/// # use backlog_core::identifier::UserId;
/// let params = GetUserStarsParams::new(12345u32)
///     .with_count(50)
///     .with_order(StarOrder::Desc);
/// ```
#[derive(Debug, Clone, Serialize, ToFormParams)]
pub struct GetUserStarsParams {
    /// User ID
    #[form(skip)]
    #[serde(skip)]
    pub user_id: UserId,

    /// Get stars with ID greater than this value (for pagination)
    #[form(name = "minId")]
    #[serde(rename = "minId", skip_serializing_if = "Option::is_none")]
    pub min_id: Option<u64>,

    /// Get stars with ID less than this value (for pagination)
    #[form(name = "maxId")]
    #[serde(rename = "maxId", skip_serializing_if = "Option::is_none")]
    pub max_id: Option<u64>,

    /// Maximum number of results to return (1-100, default: 20)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,

    /// Sort order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<StarOrder>,
}

impl GetUserStarsParams {
    /// Creates a new instance with the specified user ID.
    pub fn new(user_id: impl Into<UserId>) -> Self {
        Self {
            user_id: user_id.into(),
            min_id: None,
            max_id: None,
            count: None,
            order: None,
        }
    }

    /// Sets the minimum ID for pagination.
    pub fn with_min_id(mut self, min_id: u64) -> Self {
        self.min_id = Some(min_id);
        self
    }

    /// Sets the maximum ID for pagination.
    pub fn with_max_id(mut self, max_id: u64) -> Self {
        self.max_id = Some(max_id);
        self
    }

    /// Sets the number of results to return (1-100).
    pub fn with_count(mut self, count: u32) -> Self {
        self.count = Some(count.clamp(1, 100));
        self
    }

    /// Sets the sort order.
    pub fn with_order(mut self, order: StarOrder) -> Self {
        self.order = Some(order);
        self
    }
}

impl IntoRequest for GetUserStarsParams {
    fn path(&self) -> String {
        format!("/api/v2/users/{}/stars", self.user_id)
    }

    fn to_query(&self) -> impl Serialize {
        self
    }
}

/// Type alias for the Get User Stars API response.
pub type GetUserStarsResponse = Vec<Star>;

#[cfg(test)]
mod tests {
    use super::*;
    use backlog_core::identifier::Identifier;

    #[test]
    fn test_get_user_stars_params_basic() {
        let params = GetUserStarsParams::new(12345u32);

        assert_eq!(params.user_id.value(), 12345);
        assert_eq!(params.path(), "/api/v2/users/12345/stars");
        assert!(params.min_id.is_none());
        assert!(params.max_id.is_none());
        assert!(params.count.is_none());
        assert!(params.order.is_none());
    }

    #[test]
    fn test_get_user_stars_params_with_all_options() {
        let params = GetUserStarsParams::new(12345u32)
            .with_min_id(100)
            .with_max_id(200)
            .with_count(50)
            .with_order(StarOrder::Asc);

        assert_eq!(params.min_id, Some(100));
        assert_eq!(params.max_id, Some(200));
        assert_eq!(params.count, Some(50));
        assert!(matches!(params.order, Some(StarOrder::Asc)));
    }

    #[test]
    fn test_count_validation() {
        // Test that count is clamped to 1-100 range
        let params1 = GetUserStarsParams::new(12345u32).with_count(0);
        assert_eq!(params1.count, Some(1));

        let params2 = GetUserStarsParams::new(12345u32).with_count(150);
        assert_eq!(params2.count, Some(100));

        let params3 = GetUserStarsParams::new(12345u32).with_count(50);
        assert_eq!(params3.count, Some(50));
    }

    #[test]
    fn test_star_order_serialization() {
        let asc_json =
            serde_json::to_string(&StarOrder::Asc).expect("StarOrder::Asc should serialize");
        assert_eq!(asc_json, "\"asc\"");

        let desc_json =
            serde_json::to_string(&StarOrder::Desc).expect("StarOrder::Desc should serialize");
        assert_eq!(desc_json, "\"desc\"");
    }

    #[test]
    fn test_params_to_form() {
        let params = GetUserStarsParams::new(12345u32)
            .with_min_id(100)
            .with_count(25)
            .with_order(StarOrder::Desc);

        let _form = params.to_form();
        // ToFormParams macro generates the correct form structure
        // We'll verify this works correctly through integration tests
    }
}
