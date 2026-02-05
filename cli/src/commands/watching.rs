use backlog_core::IssueIdOrKey;
use backlog_core::identifier::WatchingId;
use clap::{Args, Subcommand};

#[derive(Args, Debug, Clone)]
pub struct WatchingCommand {
    #[command(subcommand)]
    pub command: WatchingSubcommand,
}

#[derive(Subcommand, Debug, Clone)]
pub enum WatchingSubcommand {
    /// Get details of a specific watching
    Get(GetWatchingArgs),

    /// Add a new watching for an issue
    #[cfg(feature = "watching_writable")]
    Add(AddWatchingArgs),

    /// Update an existing watching
    #[cfg(feature = "watching_writable")]
    Update(UpdateWatchingArgs),

    /// Delete a watching
    #[cfg(feature = "watching_writable")]
    Delete(DeleteWatchingArgs),

    /// Mark a watching as read
    #[cfg(feature = "watching_writable")]
    MarkRead(MarkAsReadArgs),
}

#[derive(Args, Debug, Clone)]
pub struct GetWatchingArgs {
    /// The ID of the watching to retrieve
    pub watching_id: u32,
}

#[cfg(feature = "watching_writable")]
#[derive(Args, Debug, Clone)]
pub struct AddWatchingArgs {
    /// The ID or key of the issue to watch
    pub issue: String,

    /// Optional note for the watching
    #[arg(short, long)]
    pub note: Option<String>,
}

#[cfg(feature = "watching_writable")]
#[derive(Args, Debug, Clone)]
pub struct UpdateWatchingArgs {
    /// The ID of the watching to update
    pub watching_id: u32,

    /// The note to set for the watching
    #[arg(short, long)]
    pub note: String,
}

#[cfg(feature = "watching_writable")]
#[derive(Args, Debug, Clone)]
pub struct DeleteWatchingArgs {
    /// The ID of the watching to delete
    pub watching_id: u32,
}

#[cfg(feature = "watching_writable")]
#[derive(Args, Debug, Clone)]
pub struct MarkAsReadArgs {
    /// The ID of the watching to mark as read
    pub watching_id: u32,
}

pub async fn handle_watching_command(command: WatchingCommand) -> anyhow::Result<()> {
    use backlog_api_client::client::BacklogApiClient;
    use std::env;

    let base_url = env::var("BACKLOG_BASE_URL")?;
    let api_key = env::var("BACKLOG_API_KEY")?;
    let client = BacklogApiClient::new(&base_url)?.with_api_key(api_key);
    let api = client.watching();

    match command.command {
        WatchingSubcommand::Get(args) => {
            let watching = api.get(WatchingId::from(args.watching_id)).await?;

            println!("Watching ID: {}", watching.id);
            println!("Type: {:?}", watching.watching_type);
            println!("Already Read: {}", watching.already_read);
            println!("Resource Already Read: {}", watching.resource_already_read);

            if let Some(note) = &watching.note {
                println!("Note: {note}");
            }

            if let Some(issue) = &watching.issue {
                println!("\nIssue Details:");
                println!("  Key: {}", issue.issue_key);
                println!("  Summary: {}", issue.summary);
                println!("  Status: {}", issue.status.name);
                if let Some(assignee) = &issue.assignee {
                    println!("  Assignee: {}", assignee.name);
                }
            }

            if let Some(last_updated) = &watching.last_content_updated {
                println!(
                    "\nLast Content Updated: {}",
                    last_updated.format("%Y-%m-%d %H:%M:%S")
                );
            }
            println!("Created: {}", watching.created.format("%Y-%m-%d %H:%M:%S"));
            println!("Updated: {}", watching.updated.format("%Y-%m-%d %H:%M:%S"));
        }

        #[cfg(feature = "watching_writable")]
        WatchingSubcommand::Add(args) => {
            use backlog_watching::AddWatchingParams;

            let issue_id_or_key = parse_issue_id_or_key(&args.issue)?;
            let mut params = AddWatchingParams::new(issue_id_or_key);

            if let Some(note) = args.note {
                params = params.with_note(note);
            }

            let watching = api.add(params).await?;
            println!("Successfully added watching with ID: {}", watching.id);

            if let Some(issue) = &watching.issue {
                println!("Watching issue: {} - {}", issue.issue_key, issue.summary);
            }
        }

        #[cfg(feature = "watching_writable")]
        WatchingSubcommand::Update(args) => {
            use backlog_watching::UpdateWatchingParams;

            let params =
                UpdateWatchingParams::new(WatchingId::from(args.watching_id)).with_note(args.note);

            let watching = api.update(params).await?;
            println!("Successfully updated watching {}", watching.id);

            if let Some(note) = &watching.note {
                println!("Note: {note}");
            }
        }

        #[cfg(feature = "watching_writable")]
        WatchingSubcommand::Delete(args) => {
            let deleted = api.delete(WatchingId::from(args.watching_id)).await?;
            println!("Successfully deleted watching {}", deleted.id);

            if let Some(issue) = &deleted.issue {
                println!(
                    "Was watching issue: {} - {}",
                    issue.issue_key, issue.summary
                );
            }
        }

        #[cfg(feature = "watching_writable")]
        WatchingSubcommand::MarkRead(args) => {
            api.mark_as_read(WatchingId::from(args.watching_id)).await?;
            println!("Successfully marked watching {} as read", args.watching_id);
        }
    }

    Ok(())
}

fn parse_issue_id_or_key(input: &str) -> anyhow::Result<IssueIdOrKey> {
    use backlog_core::IssueKey;
    use backlog_core::identifier::IssueId;
    use std::str::FromStr;

    if let Ok(id) = input.parse::<u32>() {
        Ok(IssueIdOrKey::Id(IssueId::from(id)))
    } else {
        Ok(IssueIdOrKey::Key(IssueKey::from_str(input)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use backlog_core::identifier::Identifier;

    #[test]
    fn test_parse_issue_id_or_key_numeric_id() {
        let result = parse_issue_id_or_key("123").unwrap();
        match result {
            IssueIdOrKey::Id(id) => assert_eq!(id.value(), 123),
            IssueIdOrKey::Key(_) => panic!("Expected Id variant"),
        }
    }

    #[test]
    fn test_parse_issue_id_or_key_string_key() {
        let result = parse_issue_id_or_key("PROJECT-456").unwrap();
        match result {
            IssueIdOrKey::Key(key) => assert_eq!(key.to_string(), "PROJECT-456"),
            IssueIdOrKey::Id(_) => panic!("Expected Key variant"),
        }
    }

    #[test]
    fn test_parse_issue_id_or_key_invalid_format() {
        // Invalid key format (no hyphen with number)
        let result = parse_issue_id_or_key("invalid");
        assert!(result.is_err());
    }
}
