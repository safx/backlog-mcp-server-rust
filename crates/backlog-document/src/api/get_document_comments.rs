use crate::models::DocumentComment;
use backlog_api_core::IntoRequest;
use backlog_core::identifier::DocumentId;

/// Response type for getting document comments
pub type GetDocumentCommentsResponse = Vec<DocumentComment>;

/// Parameters for getting comments of a document
///
/// Corresponds to `GET /api/v2/documents/:documentId/comments`.
#[derive(Debug, Clone, PartialEq)]
pub struct GetDocumentCommentsParams {
    pub document_id: DocumentId,
}

impl GetDocumentCommentsParams {
    pub fn new(document_id: impl Into<DocumentId>) -> Self {
        Self {
            document_id: document_id.into(),
        }
    }
}

impl IntoRequest for GetDocumentCommentsParams {
    fn path(&self) -> String {
        format!("/api/v2/documents/{}/comments", self.document_id)
    }
}
