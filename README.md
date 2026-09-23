# Backlog MCP Server (`mcp-backlog-server`)

[![codecov](https://codecov.io/gh/safx/backlog-mcp-server-rust/graph/badge.svg?token=0S1XJTJY6I)](https://codecov.io/gh/safx/backlog-mcp-server-rust)

`mcp-backlog-server` is a Model Context Protocol (MCP) server for interacting with the Backlog API.
This server allows MCP-compatible clients (such as AI assistants) to utilize Backlog functionalities.

## Installation

### Using Homebrew (Recommended for macOS/Linux)

The easiest way to install is via Homebrew:

```bash
# Add the tap (only needed once)
brew tap safx/tap

# Install the tools
brew install mcp-backlog-server  # MCP server for AI assistants
brew install blg                 # Backlog CLI tool (Optional)
```

To update to the latest version:
```bash
brew update
brew upgrade mcp-backlog-server
brew upgrade blg
```

To uninstall:
```bash
brew uninstall mcp-backlog-server
brew uninstall blg
brew untap safx/tap  # Optional: remove the tap
```

### Alternative Installation Methods

For other platforms or if you prefer not to use Homebrew:
- **Pre-built binaries**: Download from the [releases page](https://github.com/safx/backlog-mcp-server-rust/releases)
- **Build from source**: Clone the repository and run `cargo build --release`

## Example Configuration for MCP Client

### Claude Desktop

`~/Library/Application Support/Claude/claude_desktop_config.json`:
```json
{
  "mcpServers": {
    "backlog": {
      "command": "/path/to/target/release/mcp-backlog-server",
      "args": [],
      "env": {
        "BACKLOG_BASE_URL": "https://your-space.backlog.com",
        "BACKLOG_API_KEY": "YOUR_BACKLOG_API_KEY",
        "BACKLOG_PROJECTS": "PROJ,DEMO",
        "BACKLOG_PREFIX": "backlog_"
      }
    }
  }
}
```

### Cline

Add the following to your Cline MCP configuration:

```json
{
  "mcpServers": {
    "backlog_mcp_server": {
      "autoApprove": [],
      "disabled": false,
      "timeout": 60,
      "command": "/path/to/target/release/mcp-backlog-server",
      "args": [],
      "env": {
        "BACKLOG_BASE_URL": "https://your-space.backlog.com",
        "BACKLOG_API_KEY": "YOUR_BACKLOG_API_KEY",
        "BACKLOG_PROJECTS": "PROJ,DEMO",
        "BACKLOG_PREFIX": "backlog_"
      },
      "transportType": "stdio"
    }
  }
}
```

### Gemini CLI

`~/.gemini/settings.json`:
```json
{
  "mcpServers": {
    "backlog_mcp_server": {
      "command": "/path/to/target/release/mcp-backlog-server",
      "timeout": 10000,
      "args": [],
      "env": {
        "BACKLOG_BASE_URL": "https://your-space.backlog.com",
        "BACKLOG_API_KEY": "YOUR_BACKLOG_API_KEY",
        "BACKLOG_PROJECTS": "PROJ,DEMO",
        "BACKLOG_PREFIX": "backlog_"
      }
    }
  }
}
```

Note: Domain name must be: `backlog.com`, `backlog.jp` or `backlogtool.com`

## Available Tools

The following tools are grouped by their respective modules:

### Tool Summary

With the default configuration, you have access to **43 tools** for Backlog automation:

- **Documents** (9 tools): List and count documents, view trees and details, download attachments, create/delete documents, add/remove tags
- **Git/Pull Requests** (8 tools): Manage repositories, PRs, comments, and attachments
- **Issues** (15 tools): View, create, update issues, manage comments, attachments, shared files, related issues, and priorities
- **Projects** (3 tools): Get project status, issue types, and custom field definitions
- **Shared Files** (2 tools): Browse and download project shared files
- **Users** (1 tool): List space users
- **Wikis** (5 tools): Manage wiki pages, attachments, and content updates

The server includes both **read operations** for information gathering and **write operations** for taking actions.

**Note**: Tool names follow a `category_resource_action` pattern (e.g., `issue_details_get`, `wiki_update`) to enable category-based filtering with `--allowedTools` (e.g., `claude --allowedTools "mcp__backlog__issue_*"`).

### Document Tools
-   **`document_list_get`**: List documents with optional numeric `project_ids`, `keyword`, `sort` (`created`/`updated`), `order` (`asc`/`desc`), `offset` (default 0), and `count` (1–100, default 20)
-   **`document_count_get`**: Count documents in one project using required `project_id_or_key`; keyword and tag filters are not supported
-   **`document_tag_add`**: Add one or more `tag_names` to a `document_id`; returns the API's tag array
-   **`document_tag_remove`**: Remove named tags from a `document_id`; returns `{"success":true}` after a successful empty API response
-   **`document_details_get`**: Retrieves details for a specific Backlog document
-   **`document_attachment_download`**: Download a document attachment
-   **`document_comment_list_get`**: Get the comments (with replies) on a document
-   **`document_tree_get`**: Get the document tree for a specified project
-   **`document_add`**: Add a new document to a Backlog project
-   **`document_delete`**: Delete a document from Backlog

Names above are shown without `BACKLOG_PREFIX` (default `backlog_`). When `project_ids` is omitted,
the list tool searches participating projects within `BACKLOG_PROJECTS`. If a project restriction is
configured, the server resolves allowed keys to IDs before querying; resolution failures do not broaden
the search. Explicit empty ID arrays and requests containing disallowed projects are rejected.
When project restrictions are enabled, tag operations fetch document details and check project access
before changing tags. Without restrictions, they call the tag API directly.
The client requires at least one nonblank tag name and preserves names, including spaces and commas.

List responses include `json` and `attachments` alongside metadata and tags. `json` retains the API's
representation (including a JSON-encoded string). List results are one page; `document_count_get`
returns a project count, not a keyword-search total.

For Rust library users, `Document` now has `json` and `attachments` fields. Older JSON without those
fields still deserializes, but code constructing `Document` with a struct literal must supply the new fields.

### Git Tools
-   **`git_repository_list_get`**: Get a list of Git repositories for a specified project
-   **`git_repository_details_get`**: Get details for a specific Git repository
-   **`git_pr_list_get`**: Get a list of pull requests for a specified repository
-   **`git_pr_details_get`**: Get details for a specific pull request
-   **`git_pr_attachment_list_get`**: Get a list of attachments for a specific pull request
-   **`git_pr_comment_list_get`**: Get a list of comments for a specific pull request
-   **`git_pr_attachment_download`**: Download a pull request attachment
-   **`git_pr_comment_add`**: Add a comment to a specific pull request

### Issue Tools
-   **`issue_details_get`**: Retrieves details for a specific Backlog issue
-   **`issue_milestone_list_get`**: Retrieves a list of versions (milestones) for a specified project
-   **`issue_list_by_milestone_get`**: Retrieves a list of issues associated with a specified milestone
-   **`issue_update`**: Updates a Backlog issue including summary, description, and custom fields
-   **`issue_comment_list_get`**: Gets comments for a specific issue
-   **`issue_attachment_list_get`**: Get a list of attachments for a specified issue
-   **`issue_attachment_download`**: Download an issue attachment
-   **`issue_shared_file_list_get`**: Get a list of shared files linked to a specified issue
-   **`issue_related_issue_list_get`**: Get related issues of a specified issue (hidden issues are reported as `omittedCount`)
-   **`issue_related_issue_add`**: Add a related issue to a Backlog issue
-   **`issue_related_issue_remove`**: Remove a related issue from a Backlog issue
-   **`issue_comment_update`**: Update an existing comment on a Backlog issue
-   **`issue_add`**: Create a new issue in a Backlog project with support for custom fields
-   **`issue_comment_add`**: Add a comment to a specific issue
-   **`issue_priority_list_get`**: Get a list of priority types available in the space

### Project Tools
-   **`project_status_list_get`**: Get a list of statuses for a specified project
-   **`project_issue_type_list_get`**: Get a list of issue types for a specified project
-   **`project_custom_field_list_get`**: Get a list of custom fields defined for a specified project

### Shared File Tools
-   **`file_shared_list_get`**: Get a list of shared files for a specified project directory
-   **`file_shared_download`**: Download a shared file

### User Tools
-   **`user_list_get`**: Get a list of users in the space

### Wiki Tools
-   **`wiki_list_get`**: Get a list of wiki pages
-   **`wiki_details_get`**: Get detailed information about a specific wiki page
-   **`wiki_attachment_list_get`**: Get a list of attachments for a specified wiki page
-   **`wiki_attachment_download`**: Download an attachment from a wiki page
-   **`wiki_update`**: Update a wiki page

## File Download Features

All file download tools (`document_attachment_download`, `issue_attachment_download`, `git_pr_attachment_download`, `wiki_attachment_download`, and `file_shared_download`) support format detection and handling:

### Format Detection
- **Images**: Files with `image/*` content type are detected and returned as base64-encoded images via `rmcp::model::ContentBlock::image`
- **Text**: Files with text-based content types (`text/*`, `application/json`, `application/xml`, etc.) or files that contain valid UTF-8 text are returned as plain text via `rmcp::model::ContentBlock::text`
- **Raw bytes**: All other files are returned as JSON objects with base64-encoded content, filename, and MIME type

### Manual Format Override
You can explicitly specify the format using the optional `format` parameter:
- `"image"`: Force treatment as an image (validates content type)
- `"text"`: Force treatment as text (validates UTF-8 encoding)
- `"raw"`: Force treatment as raw bytes (no validation)

### Content Type Detection
The system uses multiple strategies to determine if a file is text:
- Content-Type header analysis
- UTF-8 validity checking
- Character composition analysis (graphic, whitespace, and valid UTF-8 characters)

## How to Build

```bash
# Default build (includes all writable features)
cargo build --package mcp-backlog-server
```

### Feature Flags

The MCP server supports multiple feature flags to enable different write operations:

-   **`issue_writable`** (enabled by default)
    -   Enables: `issue_update`, `issue_comment_update`, `issue_add`, and `issue_comment_add` tools
    -   Allows AI agents to create issues, modify issue content, and manage comments

-   **`git_writable`** (enabled by default)
    -   Enables: `git_pr_comment_add` tool
    -   Allows AI agents to add comments to pull requests

-   **`wiki_writable`** (enabled by default)
    -   Enables: `wiki_update` tool
    -   Allows AI agents to update wiki page content, names, and notification settings

-   **`document_writable`** (enabled by default)
    -   Enables: `document_add`, `document_delete`, `document_tag_add`, and `document_tag_remove`
    -   Allows AI agents to create/delete documents and add/remove tags; list/count remain available when disabled

### Build Configuration

```bash
# Intended full read-only mode (see the existing build limitation below)
cargo build --package mcp-backlog-server --no-default-features

# Disable document writes while retaining the other writable tools
cargo build --package mcp-backlog-server --no-default-features --features "issue_writable,git_writable,wiki_writable"

# Selective features
cargo build --package mcp-backlog-server --features issue_writable
cargo build --package mcp-backlog-server --features "issue_writable,git_writable"
cargo build --package mcp-backlog-server --features "issue_writable,git_writable,wiki_writable,document_writable"
```

The full `--no-default-features` build currently has existing errors in issue writable imports
and non-document tool registration. The document-write-disabled configuration above is tested;
it still enables write operations for issues, Git, and wikis.

## Configuration

To run this server, the following environment variables must be set:

-   `BACKLOG_BASE_URL`: The URL of your Backlog space (e.g., `https://your-space.backlog.com`)
-   `BACKLOG_API_KEY`: Your Backlog API key. You can issue one from your personal settings page in Backlog.

Optional environment variables:

-   `BACKLOG_PROJECTS`: Comma-separated list of allowed project keys (e.g., `MFP,DEMO,TEST`). When set, the server will only allow access to the specified projects. If not set, all projects accessible with the API key are available.
-   `BACKLOG_PREFIX`: Custom prefix for tool names (default: `backlog_`). For example, setting `BACKLOG_PREFIX=""` removes the prefix, making tools accessible as `issue_details_get` instead of `backlog_issue_details_get`. Setting `BACKLOG_PREFIX="my_"` changes tools to `my_issue_details_get`.

These environment variables are expected to be passed by the MCP client system when launching the server.

### Run (for local testing)

After setting the environment variables, you can run the server directly with the following command:

```bash
# Default run with all features
BACKLOG_BASE_URL="https://your-space.backlog.com" \
BACKLOG_API_KEY="your_backlog_api_key" \
cargo run --package mcp-backlog-server
```
