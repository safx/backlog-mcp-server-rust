use crate::models::Issue;
use backlog_api_core::{Error as ApiError, IntoRequest};
use backlog_api_macros::ToFormParams;
use chrono::{DateTime, Utc};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// An issue and its most recent view timestamp.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecentlyViewedIssue {
    pub issue: Issue,
    /// When the issue was most recently viewed, rather than modified.
    pub updated: DateTime<Utc>,
}

pub type GetRecentlyViewedIssuesResponse = Vec<RecentlyViewedIssue>;

/// Parameters for getting recently viewed issues
///
/// Corresponds to `GET /api/v2/users/myself/recentlyViewedIssues`.
#[derive(Debug, Clone, Builder, ToFormParams)]
#[builder(build_fn(error = "ApiError"))]
pub struct GetRecentlyViewedIssuesParams {
    /// Order of results ("asc" or "desc", default: "desc")
    #[builder(default, setter(into, strip_option))]
    pub order: Option<String>,

    /// Offset for pagination
    #[builder(default, setter(into, strip_option))]
    pub offset: Option<u32>,

    /// Number of results (1-100, default: 20)
    #[builder(default, setter(into, strip_option))]
    pub count: Option<u32>,
}

impl IntoRequest for GetRecentlyViewedIssuesParams {
    fn path(&self) -> String {
        "/api/v2/users/myself/recentlyViewedIssues".to_string()
    }

    fn to_query(&self) -> impl Serialize {
        let params: Vec<(String, String)> = self.into();
        params
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_params_builder() {
        let params = GetRecentlyViewedIssuesParamsBuilder::default()
            .order("asc")
            .offset(10u32)
            .count(50u32)
            .build()
            .unwrap();

        assert_eq!(params.order, Some("asc".to_string()));
        assert_eq!(params.offset, Some(10));
        assert_eq!(params.count, Some(50));
    }

    #[test]
    fn test_params_to_query() {
        let params = GetRecentlyViewedIssuesParamsBuilder::default()
            .order("desc")
            .count(25u32)
            .build()
            .unwrap();

        let query: Vec<(String, String)> = (&params).into();
        assert!(query.contains(&("order".to_string(), "desc".to_string())));
        assert!(query.contains(&("count".to_string(), "25".to_string())));
        assert!(!query.iter().any(|(k, _)| k == "offset")); // offset is None, should not be in query
    }

    #[test]
    fn test_default_params() {
        let params = GetRecentlyViewedIssuesParamsBuilder::default()
            .build()
            .unwrap();

        assert!(params.order.is_none());
        assert!(params.offset.is_none());
        assert!(params.count.is_none());

        let query: Vec<(String, String)> = (&params).into();
        assert!(query.is_empty());
    }

    #[test]
    fn test_path() {
        let params = GetRecentlyViewedIssuesParamsBuilder::default()
            .build()
            .unwrap();
        assert_eq!(params.path(), "/api/v2/users/myself/recentlyViewedIssues");
    }
}
