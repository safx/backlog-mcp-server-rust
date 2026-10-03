use crate::Project;
use backlog_api_core::{Error as ApiError, IntoRequest};
use backlog_api_macros::ToFormParams;
use chrono::{DateTime, Utc};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// A project and its most recent view timestamp.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecentlyViewedProject {
    pub project: Project,
    /// When the project was most recently viewed, rather than modified.
    pub updated: DateTime<Utc>,
}

pub type GetRecentlyViewedProjectsResponse = Vec<RecentlyViewedProject>;

/// Parameters for getting recently viewed projects
///
/// Corresponds to `GET /api/v2/users/myself/recentlyViewedProjects`.
#[derive(Debug, Clone, Builder, ToFormParams)]
#[builder(build_fn(error = "ApiError"))]
pub struct GetRecentlyViewedProjectsParams {
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

impl IntoRequest for GetRecentlyViewedProjectsParams {
    fn path(&self) -> String {
        "/api/v2/users/myself/recentlyViewedProjects".to_string()
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
        let params = GetRecentlyViewedProjectsParamsBuilder::default()
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
        let params = GetRecentlyViewedProjectsParamsBuilder::default()
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
        let params = GetRecentlyViewedProjectsParamsBuilder::default()
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
        let params = GetRecentlyViewedProjectsParamsBuilder::default()
            .build()
            .unwrap();
        assert_eq!(params.path(), "/api/v2/users/myself/recentlyViewedProjects");
    }
}
