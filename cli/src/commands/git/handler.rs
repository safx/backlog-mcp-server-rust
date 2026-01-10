use crate::commands::common::CliResult;
use crate::commands::git::args::{PrArgs, PrCommands, RepoArgs, RepoCommands};
use backlog_api_client::client::BacklogApiClient;

use super::subcommands;

/// Execute repository-related commands.
pub async fn execute_repo(client: &BacklogApiClient, args: RepoArgs) -> CliResult<()> {
    match args.command {
        RepoCommands::List { project_id } => {
            subcommands::repo::list(client, project_id).await?;
        }
        RepoCommands::Show {
            project_id,
            repo_id,
        } => {
            subcommands::repo::show(client, project_id, repo_id).await?;
        }
    }
    Ok(())
}

/// Execute pull request-related commands.
pub async fn execute_pr(client: &BacklogApiClient, args: PrArgs) -> CliResult<()> {
    match args.command {
        PrCommands::List {
            project_id,
            repo_id,
        } => {
            subcommands::pr::list(client, project_id, repo_id).await?;
        }
        PrCommands::Show {
            project_id,
            repo_id,
            pr_number,
        } => {
            subcommands::pr::show(client, project_id, repo_id, pr_number).await?;
        }
        PrCommands::DownloadAttachment(dl_args) => {
            subcommands::pr_attachments::download_attachment(client, dl_args).await?;
        }
        PrCommands::CommentCount {
            project_id,
            repo_id,
            pr_number,
        } => {
            subcommands::pr_comments::comment_count(client, project_id, repo_id, pr_number).await?;
        }
        PrCommands::Count {
            project_id,
            repo_id,
            status_ids,
            assignee_ids,
            issue_ids,
            created_user_ids,
            offset: _,
            count: _,
        } => {
            subcommands::pr::count(
                client,
                project_id,
                repo_id,
                status_ids,
                assignee_ids,
                issue_ids,
                created_user_ids,
            )
            .await?;
        }
        #[cfg(feature = "git_writable")]
        PrCommands::DeleteAttachment(del_args) => {
            subcommands::pr_attachments::delete_attachment(client, del_args).await?;
        }
        #[cfg(feature = "git_writable")]
        PrCommands::Update {
            project_id,
            repo_id,
            pr_number,
            summary,
            description,
            issue_id,
            assignee_id,
            notify_user_ids,
            comment,
        } => {
            subcommands::pr::update(
                client,
                project_id,
                repo_id,
                pr_number,
                summary,
                description,
                issue_id,
                assignee_id,
                notify_user_ids,
                comment,
            )
            .await?;
        }
        #[cfg(feature = "git_writable")]
        PrCommands::CommentUpdate {
            project_id,
            repo_id,
            pr_number,
            comment_id,
            content,
        } => {
            subcommands::pr_comments::comment_update(
                client, project_id, repo_id, pr_number, comment_id, content,
            )
            .await?;
        }
        #[cfg(feature = "git_writable")]
        PrCommands::Create {
            project_id,
            repo_id,
            summary,
            description,
            base,
            branch,
            issue_id,
            assignee_id,
            notify_user_ids,
            attachment_ids,
        } => {
            subcommands::pr::create(
                client,
                project_id,
                repo_id,
                summary,
                description,
                base,
                branch,
                issue_id,
                assignee_id,
                notify_user_ids,
                attachment_ids,
            )
            .await?;
        }
        #[cfg(not(feature = "git_writable"))]
        _ => {
            eprintln!(
                "This command requires write access. Please rebuild with --features git_writable"
            );
            std::process::exit(1);
        }
    }
    Ok(())
}
