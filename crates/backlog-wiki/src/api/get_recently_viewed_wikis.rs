use crate::models::Wiki;
use backlog_api_core::{Error as ApiError, IntoRequest};
use backlog_api_macros::ToFormParams;
use chrono::{DateTime, Utc};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// A wiki page and its most recent view timestamp.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct RecentlyViewedWiki {
    pub page: Wiki,
    /// When the page was most recently viewed, rather than modified.
    pub updated: DateTime<Utc>,
}

pub type GetRecentlyViewedWikisResponse = Vec<RecentlyViewedWiki>;

/// Parameters for getting recently viewed wikis.
///
/// Corresponds to `GET /api/v2/users/myself/recentlyViewedWikis`.
#[derive(Debug, Clone, Builder, ToFormParams)]
#[builder(build_fn(error = "ApiError"))]
pub struct GetRecentlyViewedWikisParams {
    /// Sort order of the results. Either "asc" or "desc". Default is "desc".
    #[builder(default, setter(into, strip_option))]
    pub order: Option<String>,

    /// Offset for pagination.
    #[builder(default, setter(into, strip_option))]
    pub offset: Option<u32>,

    /// Number of items to retrieve (1-100). Default is 20.
    #[builder(default, setter(into, strip_option))]
    pub count: Option<u32>,
}

impl IntoRequest for GetRecentlyViewedWikisParams {
    fn path(&self) -> String {
        "/api/v2/users/myself/recentlyViewedWikis".to_string()
    }

    fn to_query(&self) -> impl Serialize {
        let params: Vec<(String, String)> = self.into();
        params
    }
}
