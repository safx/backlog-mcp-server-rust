use crate::commands::common::{CliResult, truncate_text};
use crate::commands::wiki::args::HistoryOrderCli;
use backlog_api_client::client::BacklogApiClient;
use backlog_core::ProjectIdOrKey;
use backlog_core::identifier::{Identifier, WikiId};
use backlog_wiki::{
    GetRecentlyViewedWikisParamsBuilder, GetWikiHistoryParams, GetWikiStarsParams,
    GetWikiTagListParams,
};

/// Get recently viewed wikis
pub(crate) async fn recently_viewed(
    client: &BacklogApiClient,
    order: Option<String>,
    count: Option<u32>,
    offset: Option<u32>,
) -> CliResult<()> {
    println!("Getting recently viewed wikis");

    let mut params_builder = GetRecentlyViewedWikisParamsBuilder::default();

    if let Some(order) = order {
        params_builder.order(order);
    }
    if let Some(count) = count {
        params_builder.count(count);
    }
    if let Some(offset) = offset {
        params_builder.offset(offset);
    }

    let params = params_builder.build()?;

    match client.wiki().get_recently_viewed_wikis(params).await {
        Ok(wikis) => {
            if wikis.is_empty() {
                println!("No recently viewed wikis found");
            } else {
                println!("Recently viewed wikis ({} total):", wikis.len());
                for wiki in wikis {
                    println!("\n[{}] {}", wiki.id.value(), wiki.name);
                    println!("  Project ID: {}", wiki.project_id.value());
                    if !wiki.tags.is_empty() {
                        let tag_names: Vec<String> =
                            wiki.tags.iter().map(|t| t.name.clone()).collect();
                        println!("  Tags: {}", tag_names.join(", "));
                    }
                    println!(
                        "  Created by: {} at {}",
                        wiki.created_user.name,
                        wiki.created.format("%Y-%m-%d %H:%M:%S")
                    );
                    println!(
                        "  Updated by: {} at {}",
                        wiki.updated_user.name,
                        wiki.updated.format("%Y-%m-%d %H:%M:%S")
                    );
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to get recently viewed wikis: {e}");
            std::process::exit(1);
        }
    }

    Ok(())
}

/// List tags used in wiki pages for a project
pub(crate) async fn list_tags(client: &BacklogApiClient, project_id: String) -> CliResult<()> {
    println!("Listing tags used in wiki pages for project: {project_id}");

    let params = GetWikiTagListParams::new(project_id.parse::<ProjectIdOrKey>()?);

    match client.wiki().get_wiki_tag_list(params).await {
        Ok(tags) => {
            if tags.is_empty() {
                println!("No tags found in the project");
            } else {
                println!("Wiki Tags ({} total):", tags.len());
                for tag in tags {
                    println!("  {} (ID: {})", tag.name, tag.id.value());
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to get wiki tags: {e}");
            std::process::exit(1);
        }
    }

    Ok(())
}

/// Get stars for a wiki page
pub(crate) async fn stars(client: &BacklogApiClient, wiki_id: u32) -> CliResult<()> {
    println!("Getting stars for wiki ID: {wiki_id}");

    match client
        .wiki()
        .get_wiki_stars(GetWikiStarsParams::new(WikiId::new(wiki_id)))
        .await
    {
        Ok(stars) => {
            if stars.is_empty() {
                println!("No stars found for this wiki page");
            } else {
                println!("Found {} star(s):", stars.len());
                println!();
                for star in stars {
                    println!("Star ID: {}", star.id);
                    println!("Title: {}", star.title);
                    println!("URL: {}", star.url);
                    if let Some(comment) = &star.comment {
                        println!("Comment: {comment}");
                    }
                    println!(
                        "Presenter: {} (ID: {})",
                        star.presenter.name, star.presenter.id
                    );
                    println!("Created: {}", star.created.format("%Y-%m-%d %H:%M:%S UTC"));
                    println!("---");
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to get wiki stars: {e}");
            std::process::exit(1);
        }
    }

    Ok(())
}

/// Get wiki history
pub(crate) async fn history(
    client: &BacklogApiClient,
    wiki_id: u32,
    min_id: Option<u32>,
    max_id: Option<u32>,
    count: Option<u32>,
    order: Option<HistoryOrderCli>,
) -> CliResult<()> {
    println!("Getting history for wiki ID: {wiki_id}");

    let mut params = GetWikiHistoryParams::new(WikiId::new(wiki_id));

    if let Some(min_id) = min_id {
        params = params.min_id(min_id);
    }
    if let Some(max_id) = max_id {
        params = params.max_id(max_id);
    }
    if let Some(count) = count {
        params = params.count(count);
    }
    if let Some(order) = order {
        let order = match order {
            HistoryOrderCli::Asc => backlog_wiki::HistoryOrder::Asc,
            HistoryOrderCli::Desc => backlog_wiki::HistoryOrder::Desc,
        };
        params = params.order(order);
    }

    match client.wiki().get_wiki_history(params).await {
        Ok(history) => {
            if history.is_empty() {
                println!("No history found for wiki {wiki_id}");
            } else {
                println!("Wiki {wiki_id} History ({} entries):", history.len());
                for entry in &history {
                    println!(
                        "Version {}: {} (by {} at {})",
                        entry.version,
                        entry.name,
                        entry.created_user.name,
                        entry.created.format("%Y-%m-%d %H:%M:%S")
                    );
                    if !entry.content.is_empty() {
                        let preview = truncate_text(&entry.content, 100);
                        println!("  Content: {preview}");
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to get wiki history: {e}");
            std::process::exit(1);
        }
    }

    Ok(())
}
