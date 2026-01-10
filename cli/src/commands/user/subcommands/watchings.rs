use crate::commands::common::CliResult;
use backlog_api_client::client::BacklogApiClient;
use backlog_api_client::{UserId, WatchingOrder, WatchingSort};
use backlog_core::identifier::{Identifier, IssueId};
use backlog_user::{GetWatchingCountParams, GetWatchingListParams};

/// Get watching list
pub(crate) async fn watchings(
    client: &BacklogApiClient,
    user_id: u32,
    order: Option<String>,
    sort: Option<String>,
    count: Option<u8>,
    offset: Option<u64>,
    resource_already_read: Option<bool>,
    issue_ids: Option<String>,
) -> CliResult<()> {
    println!("Getting watchings for user {user_id}");

    let mut params = GetWatchingListParams::builder();

    if let Some(order_str) = order {
        let order_enum = match order_str.to_lowercase().as_str() {
            "asc" => WatchingOrder::Asc,
            "desc" => WatchingOrder::Desc,
            _ => {
                eprintln!("Invalid order: {order_str}. Use 'asc' or 'desc'");
                std::process::exit(1);
            }
        };
        params = params.order(order_enum);
    }

    if let Some(sort_str) = sort {
        let sort_enum = match sort_str.to_lowercase().as_str() {
            "created" => WatchingSort::Created,
            "updated" => WatchingSort::Updated,
            "issueupdated" => WatchingSort::IssueUpdated,
            _ => {
                eprintln!("Invalid sort: {sort_str}. Use 'created', 'updated', or 'issueUpdated'");
                std::process::exit(1);
            }
        };
        params = params.sort(sort_enum);
    }

    if let Some(c) = count {
        params = params.count(c);
    }

    if let Some(o) = offset {
        params = params.offset(o);
    }

    if let Some(read) = resource_already_read {
        params = params.resource_already_read(read);
    }

    if let Some(ids_str) = issue_ids {
        let ids: Vec<IssueId> = ids_str
            .split(',')
            .filter_map(|s| s.trim().parse::<u32>().ok())
            .map(IssueId::from)
            .collect();
        if !ids.is_empty() {
            params = params.issue_ids(ids);
        }
    }

    let params = params.build()?;

    match client.user().get_watching_list(user_id, params).await {
        Ok(watchings) => {
            if watchings.is_empty() {
                println!("No watchings found");
            } else {
                println!("Found {} watching(s):", watchings.len());
                println!();

                for (index, watching) in watchings.iter().enumerate() {
                    println!("{}. Watching #{}", index + 1, watching.id.value());
                    println!("   Type: {:?}", watching.watching_type);
                    println!(
                        "   Status: {}",
                        if watching.resource_already_read {
                            "Read"
                        } else {
                            "Unread"
                        }
                    );

                    if let Some(note) = &watching.note {
                        println!("   Note: {note}");
                    }

                    if let Some(issue) = &watching.issue {
                        println!("   Issue: {} - {}", issue.issue_key, issue.summary);
                        println!("   Project ID: {}", issue.project_id.value());
                        println!("   Status: {}", issue.status.name);
                        if let Some(assignee) = &issue.assignee {
                            println!("   Assignee: {}", assignee.name);
                        }
                    }

                    if let Some(last_updated) = &watching.last_content_updated {
                        println!(
                            "   Last Updated: {}",
                            last_updated.format("%Y-%m-%d %H:%M:%S")
                        );
                    }

                    println!(
                        "   Created: {}",
                        watching.created.format("%Y-%m-%d %H:%M:%S")
                    );
                    println!();
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to get watchings: {e}");
            std::process::exit(1);
        }
    }

    Ok(())
}

/// Get watching count
pub(crate) async fn watching_count(
    client: &BacklogApiClient,
    user_id: u32,
    resource_already_read: Option<bool>,
    already_read: Option<bool>,
) -> CliResult<()> {
    println!("Getting watching count for user {user_id}");

    let mut params = GetWatchingCountParams::new(UserId::from(user_id));

    if let Some(read) = resource_already_read {
        params = params.with_resource_already_read(read);
    }

    if let Some(read) = already_read {
        params = params.with_already_read(read);
    }

    match client.user().get_watching_count(params).await {
        Ok(response) => {
            println!("✅ Watching count retrieved successfully");
            println!("Total watchings: {}", response.count);

            if resource_already_read.is_some() || already_read.is_some() {
                println!("\nFilters applied:");
                if let Some(read) = resource_already_read {
                    println!("  Resource already read: {read}");
                }
                if let Some(read) = already_read {
                    println!("  Already read: {read}");
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to get watching count: {e}");
            std::process::exit(1);
        }
    }

    Ok(())
}
