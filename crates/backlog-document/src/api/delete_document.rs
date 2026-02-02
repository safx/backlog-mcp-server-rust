#[cfg(feature = "writable")]
use backlog_api_core::{HttpMethod, IntoRequest};
use backlog_core::identifier::DocumentId;
use serde::Serialize;

use crate::models::DocumentResponse;

/// Response type for deleting a document.
///
/// Corresponds to `DELETE /api/v2/documents/:documentId`.
#[cfg(feature = "writable")]
pub type DeleteDocumentResponse = DocumentResponse;

/// Parameters for deleting a document.
///
/// Corresponds to `DELETE /api/v2/documents/:documentId`.
#[cfg(feature = "writable")]
#[derive(Debug, Clone)]
pub struct DeleteDocumentParams {
    /// Document ID to delete
    pub document_id: DocumentId,
}

#[cfg(feature = "writable")]
impl DeleteDocumentParams {
    /// Creates a new instance with the specified document ID.
    pub fn new(document_id: impl Into<DocumentId>) -> Self {
        Self {
            document_id: document_id.into(),
        }
    }
}

#[cfg(feature = "writable")]
impl IntoRequest for DeleteDocumentParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Delete
    }

    fn path(&self) -> String {
        format!("/api/v2/documents/{}", self.document_id)
    }

    fn to_form(&self) -> impl Serialize {
        // No body for DELETE request
        Vec::<(String, String)>::new()
    }
}
