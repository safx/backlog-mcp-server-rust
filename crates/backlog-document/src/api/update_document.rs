#[cfg(feature = "writable")]
use backlog_api_core::{HttpMethod, IntoRequest};
use backlog_api_macros::ToFormParams;
use backlog_core::identifier::DocumentId;
use serde::Serialize;

use crate::models::DocumentResponse;

/// Response type for updating document metadata.
///
/// Corresponds to `PATCH /api/v2/documents/:documentId`.
#[cfg(feature = "writable")]
pub type UpdateDocumentResponse = DocumentResponse;

/// Parameters for updating document title and emoji.
///
/// Corresponds to `PATCH /api/v2/documents/:documentId`.
///
/// `None` leaves a field unchanged; `Some("")` clears it.
/// The API requires at least one of `title` or `emoji`.
#[cfg(feature = "writable")]
#[derive(Debug, Clone, ToFormParams)]
pub struct UpdateDocumentParams {
    /// Document ID (path parameter)
    #[form(skip)]
    pub document_id: DocumentId,

    /// New title; empty string clears it
    pub title: Option<String>,

    /// New emoji; empty string removes it
    pub emoji: Option<String>,
}

#[cfg(feature = "writable")]
impl UpdateDocumentParams {
    /// Creates a new instance with the specified document ID.
    pub fn new(document_id: impl Into<DocumentId>) -> Self {
        Self {
            document_id: document_id.into(),
            title: None,
            emoji: None,
        }
    }

    /// Sets the document title.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Sets the emoji icon.
    pub fn emoji(mut self, emoji: impl Into<String>) -> Self {
        self.emoji = Some(emoji.into());
        self
    }
}

#[cfg(feature = "writable")]
impl IntoRequest for UpdateDocumentParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Patch
    }

    fn path(&self) -> String {
        format!("/api/v2/documents/{}", self.document_id)
    }

    fn to_form(&self) -> impl Serialize {
        let params: Vec<(String, String)> = self.into();
        params
    }
}
