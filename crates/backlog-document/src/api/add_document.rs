#[cfg(feature = "writable")]
use backlog_api_core::{HttpMethod, IntoRequest};
use backlog_api_macros::ToFormParams;
use backlog_core::identifier::{DocumentId, ProjectId};
use serde::Serialize;

use crate::models::DocumentResponse;

/// Response type for adding a document.
///
/// Corresponds to `POST /api/v2/documents`.
#[cfg(feature = "writable")]
pub type AddDocumentResponse = DocumentResponse;

/// Parameters for adding a new document.
///
/// Corresponds to `POST /api/v2/documents`.
#[cfg(feature = "writable")]
#[derive(Debug, Clone, ToFormParams)]
pub struct AddDocumentParams {
    /// Project ID (sent as form data, not in path)
    #[form(skip)]
    pub project_id: ProjectId,

    /// Document title
    pub title: Option<String>,

    /// Document content (supports Markdown)
    pub content: Option<String>,

    /// Emoji icon displayed beside the title
    pub emoji: Option<String>,

    /// Parent document ID for hierarchical placement
    #[form(name = "parentId")]
    pub parent_id: Option<DocumentId>,

    /// Placement order (true = end of siblings, false = beginning)
    #[form(name = "addLast")]
    pub add_last: Option<bool>,
}

#[cfg(feature = "writable")]
impl AddDocumentParams {
    /// Creates a new instance with the specified project ID.
    pub fn new(project_id: impl Into<ProjectId>) -> Self {
        Self {
            project_id: project_id.into(),
            title: None,
            content: None,
            emoji: None,
            parent_id: None,
            add_last: None,
        }
    }

    /// Sets the document title.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Sets the document content.
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }

    /// Sets the emoji icon.
    pub fn emoji(mut self, emoji: impl Into<String>) -> Self {
        self.emoji = Some(emoji.into());
        self
    }

    /// Sets the parent document ID for hierarchical placement.
    pub fn parent_id(mut self, parent_id: impl Into<DocumentId>) -> Self {
        self.parent_id = Some(parent_id.into());
        self
    }

    /// Sets the placement order.
    pub fn add_last(mut self, add_last: bool) -> Self {
        self.add_last = Some(add_last);
        self
    }
}

#[cfg(feature = "writable")]
impl IntoRequest for AddDocumentParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Post
    }

    fn path(&self) -> String {
        "/api/v2/documents".to_string()
    }

    fn to_form(&self) -> impl Serialize {
        let mut params: Vec<(String, String)> = self.into();
        params.push(("projectId".to_string(), self.project_id.to_string()));
        params
    }
}
