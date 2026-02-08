use backlog_api_core::IntoRequest;

pub type GetResolutionListResponse = Vec<backlog_domain_models::Resolution>;

// GET /api/v2/resolutions
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct GetResolutionListParams;

impl IntoRequest for GetResolutionListParams {
    fn path(&self) -> String {
        "/api/v2/resolutions".to_string()
    }
}
