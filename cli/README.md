# Backlog API Client CLI (`blg`)

`blg` is a command-line interface (CLI) tool for interacting with the Backlog API. It is built using the `backlog-api-client` Rust library.

## Building `blg`

### Prerequisites

-   Rust and Cargo: Ensure you have a recent version of Rust and Cargo installed. You can get them from [rustup.rs](https://rustup.rs/).

### Build Command

To build the `blg` executable, navigate to the workspace root (`/Users/mac/src/_mydev/backlog-api-client`) and run:

```bash
cargo build --package blg --features "git issue project space"
```

For write operations (creating/updating/deleting resources), add the corresponding writable features:

```bash
cargo build --package blg --features "git git_writable issue issue_writable project project_writable space wiki wiki_writable"
```

Alternatively, if you are in the `cli` directory:

```bash
cargo build --features "git issue project space" 
# Or with writable features:
cargo build --features "git git_writable issue issue_writable project project_writable space wiki wiki_writable" 
```

The `git`, `issue`, `project`, `space`, and `wiki` features are required to build the `blg` binary with functionality. Add `project_writable` for project management operations like category creation/deletion, `issue_writable` for issue operations like linking shared files, `git_writable` for pull request update operations, and `wiki_writable` for wiki page update operations. The executable will be located at `target/debug/blg` (or `target/release/blg` if you add `--release`).

## Configuration

`blg` requires two environment variables to be set for authentication:

-   `BACKLOG_BASE_URL`: The base URL of your Backlog space (e.g., `https://your-space.backlog.jp`).
-   `BACKLOG_API_KEY`: Your Backlog API key. You can generate one from your personal settings page in Backlog.

Example:

```bash
export BACKLOG_BASE_URL="https://yourspace.backlog.jp"
export BACKLOG_API_KEY="yourgeneratedapikey"
```

## Basic Usage

The general syntax for `blg` is:

```bash
blg <COMMAND> <SUBCOMMAND> [OPTIONS]
```

You can get help for any command or subcommand by appending `--help`.

### Examples:

**Project Management:**
```bash
# List all projects
blg project list

# Show details of a specific project
blg project show MFP

# List statuses for a project
blg project status-list MFP

# List categories for a project
blg project category-list MFP

# List custom fields for a project
blg project custom-field-list MFP

# Add a category to a project (requires project_writable feature)
blg project category-add MFP --name "New Category"

# Update a category in a project (requires project_writable feature)
blg project category-update MFP --category-id 12345 --name "Updated Category"

# Delete a category from a project (requires project_writable feature)
blg project category-delete MFP --category-id 12345

# Update a custom field in a project (requires project_writable feature)
blg project custom-field-update MFP --custom-field-id 12345 --name "Updated Field Name" --description "New description" --required true

# Delete a custom field from a project (requires project_writable feature)
blg project custom-field-delete MFP --custom-field-id 12345

# Add a list item to a custom field (requires project_writable feature)
blg project custom-field-add-item MFP --custom-field-id 12345 --name "New Option"

# Update a list item in a custom field (requires project_writable feature)
blg project custom-field-update-item MFP --custom-field-id 12345 --item-id 67890 --name "Updated Option"

# Delete a list item from a custom field (requires project_writable feature)
blg project custom-field-delete-item MFP --custom-field-id 12345 --item-id 67890

# Add an issue type to a project (requires project_writable feature)
blg project issue-type-add MFP --name "Bug Report" --color "dark-red" --template-summary "Bug: [Title]" --template-description "## Description\n\n## Steps to reproduce"

# Update an issue type in a project (requires project_writable feature)
blg project issue-type-update MFP --issue-type-id 12345 --name "Updated Bug Report" --color "red" --template-summary "Bug: [Updated Title]"

# Delete an issue type from a project (requires project_writable feature)
blg project issue-type-delete MFP --issue-type-id 12345 --substitute-issue-type-id 67890

# Available colors: red, dark-red, purple, violet, blue, teal, green, orange, pink, gray
# You can also use hex codes: #e30000, #990000, #934981, #814fbc, #2779ca, #007e9a, #7ea800, #ff9200, #ff3265, #666665
```

**Space Management:**
```bash
# Download space logo
blg space logo --output logo.png
```

**Wiki Management:**
```bash
# List attachments for a wiki page
blg wiki list-attachments 12345

# Download an attachment from a wiki page with original filename
blg wiki download-attachment 12345 67890

# Download an attachment from a wiki page with custom filename
blg wiki download-attachment 12345 67890 --output custom_name.png

# Update a wiki page name
blg wiki update 12345 --name "New Wiki Title"

# Update a wiki page content
blg wiki update 12345 --content "Updated content for the wiki page"

# Update both name and content with email notification
blg wiki update 12345 --name "Updated Title" --content "Updated content" --mail-notify true

# Update content without email notification
blg wiki update 12345 --content "Silent update" --mail-notify false
```

**Issue Management:**
```bash
# List issues for a project
blg issue list --project-id MYPROJ 

# Show details of a specific issue
blg issue show MYPROJ-101

# Add a comment to an issue
blg issue add-comment MYPROJ-101 --content "This is a comment"

# Download an issue attachment
blg issue download-attachment MYPROJ-101 12345 --output downloaded_file.dat

# List shared files linked to an issue
blg issue list-shared-files MYPROJ-101

# Link shared files to an issue (requires issue_writable feature)
blg issue link-shared-files MYPROJ-101 --file-ids 123,456,789

# List related issues
blg issue list-related-issues MYPROJ-101

# Add a related issue (requires issue_writable feature)
blg issue add-related-issue MYPROJ-101 MYPROJ-102
```

**Repository Management:**
```bash
# List repositories in a project
blg repo list --project-id MYPROJ

# Show details of a specific repository
blg repo show --project-id MYPROJ --repo-id my-repo
```

**Pull Request Management:**
```bash
# List pull requests in a repository
blg pr list --project-id MYPROJ --repo-id my-repo

# Show details of a specific pull request
blg pr show --project-id MYPROJ --repo-id my-repo --pr-number 42

# Download a pull request attachment
blg pr download-attachment -p MYPROJ -r my-repo -n 42 -a 56789 -o pr_attachment.zip

# Update a pull request (requires git_writable feature)
blg pr update -p MYPROJ -r my-repo --pr-number 42 --summary "Updated PR Title" --description "Updated description" --comment "Updated via CLI"

# Update with issue assignment and notifications
blg pr update -p MYPROJ -r my-repo --pr-number 42 --assignee-id 12345 --issue-id 67890 --notify-user-ids 111,222,333
```

**Team Management:**
```bash
# Show information about a specific team (requires administrator permission)
blg team show 123

# List all teams with default settings
blg team list

# List teams with custom pagination
blg team list --count 50 --offset 10

# List teams in ascending order
blg team list --order asc

# Export team list as JSON
blg team list --format json > teams.json

# Export team list as CSV
blg team list --format csv > teams.csv

# Using the alias
blg team ls --order asc --format table

# Download team icon
blg team icon 123 --output team_123_icon.png
```

**User Management:**
```bash
# List all users in the space
blg user list

# Get information about the authenticated user
blg user me

# Show details of a specific user
blg user show 12345

# Download user icon
blg user icon 12345 --output user_icon.png

# Get notification count for authenticated user
blg user notification-count

# Get unread notification count only
blg user notification-count --already-read false

# List recent notifications
blg user notifications

# List notifications with pagination
blg user notifications --count 20 --max-id 1000

# List notifications from a specific sender
blg user notifications --sender-id 12345

# List notifications in ascending order
blg user notifications --order asc

# Using the alias for notifications
blg user notif --count 10

# Mark a notification as read (requires user_writable feature)
blg user mark-notification-read 12345

# Reset all unread notifications (mark all as read) (requires user_writable feature)
blg user reset-notifications
```

### Getting Help

-   For a list of all top-level commands:
    ```bash
    blg --help
    ```
-   For help with a specific command (e.g., `project`):
    ```bash
    blg project --help
    ```
-   For help with a specific subcommand (e.g., `project list`):
    ```bash
    blg project list --help
    ```

## Available Commands

The `blg` CLI currently supports the following commands:

### Project Commands
- `project list` - List all projects in the Backlog space
- `project show <PROJECT_ID_OR_KEY>` - Show detailed information about a specific project
- `project status-list <PROJECT_ID_OR_KEY>` - List all statuses for a specific project
- `project milestone-list <PROJECT_ID_OR_KEY>` - List milestones for a specific project
- `project issue-type-list <PROJECT_ID_OR_KEY>` - List issue types for a specific project
- `project category-list <PROJECT_ID_OR_KEY>` - List categories for a specific project
- `project priority-list` - List priorities (space-wide)
- `project resolution-list` - List resolutions (space-wide)
- `project icon <PROJECT_ID_OR_KEY> --output <FILE_PATH>` - Download project icon
- `project category-add <PROJECT_ID_OR_KEY> --name <CATEGORY_NAME>` - Add a category to a project (requires `project_writable` feature)
- `project category-update <PROJECT_ID_OR_KEY> --category-id <CATEGORY_ID> --name <NEW_NAME>` - Update a category in a project (requires `project_writable` feature)
- `project category-delete <PROJECT_ID_OR_KEY> --category-id <CATEGORY_ID>` - Delete a category from a project (requires `project_writable` feature)
- `project custom-field-add <PROJECT_ID_OR_KEY> --field-type <TYPE> --name <NAME> [OPTIONS]` - Add a custom field to a project (requires `project_writable` feature)
  - `--field-type <TYPE>` - Field type (text, textarea, numeric, date, single-list, multiple-list, checkbox, radio)
  - `--name <NAME>` - Field name
  - `--description <DESCRIPTION>` - Field description (optional)
  - `--required <true|false>` - Make field required (optional)
  - `--applicable-issue-types <ID1,ID2>` - Applicable issue type IDs (comma-separated)
  - Numeric field options:
    - `--min <VALUE>` - Minimum value
    - `--max <VALUE>` - Maximum value
    - `--initial-value <VALUE>` - Initial value
    - `--unit <UNIT>` - Unit (e.g., "%", "hours")
  - Date field options:
    - `--min-date <YYYY-MM-DD>` - Minimum date
    - `--max-date <YYYY-MM-DD>` - Maximum date
    - `--initial-value-type <1-3>` - Initial value type (1: current day, 2: current day + shift, 3: specified date)
    - `--initial-date <YYYY-MM-DD>` - Initial date (when initial-value-type is 3)
    - `--initial-shift <DAYS>` - Days from current date (when initial-value-type is 2)
  - List field options:
    - `--items <ITEM1,ITEM2>` - List items (comma-separated, required for list types)
    - `--allow-input <true|false>` - Allow direct input
    - `--allow-add-item <true|false>` - Allow adding new items
- `project custom-field-update <PROJECT_ID_OR_KEY> --custom-field-id <CUSTOM_FIELD_ID> [OPTIONS]` - Update a custom field in a project (requires `project_writable` feature)
  - `--name <NEW_NAME>` - Update the custom field name
  - `--description <NEW_DESCRIPTION>` - Update the custom field description
  - `--required <true|false>` - Update whether the field is required
  - `--applicable-issue-types <ID1,ID2>` - Update applicable issue types (comma-separated IDs)
- `project custom-field-delete <PROJECT_ID_OR_KEY> --custom-field-id <CUSTOM_FIELD_ID>` - Delete a custom field from a project (requires `project_writable` feature)
- `project custom-field-add-item <PROJECT_ID_OR_KEY> --custom-field-id <CUSTOM_FIELD_ID> --name <ITEM_NAME>` - Add a list item to a list type custom field (requires `project_writable` feature)
- `project custom-field-update-item <PROJECT_ID_OR_KEY> --custom-field-id <CUSTOM_FIELD_ID> --item-id <ITEM_ID> --name <NEW_NAME>` - Update a list item in a list type custom field (requires `project_writable` feature)
- `project custom-field-delete-item <PROJECT_ID_OR_KEY> --custom-field-id <CUSTOM_FIELD_ID> --item-id <ITEM_ID>` - Delete a list item from a list type custom field (requires `project_writable` feature)
- `project issue-type-add <PROJECT_ID_OR_KEY> --name <ISSUE_TYPE_NAME> --color <COLOR> [--template-summary <SUMMARY>] [--template-description <DESCRIPTION>]` - Add an issue type to a project (requires `project_writable` feature). COLOR can be a name (red, dark-red, purple, violet, blue, teal, green, orange, pink, gray) or hex code.
- `project issue-type-update <PROJECT_ID_OR_KEY> --issue-type-id <ISSUE_TYPE_ID> [--name <NEW_NAME>] [--color <NEW_COLOR>] [--template-summary <NEW_SUMMARY>] [--template-description <NEW_DESCRIPTION>]` - Update an issue type in a project (requires `project_writable` feature). All parameters except issue-type-id are optional.
- `project issue-type-delete <PROJECT_ID_OR_KEY> --issue-type-id <ISSUE_TYPE_ID> --substitute-issue-type-id <SUBSTITUTE_ID>` - Delete an issue type from a project (requires `project_writable` feature). Existing issues will be moved to the substitute issue type.

### Space Commands
- `space logo --output <FILE_PATH>` - Download the space logo

### Issue Commands
- `issue list [OPTIONS]` - List issues with optional filters
- `issue show <ISSUE_ID_OR_KEY>` - Show detailed information about a specific issue
- `issue add-comment <ISSUE_ID_OR_KEY> --content <CONTENT>` - Add a comment to an issue
- `issue update-comment --issue-id <ISSUE_ID_OR_KEY> --comment-id <COMMENT_ID> --content <NEW_CONTENT>` - Update an existing comment (requires `issue_writable` feature)
- `issue delete-comment --issue-id <ISSUE_ID_OR_KEY> --comment-id <COMMENT_ID>` - Delete a comment from an issue (requires `issue_writable` feature)
- `issue download-attachment <ISSUE_ID_OR_KEY> <ATTACHMENT_ID> --output <FILE_PATH>` - Download an issue attachment
- `issue list-shared-files <ISSUE_ID_OR_KEY>` - List shared files linked to an issue
- `issue link-shared-files <ISSUE_ID_OR_KEY> --file-ids <FILE_ID1,FILE_ID2>` - Link shared files to an issue (requires `issue_writable` feature)
- `issue list-related-issues <ISSUE_ID_OR_KEY>` - List related issues of an issue
- `issue add-related-issue <ISSUE_ID_OR_KEY> <TARGET_ISSUE_ID_OR_KEY>` - Add a related issue (requires `issue_writable` feature)
- `issue remove-related-issue <ISSUE_ID_OR_KEY> <RELATED_ISSUE_ID_OR_KEY>` - Remove a related issue (requires `issue_writable` feature)

### Repository Commands
- `repo list --project-id <PROJECT_ID_OR_KEY>` - List repositories in a project
- `repo show --project-id <PROJECT_ID_OR_KEY> --repo-id <REPO_ID_OR_NAME>` - Show repository details

### Pull Request Commands
- `pr list --project-id <PROJECT_ID_OR_KEY> --repo-id <REPO_ID_OR_NAME>` - List pull requests in a repository
- `pr show --project-id <PROJECT_ID_OR_KEY> --repo-id <REPO_ID_OR_NAME> --pr-number <NUMBER>` - Show pull request details
- `pr download-attachment -p <PROJECT_ID> -r <REPO_ID> -n <PR_NUMBER> -a <ATTACHMENT_ID> -o <FILE_PATH>` - Download a pull request attachment
- `pr update -p <PROJECT_ID> -r <REPO_ID> --pr-number <NUMBER> [OPTIONS]` - Update a pull request (requires `git_writable` feature)
  - `--summary <TITLE>` - Update pull request title
  - `--description <DESC>` - Update pull request description  
  - `--issue-id <ID>` - Link to a related issue
  - `--assignee-id <ID>` - Assign to a user
  - `--notify-user-ids <ID1,ID2>` - Notify users (comma-separated)
  - `--comment <TEXT>` - Add a comment with the update

### User Commands
- `user list` - List all users in the space
- `user me` - Get information about the authenticated user
- `user show <USER_ID>` - Show detailed information about a specific user (requires `user` feature)
- `user icon <USER_ID> --output <FILE_PATH>` - Download user icon to a file
- `user notification-count [OPTIONS]` - Get notification count for authenticated user
  - `--already-read <true|false>` - Include already read notifications
  - `--resource-already-read <true|false>` - Include notifications where resource is already read
- `user notifications [OPTIONS]` - List notifications for authenticated user (alias: `notif`)
  - `--min-id <ID>` - Show notifications with ID greater than this value
  - `--max-id <ID>` - Show notifications with ID less than this value
  - `-n, --count <NUMBER>` - Maximum number of results to return (1-100)
  - `--order <asc|desc>` - Sort order for notifications
  - `--sender-id <USER_ID>` - Filter notifications by sender
- `user mark-notification-read <NOTIFICATION_ID>` - Mark a notification as read (requires `user_writable` feature)
- `user reset-notifications` - Reset all unread notifications by marking them as read (requires `user_writable` feature)

### Document Commands

Document commands are included in the default build. A build with only document read commands uses
`cargo build -p blg --no-default-features --features document`; add `document_writable` for tag changes
and document creation/deletion.

```sh
# One project (numeric ID), multiple projects, or all participating projects
blg document list -p 123 --json
blg document list -p 123 -p 456 --keyword 設計 --sort updated --order desc --offset 0 --count 20 --json
blg document list --keyword 設計 --json

# Counts one project; both ID and key are supported here
blg document count --project-id MYPROJECT --json

# Repeat --tag-name; commas are part of a name, not separators
blg document tag-add 01939983409c79d5a06a49859789e38f --tag-name '設計,仕様' --tag-name 要確認 --json
blg document tag-remove 01939983409c79d5a06a49859789e38f --tag-name 要確認 --json
```

- `document list` accepts numeric IDs only. Omitting `--project-id` searches all participating projects.
  `--offset` defaults to 0, `--count` accepts 1–100 (default 20), and `--sort` accepts `created` or `updated`.
  Results are one page, and the normal display includes complete document IDs and project IDs.
- `document count` requires `--project-id <ID_OR_KEY>` and returns a project count without keyword filtering.
- `document tag-add` and `document tag-remove` require `document_writable`, a document ID and at least one
  `--tag-name`. Empty or whitespace-only names are rejected; other names are sent without normalization.
- With `--json`, stdout contains only JSON: a document array, `{"count":N}`, the API's tag array, or
  `{"success":true}` for removal. List JSON now includes `json` and `attachments`; `json` preserves the API representation.
- Existing `document get`, `tree`, `download`, `add`, and `delete` commands remain available with their existing features.

### Wiki Commands
- `wiki list-attachments <WIKI_ID>` - List attachments for a specific wiki page
- `wiki download-attachment <WIKI_ID> <ATTACHMENT_ID> [--output <FILE_PATH>]` - Download an attachment from a wiki page
- `wiki update <WIKI_ID> [--name <NEW_NAME>] [--content <NEW_CONTENT>] [--mail-notify <true|false>]` - Update a wiki page (requires `wiki_writable` feature)

### Team Commands
- `team show <TEAM_ID>` - Show information about a specific team (requires administrator permission)
- `team list` - List all teams (requires administrator or project administrator permission)
- `team ls` - Alias for `team list`
  - `--order <asc|desc>` - Sort order (default: desc)
  - `--offset <NUMBER>` - Number of items to skip for pagination
  - `--count <NUMBER>` - Number of items to retrieve (1-100, default: 20)
  - `--format <table|json|csv>` - Output format (default: table)
- `team icon <TEAM_ID> --output <FILE_PATH>` - Download team icon image
