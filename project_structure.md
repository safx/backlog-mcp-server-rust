# About this repository

Rust client library, command-line interface (CLI), and Model Context Protocol (MCP) server for the Backlog API.

## Project Structure

```
cli/                        # CLI application (`blg` binary)
backlog-mcp-server/         # Model Context Protocol server for AI integration
crates/                     # Internal library crates
├── backlog-api-client/     # Main library facade (aggregates all API modules)
├── backlog-core/           # Core types and identifiers shared across all modules
├── backlog-api-core/       # Common API utilities and error types
├── backlog-api-macros/     # Procedural macros for API parameter serialization
├── backlog-domain-models/  # Shared domain models (Priority, Status, Category, etc.)
├── backlog-issue/          # Issue management API
├── backlog-project/        # Project management API
├── backlog-space/          # Space management API
├── backlog-user/           # User management API
├── backlog-document/       # Document API
├── backlog-wiki/           # Wiki API (full CRUD operations and file attachment)
├── backlog-git/            # Git repository and Pull Request API
├── backlog-file/           # Shared file API
└── client/                 # Generic HTTP client wrapper
```

### Applications

#### CLI (`cli`)
Command-line interface for Backlog API operations. The `blg` binary provides access to Backlog from the terminal.

#### MCP Server (`backlog-mcp-server`)
Model Context Protocol server that exposes Backlog API functionalities as tools with unified file download capabilities.

### Internal Libraries (`crates`)

#### Main Library
- **`backlog-api-client`**: Primary library crate that aggregates all API modules and provides a unified client interface.

#### Core Libraries
- **`backlog-core`**: Fundamental data structures, newtype identifiers (e.g., `ProjectId`, `IssueKey`, `SharedFileId`), and shared enums (`FileType`, etc.).
- **`backlog-api-core`**: Core utilities shared across API client modules, such as common error types and result aliases.
- **`backlog-domain-models`**: Shared domain models (e.g., `Priority`, `Resolution`, `Status`, `Category`, `IssueType`, `Milestone`).
- **`client`**: Foundational crate providing a generic HTTP client wrapper (around `reqwest`) and shared test utilities.
- **`backlog-api-macros`**: Procedural macros for API parameter serialization

#### API Domain Modules
- **`backlog-document`**: Document API endpoints (9 endpoints) - list/count, details and trees, attachment downloads, document creation/deletion, and tag addition/removal.
- **`backlog-file`**: Shared File API endpoints (2 endpoints) - project file management with type-safe directory/file distinction.
- **`backlog-git`**: Git repository and Pull Request API endpoints (16 endpoints) - Git workflow including PR management.
- **`backlog-issue`**: Issue management API endpoints (14 endpoints) - issue lifecycle and shared file linking.
- **`backlog-project`**: Project management API endpoints (22 endpoints) - covers categories, statuses, versions, issue types.
- **`backlog-space`**: Space API endpoints (2 endpoints) - space information.
- **`backlog-user`**: User management API endpoints (4 endpoints) - user information and icons.
- **`backlog-wiki`**: Wiki API endpoints (8 endpoints) - wiki pages with CRUD operations, attachment management, and file attachment capabilities.

## Feature Flags

### API Module Features
- **`issue`**: Issue API support (comments, attachments, custom fields)
- **`project`**: Project API support (categories, statuses, milestones, custom fields)
- **`space`**: Space API support
- **`user`**: User API support
- **`document`**: Document API support
- **`wiki`**: Wiki API support (full CRUD operations)
- **`git`**: Git repository and Pull Request API support
- **`file`**: Shared File API support

### Writable Features
By default, only read operations are enabled. To enable write operations (create, update, delete), use the corresponding `*_writable` features:
- **`issue_writable`**: Write operations for issues (add, update, delete issues and comments, link shared files)
- **`project_writable`**: Write operations for projects (add, update, delete categories, statuses, versions, issue types)
- **`git_writable`**: Write operations for Git/PR (add, update pull requests and comments, delete attachments)
- **`wiki_writable`**: Write operations for wikis (update wiki pages with name, content, and email notifications)
- **`space_writable`**: Write operations for space (planned feature)
- **`user_writable`**: Write operations for users (planned feature)
- **`all_writable`**: All write operations

### Additional Features
- **`schemars`**: JSON Schema generation support (for MCP server)
