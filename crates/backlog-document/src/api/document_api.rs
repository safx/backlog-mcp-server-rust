use super::{
    DownloadAttachmentParams, GetDocumentCountParams, GetDocumentCountResponse, GetDocumentParams,
    GetDocumentTreeParams, GetDocumentTreeResponse, ListDocumentsParams, ListDocumentsResponse,
};

#[cfg(feature = "writable")]
use super::{
    AddDocumentParams, AddDocumentResponse, AddDocumentTagParams, AddDocumentTagResponse,
    DeleteDocumentParams, DeleteDocumentResponse, RemoveDocumentTagParams,
    RemoveDocumentTagResponse,
};
use crate::models::DocumentDetail;
use backlog_api_core::Result;
use client::{Client, DownloadedFile};

pub struct DocumentApi(Client);

impl DocumentApi {
    pub fn new(client: Client) -> Self {
        Self(client)
    }

    /// Get documents
    ///
    /// Corresponds to `GET /api/v2/documents`.
    pub async fn list_documents(
        &self,
        mut params: ListDocumentsParams,
    ) -> Result<ListDocumentsResponse> {
        params.validate()?;
        params.offset.get_or_insert(0);
        self.0.execute(params).await
    }

    /// Corresponds to `GET /api/v2/documents/count`.
    pub async fn get_document_count(
        &self,
        params: GetDocumentCountParams,
    ) -> Result<GetDocumentCountResponse> {
        self.0.execute(params).await
    }

    /// Corresponds to `POST /api/v2/documents/:documentId/tags`.
    #[cfg(feature = "writable")]
    pub async fn add_document_tag(
        &self,
        params: AddDocumentTagParams,
    ) -> Result<AddDocumentTagResponse> {
        params.validate()?;
        self.0.execute(params).await
    }

    /// Corresponds to `DELETE /api/v2/documents/:documentId/tags` (204 No Content).
    #[cfg(feature = "writable")]
    pub async fn remove_document_tag(
        &self,
        params: RemoveDocumentTagParams,
    ) -> Result<RemoveDocumentTagResponse> {
        params.validate()?;
        self.0.execute_no_content(params).await
    }

    /// Get document tree
    ///
    /// Corresponds to `GET /api/v2/documents/tree`.
    pub async fn get_document_tree(
        &self,
        params: GetDocumentTreeParams,
    ) -> Result<GetDocumentTreeResponse> {
        self.0.execute(params).await
    }

    /// Get document
    ///
    /// Corresponds to `GET /api/v2/documents/:documentId`.
    pub async fn get_document(&self, params: GetDocumentParams) -> Result<DocumentDetail> {
        self.0.execute(params).await
    }

    /// Get document attachment
    ///
    /// Corresponds to `GET /api/v2/documents/:documentId/attachments/:attachmentId`.
    pub async fn download_attachment(
        &self,
        params: DownloadAttachmentParams,
    ) -> Result<DownloadedFile> {
        self.0.download_file(params).await
    }

    /// Add document
    ///
    /// Corresponds to `POST /api/v2/documents`.
    #[cfg(feature = "writable")]
    pub async fn add_document(&self, params: AddDocumentParams) -> Result<AddDocumentResponse> {
        self.0.execute(params).await
    }

    /// Delete document
    ///
    /// Corresponds to `DELETE /api/v2/documents/:documentId`.
    #[cfg(feature = "writable")]
    pub async fn delete_document(
        &self,
        params: DeleteDocumentParams,
    ) -> Result<DeleteDocumentResponse> {
        self.0.execute(params).await
    }
}
