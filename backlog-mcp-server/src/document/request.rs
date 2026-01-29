use rmcp::schemars;

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub(crate) struct GetDocumentDetailsRequest {
    #[schemars(description = "The document id to retrieve details for. 
    This should be in the format 32 digit hex string. Ensure there are no leading or trailing spaces.
    When you access https://example.backlog.com/document/PROJECT/0195faa11fcb7aaab4c4005a7ada4b6f,
    the document id is '0195faa11fcb7aaab4c4005a7ada4b6f'.")]
    pub document_id: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub(crate) struct DownloadDocumentAttachmentRequest {
    #[schemars(description = "The document ID (a 32-digit hexadecimal string).")]
    pub document_id: String,
    #[schemars(description = "The numeric ID of the attachment to download.")]
    pub attachment_id: u32,
    #[schemars(
        description = "Optional format specification: 'image', 'text', or 'raw'. If not specified, format will be auto-detected."
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub(crate) struct GetDocumentTreeRequest {
    #[schemars(
        description = "The project ID or project key for which to retrieve the document tree. Examples: \"MYPROJECTKEY\", \"123\"."
    )]
    pub project_id_or_key: String,
}

#[cfg(feature = "document_writable")]
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub(crate) struct AddDocumentRequest {
    #[schemars(
        description = "The numeric project ID where the document will be created. Use project_issue_type_list_get to find your project's ID."
    )]
    pub project_id: u32,
    #[schemars(description = "Optional document title.")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[schemars(description = "Optional document content (supports Markdown).")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[schemars(description = "Optional emoji icon displayed beside the title.")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
    #[schemars(
        description = "Optional parent document ID (32-digit hex string) for hierarchical placement."
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[schemars(
        description = "Optional placement order: true = add at end of siblings, false = add at beginning."
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_last: Option<bool>,
}

#[cfg(feature = "document_writable")]
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub(crate) struct DeleteDocumentRequest {
    #[schemars(
        description = "The document ID to delete (32-digit hexadecimal string). Example: '0195faa11fcb7aaab4c4005a7ada4b6f'."
    )]
    pub document_id: String,
}
