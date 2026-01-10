//! Project command arguments and definitions
//!
//! This module contains all command-line argument structures for project-related commands.

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
pub struct ProjectArgs {
    #[clap(subcommand)]
    pub command: ProjectCommands,
}

#[derive(Parser)]
pub enum ProjectCommands {
    /// List all projects
    List,
    /// Show details of a specific project
    Show {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
    },
    /// Add a new project
    #[cfg(feature = "project_writable")]
    Add {
        /// Project name
        #[clap(short, long)]
        name: String,
        /// Project key (uppercase letters, numbers, underscores only)
        #[clap(short, long)]
        key: String,
        /// Enable/disable charts
        #[clap(long)]
        chart_enabled: Option<bool>,
        /// Use resolved for chart
        #[clap(long)]
        use_resolved_for_chart: Option<bool>,
        /// Enable/disable subtasking
        #[clap(long)]
        subtasking_enabled: Option<bool>,
        /// Project leader can edit project leader
        #[clap(long)]
        project_leader_can_edit_project_leader: Option<bool>,
        /// Enable/disable Wiki
        #[clap(long)]
        use_wiki: Option<bool>,
        /// Enable/disable file sharing
        #[clap(long)]
        use_file_sharing: Option<bool>,
        /// Enable/disable Wiki tree view
        #[clap(long)]
        use_wiki_tree_view: Option<bool>,
        /// Enable/disable Subversion
        #[clap(long)]
        use_subversion: Option<bool>,
        /// Enable/disable Git
        #[clap(long)]
        use_git: Option<bool>,
        /// Use original image size at Wiki
        #[clap(long)]
        use_original_image_size_at_wiki: Option<bool>,
        /// Text formatting rule (backlog or markdown)
        #[clap(long)]
        text_formatting_rule: Option<String>,
        /// Use dev attributes
        #[clap(long)]
        use_dev_attributes: Option<bool>,
    },
    /// Update project settings
    #[cfg(feature = "project_writable")]
    Update {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
        /// Update project name
        #[clap(long)]
        name: Option<String>,
        /// Update project key
        #[clap(long)]
        key: Option<String>,
        /// Enable/disable charts
        #[clap(long)]
        chart_enabled: Option<bool>,
        /// Use resolved for chart
        #[clap(long)]
        use_resolved_for_chart: Option<bool>,
        /// Enable/disable subtasking
        #[clap(long)]
        subtasking_enabled: Option<bool>,
        /// Project leader can edit project leader
        #[clap(long)]
        project_leader_can_edit_project_leader: Option<bool>,
        /// Enable/disable Wiki
        #[clap(long)]
        use_wiki: Option<bool>,
        /// Enable/disable file sharing
        #[clap(long)]
        use_file_sharing: Option<bool>,
        /// Enable/disable Wiki tree view
        #[clap(long)]
        use_wiki_tree_view: Option<bool>,
        /// Enable/disable Subversion
        #[clap(long)]
        use_subversion: Option<bool>,
        /// Enable/disable Git
        #[clap(long)]
        use_git: Option<bool>,
        /// Use original image size at Wiki
        #[clap(long)]
        use_original_image_size_at_wiki: Option<bool>,
        /// Text formatting rule (backlog or markdown)
        #[clap(long)]
        text_formatting_rule: Option<String>,
        /// Archive/unarchive project
        #[clap(long)]
        archived: Option<bool>,
        /// Use dev attributes
        #[clap(long)]
        use_dev_attributes: Option<bool>,
    },
    /// Delete a project
    #[cfg(feature = "project_writable")]
    Delete {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
    },
    /// List recently viewed projects
    RecentlyViewed {
        /// Sort order ("asc" or "desc", default: "desc")
        #[clap(long)]
        order: Option<String>,
        /// Number of results to return (1-100, default: 20)
        #[clap(long)]
        count: Option<u32>,
        /// Offset for pagination
        #[clap(long)]
        offset: Option<u32>,
    },
    /// List statuses for a project
    StatusList {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
    },
    /// List milestones for a project
    MilestoneList {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
    },
    /// List issue types for a project
    IssueTypeList {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
    },
    /// List categories for a project
    CategoryList {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
    },
    /// Get disk usage for a project
    DiskUsage {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
        /// Show sizes in human-readable format (e.g., 1.2GB)
        #[clap(short = 'H', long)]
        human_readable: bool,
    },
    /// List users for a project
    UserList {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
    },
    /// List administrators for a project
    AdminList {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
    },
    /// Add a user as a project administrator
    #[cfg(feature = "project_writable")]
    AdminAdd {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
        /// User ID to add as administrator
        #[clap(short, long)]
        user_id: u32,
    },
    /// Remove an administrator from a project
    #[cfg(feature = "project_writable")]
    AdminRemove {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
        /// User ID to remove as administrator
        #[clap(short, long)]
        user_id: u32,
    },
    /// Add a user to a project
    #[cfg(feature = "project_writable")]
    UserAdd {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
        /// User ID to add
        #[clap(short, long)]
        user_id: u32,
    },
    /// Remove a user from a project
    #[cfg(feature = "project_writable")]
    UserRemove {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
        /// User ID to remove
        #[clap(short, long)]
        user_id: u32,
    },
    /// List custom fields for a project
    CustomFieldList {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
    },
    /// Update a custom field in a project
    #[cfg(feature = "project_writable")]
    CustomFieldUpdate {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
        /// Custom field ID
        #[clap(long)]
        custom_field_id: u32,
        /// Update the name
        #[clap(long)]
        name: Option<String>,
        /// Update the description
        #[clap(long)]
        description: Option<String>,
        /// Update whether field is required
        #[clap(long)]
        required: Option<bool>,
        /// Update applicable issue types (comma-separated IDs)
        #[clap(long)]
        applicable_issue_types: Option<String>,
        // Date field specific parameters
        /// Minimum date (YYYY-MM-DD format)
        #[clap(long)]
        min_date: Option<String>,
        /// Maximum date (YYYY-MM-DD format)
        #[clap(long)]
        max_date: Option<String>,
        /// Initial value type (1-7)
        #[clap(long)]
        initial_value_type: Option<i32>,
        /// Initial date (YYYY-MM-DD format)
        #[clap(long)]
        initial_date: Option<String>,
        /// Initial shift (days from current date)
        #[clap(long)]
        initial_shift: Option<i32>,
    },
    /// Add a custom field to a project
    #[cfg(feature = "project_writable")]
    CustomFieldAdd {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
        /// Field type (text, textarea, numeric, date, single-list, multiple-list, checkbox, radio)
        #[clap(short = 't', long)]
        field_type: String,
        /// Field name
        #[clap(short, long)]
        name: String,
        /// Field description (optional)
        #[clap(short = 'd', long)]
        description: Option<String>,
        /// Make field required (optional)
        #[clap(short = 'r', long)]
        required: Option<bool>,
        /// Applicable issue type IDs (comma-separated)
        #[clap(long)]
        applicable_issue_types: Option<String>,
        // Numeric field parameters
        /// Minimum value (for numeric fields)
        #[clap(long)]
        min: Option<f64>,
        /// Maximum value (for numeric fields)
        #[clap(long)]
        max: Option<f64>,
        /// Initial value (for numeric fields)
        #[clap(long)]
        initial_value: Option<f64>,
        /// Unit (for numeric fields)
        #[clap(long)]
        unit: Option<String>,
        // Date field parameters
        /// Minimum date (YYYY-MM-DD format)
        #[clap(long)]
        min_date: Option<String>,
        /// Maximum date (YYYY-MM-DD format)
        #[clap(long)]
        max_date: Option<String>,
        /// Initial value type (1-3)
        #[clap(long)]
        initial_value_type: Option<i32>,
        /// Initial date (YYYY-MM-DD format)
        #[clap(long)]
        initial_date: Option<String>,
        /// Initial shift (days from current date)
        #[clap(long)]
        initial_shift: Option<i32>,
        // List field parameters
        /// List items (comma-separated)
        #[clap(long)]
        items: Option<String>,
        /// Allow direct input
        #[clap(long)]
        allow_input: Option<bool>,
        /// Allow adding new items
        #[clap(long)]
        allow_add_item: Option<bool>,
    },
    /// Delete a custom field from a project
    #[cfg(feature = "project_writable")]
    CustomFieldDelete {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
        /// Custom Field ID
        #[clap(short, long)]
        custom_field_id: u32,
    },
    /// Add a list item to a list type custom field
    #[cfg(feature = "project_writable")]
    CustomFieldAddItem {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
        /// Custom Field ID
        #[clap(short, long)]
        custom_field_id: u32,
        /// Name of the new list item
        #[clap(short, long)]
        name: String,
    },
    /// Update a list item in a list type custom field
    #[cfg(feature = "project_writable")]
    CustomFieldUpdateItem {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
        /// Custom Field ID
        #[clap(short, long)]
        custom_field_id: u32,
        /// List Item ID
        #[clap(short, long)]
        item_id: u32,
        /// New name for the list item
        #[clap(short, long)]
        name: String,
    },
    /// Delete a list item from a list type custom field
    #[cfg(feature = "project_writable")]
    CustomFieldDeleteItem {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
        /// Custom Field ID
        #[clap(short, long)]
        custom_field_id: u32,
        /// List Item ID to delete
        #[clap(short, long)]
        item_id: u32,
    },
    /// Add a category to a project
    #[cfg(feature = "project_writable")]
    CategoryAdd {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
        /// Category name
        #[clap(short, long)]
        name: String,
    },
    /// Update a category in a project
    #[cfg(feature = "project_writable")]
    CategoryUpdate {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
        /// Category ID
        #[clap(short, long)]
        category_id: u32,
        /// New category name
        #[clap(short, long)]
        name: String,
    },
    /// Delete a category from a project
    #[cfg(feature = "project_writable")]
    CategoryDelete {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
        /// Category ID
        #[clap(short, long)]
        category_id: u32,
    },
    /// Add an issue type to a project
    #[cfg(feature = "project_writable")]
    IssueTypeAdd {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
        /// Issue type name
        #[clap(short, long)]
        name: String,
        /// Issue type color (available: red, dark-red, purple, violet, blue, teal, green, orange, pink, gray)
        #[clap(short, long)]
        color: String,
        /// Template summary (optional)
        #[clap(short = 's', long)]
        template_summary: Option<String>,
        /// Template description (optional)
        #[clap(short = 'd', long)]
        template_description: Option<String>,
    },
    /// Delete an issue type from a project
    #[cfg(feature = "project_writable")]
    IssueTypeDelete {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
        /// Issue type ID to delete
        #[clap(long)]
        issue_type_id: u32,
        /// Substitute issue type ID for existing issues
        #[clap(long)]
        substitute_issue_type_id: u32,
    },
    /// Update an issue type in a project
    #[cfg(feature = "project_writable")]
    IssueTypeUpdate {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
        /// Issue type ID to update
        #[clap(long)]
        issue_type_id: u32,
        /// New issue type name (optional)
        #[clap(short, long)]
        name: Option<String>,
        /// New issue type color (optional: red, dark-red, purple, violet, blue, teal, green, orange, pink, gray)
        #[clap(short, long)]
        color: Option<String>,
        /// New template summary (optional)
        #[clap(short = 's', long)]
        template_summary: Option<String>,
        /// New template description (optional)
        #[clap(short = 'd', long)]
        template_description: Option<String>,
    },
    /// List priorities (space-wide)
    PriorityList,
    /// List resolutions (space-wide)
    ResolutionList,
    /// Add a version/milestone to a project
    #[cfg(feature = "project_writable")]
    VersionAdd {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
        /// Version name
        #[clap(short, long)]
        name: String,
        /// Version description
        #[clap(short, long)]
        description: Option<String>,
        /// Start date (YYYY-MM-DD)
        #[clap(long)]
        start_date: Option<String>,
        /// Release due date (YYYY-MM-DD)
        #[clap(long)]
        release_due_date: Option<String>,
    },
    /// Update a version/milestone in a project
    #[cfg(feature = "project_writable")]
    VersionUpdate {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
        /// Version ID to update
        #[clap(long)]
        version_id: u32,
        /// New version name
        #[clap(short, long)]
        name: String,
        /// New version description
        #[clap(short, long)]
        description: Option<String>,
        /// New start date (YYYY-MM-DD)
        #[clap(long)]
        start_date: Option<String>,
        /// New release due date (YYYY-MM-DD)
        #[clap(long)]
        release_due_date: Option<String>,
        /// Archive the version
        #[clap(long)]
        archived: Option<bool>,
    },
    /// Delete a version/milestone from a project
    #[cfg(feature = "project_writable")]
    VersionDelete {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
        /// Version ID to delete
        #[clap(long)]
        version_id: u32,
    },
    /// Add a status to a project
    #[cfg(feature = "project_writable")]
    StatusAdd {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
        /// Status name
        #[clap(short, long)]
        name: String,
        /// Status color (red, coral, pink, light-purple, blue, green, light-green, orange, magenta, dark-gray)
        #[clap(short, long)]
        color: String,
    },
    /// Update a status in a project
    #[cfg(feature = "project_writable")]
    StatusUpdate {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
        /// Status ID to update
        #[clap(long)]
        status_id: u32,
        /// New status name (optional)
        #[clap(short, long)]
        name: Option<String>,
        /// New status color (optional: red, coral, pink, light-purple, blue, green, light-green, orange, magenta, dark-gray)
        #[clap(short, long)]
        color: Option<String>,
    },
    /// Delete a status from a project
    #[cfg(feature = "project_writable")]
    StatusDelete {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
        /// Status ID to delete
        #[clap(long)]
        status_id: u32,
        /// Substitute status ID for existing issues
        #[clap(long)]
        substitute_status_id: u32,
    },
    /// Update the display order of statuses in a project
    #[cfg(feature = "project_writable")]
    StatusOrderUpdate {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
        /// Status IDs in desired display order (comma-separated)
        #[clap(long)]
        status_ids: String,
    },
    /// Download project icon
    Icon {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
        /// Output file path to save the icon
        #[clap(short, long, value_name = "FILE_PATH")]
        output: PathBuf,
    },
    /// List teams for a project
    TeamList {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
    },
    /// Add a team to a project
    #[cfg(feature = "project_writable")]
    TeamAdd {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
        /// Team ID to add
        #[clap(short, long)]
        team_id: u32,
    },
    /// Remove a team from a project
    #[cfg(feature = "project_writable")]
    TeamDelete {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
        /// Team ID to remove
        #[clap(short, long)]
        team_id: u32,
    },
}
