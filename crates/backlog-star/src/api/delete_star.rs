use backlog_api_core::{HttpMethod, IntoRequest};
use backlog_core::identifier::StarId;
use serde::Serialize;

/// Parameters for deleting a star.
///
/// Corresponds to `DELETE /api/v2/stars/:starId`.
#[derive(Debug, Clone)]
pub struct DeleteStarParams {
    pub star_id: StarId,
}

impl DeleteStarParams {
    pub fn new(star_id: impl Into<StarId>) -> Self {
        Self {
            star_id: star_id.into(),
        }
    }
}

impl IntoRequest for DeleteStarParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Delete
    }

    fn path(&self) -> String {
        format!("/api/v2/stars/{}", self.star_id)
    }

    fn to_form(&self) -> impl Serialize {
        Vec::<(String, String)>::new()
    }
}
