//! Comment operations for issues
//!
//! This module provides handlers for comment management:
//! - Adding, updating, and deleting comments
//! - Getting comment details and notifications
//! - Managing comment notifications

use crate::commands::common::CliResult;
use anyhow::Context;
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

    let parsed_issue_id_or_key: IssueIdOrKey = args
        .issue_id_or_key
        .parse()
        .with_context(|| format!("Failed to parse issue_id_or_key '{}'", args.issue_id_or_key))?;

    let mut builder = AddCommentParamsBuilder::default();
    builder.issue_id_or_key(parsed_issue_id_or_key);
    builder.content(&args.content);

    // Parse notify_users if provided
    if let Some(notify_str) = &args.notify_users {
        let user_ids: Vec<UserId> = notify_str
            .split(',')
            .map(|s| {
                s.trim()
                    .parse::<u32>()
                    .map(UserId::new)
                    .with_context(|| format!("Invalid user ID in notify_users: '{}'", s.trim()))
            })
            .collect::<anyhow::Result<_>>()?;
        builder.notified_user_id(user_ids);
    }

    // Parse attachments if provided
    if let Some(attach_str) = &args.attachments {
        let attachment_ids: Vec<AttachmentId> = attach_str
            .split(',')
            .map(|s| {
                s.trim()
                    .parse::<u32>()
                    .map(AttachmentId::new)
                    .with_context(|| format!("Invalid attachment ID: '{}'", s.trim()))
            })
            .collect::<anyhow::Result<_>>()?;
        builder.attachment_id(attachment_ids);
    }

    let params = builder.build()?;

    let comment = client.issue().add_comment(params).await?;
    println!("Comment added successfully!");
    println!("Comment ID: {}", comment.id);
    println!("Created by: {}", comment.created_user.name);
    println!("Created at: {}", comment.created);
    if let Some(content) = &comment.content {
        println!("Content: {content}");
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

    let comment = client.issue().update_comment(params).await?;
    println!("✅ Comment updated successfully");
    println!("Comment ID: {}", comment.id);
    println!("Content: {}", comment.content.unwrap_or_default());
    println!("Updated: {}", comment.updated);
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

    let comment = client.issue().delete_comment(params).await?;
    println!("✅ Comment deleted successfully");
    println!("Deleted Comment ID: {}", comment.id);
    println!("Deleted Content: {}", comment.content.unwrap_or_default());
    println!("Originally Created: {}", comment.created);
    Ok(())
}

/// Count comments for an issue
///
/// Corresponds to `GET /api/v2/issues/:issueIdOrKey/comments/count`
pub async fn count_comment(client: &BacklogApiClient, issue_id_or_key: String) -> CliResult<()> {
    println!("Counting comments for issue: {issue_id_or_key}");

    let parsed_issue_id_or_key: IssueIdOrKey = issue_id_or_key
        .parse()
        .with_context(|| format!("Failed to parse issue_id_or_key '{issue_id_or_key}'"))?;

    let response = client
        .issue()
        .count_comment(CountCommentParams::new(parsed_issue_id_or_key))
        .await?;
    println!(
        "Comment count for issue {issue_id_or_key}: {}",
        response.count
    );
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

    let parsed_issue_id_or_key: IssueIdOrKey = issue_id_or_key
        .parse()
        .with_context(|| format!("Failed to parse issue_id_or_key '{issue_id_or_key}'"))?;

    let comment_id = CommentId::new(comment_id);

    let comment = client
        .issue()
        .get_comment(GetCommentParams::new(parsed_issue_id_or_key, comment_id))
        .await?;
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

    let parsed_issue_id_or_key: IssueIdOrKey = issue_id_or_key
        .parse()
        .with_context(|| format!("Failed to parse issue_id_or_key '{issue_id_or_key}'"))?;

    let comment_id = CommentId::new(comment_id);

    let notifications = client
        .issue()
        .get_comment_notifications(GetCommentNotificationsParams::new(
            parsed_issue_id_or_key,
            comment_id,
        ))
        .await?;
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

    let parsed_issue_id_or_key: IssueIdOrKey = issue_id_or_key
        .parse()
        .with_context(|| format!("Failed to parse issue_id_or_key '{issue_id_or_key}'"))?;

    let comment_id = CommentId::new(comment_id);

    // Parse user IDs from comma-separated string
    let user_ids: Vec<UserId> = users
        .split(',')
        .map(|id_str| {
            id_str
                .trim()
                .parse::<u32>()
                .map(UserId::new)
                .with_context(|| format!("Invalid user ID '{}'", id_str.trim()))
        })
        .collect::<anyhow::Result<_>>()?;

    let params = AddCommentNotificationParams::new(parsed_issue_id_or_key, comment_id, user_ids);

    let comment = client.issue().add_comment_notification(params).await?;
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
    Ok(())
}
