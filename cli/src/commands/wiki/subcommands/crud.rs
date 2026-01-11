use crate::commands::common::CliResult;
use backlog_api_client::client::BacklogApiClient;
use backlog_core::identifier::{Identifier, ProjectId, WikiId};
use backlog_wiki::{AddWikiParams, DeleteWikiParams, UpdateWikiParams};
use std::str::FromStr;

/// Create a new wiki page
#[cfg(feature = "wiki_writable")]
pub(crate) async fn create(
    client: &BacklogApiClient,
    project_id: String,
    name: String,
    content: String,
    mail_notify: Option<bool>,
) -> CliResult<()> {
    println!("Creating new wiki page in project: {project_id}");

    let params = AddWikiParams::new(ProjectId::from_str(&project_id)?, name, content);

    let params = if let Some(mail_notify) = mail_notify {
        params.mail_notify(mail_notify)
    } else {
        params
    };

    let wiki_detail = client.wiki().add_wiki(params).await?;
    println!("✅ Wiki page created successfully");
    println!("   ID: {}", wiki_detail.id.value());
    println!("   Name: {}", wiki_detail.name);
    println!("   Project ID: {}", wiki_detail.project_id.value());
    println!(
        "   Created by: {} at {}",
        wiki_detail.created_user.name,
        wiki_detail.created.format("%Y-%m-%d %H:%M:%S")
    );

    Ok(())
}

/// Update an existing wiki page
#[cfg(feature = "wiki_writable")]
pub(crate) async fn update(
    client: &BacklogApiClient,
    wiki_id: u32,
    name: Option<String>,
    content: Option<String>,
    mail_notify: Option<bool>,
) -> CliResult<()> {
    println!("Updating wiki ID: {wiki_id}");

    // Create params with provided options
    let mut params = UpdateWikiParams::new(WikiId::new(wiki_id));

    if let Some(name) = name {
        params = params.name(name);
    }

    if let Some(content) = content {
        params = params.content(content);
    }

    if let Some(mail_notify) = mail_notify {
        params = params.mail_notify(mail_notify);
    }

    let wiki_detail = client.wiki().update_wiki(params).await?;
    println!("✅ Wiki updated successfully");
    println!("ID: {}", wiki_detail.id.value());
    println!("Name: {}", wiki_detail.name);
    println!("Project ID: {}", wiki_detail.project_id.value());
    println!("Updated by: {}", wiki_detail.updated_user.name);
    println!(
        "Updated at: {}",
        wiki_detail.updated.format("%Y-%m-%d %H:%M:%S")
    );

    if !wiki_detail.tags.is_empty() {
        let tag_names: Vec<String> = wiki_detail
            .tags
            .iter()
            .map(|tag| tag.name.clone())
            .collect();
        println!("Tags: {}", tag_names.join(", "));
    }

    Ok(())
}

/// Delete a wiki page
#[cfg(feature = "wiki_writable")]
pub(crate) async fn delete(
    client: &BacklogApiClient,
    wiki_id: u32,
    mail_notify: Option<bool>,
) -> CliResult<()> {
    println!("Deleting wiki ID: {wiki_id}");

    let mut params = DeleteWikiParams::new(WikiId::new(wiki_id));

    if let Some(mail_notify) = mail_notify {
        params = params.mail_notify(mail_notify);
    }

    let wiki_detail = client.wiki().delete_wiki(params).await?;
    println!("✅ Wiki deleted successfully");
    println!("   ID: {}", wiki_detail.id.value());
    println!("   Name: {}", wiki_detail.name);
    println!("   Project ID: {}", wiki_detail.project_id.value());
    println!(
        "   Created by: {} at {}",
        wiki_detail.created_user.name,
        wiki_detail.created.format("%Y-%m-%d %H:%M:%S")
    );
    println!(
        "   Last updated by: {} at {}",
        wiki_detail.updated_user.name,
        wiki_detail.updated.format("%Y-%m-%d %H:%M:%S")
    );

    Ok(())
}
