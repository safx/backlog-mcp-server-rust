use std::collections::HashSet;
use std::str::FromStr;
use std::sync::Arc;

use tokio::sync::Mutex;

use backlog_api_client::client::BacklogApiClient;
use backlog_api_client::{
    DocumentDetail, DownloadAttachmentParams, DownloadedFile, GetDocumentCommentsParams,
    GetDocumentCommentsResponse, GetDocumentCountParams, GetDocumentCountResponse,
    GetDocumentParams, GetDocumentTreeParams, GetDocumentTreeResponse, ListDocumentsParamsBuilder,
    ListDocumentsResponse,
};
use backlog_core::{
    ProjectIdOrKey,
    identifier::{DocumentAttachmentId, DocumentId, ProjectId},
};

#[cfg(feature = "document_writable")]
use super::request::{AddDocumentRequest, DeleteDocumentRequest, DocumentTagsRequest};
use super::request::{
    DownloadDocumentAttachmentRequest, GetDocumentCommentsRequest, GetDocumentCountRequest,
    GetDocumentDetailsRequest, GetDocumentTreeRequest, ListDocumentsRequest,
};

use crate::access_control::AccessControl;
use crate::error::{Error, Result};

#[cfg(feature = "document_writable")]
use backlog_api_client::{
    AddDocumentParams, AddDocumentResponse, AddDocumentTagParams, AddDocumentTagResponse,
    DeleteDocumentParams, DeleteDocumentResponse, RemoveDocumentTagParams,
};

fn parameter_error(error: impl std::fmt::Display) -> Error {
    Error::Parameter(error.to_string())
}

pub(crate) async fn list_documents_bridge(
    client: Arc<Mutex<BacklogApiClient>>,
    req: ListDocumentsRequest,
    access_control: &AccessControl,
) -> Result<ListDocumentsResponse> {
    let mut builder = ListDocumentsParamsBuilder::default();
    if let Some(ids) = req.project_ids {
        builder.project_ids(ids.into_iter().map(ProjectId::new).collect::<Vec<_>>());
    }
    if let Some(keyword) = req.keyword {
        builder.keyword(keyword);
    }
    if let Some(sort) = req.sort {
        builder.sort(sort);
    }
    if let Some(order) = req.order {
        builder.order(order);
    }
    if let Some(offset) = req.offset {
        builder.offset(offset);
    }
    if let Some(count) = req.count {
        builder.count(count);
    }
    let mut params = builder.build().map_err(parameter_error)?;
    params.validate().map_err(parameter_error)?;
    let client = client.lock().await;
    params.project_ids = access_control
        .scope_document_projects(params.project_ids, &client)
        .await?;
    let documents = client.document().list_documents(params).await?;
    if access_control.is_enabled() {
        let mut checked_projects = HashSet::new();
        for document in &documents {
            if checked_projects.insert(document.project_id) {
                access_control
                    .check_project_access_by_id_async(&document.project_id, &client)
                    .await?;
            }
        }
    }
    Ok(documents)
}

pub(crate) async fn get_document_count_bridge(
    client: Arc<Mutex<BacklogApiClient>>,
    req: GetDocumentCountRequest,
    access_control: &AccessControl,
) -> Result<GetDocumentCountResponse> {
    let project =
        ProjectIdOrKey::from_str(req.project_id_or_key.trim()).map_err(parameter_error)?;
    let client = client.lock().await;
    access_control
        .check_project_access_id_or_key_async(&project, &client)
        .await?;
    Ok(client
        .document()
        .get_document_count(GetDocumentCountParams::new(project))
        .await?)
}

#[cfg(feature = "document_writable")]
async fn check_document_access(
    client: &BacklogApiClient,
    document_id: &DocumentId,
    access_control: &AccessControl,
) -> Result<()> {
    if !access_control.is_enabled() {
        return Ok(());
    }
    let document = client
        .document()
        .get_document(GetDocumentParams::new(document_id.clone()))
        .await?;
    access_control
        .check_project_access_by_id_async(&document.project_id, client)
        .await
}

#[cfg(feature = "document_writable")]
pub(crate) async fn add_document_tag_bridge(
    client: Arc<Mutex<BacklogApiClient>>,
    req: DocumentTagsRequest,
    access_control: &AccessControl,
) -> Result<AddDocumentTagResponse> {
    let document_id = DocumentId::from_str(req.document_id.trim()).map_err(parameter_error)?;
    let params = AddDocumentTagParams::new(document_id, req.tag_names);
    params.validate().map_err(parameter_error)?;
    let client = client.lock().await;
    check_document_access(&client, &params.document_id, access_control).await?;
    Ok(client.document().add_document_tag(params).await?)
}

#[cfg(feature = "document_writable")]
pub(crate) async fn remove_document_tag_bridge(
    client: Arc<Mutex<BacklogApiClient>>,
    req: DocumentTagsRequest,
    access_control: &AccessControl,
) -> Result<()> {
    let document_id = DocumentId::from_str(req.document_id.trim()).map_err(parameter_error)?;
    let params = RemoveDocumentTagParams::new(document_id, req.tag_names);
    params.validate().map_err(parameter_error)?;
    let client = client.lock().await;
    check_document_access(&client, &params.document_id, access_control).await?;
    Ok(client.document().remove_document_tag(params).await?)
}

pub(crate) async fn get_document_details(
    client: Arc<Mutex<BacklogApiClient>>,
    req: GetDocumentDetailsRequest,
    access_control: &AccessControl,
) -> Result<DocumentDetail> {
    let client_guard = client.lock().await;
    let document_id = DocumentId::from_str(req.document_id.trim())?;
    let params = GetDocumentParams::new(document_id.clone());
    let document = client_guard.document().get_document(params).await?;

    // Check project access
    access_control
        .check_project_access_by_id_async(&document.project_id, &client_guard)
        .await?;

    Ok(document)
}

pub(crate) async fn get_document_comments_bridge(
    client: Arc<Mutex<BacklogApiClient>>,
    req: GetDocumentCommentsRequest,
    access_control: &AccessControl,
) -> Result<GetDocumentCommentsResponse> {
    let client_guard = client.lock().await;
    let document_id = DocumentId::from_str(req.document_id.trim())?;

    // Comments carry no project id, so resolve it via the document first
    let document = client_guard
        .document()
        .get_document(GetDocumentParams::new(document_id.clone()))
        .await?;

    access_control
        .check_project_access_by_id_async(&document.project_id, &client_guard)
        .await?;

    client_guard
        .document()
        .get_document_comments(GetDocumentCommentsParams::new(document_id))
        .await
        .map_err(crate::error::Error::from)
}

pub(crate) async fn download_document_attachment_bridge(
    client: Arc<Mutex<BacklogApiClient>>,
    req: DownloadDocumentAttachmentRequest,
    access_control: &AccessControl,
) -> Result<DownloadedFile> {
    let client_guard = client.lock().await;
    let document_id = DocumentId::from_str(req.document_id.trim())?;

    // First get document details to check project access
    let document = client_guard
        .document()
        .get_document(GetDocumentParams::new(document_id.clone()))
        .await?;

    // Check project access
    access_control
        .check_project_access_by_id_async(&document.project_id, &client_guard)
        .await?;

    let attachment_id = DocumentAttachmentId::new(req.attachment_id);
    let params = DownloadAttachmentParams::new(document_id, attachment_id);
    client_guard
        .document()
        .download_attachment(params)
        .await
        .map_err(crate::error::Error::from)
}

pub(crate) async fn get_document_tree_tool(
    client: Arc<Mutex<BacklogApiClient>>,
    req: GetDocumentTreeRequest,
    access_control: &AccessControl,
) -> Result<GetDocumentTreeResponse> {
    let client_guard = client.lock().await;
    let project_id_or_key_val = ProjectIdOrKey::from_str(req.project_id_or_key.trim())?;

    // Check project access with parsed type
    access_control
        .check_project_access_id_or_key_async(&project_id_or_key_val, &client_guard)
        .await?;
    // Construct directly instead of using the builder, to sidestep the E0599 error for now.
    let params = GetDocumentTreeParams {
        project_id_or_key: project_id_or_key_val,
    };

    client_guard
        .document()
        .get_document_tree(params)
        .await
        .map_err(crate::error::Error::from)
}

#[cfg(feature = "document_writable")]
pub(crate) async fn add_document_bridge(
    client: Arc<Mutex<BacklogApiClient>>,
    req: AddDocumentRequest,
    access_control: &AccessControl,
) -> Result<AddDocumentResponse> {
    let client_guard = client.lock().await;
    let project_id = ProjectId::new(req.project_id);

    // Check project access first
    access_control
        .check_project_access_by_id_async(&project_id, &client_guard)
        .await?;

    // Build AddDocumentParams
    let mut params = AddDocumentParams::new(project_id);

    if let Some(title) = req.title {
        params = params.title(title);
    }
    if let Some(content) = req.content {
        params = params.content(content);
    }
    if let Some(emoji) = req.emoji {
        params = params.emoji(emoji);
    }
    if let Some(parent_id_str) = req.parent_id {
        let parent_id = DocumentId::from_str(parent_id_str.trim())?;
        params = params.parent_id(parent_id);
    }
    if let Some(add_last) = req.add_last {
        params = params.add_last(add_last);
    }

    client_guard
        .document()
        .add_document(params)
        .await
        .map_err(crate::error::Error::from)
}

#[cfg(feature = "document_writable")]
pub(crate) async fn delete_document_bridge(
    client: Arc<Mutex<BacklogApiClient>>,
    req: DeleteDocumentRequest,
    access_control: &AccessControl,
) -> Result<DeleteDocumentResponse> {
    let client_guard = client.lock().await;
    let document_id = DocumentId::from_str(req.document_id.trim())?;

    // First get document details to verify project access
    let document = client_guard
        .document()
        .get_document(GetDocumentParams::new(document_id.clone()))
        .await?;

    // Check project access
    access_control
        .check_project_access_by_id_async(&document.project_id, &client_guard)
        .await?;

    // Delete the document
    let params = DeleteDocumentParams::new(document_id);
    client_guard
        .document()
        .delete_document(params)
        .await
        .map_err(crate::error::Error::from)
}
