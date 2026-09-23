#[cfg(feature = "star")]
use backlog_api_client::StarApi;
#[cfg(all(feature = "star", feature = "star_writable"))]
use backlog_api_client::{AddStarParams, DeleteStarParams};
use clap::{Args, Subcommand};

#[derive(Args)]
pub struct StarArgs {
    #[clap(subcommand)]
    pub command: StarCommands,
}

#[derive(Subcommand)]
pub enum StarCommands {
    #[cfg(feature = "star_writable")]
    /// Add a star to a resource
    Add {
        #[clap(subcommand)]
        target: StarTarget,
    },
    #[cfg(feature = "star_writable")]
    /// Delete a star
    Delete {
        /// Star ID
        star_id: u32,
    },
}

#[cfg(feature = "star_writable")]
#[derive(Subcommand)]
pub enum StarTarget {
    /// Add star to an issue
    Issue {
        /// Issue ID
        issue_id: u32,
    },
    /// Add star to a comment
    Comment {
        /// Issue ID
        issue_id: u32,
        /// Comment ID
        comment_id: u32,
    },
    /// Add star to a wiki page
    Wiki {
        /// Wiki ID
        wiki_id: u32,
    },
    /// Add star to a pull request
    Pr {
        /// Pull request ID
        pr_id: u32,
    },
    /// Add star to a pull request comment
    PrComment {
        /// Pull request comment ID
        pr_comment_id: u32,
    },
}

#[cfg(feature = "star")]
pub async fn handle_star_command(api: &StarApi, command: &StarCommands) -> anyhow::Result<()> {
    match command {
        #[cfg(feature = "star_writable")]
        StarCommands::Add { target } => handle_add_star(api, target).await,
        #[cfg(feature = "star_writable")]
        StarCommands::Delete { star_id } => {
            api.delete_star(DeleteStarParams::new(*star_id)).await?;
            println!("Star deleted successfully");
            Ok(())
        }
    }
}

#[cfg(feature = "star_writable")]
async fn handle_add_star(api: &StarApi, target: &StarTarget) -> anyhow::Result<()> {
    let params = match target {
        StarTarget::Issue { issue_id } => AddStarParams::issue(*issue_id),
        StarTarget::Comment {
            issue_id,
            comment_id,
        } => AddStarParams::comment(*issue_id, *comment_id),
        StarTarget::Wiki { wiki_id } => AddStarParams::wiki(*wiki_id),
        StarTarget::Pr { pr_id } => AddStarParams::pull_request(*pr_id),
        StarTarget::PrComment { pr_comment_id } => {
            AddStarParams::pull_request_comment(*pr_comment_id)
        }
    };

    api.add_star(params).await?;
    println!("Star added successfully");
    Ok(())
}
