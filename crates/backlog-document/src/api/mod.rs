// Main API struct
mod document_api;
pub use document_api::DocumentApi;

// Read-only API modules
mod download_attachment;
mod get_document;
mod get_document_comments;
mod get_document_count;
mod get_document_tree;
mod list_documents;

// Writable API modules
#[cfg(feature = "writable")]
mod add_document;
#[cfg(feature = "writable")]
mod add_document_tag;
#[cfg(feature = "writable")]
mod delete_document;
#[cfg(feature = "writable")]
mod remove_document_tag;

// Re-export parameter types and response types
pub use download_attachment::DownloadAttachmentParams;
pub use get_document::{GetDocumentParams, GetDocumentResponse};
pub use get_document_comments::{GetDocumentCommentsParams, GetDocumentCommentsResponse};
pub use get_document_count::{GetDocumentCountParams, GetDocumentCountResponse};
pub use get_document_tree::{
    GetDocumentTreeParams, GetDocumentTreeParamsBuilder, GetDocumentTreeResponse,
};
pub use list_documents::{
    DocumentOrder, DocumentSortKey, ListDocumentsParams, ListDocumentsParamsBuilder,
    ListDocumentsResponse,
};

#[cfg(feature = "writable")]
pub use add_document::{AddDocumentParams, AddDocumentResponse};
#[cfg(feature = "writable")]
pub use add_document_tag::{AddDocumentTagParams, AddDocumentTagResponse};
#[cfg(feature = "writable")]
pub use delete_document::{DeleteDocumentParams, DeleteDocumentResponse};
#[cfg(feature = "writable")]
pub use remove_document_tag::{RemoveDocumentTagParams, RemoveDocumentTagResponse};

#[cfg(feature = "writable")]
fn validate_tag_names(names: &[String]) -> backlog_core::Result<()> {
    if names.is_empty() || names.iter().any(|name| name.trim().is_empty()) {
        return Err(backlog_core::Error::InvalidParameter(
            "tag_names must contain at least one name, and names must not be blank".into(),
        ));
    }
    Ok(())
}
