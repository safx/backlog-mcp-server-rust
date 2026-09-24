// Main API struct
mod file_api;
pub use file_api::FileApi;

// Read-only API modules
mod get_file;
mod get_shared_files_list;

// Re-export parameter types and response types
pub use get_file::{GetFileParams, GetFileParamsBuilder};
pub use get_shared_files_list::{
    GetSharedFilesListParams, GetSharedFilesListParamsBuilder, GetSharedFilesListResponse,
};
