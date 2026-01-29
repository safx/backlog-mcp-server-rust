#![allow(unused_imports, dead_code)]

use crate::file_utils::{FileFormat, SerializableFile};
#[cfg(feature = "issue_writable")]
use crate::issue::request::{AddIssueRequest, UpdateCommentRequest};
use crate::issue::request::{
    GetIssueCommentsRequest, GetIssueSharedFilesRequest, UpdateIssueRequest,
};
use crate::issue::response_transformer::IssueResponse;
use crate::{
    document::{
        self,
        request::{
            DownloadDocumentAttachmentRequest, GetDocumentDetailsRequest, GetDocumentTreeRequest,
        },
    },
    file::{
        self,
        request::{DownloadSharedFileRequest, GetSharedFilesListRequest},
    },
    git::{
        self,
        request::{
            DownloadPullRequestAttachmentRequest, GetPullRequestAttachmentListRequest,
            GetPullRequestCommentListRequest, GetPullRequestDetailsRequest,
            GetRepositoryDetailsRequest, GetRepositoryListRequest, ListPullRequestsRequest,
        },
    },
    issue::{
        self,
        request::{
            AddCommentRequest, DownloadAttachmentRequest, GetAttachmentListRequest,
            GetIssueDetailsRequest, GetIssuesByMilestoneNameRequest,
            GetVersionMilestoneListRequest,
        },
    },
    project::{
        self,
        request::{
            GetCustomFieldListRequest, GetPrioritiesRequest, GetProjectIssueTypesRequest,
            GetProjectStatusListRequest,
        },
    },
    user::{self, request::GetUserListRequest},
    wiki::{
        self,
        request::{
            DownloadWikiAttachmentRequest, GetWikiAttachmentListRequest, GetWikiDetailRequest,
            GetWikiListRequest,
        },
    },
};

#[cfg(feature = "wiki_writable")]
use crate::wiki::request::UpdateWikiRequest;

#[cfg(feature = "document_writable")]
use crate::document::request::{AddDocumentRequest, DeleteDocumentRequest};

use crate::access_control::AccessControl;
#[cfg(feature = "git_writable")]
use crate::git::request::AddPullRequestCommentRequest;
use backlog_api_client::client::BacklogApiClient;
use rmcp::handler::server::router::tool;
use rmcp::{
    ErrorData as McpError,
    handler::server::{tool::ToolRouter, wrapper::Parameters},
    model::{CallToolResult, Content, ServerCapabilities, ServerInfo},
    tool, tool_handler, tool_router,
};
use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct Server {
    client: Arc<Mutex<BacklogApiClient>>,
    access_control: AccessControl,
    pub tool_router: ToolRouter<Self>,
}

type McpResult = Result<CallToolResult, McpError>;

#[tool_router]
impl Server {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let base_url = env::var("BACKLOG_BASE_URL")
            .map_err(|_| "BACKLOG_BASE_URL environment variable not set")?;
        let api_key = env::var("BACKLOG_API_KEY")
            .map_err(|_| "BACKLOG_API_KEY environment variable not set")?;
        let prefix = env::var("BACKLOG_PREFIX").unwrap_or("backlog_".to_string());

        eprintln!("Initializing with base_url: {base_url}");

        let client = BacklogApiClient::new(&base_url)?.with_api_key(api_key);
        let access_control = AccessControl::new()?;

        Ok(Self {
            client: Arc::new(Mutex::new(client)),
            access_control,
            tool_router: Self::create_tool_router(&prefix),
        })
    }

    fn create_tool_router(prefix: &str) -> ToolRouter<Self> {
        let mut tool_router = Self::tool_router();

        if prefix.is_empty() {
            return tool_router;
        }

        let original_keys: Vec<_> = tool_router.map.keys().cloned().collect();
        for key in original_keys {
            let new_key = format!("{}{}", prefix, key);
            if let Some(mut route) = tool_router.map.remove(&key) {
                // Update the tool name in the attribute
                route.attr.name = new_key.clone().into();
                tool_router.map.insert(new_key.into(), route);
            }
        }
        tool_router
    }

    #[tool(
        description = "Get a list of Git repositories for a specified project. Requires project_id_or_key parameter."
    )]
    async fn git_repository_list_get(
        &self,
        request: Parameters<GetRepositoryListRequest>,
    ) -> McpResult {
        let repositories =
            git::bridge::get_repository_list(self.client.clone(), request.0, &self.access_control)
                .await?;
        Ok(CallToolResult::success(vec![Content::json(repositories)?]))
    }

    #[tool(
        description = "Get details for a specific Git repository. Requires project_id_or_key and repository_id_or_name parameters."
    )]
    async fn git_repository_details_get(
        &self,
        request: Parameters<GetRepositoryDetailsRequest>,
    ) -> McpResult {
        let repository =
            git::bridge::get_repository(self.client.clone(), request.0, &self.access_control)
                .await?;
        Ok(CallToolResult::success(vec![Content::json(repository)?]))
    }

    #[tool(
        description = "Get a list of pull requests for a specified repository. Requires project_id_or_key and repository_id_or_name. Optional: status, assignee_id, issue_id, created_by_id, offset, count."
    )]
    async fn git_pr_list_get(&self, request: Parameters<ListPullRequestsRequest>) -> McpResult {
        let pull_requests = git::bridge::get_pull_request_list(
            self.client.clone(),
            request.0,
            &self.access_control,
        )
        .await?;
        Ok(CallToolResult::success(vec![Content::json(pull_requests)?]))
    }

    #[tool(
        description = "Get details for a specific pull request. Requires project_id_or_key, repository_id_or_name, and number (PR number) parameters."
    )]
    async fn git_pr_details_get(
        &self,
        request: Parameters<GetPullRequestDetailsRequest>,
    ) -> McpResult {
        let pull_request =
            git::bridge::get_pull_request(self.client.clone(), request.0, &self.access_control)
                .await?;
        Ok(CallToolResult::success(vec![Content::json(pull_request)?]))
    }

    #[tool(
        description = "Get details for a specific Backlog issue including custom fields. Requires issue_id_or_key parameter (e.g., 'PROJ-123' or issue ID)."
    )]
    async fn issue_details_get(&self, request: Parameters<GetIssueDetailsRequest>) -> McpResult {
        let issue =
            issue::bridge::get_issue_details(self.client.clone(), request.0, &self.access_control)
                .await?;
        let issue_response = IssueResponse::from(issue);
        Ok(CallToolResult::success(vec![Content::json(
            issue_response,
        )?]))
    }

    #[tool(
        description = "Get details for a specific Backlog document. Returns document title, content as both Markdown ('plain') and ProseMirror JSON ('json'), and metadata. Requires project_id_or_key and document_id."
    )]
    async fn document_details_get(
        &self,
        request: Parameters<GetDocumentDetailsRequest>,
    ) -> McpResult {
        let document = document::bridge::get_document_details(
            self.client.clone(),
            request.0,
            &self.access_control,
        )
        .await?;

        Ok(CallToolResult::success(vec![Content::json(document)?]))
    }

    #[tool(
        description = "Download a document attachment. Requires project_id_or_key and attachment_id. Optional format parameter: 'image', 'text', or 'raw' (auto-detected if not specified). Returns file content in the appropriate format."
    )]
    async fn document_attachment_download(
        &self,
        request: Parameters<DownloadDocumentAttachmentRequest>,
    ) -> McpResult {
        let explicit_format = request
            .0
            .format
            .as_deref()
            .map(str::parse::<FileFormat>)
            .transpose()?;

        let file = document::bridge::download_document_attachment_bridge(
            self.client.clone(),
            request.0,
            &self.access_control,
        )
        .await?;

        let response_data = SerializableFile::new(file, explicit_format)?;
        Ok(CallToolResult::success(vec![response_data.try_into()?]))
    }

    #[tool(
        description = "Get the document tree structure for a specified project. Requires project_id_or_key parameter. Returns hierarchical document organization."
    )]
    async fn document_tree_get(&self, request: Parameters<GetDocumentTreeRequest>) -> McpResult {
        let document_tree = document::bridge::get_document_tree_tool(
            self.client.clone(),
            request.0,
            &self.access_control,
        )
        .await?;
        Ok(CallToolResult::success(vec![Content::json(
            document_tree.active_tree,
        )?]))
    }

    #[tool(
        description = "Get a list of versions (milestones) for a specified project. Requires project_id_or_key parameter. Returns milestone names, dates, and descriptions."
    )]
    async fn issue_milestone_list_get(
        &self,
        request: Parameters<GetVersionMilestoneListRequest>,
    ) -> McpResult {
        let milestones = issue::bridge::get_version_milestone_list(
            self.client.clone(),
            request.0,
            &self.access_control,
        )
        .await?;
        Ok(CallToolResult::success(vec![Content::json(milestones)?]))
    }

    #[tool(
        description = "Get a list of issues for a specified milestone name within a project. Requires project_id_or_key and milestone_name parameters."
    )]
    async fn issue_list_by_milestone_get(
        &self,
        request: Parameters<GetIssuesByMilestoneNameRequest>,
    ) -> McpResult {
        let issues = issue::bridge::get_issues_by_milestone_name(
            self.client.clone(),
            request.0,
            &self.access_control,
        )
        .await?;
        let issue_responses: Vec<IssueResponse> =
            issues.into_iter().map(IssueResponse::from).collect();
        Ok(CallToolResult::success(vec![Content::json(
            issue_responses,
        )?]))
    }

    #[cfg(feature = "issue_writable")]
    #[tool(
        description = "Update a Backlog issue. Requires issue_id_or_key. Optional: summary, description, status_id, assignee_id, priority_id, due_date, custom fields, etc."
    )]
    async fn issue_update(&self, request: Parameters<UpdateIssueRequest>) -> McpResult {
        let updated_issue =
            issue::bridge::update_issue_impl(self.client.clone(), request.0, &self.access_control)
                .await?;
        let issue_response = IssueResponse::from(updated_issue);
        Ok(CallToolResult::success(vec![Content::json(
            issue_response,
        )?]))
    }

    #[cfg(feature = "issue_writable")]
    #[tool(
        description = "Add a comment to a Backlog issue. Requires issue_id_or_key and content. Optional: notified_user_ids array for mentioning users."
    )]
    async fn issue_comment_add(&self, request: Parameters<AddCommentRequest>) -> McpResult {
        let comment =
            issue::bridge::add_comment_impl(self.client.clone(), request.0, &self.access_control)
                .await?;
        Ok(CallToolResult::success(vec![Content::json(comment)?]))
    }

    #[cfg(feature = "issue_writable")]
    #[tool(
        description = "Update an existing comment on a Backlog issue. Requires issue_id_or_key, comment_id, and content parameters."
    )]
    async fn issue_comment_update(&self, request: Parameters<UpdateCommentRequest>) -> McpResult {
        let comment = issue::bridge::update_comment_impl(
            self.client.clone(),
            request.0,
            &self.access_control,
        )
        .await?;
        Ok(CallToolResult::success(vec![Content::json(comment)?]))
    }

    #[cfg(feature = "issue_writable")]
    #[tool(
        description = "Create a new issue in a Backlog project. Requires project_id, issue_type_id, and summary. Optional: description, assignee_id, priority_id, due_date, custom fields, etc."
    )]
    async fn issue_add(&self, request: Parameters<AddIssueRequest>) -> McpResult {
        let issue =
            issue::bridge::add_issue_impl(self.client.clone(), request.0, &self.access_control)
                .await?;
        let issue_response = IssueResponse::from(issue);
        Ok(CallToolResult::success(vec![Content::json(
            issue_response,
        )?]))
    }

    #[tool(
        name = "issue_comment_list_get",
        description = "Get comments for a specific issue. Requires issue_id_or_key. Optional: min_id, max_id, count (1-100, default 20), order ('asc' or 'desc', default 'desc')."
    )]
    async fn issue_comment_list_get(
        &self,
        request: Parameters<GetIssueCommentsRequest>,
    ) -> McpResult {
        let comments = issue::bridge::get_issue_comments_impl(
            self.client.clone(),
            request.0,
            &self.access_control,
        )
        .await?;
        Ok(CallToolResult::success(vec![Content::json(comments)?]))
    }

    #[tool(
        name = "issue_attachment_list_get",
        description = "Get a list of attachments for a specified issue. Requires issue_id_or_key parameter. Returns attachment metadata including file names, sizes, and IDs."
    )]
    async fn issue_attachment_list_get(
        &self,
        request: Parameters<GetAttachmentListRequest>,
    ) -> McpResult {
        let attachments = issue::bridge::get_attachment_list_impl(
            self.client.clone(),
            request.0,
            &self.access_control,
        )
        .await?;
        Ok(CallToolResult::success(vec![Content::json(attachments)?]))
    }

    #[tool(
        name = "issue_shared_file_list_get",
        description = "Get a list of shared files linked to a specified issue. Requires issue_id_or_key parameter. Returns linked shared file information."
    )]
    async fn issue_shared_file_list_get(
        &self,
        request: Parameters<GetIssueSharedFilesRequest>,
    ) -> McpResult {
        let shared_files = issue::bridge::get_issue_shared_files_impl(
            self.client.clone(),
            request.0,
            &self.access_control,
        )
        .await?;
        Ok(CallToolResult::success(vec![Content::json(shared_files)?]))
    }

    #[tool(
        description = "Get a list of users in the space. No required parameters. Returns user information including ID, name, email, and role."
    )]
    async fn user_list_get(&self, request: Parameters<GetUserListRequest>) -> McpResult {
        let users = user::bridge::get_user_list_bridge(self.client.clone(), request.0).await?;
        Ok(CallToolResult::success(vec![Content::json(users)?]))
    }

    #[tool(
        description = "Download an issue attachment. Requires issue_id_or_key and attachment_id. Optional format: 'image', 'text', or 'raw' (auto-detected if not specified)."
    )]
    async fn issue_attachment_download(
        &self,
        request: Parameters<DownloadAttachmentRequest>,
    ) -> McpResult {
        let explicit_format = request
            .0
            .format
            .as_deref()
            .map(str::parse::<FileFormat>)
            .transpose()?;

        let file = issue::bridge::download_issue_attachment_file(
            self.client.clone(),
            request.0,
            &self.access_control,
        )
        .await?;

        let response_data = SerializableFile::new(file, explicit_format)?;
        Ok(CallToolResult::success(vec![response_data.try_into()?]))
    }

    #[tool(
        description = "Get a list of statuses for a specified project. Requires project_id_or_key. Returns available issue statuses with IDs and names."
    )]
    async fn project_status_list_get(
        &self,
        request: Parameters<GetProjectStatusListRequest>,
    ) -> McpResult {
        let statuses = project::bridge::get_project_status_list_tool(
            self.client.clone(),
            request.0,
            &self.access_control,
        )
        .await?;
        Ok(CallToolResult::success(vec![Content::json(statuses)?]))
    }

    #[tool(
        description = "Get a list of issue types for a specified project. Requires project_id_or_key. Returns issue types (e.g., Task, Bug, Feature) with IDs."
    )]
    async fn project_issue_type_list_get(
        &self,
        request: Parameters<GetProjectIssueTypesRequest>,
    ) -> McpResult {
        let issue_types = project::bridge::get_project_issue_types_tool(
            self.client.clone(),
            request.0,
            &self.access_control,
        )
        .await?;
        Ok(CallToolResult::success(vec![Content::json(issue_types)?]))
    }

    #[tool(
        description = "Get a list of priorities available in Backlog. No required parameters. Returns priority levels (High, Normal, Low) with IDs."
    )]
    async fn issue_priority_list_get(
        &self,
        request: Parameters<GetPrioritiesRequest>,
    ) -> McpResult {
        let priorities =
            project::bridge::get_priorities_tool(self.client.clone(), request.0).await?;
        Ok(CallToolResult::success(vec![Content::json(priorities)?]))
    }

    #[tool(
        description = "Get a list of custom fields for a specified project. Requires project_id_or_key. Returns field definitions with types, IDs, and example values for AI usage."
    )]
    async fn project_custom_field_list_get(
        &self,
        request: Parameters<GetCustomFieldListRequest>,
    ) -> McpResult {
        let custom_fields = project::bridge::get_custom_field_list_tool(
            self.client.clone(),
            request.0,
            &self.access_control,
        )
        .await?;
        Ok(CallToolResult::success(vec![Content::json(custom_fields)?]))
    }

    #[tool(
        description = "Get a list of shared files in a project directory. Requires project_id_or_key and path. Optional: order, offset, count. Returns file/folder information."
    )]
    async fn file_shared_list_get(
        &self,
        request: Parameters<GetSharedFilesListRequest>,
    ) -> McpResult {
        let files = file::bridge::get_shared_files_list_tool(
            self.client.clone(),
            request.0,
            &self.access_control,
        )
        .await?;
        Ok(CallToolResult::success(vec![Content::json(files)?]))
    }

    #[tool(
        description = "Download a shared file from a project. Requires project_id_or_key and shared_file_id. Optional format: 'image', 'text', or 'raw' (auto-detected if not specified)."
    )]
    async fn file_shared_download(
        &self,
        request: Parameters<DownloadSharedFileRequest>,
    ) -> McpResult {
        let explicit_format = request
            .0
            .format
            .as_deref()
            .map(str::parse::<FileFormat>)
            .transpose()?;

        let file = file::bridge::download_shared_file_bridge(
            self.client.clone(),
            request.0,
            &self.access_control,
        )
        .await?;

        let response_data = SerializableFile::new(file, explicit_format)?;
        Ok(CallToolResult::success(vec![response_data.try_into()?]))
    }

    #[tool(
        description = "Get a list of attachments for a specific pull request. Requires project_id_or_key, repository_id_or_name, and number (PR number)."
    )]
    async fn git_pr_attachment_list_get(
        &self,
        request: Parameters<GetPullRequestAttachmentListRequest>,
    ) -> McpResult {
        let attachments = git::bridge::get_pull_request_attachment_list_tool(
            self.client.clone(),
            request.0,
            &self.access_control,
        )
        .await?;
        Ok(CallToolResult::success(vec![Content::json(attachments)?]))
    }

    #[tool(
        description = "Download a pull request attachment. Requires project_id_or_key, repository_id_or_name, number (PR number), and attachment_id. Optional format: 'image', 'text', or 'raw'."
    )]
    async fn git_pr_attachment_download(
        &self,
        request: Parameters<DownloadPullRequestAttachmentRequest>,
    ) -> McpResult {
        let explicit_format = request
            .0
            .format
            .as_deref()
            .map(str::parse::<FileFormat>)
            .transpose()?;

        let file = git::bridge::download_pr_attachment_bridge(
            self.client.clone(),
            request.0,
            &self.access_control,
        )
        .await?;

        let response_data = SerializableFile::new(file, explicit_format)?;
        Ok(CallToolResult::success(vec![response_data.try_into()?]))
    }

    #[tool(
        description = "Get a list of comments for a specific pull request. Requires project_id_or_key, repository_id_or_name, and number (PR number). Optional: min_id, max_id, count, order."
    )]
    async fn git_pr_comment_list_get(
        &self,
        request: Parameters<GetPullRequestCommentListRequest>,
    ) -> McpResult {
        let comments = git::bridge::get_pull_request_comment_list_tool(
            self.client.clone(),
            request.0,
            &self.access_control,
        )
        .await?;
        Ok(CallToolResult::success(vec![Content::json(comments)?]))
    }

    #[tool(
        description = "Get detailed information about a specific wiki page. Requires wiki_id parameter. Returns page content, attachments, and metadata."
    )]
    async fn wiki_details_get(&self, request: Parameters<GetWikiDetailRequest>) -> McpResult {
        let client = self.client.lock().await;
        let detail =
            wiki::bridge::get_wiki_detail(&client, request.0, &self.access_control).await?;

        Ok(CallToolResult::success(vec![Content::json(detail)?]))
    }

    #[tool(
        description = "Get a list of wiki pages. Optional: project_id_or_key (filter by project), keyword (search term). Returns wiki page summaries."
    )]
    async fn wiki_list_get(&self, request: Parameters<GetWikiListRequest>) -> McpResult {
        let client = self.client.lock().await;
        let wikis = wiki::bridge::get_wiki_list(&client, request.0, &self.access_control).await?;

        Ok(CallToolResult::success(vec![Content::json(wikis)?]))
    }

    #[tool(
        description = "Get a list of attachments for a specified wiki page. Requires wiki_id parameter. Returns attachment metadata."
    )]
    async fn wiki_attachment_list_get(
        &self,
        request: Parameters<GetWikiAttachmentListRequest>,
    ) -> McpResult {
        let client = self.client.lock().await;
        let attachments =
            wiki::bridge::get_wiki_attachment_list(&client, request.0, &self.access_control)
                .await?;
        Ok(CallToolResult::success(vec![Content::json(attachments)?]))
    }

    #[tool(
        description = "Download an attachment from a wiki page. Requires wiki_id and attachment_id. Optional format: 'image', 'text', or 'raw' (auto-detected if not specified)."
    )]
    async fn wiki_attachment_download(
        &self,
        request: Parameters<DownloadWikiAttachmentRequest>,
    ) -> McpResult {
        let client = self.client.lock().await;
        let explicit_format = request
            .0
            .format
            .as_deref()
            .map(str::parse::<FileFormat>)
            .transpose()?;

        let file = wiki::bridge::download_wiki_attachment(&client, request.0, &self.access_control)
            .await?;

        let response_data = SerializableFile::new(file, explicit_format)?;
        Ok(CallToolResult::success(vec![response_data.try_into()?]))
    }

    #[cfg(feature = "wiki_writable")]
    #[tool(
        description = "Update a wiki page. Requires wiki_id. Optional: name (page title), content (markdown), mail_notify (boolean for notifications)."
    )]
    async fn wiki_update(&self, request: Parameters<UpdateWikiRequest>) -> McpResult {
        let client = self.client.lock().await;
        let wiki_detail =
            wiki::bridge::update_wiki(&client, request.0, &self.access_control).await?;
        Ok(CallToolResult::success(vec![Content::json(wiki_detail)?]))
    }

    #[cfg(feature = "git_writable")]
    #[tool(
        description = "Add a comment to a pull request. Requires project_id_or_key, repository_id_or_name, number (PR number), and content. Optional: notified_user_ids array."
    )]
    async fn git_pr_comment_add(
        &self,
        request: Parameters<AddPullRequestCommentRequest>,
    ) -> McpResult {
        let comment = git::bridge::add_pull_request_comment_bridge(
            self.client.clone(),
            request.0,
            &self.access_control,
        )
        .await?;
        Ok(CallToolResult::success(vec![Content::json(comment)?]))
    }

    #[cfg(feature = "document_writable")]
    #[tool(
        description = "Add a new document to a Backlog project. Requires project_id (numeric). Optional: title, content (markdown), emoji, parent_id (for hierarchy), add_last (placement order)."
    )]
    async fn document_add(&self, request: Parameters<AddDocumentRequest>) -> McpResult {
        let document = document::bridge::add_document_bridge(
            self.client.clone(),
            request.0,
            &self.access_control,
        )
        .await?;
        Ok(CallToolResult::success(vec![Content::json(document)?]))
    }

    #[cfg(feature = "document_writable")]
    #[tool(
        description = "Delete a document from Backlog. Requires document_id (32-digit hex string). Returns the deleted document information."
    )]
    async fn document_delete(&self, request: Parameters<DeleteDocumentRequest>) -> McpResult {
        let deleted_document = document::bridge::delete_document_bridge(
            self.client.clone(),
            request.0,
            &self.access_control,
        )
        .await?;
        Ok(CallToolResult::success(vec![Content::json(deleted_document)?]))
    }
}

#[tool_handler]
impl rmcp::ServerHandler for Server {
    fn get_info(&self) -> ServerInfo {
        let instructions = "Backlog MCP Server\n\n\
This server provides tools to interact with Backlog, a project management service.
"
        .to_string();
        ServerInfo {
            instructions: Some(instructions),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}
