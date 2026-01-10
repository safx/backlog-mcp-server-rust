use crate::commands::common::CliResult;
use backlog_api_client::UserId;
use backlog_api_client::client::BacklogApiClient;
use backlog_core::identifier::Identifier;
use backlog_user::{GetNotificationCountParams, GetNotificationsParams, NotificationOrder};

/// Get notification count
pub(crate) async fn notification_count(
    client: &BacklogApiClient,
    already_read: Option<bool>,
    resource_already_read: Option<bool>,
) -> CliResult<()> {
    println!("Getting notification count for authenticated user");

    let mut params = GetNotificationCountParams::new();

    if let Some(already_read) = already_read {
        params = params.with_already_read(already_read);
    }

    if let Some(resource_already_read) = resource_already_read {
        params = params.with_resource_already_read(resource_already_read);
    }

    match client.user().get_notification_count(params).await {
        Ok(notification_count) => {
            println!("✅ Notification count: {}", notification_count.count);
        }
        Err(e) => {
            eprintln!("❌ Failed to get notification count: {e}");
            std::process::exit(1);
        }
    }

    Ok(())
}

/// Get notifications
pub(crate) async fn notifications(
    client: &BacklogApiClient,
    min_id: Option<u64>,
    max_id: Option<u64>,
    count: Option<u8>,
    order: Option<String>,
    sender_id: Option<u32>,
) -> CliResult<()> {
    println!("Getting notifications for authenticated user");

    let mut params = GetNotificationsParams::new();

    if let Some(min_id) = min_id {
        params = params.with_min_id(min_id);
    }

    if let Some(max_id) = max_id {
        params = params.with_max_id(max_id);
    }

    if let Some(count) = count {
        params = params.with_count(count);
    }

    if let Some(order_str) = order {
        let notification_order = match order_str.to_lowercase().as_str() {
            "asc" => NotificationOrder::Asc,
            "desc" => NotificationOrder::Desc,
            _ => {
                eprintln!("❌ Invalid order. Use 'asc' or 'desc'");
                std::process::exit(1);
            }
        };
        params = params.with_order(notification_order);
    }

    if let Some(sender_id) = sender_id {
        params = params.with_sender_id(UserId::new(sender_id));
    }

    match client.user().get_notifications(params).await {
        Ok(notifications) => {
            if notifications.is_empty() {
                println!("No notifications found");
            } else {
                println!("Found {} notification(s):", notifications.len());
                println!();

                for (index, notification) in notifications.iter().enumerate() {
                    println!("{}. Notification #{}", index + 1, notification.id.value());
                    println!(
                        "   Status: {}",
                        if notification.already_read {
                            "Read"
                        } else {
                            "Unread"
                        }
                    );
                    println!("   Reason: {:?}", notification.reason);
                    println!(
                        "   Project: {} ({})",
                        notification.project.name, notification.project.project_key
                    );
                    println!(
                        "   From: {} ({})",
                        notification.sender.name, notification.sender.mail_address
                    );

                    if let Some(issue) = &notification.issue {
                        println!("   Issue: {} - {}", issue.issue_key, issue.summary);
                    }

                    if let Some(comment) = &notification.comment
                        && let Some(content) = &comment.content
                    {
                        let preview = content.chars().take(100).collect::<String>();
                        println!("   Comment: {preview}");
                    }

                    println!(
                        "   Created: {}",
                        notification.created.format("%Y-%m-%d %H:%M:%S")
                    );
                    println!();
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to get notifications: {e}");
            std::process::exit(1);
        }
    }

    Ok(())
}

/// Mark notification as read
#[cfg(feature = "user_writable")]
pub(crate) async fn mark_notification_read(
    client: &BacklogApiClient,
    notification_id: u32,
) -> CliResult<()> {
    println!("Marking notification {notification_id} as read");

    match client
        .user()
        .mark_notification_as_read(notification_id)
        .await
    {
        Ok(()) => {
            println!("✅ Notification marked as read");
        }
        Err(e) => {
            eprintln!("❌ Failed to mark notification as read: {e}");
            std::process::exit(1);
        }
    }

    Ok(())
}

/// Reset unread notifications
#[cfg(feature = "user_writable")]
pub(crate) async fn reset_notifications(client: &BacklogApiClient) -> CliResult<()> {
    println!("Marking all unread notifications as read...");

    match client.user().reset_unread_notification_count().await {
        Ok(result) => {
            println!("✅ All unread notifications marked as read");
            println!("   Previously unread count: {}", result.count);
        }
        Err(e) => {
            eprintln!("❌ Failed to reset notifications: {e}");
            std::process::exit(1);
        }
    }

    Ok(())
}
