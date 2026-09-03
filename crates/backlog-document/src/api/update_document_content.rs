#[cfg(feature = "writable")]
use backlog_api_core::{HttpMethod, IntoRequest};
use backlog_api_macros::ToFormParams;
use backlog_core::identifier::DocumentId;
use serde::{Deserialize, Serialize};

use crate::models::DocumentResponse;

/// Response type for updating document content.
///
/// Corresponds to `PATCH /api/v2/documents/:documentId/content`.
#[cfg(feature = "writable")]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct UpdateDocumentContentResponse {
    #[serde(flatten)]
    pub document: DocumentResponse,

    /// Why content was not updated (e.g. "NO_CHANGE")
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,

    /// When true, `plain` is not Markdown
    #[serde(default)]
    pub markdown_is_fallback: bool,
}

/// Parameters for replacing document content.
///
/// Corresponds to `PATCH /api/v2/documents/:documentId/content`.
#[cfg(feature = "writable")]
#[derive(Debug, Clone, ToFormParams)]
pub struct UpdateDocumentContentParams {
    /// Document ID (path parameter)
    #[form(skip)]
    pub document_id: DocumentId,

    /// Full document body as Markdown
    pub content: String,
}

#[cfg(feature = "writable")]
impl UpdateDocumentContentParams {
    /// Creates a new instance with document ID and content.
    pub fn new(document_id: impl Into<DocumentId>, content: impl Into<String>) -> Self {
        Self {
            document_id: document_id.into(),
            content: content.into(),
        }
    }
}

#[cfg(feature = "writable")]
impl IntoRequest for UpdateDocumentContentParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Patch
    }

    fn path(&self) -> String {
        format!("/api/v2/documents/{}/content", self.document_id)
    }

    fn to_form(&self) -> impl Serialize {
        let params: Vec<(String, String)> = self.into();
        params
    }
}
