use rmcp::schemars;

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub(crate) struct GetWikiDetailRequest {
    #[schemars(description = "Wiki page ID to retrieve details for. Must be a positive integer.")]
    pub wiki_id: u32,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub(crate) struct GetWikiListRequest {
    #[schemars(
        description = "Optional project ID or project key to filter wiki pages. Examples: \"MYPROJECTKEY\", \"123\"."
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id_or_key: Option<String>,
    #[schemars(description = "Optional keyword to search for in wiki page names or content.")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword: Option<String>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub(crate) struct GetWikiAttachmentListRequest {
    #[schemars(
        description = "Wiki page ID to retrieve attachments for. Must be a positive integer."
    )]
    pub wiki_id: u32,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub(crate) struct DownloadWikiAttachmentRequest {
    #[schemars(
        description = "Wiki page ID to download attachment from. Must be a positive integer."
    )]
    pub wiki_id: u32,
    #[schemars(description = "Attachment ID to download. Must be a positive integer.")]
    pub attachment_id: u32,
    #[schemars(
        description = "Optional format specification: 'image', 'text', or 'raw'. If not specified, format will be auto-detected."
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}

#[cfg(feature = "wiki_writable")]
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub(crate) struct UpdateWikiRequest {
    #[schemars(description = "Wiki page ID to update. Must be a positive integer.")]
    pub wiki_id: u32,
    #[schemars(description = "Optional new page name.")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[schemars(description = "Optional new page content.")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[schemars(description = "Optional whether to send email notification of update.")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mail_notify: Option<bool>,
}
