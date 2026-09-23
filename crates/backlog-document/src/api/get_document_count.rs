use crate::models::DocumentCount;
use backlog_api_core::IntoRequest;
use backlog_api_macros::ToFormParams;
use backlog_core::ProjectIdOrKey;
use serde::Serialize;

pub type GetDocumentCountResponse = DocumentCount;

/// Parameters for `GET /api/v2/documents/count` (one project, without search filters).
#[derive(Debug, Clone, ToFormParams)]
pub struct GetDocumentCountParams {
    pub project_id_or_key: ProjectIdOrKey,
}

impl GetDocumentCountParams {
    pub fn new(project_id_or_key: impl Into<ProjectIdOrKey>) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
        }
    }
}

impl IntoRequest for GetDocumentCountParams {
    fn path(&self) -> String {
        "/api/v2/documents/count".to_string()
    }

    fn to_query(&self) -> impl Serialize {
        // Display also handles numeric strings parsed as EitherIdOrKey.
        Vec::<(String, String)>::from(self)
    }
}
