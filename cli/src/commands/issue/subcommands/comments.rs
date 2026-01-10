//! Comment operations for issues
//!
//! This module provides handlers for comment management:
//! - Adding, updating, and deleting comments
//! - Getting comment details and notifications
//! - Managing comment notifications

use crate::commands::common::CliResult;
use backlog_api_client::client::BacklogApiClient;
use backlog_api_client::{
    AddCommentParamsBuilder, AttachmentId, GetCommentNotificationsParams, IssueIdOrKey,
};
use backlog_core::IssueKey;
use backlog_core::identifier::{CommentId, Identifier, UserId};
use backlog_issue::{
    AddCommentNotificationParams, CountCommentParams, DeleteCommentParams, GetCommentParams,
    UpdateCommentParams,
};
use std::str::FromStr;

/// Add a comment to an issue
///
/// Corresponds to `POST /api/v2/issues/:issueIdOrKey/comments`
#[cfg(feature = "issue_writable")]
pub async fn add_comment(
    client: &BacklogApiClient,
    args: crate::commands::issue::args::AddCommentArgs,
) -> CliResult<()> {
    println!(
        "Adding comment to issue {}: {}",
        args.issue_id_or_key, args.content
    );

    let parsed_issue_id_or_key = IssueIdOrKey::from_str(&args.issue_id_or_key).map_err(|e| {
        format!(
            "Failed to parse issue_id_or_key '{}': {}",
            args.issue_id_or_key, e
        )
    })?;

    let mut builder = AddCommentParamsBuilder::default();
    builder.issue_id_or_key(parsed_issue_id_or_key);
    builder.content(&args.content);

    // Parse notify_users if provided
    if let Some(notify_str) = &args.notify_users {
        let user_ids: Result<Vec<UserId>, _> = notify_str
            .split(',')
            .map(|s| s.trim().parse::<u32>().map(UserId::new))
            .collect();
        match user_ids {
            Ok(ids) => builder.notified_user_id(ids),
            Err(e) => {
                eprintln!("Error parsing notify_users '{notify_str}': {e}");
                return Ok(());
            }
        };
    }

    // Parse attachments if provided
    if let Some(attach_str) = &args.attachments {
        let attachment_ids: Result<Vec<AttachmentId>, _> = attach_str
            .split(',')
            .map(|s| s.trim().parse::<u32>().map(AttachmentId::new))
            .collect();
        match attachment_ids {
            Ok(ids) => builder.attachment_id(ids),
            Err(e) => {
                eprintln!("Error parsing attachments '{attach_str}': {e}");
                return Ok(());
            }
        };
    }

    let params = builder.build()?;

    match client.issue().add_comment(params).await {
        Ok(comment) => {
            println!("Comment added successfully!");
            println!("Comment ID: {}", comment.id);
            println!("Created by: {}", comment.created_user.name);
            println!("Created at: {}", comment.created);
            if let Some(content) = &comment.content {
                println!("Content: {content}");
            }
        }
        Err(e) => {
            eprintln!("Error adding comment: {e}");
        }
    }
    Ok(())
}

/// Update an existing comment
///
/// Corresponds to `PATCH /api/v2/issues/:issueIdOrKey/comments/:commentId`
#[cfg(feature = "issue_writable")]
pub async fn update_comment(
    client: &BacklogApiClient,
    args: crate::commands::issue::args::UpdateCommentArgs,
) -> CliResult<()> {
    let params = UpdateCommentParams {
        issue_id_or_key: args.issue_id.parse::<IssueKey>()?.into(),
        comment_id: CommentId::new(args.comment_id),
        content: args.content,
    };

    match client.issue().update_comment(params).await {
        Ok(comment) => {
            println!("✅ Comment updated successfully");
            println!("Comment ID: {}", comment.id);
            println!("Content: {}", comment.content.unwrap_or_default());
            println!("Updated: {}", comment.updated);
        }
        Err(e) => {
            eprintln!("❌ Failed to update comment: {e}");
            std::process::exit(1);
        }
    }
    Ok(())
}

/// Delete a comment from an issue
///
/// Corresponds to `DELETE /api/v2/issues/:issueIdOrKey/comments/:commentId`
#[cfg(feature = "issue_writable")]
pub async fn delete_comment(
    client: &BacklogApiClient,
    args: crate::commands::issue::args::DeleteCommentArgs,
) -> CliResult<()> {
    let params = DeleteCommentParams {
        issue_id_or_key: args.issue_id.parse::<IssueKey>()?.into(),
        comment_id: CommentId::new(args.comment_id),
    };

    match client.issue().delete_comment(params).await {
        Ok(comment) => {
            println!("✅ Comment deleted successfully");
            println!("Deleted Comment ID: {}", comment.id);
            println!("Deleted Content: {}", comment.content.unwrap_or_default());
            println!("Originally Created: {}", comment.created);
        }
        Err(e) => {
            eprintln!("❌ Failed to delete comment: {e}");
            std::process::exit(1);
        }
    }
    Ok(())
}

/// Count comments for an issue
///
/// Corresponds to `GET /api/v2/issues/:issueIdOrKey/comments/count`
pub async fn count_comment(client: &BacklogApiClient, issue_id_or_key: String) -> CliResult<()> {
    println!("Counting comments for issue: {issue_id_or_key}");

    let parsed_issue_id_or_key = IssueIdOrKey::from_str(&issue_id_or_key)
        .map_err(|e| format!("Failed to parse issue_id_or_key '{issue_id_or_key}': {e}"))?;

    match client
        .issue()
        .count_comment(CountCommentParams::new(parsed_issue_id_or_key))
        .await
    {
        Ok(response) => {
            println!(
                "Comment count for issue {issue_id_or_key}: {}",
                response.count
            );
        }
        Err(e) => {
            eprintln!("Error counting comments: {e}");
        }
    }
    Ok(())
}

/// Get a specific comment for an issue
///
/// Corresponds to `GET /api/v2/issues/:issueIdOrKey/comments/:commentId`
pub async fn get_comment(
    client: &BacklogApiClient,
    issue_id_or_key: String,
    comment_id: u32,
) -> CliResult<()> {
    println!("Getting comment {comment_id} for issue: {issue_id_or_key}");

    let parsed_issue_id_or_key = IssueIdOrKey::from_str(&issue_id_or_key)
        .map_err(|e| format!("Failed to parse issue_id_or_key '{issue_id_or_key}': {e}"))?;

    let comment_id = CommentId::new(comment_id);

    match client
        .issue()
        .get_comment(GetCommentParams::new(parsed_issue_id_or_key, comment_id))
        .await
    {
        Ok(comment) => {
            println!("Comment ID: {}", comment.id);
            println!("Created by: {}", comment.created_user.name);
            println!("Created at: {}", comment.created);
            println!("Updated at: {}", comment.updated);
            if let Some(content) = &comment.content {
                println!("Content: {content}");
            } else {
                println!("Content: (empty)");
            }
            if !comment.change_log.is_empty() {
                println!("Change log entries: {}", comment.change_log.len());
            }
            if !comment.notifications.is_empty() {
                println!("Notifications: {}", comment.notifications.len());
            }
            if !comment.stars.is_empty() {
                println!("Stars: {}", comment.stars.len());
            }
        }
        Err(e) => {
            eprintln!("Error getting comment: {e}");
        }
    }
    Ok(())
}

/// Get notifications for a comment
///
/// Corresponds to `GET /api/v2/issues/:issueIdOrKey/comments/:commentId/notifications`
pub async fn get_comment_notifications(
    client: &BacklogApiClient,
    issue_id_or_key: String,
    comment_id: u32,
) -> CliResult<()> {
    println!("Getting notifications for comment {comment_id} in issue: {issue_id_or_key}");

    let parsed_issue_id_or_key = IssueIdOrKey::from_str(&issue_id_or_key)
        .map_err(|e| format!("Failed to parse issue_id_or_key '{issue_id_or_key}': {e}"))?;

    let comment_id = CommentId::new(comment_id);

    match client
        .issue()
        .get_comment_notifications(GetCommentNotificationsParams::new(
            parsed_issue_id_or_key,
            comment_id,
        ))
        .await
    {
        Ok(notifications) => {
            if notifications.is_empty() {
                println!("No notifications found for this comment.");
            } else {
                println!("Found {} notification(s):", notifications.len());
                for (i, notification) in notifications.iter().enumerate() {
                    println!("  {}. Notification ID: {}", i + 1, notification.id.value());
                    println!("     User: {}", notification.user.name);
                    if let Some(user_id) = &notification.user.user_id {
                        println!("     User ID: {user_id}");
                    }
                    println!("     Already Read: {}", notification.already_read);
                    println!(
                        "     Resource Already Read: {}",
                        notification.resource_already_read
                    );
                    println!("     Reason: {:?}", notification.reason);
                    println!();
                }
            }
        }
        Err(e) => {
            eprintln!("Error getting comment notifications: {e}");
        }
    }
    Ok(())
}

/// Add notifications to a comment
///
/// Corresponds to `POST /api/v2/issues/:issueIdOrKey/comments/:commentId/notifications`
#[cfg(feature = "issue_writable")]
pub async fn add_comment_notification(
    client: &BacklogApiClient,
    issue_id_or_key: String,
    comment_id: u32,
    users: String,
) -> CliResult<()> {
    println!("Adding notifications to comment {comment_id} in issue: {issue_id_or_key}");

    let parsed_issue_id_or_key = IssueIdOrKey::from_str(&issue_id_or_key)
        .map_err(|e| format!("Failed to parse issue_id_or_key '{issue_id_or_key}': {e}"))?;

    let comment_id = CommentId::new(comment_id);

    // Parse user IDs from comma-separated string
    let user_ids: Result<Vec<UserId>, _> = users
        .split(',')
        .map(|id_str| {
            id_str
                .trim()
                .parse::<u32>()
                .map(UserId::new)
                .map_err(|e| format!("Invalid user ID '{}': {}", id_str.trim(), e))
        })
        .collect();

    let user_ids = match user_ids {
        Ok(ids) => ids,
        Err(e) => {
            eprintln!("Error parsing user IDs: {e}");
            return Ok(());
        }
    };

    let params = AddCommentNotificationParams::new(parsed_issue_id_or_key, comment_id, user_ids);

    match client.issue().add_comment_notification(params).await {
        Ok(comment) => {
            println!("✅ Comment notifications added successfully!");
            println!("Comment ID: {}", comment.id.value());
            println!("Notifications count: {}", comment.notifications.len());
            if !comment.notifications.is_empty() {
                println!("Notified users:");
                for notification in &comment.notifications {
                    println!(
                        "  - {} (ID: {})",
                        notification.user.name,
                        notification.user.id.value()
                    );
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Error adding comment notifications: {e}");
        }
    }
    Ok(())
}
