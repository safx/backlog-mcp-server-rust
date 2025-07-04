#[cfg(feature = "writable")]
use backlog_api_core::IntoUploadRequest;
#[cfg(feature = "writable")]
use std::path::PathBuf;

/// Response type for uploading an attachment
pub type UploadAttachmentResponse = AttachmentInfo;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct AttachmentInfo {
    pub id: u32,
    pub name: String,
    pub size: u64,
}

/// Parameters for uploading an attachment
#[cfg(feature = "writable")]
#[derive(Debug, Clone)]
pub struct UploadAttachmentParams {
    pub file_path: PathBuf,
}

#[cfg(feature = "writable")]
impl UploadAttachmentParams {
    pub fn new(file_path: PathBuf) -> Self {
        Self { file_path }
    }
}

#[cfg(feature = "writable")]
impl IntoUploadRequest for UploadAttachmentParams {
    fn path(&self) -> String {
        "/api/v2/space/attachment".to_string()
    }

    fn file_path(&self) -> &PathBuf {
        &self.file_path
    }
}
