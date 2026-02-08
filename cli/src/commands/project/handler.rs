//! Project command handler
//!
//! Dispatches project subcommands to their respective implementations.

use crate::commands::common::CliResult;
use crate::commands::project::args::{ProjectArgs, ProjectCommands};
use backlog_api_client::client::BacklogApiClient;

use super::subcommands;

/// Execute a project command
pub async fn execute(client: &BacklogApiClient, args: ProjectArgs) -> CliResult<()> {
    match args.command {
        // List, Show, RecentlyViewed
        ProjectCommands::List => subcommands::list::list(client).await?,
        ProjectCommands::Show { project_id_or_key } => {
            subcommands::list::show(client, &project_id_or_key).await?
        }
        ProjectCommands::RecentlyViewed {
            order,
            count,
            offset,
        } => subcommands::list::recently_viewed(client, order, count, offset).await?,

        // CRUD operations
        #[cfg(feature = "project_writable")]
        ProjectCommands::Add {
            name,
            key,
            chart_enabled,
            use_resolved_for_chart,
            subtasking_enabled,
            project_leader_can_edit_project_leader,
            use_wiki,
            use_file_sharing,
            use_wiki_tree_view,
            use_subversion,
            use_git,
            use_original_image_size_at_wiki,
            text_formatting_rule,
            use_dev_attributes,
        } => {
            subcommands::crud::add(
                client,
                &name,
                &key,
                chart_enabled,
                use_resolved_for_chart,
                subtasking_enabled,
                project_leader_can_edit_project_leader,
                use_wiki,
                use_file_sharing,
                use_wiki_tree_view,
                use_subversion,
                use_git,
                use_original_image_size_at_wiki,
                text_formatting_rule.as_deref(),
                use_dev_attributes,
            )
            .await?
        }
        #[cfg(feature = "project_writable")]
        ProjectCommands::Update {
            project_id_or_key,
            name,
            key,
            chart_enabled,
            use_resolved_for_chart,
            subtasking_enabled,
            project_leader_can_edit_project_leader,
            use_wiki,
            use_file_sharing,
            use_wiki_tree_view,
            use_subversion,
            use_git,
            use_original_image_size_at_wiki,
            text_formatting_rule,
            archived,
            use_dev_attributes,
        } => {
            subcommands::crud::update(
                client,
                &project_id_or_key,
                name,
                key,
                chart_enabled,
                use_resolved_for_chart,
                subtasking_enabled,
                project_leader_can_edit_project_leader,
                use_wiki,
                use_file_sharing,
                use_wiki_tree_view,
                use_subversion,
                use_git,
                use_original_image_size_at_wiki,
                text_formatting_rule,
                archived,
                use_dev_attributes,
            )
            .await?
        }
        #[cfg(feature = "project_writable")]
        ProjectCommands::Delete { project_id_or_key } => {
            subcommands::crud::delete(client, &project_id_or_key).await?
        }

        // Users
        ProjectCommands::UserList { project_id_or_key } => {
            subcommands::users::list(client, &project_id_or_key).await?
        }
        ProjectCommands::AdminList { project_id_or_key } => {
            subcommands::users::admin_list(client, &project_id_or_key).await?
        }
        #[cfg(feature = "project_writable")]
        ProjectCommands::AdminAdd {
            project_id_or_key,
            user_id,
        } => subcommands::users::admin_add(client, &project_id_or_key, user_id).await?,
        #[cfg(feature = "project_writable")]
        ProjectCommands::AdminRemove {
            project_id_or_key,
            user_id,
        } => subcommands::users::admin_remove(client, &project_id_or_key, user_id).await?,
        #[cfg(feature = "project_writable")]
        ProjectCommands::UserAdd {
            project_id_or_key,
            user_id,
        } => subcommands::users::user_add(client, &project_id_or_key, user_id).await?,
        #[cfg(feature = "project_writable")]
        ProjectCommands::UserRemove {
            project_id_or_key,
            user_id,
        } => subcommands::users::user_remove(client, &project_id_or_key, user_id).await?,

        // Issue Types
        ProjectCommands::IssueTypeList { project_id_or_key } => {
            subcommands::issue_types::list(client, &project_id_or_key).await?
        }
        #[cfg(feature = "project_writable")]
        ProjectCommands::IssueTypeAdd {
            project_id_or_key,
            name,
            color,
            template_summary,
            template_description,
        } => {
            subcommands::issue_types::add(
                client,
                &project_id_or_key,
                &name,
                &color,
                template_summary,
                template_description,
            )
            .await?
        }
        #[cfg(feature = "project_writable")]
        ProjectCommands::IssueTypeUpdate {
            project_id_or_key,
            issue_type_id,
            name,
            color,
            template_summary,
            template_description,
        } => {
            subcommands::issue_types::update(
                client,
                &project_id_or_key,
                issue_type_id,
                name,
                color,
                template_summary,
                template_description,
            )
            .await?
        }
        #[cfg(feature = "project_writable")]
        ProjectCommands::IssueTypeDelete {
            project_id_or_key,
            issue_type_id,
            substitute_issue_type_id,
        } => {
            subcommands::issue_types::delete(
                client,
                &project_id_or_key,
                issue_type_id,
                substitute_issue_type_id,
            )
            .await?
        }

        // Categories
        ProjectCommands::CategoryList { project_id_or_key } => {
            subcommands::categories::list(client, &project_id_or_key).await?
        }
        #[cfg(feature = "project_writable")]
        ProjectCommands::CategoryAdd {
            project_id_or_key,
            name,
        } => subcommands::categories::add(client, &project_id_or_key, &name).await?,
        #[cfg(feature = "project_writable")]
        ProjectCommands::CategoryUpdate {
            project_id_or_key,
            category_id,
            name,
        } => {
            subcommands::categories::update(client, &project_id_or_key, category_id, &name).await?
        }
        #[cfg(feature = "project_writable")]
        ProjectCommands::CategoryDelete {
            project_id_or_key,
            category_id,
        } => subcommands::categories::delete(client, &project_id_or_key, category_id).await?,

        // Versions/Milestones
        ProjectCommands::MilestoneList { project_id_or_key } => {
            subcommands::versions::list(client, &project_id_or_key).await?
        }
        #[cfg(feature = "project_writable")]
        ProjectCommands::VersionAdd {
            project_id_or_key,
            name,
            description,
            start_date,
            release_due_date,
        } => {
            subcommands::versions::add(
                client,
                &project_id_or_key,
                &name,
                description,
                start_date,
                release_due_date,
            )
            .await?
        }
        #[cfg(feature = "project_writable")]
        ProjectCommands::VersionUpdate {
            project_id_or_key,
            version_id,
            name,
            description,
            start_date,
            release_due_date,
            archived,
        } => {
            subcommands::versions::update(
                client,
                &project_id_or_key,
                version_id,
                &name,
                description,
                start_date,
                release_due_date,
                archived,
            )
            .await?
        }
        #[cfg(feature = "project_writable")]
        ProjectCommands::VersionDelete {
            project_id_or_key,
            version_id,
        } => subcommands::versions::delete(client, &project_id_or_key, version_id).await?,

        // Custom Fields
        ProjectCommands::CustomFieldList { project_id_or_key } => {
            subcommands::custom_fields::list(client, &project_id_or_key).await?
        }
        #[cfg(feature = "project_writable")]
        ProjectCommands::CustomFieldAdd {
            project_id_or_key,
            field_type,
            name,
            description,
            required,
            applicable_issue_types,
            min,
            max,
            initial_value,
            unit,
            min_date,
            max_date,
            initial_value_type,
            initial_date,
            initial_shift,
            items,
            allow_input,
            allow_add_item,
        } => {
            subcommands::custom_fields::add(
                client,
                &project_id_or_key,
                &field_type,
                &name,
                description,
                required,
                applicable_issue_types,
                min,
                max,
                initial_value,
                unit,
                min_date,
                max_date,
                initial_value_type,
                initial_date,
                initial_shift,
                items,
                allow_input,
                allow_add_item,
            )
            .await?
        }
        #[cfg(feature = "project_writable")]
        ProjectCommands::CustomFieldUpdate {
            project_id_or_key,
            custom_field_id,
            name,
            description,
            required,
            applicable_issue_types,
            min_date,
            max_date,
            initial_value_type,
            initial_date,
            initial_shift,
        } => {
            subcommands::custom_fields::update(
                client,
                &project_id_or_key,
                custom_field_id,
                name,
                description,
                required,
                applicable_issue_types,
                min_date,
                max_date,
                initial_value_type,
                initial_date,
                initial_shift,
            )
            .await?
        }
        #[cfg(feature = "project_writable")]
        ProjectCommands::CustomFieldDelete {
            project_id_or_key,
            custom_field_id,
        } => {
            subcommands::custom_fields::delete(client, &project_id_or_key, custom_field_id).await?
        }
        #[cfg(feature = "project_writable")]
        ProjectCommands::CustomFieldAddItem {
            project_id_or_key,
            custom_field_id,
            name,
        } => {
            subcommands::custom_fields::add_item(client, &project_id_or_key, custom_field_id, &name)
                .await?
        }
        #[cfg(feature = "project_writable")]
        ProjectCommands::CustomFieldUpdateItem {
            project_id_or_key,
            custom_field_id,
            item_id,
            name,
        } => {
            subcommands::custom_fields::update_item(
                client,
                &project_id_or_key,
                custom_field_id,
                item_id,
                &name,
            )
            .await?
        }
        #[cfg(feature = "project_writable")]
        ProjectCommands::CustomFieldDeleteItem {
            project_id_or_key,
            custom_field_id,
            item_id,
        } => {
            subcommands::custom_fields::delete_item(
                client,
                &project_id_or_key,
                custom_field_id,
                item_id,
            )
            .await?
        }

        // Statuses
        ProjectCommands::StatusList { project_id_or_key } => {
            subcommands::statuses::list(client, &project_id_or_key).await?
        }
        #[cfg(feature = "project_writable")]
        ProjectCommands::StatusAdd {
            project_id_or_key,
            name,
            color,
        } => subcommands::statuses::add(client, &project_id_or_key, &name, &color).await?,
        #[cfg(feature = "project_writable")]
        ProjectCommands::StatusUpdate {
            project_id_or_key,
            status_id,
            name,
            color,
        } => {
            subcommands::statuses::update(client, &project_id_or_key, status_id, name, color)
                .await?
        }
        #[cfg(feature = "project_writable")]
        ProjectCommands::StatusDelete {
            project_id_or_key,
            status_id,
            substitute_status_id,
        } => {
            subcommands::statuses::delete(
                client,
                &project_id_or_key,
                status_id,
                substitute_status_id,
            )
            .await?
        }
        #[cfg(feature = "project_writable")]
        ProjectCommands::StatusOrderUpdate {
            project_id_or_key,
            status_ids,
        } => subcommands::statuses::update_order(client, &project_id_or_key, &status_ids).await?,

        // Teams
        ProjectCommands::TeamList { project_id_or_key } => {
            subcommands::teams::list(client, &project_id_or_key).await?
        }
        #[cfg(feature = "project_writable")]
        ProjectCommands::TeamAdd {
            project_id_or_key,
            team_id,
        } => subcommands::teams::add(client, &project_id_or_key, team_id).await?,
        #[cfg(feature = "project_writable")]
        ProjectCommands::TeamDelete {
            project_id_or_key,
            team_id,
        } => subcommands::teams::delete(client, &project_id_or_key, team_id).await?,

        // Miscellaneous
        ProjectCommands::PriorityList => subcommands::misc::priority_list(client).await?,
        ProjectCommands::ResolutionList => subcommands::misc::resolution_list(client).await?,
        ProjectCommands::Icon {
            project_id_or_key,
            output,
        } => subcommands::misc::download_icon(client, &project_id_or_key, &output).await?,
        ProjectCommands::DiskUsage {
            project_id_or_key,
            human_readable,
        } => subcommands::misc::disk_usage(client, &project_id_or_key, human_readable).await?,

        // Fallback for disabled write features
        #[cfg(not(feature = "project_writable"))]
        _ => {
            anyhow::bail!(
                "This command requires write access to projects and is not available. \
                Please build with the 'project_writable' feature flag:\n\
                cargo build --package blg --features project_writable"
            );
        }
    }
    Ok(())
}
