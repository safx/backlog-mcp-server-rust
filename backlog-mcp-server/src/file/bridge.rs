use crate::access_control::AccessControl;
use crate::error::{Error as McpError, Result};
use crate::file::request::{DownloadSharedFileRequest, GetSharedFilesListRequest};
use backlog_api_client::{DownloadedFile, client::BacklogApiClient};
use backlog_core::{ProjectIdOrKey, identifier::SharedFileId};
use backlog_file::{GetFileParams, GetSharedFilesListParams, SharedFile};
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::Mutex;

pub(crate) async fn get_shared_files_list_tool(
    client: Arc<Mutex<BacklogApiClient>>,
    request: GetSharedFilesListRequest,
    access_control: &AccessControl,
) -> Result<Vec<SharedFile>> {
    let client_guard = client.lock().await;

    let project_id_or_key = ProjectIdOrKey::from_str(&request.project_id_or_key)?;

    // Check project access with parsed type
    access_control
        .check_project_access_id_or_key_async(&project_id_or_key, &client_guard)
        .await?;

    let params = GetSharedFilesListParams {
        project_id_or_key,
        path: request.path,
        order: request.order,
        offset: request.offset,
        count: request.count,
    };

    Ok(client_guard.file().get_shared_files_list(params).await?)
}

pub(crate) async fn download_shared_file_bridge(
    client: Arc<Mutex<BacklogApiClient>>,
    request: DownloadSharedFileRequest,
    access_control: &AccessControl,
) -> Result<DownloadedFile> {
    let client_guard = client.lock().await;

    let project_id_or_key = ProjectIdOrKey::from_str(&request.project_id_or_key)?;

    // Check project access with parsed type
    access_control
        .check_project_access_id_or_key_async(&project_id_or_key, &client_guard)
        .await?;

    let shared_file_id = SharedFileId::new(request.shared_file_id);
    let params = GetFileParams::new(project_id_or_key, shared_file_id);

    Ok(client_guard.file().get_file(params).await?)
}
