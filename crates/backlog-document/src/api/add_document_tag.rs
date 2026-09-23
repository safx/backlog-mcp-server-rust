use crate::models::DocumentTag;
use backlog_api_core::{HttpMethod, IntoRequest};
use backlog_api_macros::ToFormParams;
use backlog_core::identifier::DocumentId;
use serde::Serialize;

pub type AddDocumentTagResponse = Vec<DocumentTag>;

/// Parameters for `POST /api/v2/documents/:documentId/tags`.
#[derive(Debug, Clone, ToFormParams)]
pub struct AddDocumentTagParams {
    #[form(skip)]
    pub document_id: DocumentId,
    #[form(array, name = "tagNames")]
    pub tag_names: Vec<String>,
}

impl AddDocumentTagParams {
    pub fn new(document_id: DocumentId, tag_names: Vec<String>) -> Self {
        Self {
            document_id,
            tag_names,
        }
    }

    /// Reject empty requests and blank names without altering the supplied names.
    pub fn validate(&self) -> backlog_core::Result<()> {
        super::validate_tag_names(&self.tag_names)
    }
}

impl IntoRequest for AddDocumentTagParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Post
    }
    fn path(&self) -> String {
        format!("/api/v2/documents/{}/tags", self.document_id)
    }
    fn to_form(&self) -> impl Serialize {
        Vec::<(String, String)>::from(self)
    }
}
