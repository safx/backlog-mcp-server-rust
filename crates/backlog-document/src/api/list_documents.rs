use crate::models::Document;
use backlog_api_core::IntoRequest;
use backlog_api_macros::ToFormParams;
use backlog_core::identifier::ProjectId;
use derive_builder::Builder;
use serde::Serialize;
use std::fmt;

/// Response type for listing documents
pub type ListDocumentsResponse = Vec<Document>;

/// Parameters for listing documents
///
/// Corresponds to `GET /api/v2/documents`.
#[derive(Debug, Builder, Clone, PartialEq, ToFormParams)]
#[builder(setter(strip_option))]
pub struct ListDocumentsParams {
    // Based on curl: /api/v2/documents?apiKey=xxx&projectId=601486&offset=0&count=1
    // Based on OpenAPI: /api/v2/:projectKey/list?keyword=X&sort=Y
    // User confirmed routing definition /api/v2/documents is primary.
    // So, projectId is a query param.
    #[builder(default, setter(into))]
    #[form(array, name = "projectId")]
    pub project_ids: Option<Vec<ProjectId>>, // Array of project IDs (optional)
    #[builder(default, setter(into))]
    pub keyword: Option<String>,
    #[builder(default, setter(into))]
    pub sort: Option<DocumentSortKey>, // Enum to be defined
    #[builder(default, setter(into))]
    pub order: Option<DocumentOrder>, // Sort order
    #[builder(default = "Some(0)")]
    pub offset: Option<u32>,
    #[builder(default)]
    pub count: Option<u32>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum DocumentSortKey {
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "updated")]
    Updated,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum DocumentOrder {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}

impl fmt::Display for DocumentSortKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DocumentSortKey::Created => write!(f, "created"),
            DocumentSortKey::Updated => write!(f, "updated"),
        }
    }
}

impl fmt::Display for DocumentOrder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DocumentOrder::Asc => write!(f, "asc"),
            DocumentOrder::Desc => write!(f, "desc"),
        }
    }
}

// This From implementation is crucial for client.get_with_params
impl From<ListDocumentsParams> for Vec<(String, String)> {
    fn from(params: ListDocumentsParams) -> Self {
        (&params).into()
    }
}

// IntoRequest implementations for unified access control
impl IntoRequest for ListDocumentsParams {
    fn path(&self) -> String {
        "/api/v2/documents".to_string()
    }

    fn to_query(&self) -> impl Serialize {
        let params: Vec<(String, String)> = self.into();
        params
    }
}
