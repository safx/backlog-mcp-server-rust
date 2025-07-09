# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Status
- **Primary Branch**: `main` 
- **Current Focus**: OAuth support and webhook management
- **Rust Edition**: 2024 (stable toolchain)


## Architecture Overview

Rust workspace for Backlog API client ecosystem:
- **Library** (`crates/backlog-api-client/`): Core client library with modular API wrappers
- **CLI** (`cli/`): Command-line interface built on the library
- **MCP Server** (`backlog-mcp-server/`): Model Context Protocol server for AI integration

### Workspace Structure
```
cli/                        # CLI binary application
backlog-mcp-server/         # MCP server implementation
crates/
├── backlog-api-client/     # Main library facade
├── backlog-api-macros/     # Procedural macros for API parameter serialization
├── backlog-core/           # Core types and identifiers
├── backlog-api-core/       # Common API utilities and error types
├── backlog-domain-models/  # Shared domain models
├── backlog-issue/          # Issue management API
├── backlog-project/        # Project management API
├── backlog-space/          # Space management API
├── backlog-user/           # User management API
├── backlog-document/       # Document API
├── backlog-wiki/           # Wiki API
├── backlog-git/            # Git repository API
├── backlog-file/           # Shared file API
├── backlog-activity/       # Activity API
├── backlog-team/           # Team management API
├── backlog-star/           # Star management API
├── backlog-rate-limit/     # Rate limit API
├── backlog-watching/       # Watching management API
├── backlog-webhook/        # Webhook management API
└── client/                 # Generic HTTP client wrapper
```

## Quick Start

### Environment Setup
```bash
export BACKLOG_BASE_URL="https://your-space.backlog.jp"
export BACKLOG_API_KEY="your_api_key"
```

### Development Commands
```bash
# Build and test
cargo check --all-targets --all-features
cargo test --all-features --all-targets
cargo clippy --all-features --all-targets -- -D warnings
cargo fmt --all

# Run specific tests
cargo test test_name --package package_name
cargo test --doc  # Run documentation tests

# Build variants
cargo build --package blg                           # Read-only CLI
cargo build --package blg --features "all_writable" # Full CLI
cargo build --package mcp-backlog-server           # MCP server

# Generate documentation
cargo doc --open --all-features

```

## Key Design Patterns

### Date Handling
| Context | Type | Format | Example |
|---------|------|--------|---------|
| Request parameters | `ApiDate` | yyyy-MM-dd | `start_date`, `due_date` |
| Response timestamps | `DateTime<Utc>` | ISO 8601 | `created`, `updated` |
| Legacy fields | `String` | varies | `Issue.start_date` |

### Form Encoding
Array parameters require special `foo[]` syntax and manual serialization:
```rust
// Manual form encoding
impl From<&AddCommentParams> for Vec<(String, String)> {
    fn from(params: &AddCommentParams) -> Self {
        let mut seq = Vec::new();
        seq.push(("content".to_string(), params.content.clone()));
        
        if let Some(user_ids) = &params.notified_user_ids {
            user_ids.iter().for_each(|id| {
                seq.push(("notifiedUserId[]".to_string(), id.to_string()));
            });
        }
        seq
    }
}
```

### ToFormParams Macro
Automates form parameter serialization:
```rust
use backlog_api_macros::ToFormParams;

#[derive(ToFormParams)]
struct AddCommentParams {
    content: String,                           // → "content"
    #[form(array, name = "notifiedUserId")]
    notified_user_ids: Option<Vec<u32>>,      // → "notifiedUserId[]"
    #[form(skip)]
    issue_id_or_key: IssueIdOrKey,            // Skipped
}
```

### Domain Crate Structure
Standard template for all domain crates:
```
crates/backlog-{domain}/
├── src/
│   ├── lib.rs                    # Public exports
│   ├── models.rs                 # Domain models
│   └── api/
│       ├── mod.rs                # Module exports
│       ├── {domain}_api.rs       # Main API struct
│       ├── get_*.rs              # Read operations
│       ├── add_*.rs              # Create operations (feature-gated)
│       ├── update_*.rs           # Update operations (feature-gated)
│       └── delete_*.rs           # Delete operations (feature-gated)
└── tests/
    ├── common/mod.rs             # Shared test utilities
    ├── {domain}_api_test.rs      # Read-only tests
    └── {domain}_writable_test.rs # Write operation tests
```

### API Implementation Pattern

#### Using derive_builder
For read-only APIs with optional parameters, use `derive_builder`:
```rust
use backlog_api_core::{Error as ApiError, IntoRequest};
use backlog_api_macros::ToFormParams;
use derive_builder::Builder;
use serde::Serialize;

#[derive(Debug, Clone, Builder, ToFormParams)]
#[builder(build_fn(error = "ApiError"))]
pub struct GetRecentlyViewedWikisParams {
    #[builder(default, setter(into, strip_option))]
    pub order: Option<String>,
    #[builder(default, setter(into, strip_option))]
    pub count: Option<u32>,
}

impl IntoRequest for GetRecentlyViewedWikisParams {
    fn path(&self) -> String {
        "/api/v2/users/myself/recentlyViewedWikis".to_string()
    }
    
    fn to_query(&self) -> impl Serialize {
        let params: Vec<(String, String)> = self.into();
        params
    }
}
```

Important: When using `derive_builder`, export the builder type in `mod.rs`:
```rust
pub use get_recently_viewed_wikis::{
    GetRecentlyViewedWikisParams, GetRecentlyViewedWikisParamsBuilder,
    GetRecentlyViewedWikisResponse,
};
```

#### Common builder patterns:
- `#[builder(default)]` - Optional field with None default
- `#[builder(setter(into))]` - Allow Into<T> conversions
- `#[builder(setter(strip_option))]` - Allow direct value setting for Option<T>
- `#[builder(setter(custom))]` - Custom setter implementation

#### Manual implementation for write operations
```rust
// Parameter struct
#[cfg(feature = "writable")]
#[derive(Debug, Clone)]
pub struct UpdateEntityParams {
    pub required_field: String,
    pub optional_field: Option<String>,
}

// Form serialization
#[cfg(feature = "writable")]
impl From<&UpdateEntityParams> for Vec<(String, String)> {
    fn from(params: &UpdateEntityParams) -> Self {
        let mut seq = Vec::new();
        seq.push(("requiredField".to_string(), params.required_field.clone()));
        
        if let Some(value) = &params.optional_field {
            seq.push(("optionalField".to_string(), value.clone()));
        }
        seq
    }
}

// API method
#[cfg(feature = "writable")]
pub async fn update_entity(
    &self,
    project_id_or_key: impl Into<ProjectIdOrKey>,
    entity_id: impl Into<EntityId>,
    params: &UpdateEntityParams,
) -> Result<Entity> {
    let params_vec: Vec<(String, String)> = params.into();
    let path = format!("/api/v2/projects/{}/entities/{}", 
        project_id_or_key.into(), entity_id.into());
    self.client.patch(&path, &params_vec).await
}
```

### Custom Fields
Type-safe handling with dynamic form parameter names:
```rust
// Response type (reading)
pub enum CustomFieldValue {
    Text(String),
    Numeric(f64),
    SingleList { item: CustomFieldListItem, other_value: Option<String> },
    MultipleList { items: Vec<CustomFieldListItem>, other_value: Option<String> },
    // ...
}

// Request type (writing)
pub enum CustomFieldInput {
    Text(String),
    Numeric(f64),
    SingleList { id: u32, other_value: Option<String> },
    MultipleList { ids: Vec<u32>, other_value: Option<String> },
    // ...
}

// Form serialization
params.push((format!("customField_{}", id.value()), value));
```

## Development Process

### TDD Workflow
1. **Research**: Read official Backlog API docs
2. **Test First**: Write comprehensive unit tests
   - Create test file in `tests/` directory of the domain crate
   - Use common test utilities from `tests/common/mod.rs`
   - Mock different HTTP status codes and edge cases
3. **Implement**: Minimal implementation to pass tests
   - Follow the domain crate structure pattern
   - Use `derive_builder` for read-only APIs with optional parameters
   - Manual implementation for write operations
4. **CLI**: Add commands when applicable
   - Update `cli/src/commands/` with new subcommands
   - Follow existing patterns for command structure
5. **Integration**: Test with real Backlog instance
   - Use environment variables for authentication
   - Test both success and error cases
6. **Document**: Update API.md, README.md, CLAUDE.md
   - Mark implemented APIs in API.md
   - Add usage examples to CLI README.md
   - Update feature counts in project_structure.md

### Testing Requirements
- Test success, error, and edge cases
- Mock different HTTP status codes (200, 400, 401, 403, 404, 500)
- Test minimal and maximal parameter sets
- Include integration tests when possible
- Use `assert_json_eq!` for response verification
- Test array parameter serialization separately

### Pre-commit Checklist
```bash
cargo check --all-targets --all-features
cargo test --all-features --all-targets
cargo clippy --all-features --all-targets -- -D warnings
cargo fmt --all
```

### Recommended Development Tools
- **ripgrep (`rg`)**: Fast code search - use instead of grep
- **fd**: Fast file finding - use instead of find
- **ast-grep (`sg`)**: Structural code search and refactoring
- **jq**: JSON processing for API responses
- **yq**: YAML/XML processing

### derive_builder Pattern
For read operations with optional parameters:
```rust
#[derive(Debug, Clone, Builder, ToFormParams)]
#[builder(build_fn(error = "ApiError"))]
pub struct GetIssuesParams {
    #[builder(default, setter(into, strip_option))]
    pub project_ids: Option<Vec<u32>>,
    #[builder(default, setter(into, strip_option))]
    pub issue_type_ids: Option<Vec<u32>>,
    // ... more fields
}
```

Remember to export the builder type in `mod.rs`:
```rust
pub use get_issues::{GetIssuesParams, GetIssuesParamsBuilder};
```

## Important Guidelines

### Feature Flags
- Read operations: Always available
- Write operations: Behind `writable` feature flags
- Common combinations:
  - `--features "cli git issue project space"` (read-only)
  - `--features "cli git issue project project_writable space"` (with writes)
  - `--all-features` (development)
- All available feature flags:
  - Domain modules: `git`, `issue`, `project`, `space`, `user`, `document`, `file`, `wiki`, `activity`, `team`, `star`, `rate-limit`, `watching`, `webhook`
  - Write features: `issue_writable`, `project_writable`, `space_writable`, `git_writable`, `wiki_writable`, `team_writable`, `star_writable`, `user_writable`, `watching_writable`, `webhook_writable`
  - Bundles: `all` (all read operations), `all_writable` (all write operations)

### Error Handling
- Use `ApiError` from `backlog-api-core`
- Domain-specific errors wrap core types
- MCP server converts to `rmcp::Error`

### Type Safety
- Use strongly-typed identifiers (`ProjectId`, `IssueKey`, etc.)
- Use `XxxResponse` type aliases for API responses
- Custom deserialization with `Raw*` structs for complex JSON
- Note: `Activity` type is located in `backlog-core`, not `backlog-activity`
- Custom field types are in `backlog-domain-models`

### Documentation
- API methods must include endpoint mapping
- Example: `/// Corresponds to \`GET /api/v2/projects/:projectIdOrKey\`.`
- Update API.md counts after adding endpoints

## Recent Updates
- **OAuth Support**: Added OAuth 2.0 authentication flow and webhook management APIs  
- **New Domain Modules**: Added support for Team, Star, Rate Limit, Watching, and Webhook APIs
- **Layered Architecture Refactoring**: Completed migration to clean dependency structure (see LAYERED_ARCHITECTURE_REFACTORING_PLAN.md)
- **Activity API Integration**: Unified Activity, Content, and Notification types in backlog-core
- **Custom Field System**: Full type-safe implementation with `CustomFieldValue`/`CustomFieldInput` enums
- **ToFormParams Macro**: Procedural macro for automated form parameter serialization
- **Unified File Downloads**: Intelligent format detection (Image/Text/Raw) across all file operations
- **Wiki API**: Complete implementation with create/update/delete and file attachment support
- **Issue API Enhancements**: Comment updates, participant lists, and comment notifications
- **Date Range Filtering**: Added date-based filtering for issue lists
- **Recently Viewed APIs**: Added support for recently viewed issues, projects, and wikis
- **MCP Server Improvements**: 
  - AI-friendly custom field transformation
  - Project-level access control via `BACKLOG_PROJECTS` environment variable
  - Extended to Document and Wiki modules with access control

## MCP Server
- Domain modules: `issue/`, `git/`, `document/`, `project/`, `file/`, `user/`, `wiki/`
- Each module has `request.rs` and `bridge.rs`
- Project access control via `BACKLOG_PROJECTS` environment variable
- Unified file handling with intelligent format detection
- Available tools follow `category_resource_action` naming pattern (e.g., `issue_details_get`, `wiki_update`)
- 34 tools available covering read and write operations across all domains

## Release Process

### Creating a New Release

1. **Update version in Cargo.toml**:
   ```bash
   # Update version in workspace Cargo.toml
   # version = "0.1.0" → "0.1.1"
   ```

2. **Commit version changes**:
   ```bash
   git add Cargo.toml
   git commit -m "chore: bump version to 0.1.1"
   git push github main
   ```

3. **Create and push a tag**:
   ```bash
   git tag v0.1.1
   git push github v0.1.1
   ```

4. **Automated release process**:
   - GitHub Actions will automatically:
     - Build binaries for multiple platforms (Linux, macOS, Windows)
     - Create a GitHub Release with the binaries
     - Update the Homebrew tap at https://github.com/safx/homebrew-tap

5. **Verify the release**:
   - Check https://github.com/safx/backlog-mcp-server-rust/releases
   - Test Homebrew installation: `brew update && brew upgrade blg`

### Prerequisites

- `TAP_GITHUB_TOKEN` must be set in GitHub repository secrets
- Token needs write access to safx/homebrew-tap repository

## Common Pitfalls and Solutions

### 1. Array Parameter Serialization
**Problem**: Backlog API expects array parameters as `foo[]` format.
**Solution**: Use `#[form(array, name = "fieldName")]` attribute with ToFormParams macro, or manually append `[]` in From implementation.

### 2. Custom Field Handling
**Problem**: Custom fields require dynamic parameter names like `customField_123`.
**Solution**: Use the custom field helper functions and CustomFieldInput enum for type-safe handling.

### 3. Date Format Inconsistencies
**Problem**: Different date formats for requests (yyyy-MM-dd) vs responses (ISO 8601).
**Solution**: Use `ApiDate` for request parameters and `DateTime<Utc>` for response fields.

### 4. Missing Builder Export
**Problem**: `derive_builder` generates a builder type that needs to be exported.
**Solution**: Always export `XxxParamsBuilder` alongside `XxxParams` in the module's public API.

### 5. Forgotten Feature Gates
**Problem**: Write operations must be behind feature flags but easy to forget.
**Solution**: Always use `#[cfg(feature = "domain_writable")]` for write operations and their tests.

## CLI Custom Fields Usage
See `cli/CUSTOM_FIELDS_USAGE.md` for detailed examples of using custom fields in the CLI, including:
- Command-line argument format: `--custom-field "id:type:value[:other]"`
- JSON file format for complex configurations
- Support for all custom field types (text, numeric, date, lists, checkbox, radio)