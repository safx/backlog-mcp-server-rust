use crate::commands::common::CliResult;
use backlog_api_client::client::BacklogApiClient;
use backlog_core::ProjectIdOrKey;
use backlog_core::identifier::{DocumentAttachmentId, DocumentId, Identifier, ProjectId};
use backlog_document::{
    DocumentOrder, DocumentSortKey, DownloadAttachmentParams, GetDocumentParams,
    GetDocumentTreeParamsBuilder, ListDocumentsParamsBuilder,
};
#[cfg(feature = "document_writable")]
use backlog_document::{AddDocumentParams, DeleteDocumentParams};
use std::str::FromStr;

/// Parameters for list command
pub(crate) struct ListOptions {
    pub project_id: String,
    pub keyword: Option<String>,
    pub sort: Option<String>,
    pub order: Option<String>,
    pub offset: Option<u32>,
    pub count: Option<u32>,
    pub json: bool,
}

/// Parameters for add command
#[cfg(feature = "document_writable")]
pub(crate) struct AddOptions {
    pub project_id: String,
    pub title: String,
    pub content: Option<String>,
    pub emoji: Option<String>,
    pub parent_id: Option<String>,
    pub add_last: bool,
    pub json: bool,
}

/// List documents in a project
pub(crate) async fn list(
    client: &BacklogApiClient,
    options: ListOptions,
) -> CliResult<()> {
    let ListOptions {
        project_id,
        keyword,
        sort,
        order,
        offset,
        count,
        json,
    } = options;
    if !json {
        println!("Listing documents in project: {project_id}");
    }

    let mut params_builder = ListDocumentsParamsBuilder::default();

    // Parse project_id
    let project_id_or_key = project_id.parse::<ProjectIdOrKey>()?;
    let project_id_value: backlog_core::identifier::ProjectId = match project_id_or_key {
        ProjectIdOrKey::Id(id) => id,
        ProjectIdOrKey::Key(key) => {
            return Err(format!("Project key '{}' is not supported for list command. Please use numeric project ID.", key).into());
        }
        ProjectIdOrKey::EitherIdOrKey(id, _) => id,
    };
    params_builder.project_ids(vec![project_id_value]);

    if let Some(keyword) = keyword {
        params_builder.keyword(keyword);
    }

    if let Some(sort_str) = sort {
        let sort_key = match sort_str.as_str() {
            "created" => DocumentSortKey::Created,
            "updated" => DocumentSortKey::Updated,
            _ => return Err(format!("Invalid sort key: {}. Valid options are: created, updated", sort_str).into()),
        };
        params_builder.sort(sort_key);
    }

    if let Some(order_str) = order {
        let order_val = match order_str.as_str() {
            "asc" => DocumentOrder::Asc,
            "desc" => DocumentOrder::Desc,
            _ => return Err(format!("Invalid order: {}", order_str).into()),
        };
        params_builder.order(order_val);
    }

    if let Some(offset) = offset {
        params_builder.offset(offset);
    }

    if let Some(count) = count {
        params_builder.count(count);
    }

    let params = params_builder.build()?;
    let documents = client.document().list_documents(params).await?;

    if json {
        println!("{}", serde_json::to_string_pretty(&documents)?);
    } else if documents.is_empty() {
        println!("No documents found");
    } else {
        println!("\nDocuments ({} total):", documents.len());
        for doc in documents {
            let emoji = doc.emoji.as_deref().unwrap_or("📄");
            let id_short = &doc.id.to_string()[..8];
            println!("\n{} {}... \"{}\"", emoji, id_short, doc.title);
            println!("  Updated: {}", doc.updated.format("%Y-%m-%d %H:%M:%S"));
            println!("  Updated by: {}", doc.updated_user.name);
            if !doc.tags.is_empty() {
                let tag_names: Vec<String> = doc.tags.iter().map(|t| t.name.clone()).collect();
                println!("  Tags: {}", tag_names.join(", "));
            }
        }
    }

    Ok(())
}

/// Get document details
pub(crate) async fn get(
    client: &BacklogApiClient,
    document_id: String,
    json: bool,
) -> CliResult<()> {
    if !json {
        println!("Getting document: {document_id}");
    }

    let doc_id = DocumentId::from_str(&document_id)?;
    let params = GetDocumentParams::new(doc_id);
    let doc = client.document().get_document(params).await?;

    if json {
        println!("{}", serde_json::to_string_pretty(&doc)?);
    } else {
        let emoji = doc.emoji.as_deref().unwrap_or("📄");
        println!("\n{} {}", emoji, doc.title);
        println!("ID: {}", doc.id);
        println!("Project ID: {}", doc.project_id.value());
        println!("Status ID: {}", doc.status_id);
        println!("\nPlain text content:");
        println!("{}", doc.plain);
        println!("\nCreated: {} by {}",
            doc.created.format("%Y-%m-%d %H:%M:%S"),
            doc.created_user.name);
        println!("Updated: {} by {}",
            doc.updated.format("%Y-%m-%d %H:%M:%S"),
            doc.updated_user.name);

        if !doc.attachments.is_empty() {
            println!("\nAttachments ({}):", doc.attachments.len());
            for att in &doc.attachments {
                println!("  [{}] {} ({} bytes)",
                    att.id.value(),
                    att.name,
                    att.size);
            }
        }

        if !doc.tags.is_empty() {
            let tag_names: Vec<String> = doc.tags.iter().map(|t| t.name.clone()).collect();
            println!("\nTags: {}", tag_names.join(", "));
        }
    }

    Ok(())
}

/// Get document tree structure
pub(crate) async fn tree(
    client: &BacklogApiClient,
    project_id: String,
    json: bool,
) -> CliResult<()> {
    if !json {
        println!("Getting document tree for project: {project_id}");
    }

    let project_id_or_key = project_id.parse::<ProjectIdOrKey>()?;
    let params = GetDocumentTreeParamsBuilder::default()
        .project_id_or_key(project_id_or_key)
        .build()?;
    let tree = client.document().get_document_tree(params).await?;

    if json {
        println!("{}", serde_json::to_string_pretty(&tree)?);
    } else {
        println!("\nDocument Tree (Active):");
        print_tree_node(&tree.active_tree.children, 0);
        if !tree.trash_tree.children.is_empty() {
            println!("\nDocument Tree (Trash):");
            print_tree_node(&tree.trash_tree.children, 0);
        }
    }

    Ok(())
}

fn print_tree_node(nodes: &[backlog_document::DocumentTreeNode], depth: usize) {
    let indent = "  ".repeat(depth);
    for node in nodes {
        let emoji = node.emoji.as_deref().unwrap_or("📄");
        println!("{}{} {} (ID: {})", indent, emoji, node.name, node.id);
        if !node.children.is_empty() {
            print_tree_node(&node.children, depth + 1);
        }
    }
}

/// Download attachment from a document
pub(crate) async fn download(
    client: &BacklogApiClient,
    document_id: String,
    attachment_id: u32,
    output: Option<String>,
) -> CliResult<()> {
    println!("Downloading attachment {} from document {}", attachment_id, document_id);

    let doc_id = DocumentId::from_str(&document_id)?;
    let att_id = DocumentAttachmentId::new(attachment_id);
    let params = DownloadAttachmentParams::new(doc_id, att_id);

    let downloaded_file = client.document().download_attachment(params).await?;

    let output_path = output.unwrap_or_else(|| downloaded_file.filename.clone());

    std::fs::write(&output_path, &downloaded_file.bytes)?;
    println!("✅ Downloaded to: {}", output_path);
    println!("   Size: {} bytes", downloaded_file.bytes.len());
    println!("   Content-Type: {}", downloaded_file.content_type);

    Ok(())
}

/// Create a new document
#[cfg(feature = "document_writable")]
pub(crate) async fn add(client: &BacklogApiClient, options: AddOptions) -> CliResult<()> {
    let AddOptions {
        project_id,
        title,
        content,
        emoji,
        parent_id,
        add_last,
        json,
    } = options;

    if !json {
        println!("Creating document in project: {project_id}");
    }

    // Parse project_id (numeric only)
    let project_id_value = ProjectId::from_str(&project_id)?;

    // Build params
    let mut params = AddDocumentParams::new(project_id_value).title(title);

    if let Some(content) = content {
        params = params.content(content);
    }
    if let Some(emoji) = emoji {
        params = params.emoji(emoji);
    }
    if let Some(parent_id_str) = parent_id {
        let parent_doc_id = DocumentId::from_str(&parent_id_str)?;
        params = params.parent_id(parent_doc_id);
    }
    if add_last {
        params = params.add_last(true);
    }

    // Execute API call
    let doc = client.document().add_document(params).await?;

    // Output
    if json {
        println!("{}", serde_json::to_string_pretty(&doc)?);
    } else {
        let emoji_str = doc.emoji.as_deref().unwrap_or("📄");
        let id_short = &doc.id.to_string()[..8];
        println!("✅ Document created successfully");
        println!("   {} {}... \"{}\"", emoji_str, id_short, doc.title);
        println!("   Project ID: {}", doc.project_id.value());
        println!(
            "   Created by user ID: {} at {}",
            doc.created_user_id,
            doc.created.format("%Y-%m-%d %H:%M:%S")
        );
    }

    Ok(())
}

/// Delete a document
#[cfg(feature = "document_writable")]
pub(crate) async fn delete(
    client: &BacklogApiClient,
    document_id: String,
    json: bool,
) -> CliResult<()> {
    if !json {
        println!("Deleting document: {document_id}");
    }

    let doc_id = DocumentId::from_str(&document_id)?;
    let params = DeleteDocumentParams::new(doc_id);

    let doc = client.document().delete_document(params).await?;

    if json {
        println!("{}", serde_json::to_string_pretty(&doc)?);
    } else {
        let emoji_str = doc.emoji.as_deref().unwrap_or("📄");
        let id_short = &doc.id.to_string()[..8];
        println!("✅ Document deleted successfully");
        println!("   {} {}... \"{}\"", emoji_str, id_short, doc.title);
        println!(
            "   Created by user ID: {} at {}",
            doc.created_user_id,
            doc.created.format("%Y-%m-%d %H:%M:%S")
        );
    }

    Ok(())
}
