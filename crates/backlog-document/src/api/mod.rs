// Main API struct
mod document_api;
pub use document_api::DocumentApi;

// Read-only API modules
mod download_attachment;
mod get_document;
mod get_document_tree;
mod list_documents;

// Writable API modules
#[cfg(feature = "writable")]
mod add_document;
#[cfg(feature = "writable")]
mod delete_document;

// Re-export parameter types and response types
pub use download_attachment::DownloadAttachmentParams;
pub use get_document::{GetDocumentParams, GetDocumentResponse};
pub use get_document_tree::{GetDocumentTreeParams, GetDocumentTreeResponse};
pub use list_documents::{
    DocumentOrder, DocumentSortKey, ListDocumentsParams, ListDocumentsParamsBuilder,
    ListDocumentsResponse,
};

#[cfg(feature = "writable")]
pub use add_document::{AddDocumentParams, AddDocumentResponse};
#[cfg(feature = "writable")]
pub use delete_document::{DeleteDocumentParams, DeleteDocumentResponse};
