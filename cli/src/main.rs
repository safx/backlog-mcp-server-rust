#[cfg(any(
    feature = "project",
    feature = "issue",
    feature = "team",
    feature = "star",
    feature = "rate-limit",
    feature = "watching",
    feature = "webhook",
    feature = "user",
    feature = "wiki",
    feature = "git",
    feature = "space"
))]
mod commands;
#[cfg(feature = "project")]
use commands::activity::ActivityArgs;
#[cfg(feature = "project")]
use commands::project::ProjectArgs;
#[cfg(feature = "rate-limit")]
use commands::rate_limit::{RateLimitCommand, handle_rate_limit_command};
#[cfg(feature = "star")]
use commands::star::{StarArgs, handle_star_command};
#[cfg(feature = "team")]
use commands::team::{TeamArgs, handle_team_command};
#[cfg(feature = "user")]
use commands::user::UserArgs;
#[cfg(feature = "watching")]
use commands::watching::handle_watching_command;
#[cfg(feature = "wiki")]
use commands::wiki::WikiArgs;

use backlog_api_client::client::BacklogApiClient;
use clap::{Args, Parser};
use std::env;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Parser)]
enum Commands {
    /// Manage repositories
    #[cfg(feature = "git")]
    Repo(commands::git::RepoArgs),
    /// Manage pull requests
    #[cfg(feature = "git")]
    Pr(commands::git::PrArgs),
    /// Manage issues
    #[cfg(feature = "issue")]
    Issue(commands::issue::IssueArgs),
    /// Manage space
    #[cfg(feature = "space")]
    Space(commands::space::SpaceArgs),
    /// Manage projects
    #[cfg(feature = "project")]
    Project(ProjectArgs),
    /// Manage users
    #[cfg(feature = "user")]
    User(UserArgs),
    /// Manage wikis
    #[cfg(feature = "wiki")]
    Wiki(WikiArgs),
    /// View activities
    #[cfg(feature = "project")]
    Activity(ActivityArgs),
    /// Manage teams
    #[cfg(feature = "team")]
    Team(TeamArgs),
    /// Manage stars
    #[cfg(feature = "star")]
    Star(StarArgs),
    /// View rate limit information
    #[cfg(feature = "rate-limit")]
    RateLimit(RateLimitArgs),
    /// Manage watchings
    #[cfg(feature = "watching")]
    Watching(WatchingArgs),
    /// Manage webhooks
    #[cfg(feature = "webhook")]
    Webhook(commands::webhook::WebhookArgs),
}

#[cfg(feature = "rate-limit")]
#[derive(Args)]
struct RateLimitArgs {
    #[clap(subcommand)]
    command: RateLimitCommand,
}

#[cfg(feature = "watching")]
#[derive(Args)]
struct WatchingArgs {
    #[clap(subcommand)]
    command: commands::watching::WatchingSubcommand,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_url = env::var("BACKLOG_BASE_URL")?;
    let api_key = env::var("BACKLOG_API_KEY")?;

    let client = BacklogApiClient::new(&base_url)?.with_api_key(api_key);

    let cli = Cli::parse();
    match cli.command {
        #[cfg(feature = "git")]
        Commands::Repo(repo_args) => {
            commands::git::execute_repo(&client, repo_args).await?;
        }
        #[cfg(feature = "git")]
        Commands::Pr(pr_args) => {
            commands::git::execute_pr(&client, pr_args).await?;
        }
        #[cfg(feature = "issue")]
        Commands::Issue(issue_args) => {
            commands::issue::execute(&client, issue_args).await?;
        }
        #[cfg(feature = "space")]
        Commands::Space(space_args) => {
            commands::space::execute(&client, space_args).await?;
        }
        #[cfg(feature = "project")]
        Commands::Project(project_args) => {
            commands::project::execute(&client, project_args).await?;
        }
        #[cfg(feature = "user")]
        Commands::User(user_args) => {
            commands::user::execute(&client, user_args).await?;
        }
        #[cfg(feature = "wiki")]
        Commands::Wiki(wiki_args) => {
            commands::wiki::execute(&client, wiki_args).await?;
        }
        #[cfg(feature = "project")]
        Commands::Activity(activity_args) => {
            commands::activity::execute(&client, activity_args).await?;
        }
        #[cfg(feature = "team")]
        Commands::Team(team_args) => {
            handle_team_command(client.team(), team_args).await;
        }
        #[cfg(feature = "star")]
        Commands::Star(star_args) => {
            handle_star_command(&client.star(), &star_args.command).await?;
        }
        #[cfg(feature = "rate-limit")]
        Commands::RateLimit(rate_limit_args) => {
            handle_rate_limit_command(rate_limit_args.command).await?;
        }
        #[cfg(feature = "watching")]
        Commands::Watching(watching_args) => {
            handle_watching_command(commands::watching::WatchingCommand {
                command: watching_args.command,
            })
            .await?;
        }
        #[cfg(feature = "webhook")]
        Commands::Webhook(webhook_args) => {
            commands::webhook::execute(&client, webhook_args).await?;
        }
    }

    Ok(())
}
