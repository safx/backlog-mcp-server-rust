use crate::commands::common::CliResult;
use backlog_api_client::client::BacklogApiClient;
use backlog_core::identifier::{Identifier, SharedFileId, WikiId};
use backlog_wiki::{
    GetWikiSharedFileListParams, LinkSharedFilesToWikiParams, UnlinkSharedFileFromWikiParams,
};

/// List shared files linked to a wiki page
pub(crate) async fn list_shared_files(client: &BacklogApiClient, wiki_id: u32) -> CliResult<()> {
    println!("Listing shared files for wiki ID: {wiki_id}");

    match client
        .wiki()
        .get_wiki_shared_file_list(GetWikiSharedFileListParams::new(WikiId::new(wiki_id)))
        .await
    {
        Ok(shared_files) => {
            if shared_files.is_empty() {
                println!("No shared files found linked to this wiki page");
            } else {
                println!("Found {} shared file(s):", shared_files.len());
                for shared_file in shared_files {
                    println!(
                        "[{}] {} ({} bytes)",
                        shared_file.id.value(),
                        shared_file.name,
                        match &shared_file.content {
                            backlog_api_client::FileContent::File { size } => *size,
                            backlog_api_client::FileContent::Directory => 0,
                        }
                    );
                    println!("  Path: {}", shared_file.dir);
                    println!(
                        "  Created by: {} at {}",
                        shared_file.created_user.name,
                        shared_file.created.format("%Y-%m-%d %H:%M:%S")
                    );
                    if let Some(updated_user) = &shared_file.updated_user
                        && let Some(updated) = &shared_file.updated
                    {
                        println!(
                            "  Updated by: {} at {}",
                            updated_user.name,
                            updated.format("%Y-%m-%d %H:%M:%S")
                        );
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to list wiki shared files: {e}");
            std::process::exit(1);
        }
    }

    Ok(())
}

/// Link shared files to a wiki page
#[cfg(feature = "wiki_writable")]
pub(crate) async fn link_shared_files(
    client: &BacklogApiClient,
    wiki_id: u32,
    file_ids: Vec<u32>,
) -> CliResult<()> {
    println!(
        "Linking {} shared file(s) to wiki ID: {}",
        file_ids.len(),
        wiki_id
    );

    let shared_file_ids: Vec<SharedFileId> =
        file_ids.iter().map(|&id| SharedFileId::new(id)).collect();

    let params = LinkSharedFilesToWikiParams::new(WikiId::new(wiki_id), shared_file_ids);

    match client.wiki().link_shared_files_to_wiki(params).await {
        Ok(shared_files) => {
            println!(
                "✅ Successfully linked {} shared file(s) to wiki",
                shared_files.len()
            );
            println!();

            for (index, file) in shared_files.iter().enumerate() {
                println!("{}. {}", index + 1, file.name);
                println!("   ID: {}", file.id.value());
                println!("   Directory: {}", file.dir);
                match &file.content {
                    backlog_api_client::FileContent::File { size } => {
                        println!("   Type: File");
                        println!("   Size: {size} bytes");
                    }
                    backlog_api_client::FileContent::Directory => {
                        println!("   Type: Directory");
                    }
                }
                println!("   Created by: {}", file.created_user.name);
                println!("   Created at: {}", file.created);
                if let Some(updated_user) = &file.updated_user {
                    println!("   Updated by: {}", updated_user.name);
                }
                if let Some(updated) = &file.updated {
                    println!("   Updated at: {updated}");
                }
                println!();
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to link shared files to wiki: {e}");
            std::process::exit(1);
        }
    }

    Ok(())
}

/// Unlink a shared file from a wiki page
#[cfg(feature = "wiki_writable")]
pub(crate) async fn unlink_shared_file(
    client: &BacklogApiClient,
    wiki_id: u32,
    file_id: u32,
) -> CliResult<()> {
    println!("Unlinking shared file {file_id} from wiki ID: {wiki_id}");

    let params =
        UnlinkSharedFileFromWikiParams::new(WikiId::new(wiki_id), SharedFileId::new(file_id));

    match client.wiki().unlink_shared_file_from_wiki(params).await {
        Ok(shared_file) => {
            println!("✅ Successfully unlinked shared file from wiki:");
            println!("   Name: {}", shared_file.name);
            println!("   ID: {}", shared_file.id.value());
            println!("   Directory: {}", shared_file.dir);
            match &shared_file.content {
                backlog_api_client::FileContent::File { size } => {
                    println!("   Type: File");
                    println!("   Size: {size} bytes");
                }
                backlog_api_client::FileContent::Directory => {
                    println!("   Type: Directory");
                }
            }
            println!("   Created by: {}", shared_file.created_user.name);
            println!("   Created at: {}", shared_file.created);
        }
        Err(e) => {
            eprintln!("❌ Failed to unlink shared file from wiki: {e}");
            std::process::exit(1);
        }
    }

    Ok(())
}
