use crate::commands::common::CliResult;
use backlog_api_client::client::BacklogApiClient;
use backlog_user::{GetOwnUserParams, GetUserIconParams, GetUserListParams, GetUserParams};
use std::path::PathBuf;
use tokio::fs;

/// List all users
pub(crate) async fn list(client: &BacklogApiClient) -> CliResult<()> {
    println!("Listing all users:");

    match client.user().get_user_list(GetUserListParams::new()).await {
        Ok(users) => {
            if users.is_empty() {
                println!("No users found");
            } else {
                for user in users {
                    let user_id_str = user.user_id.as_deref().unwrap_or("N/A");
                    println!("[{}] {} ({})", user.id, user.name, user_id_str);
                    if !user.mail_address.is_empty() {
                        println!("  Email: {}", user.mail_address);
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error listing users: {e}");
        }
    }

    Ok(())
}

/// Get current user info
pub(crate) async fn me(client: &BacklogApiClient) -> CliResult<()> {
    println!("Getting current user info:");

    match client.user().get_own_user(GetOwnUserParams::new()).await {
        Ok(user) => {
            println!("User ID: {}", user.id);
            if let Some(login_id) = &user.user_id {
                println!("Login ID: {login_id}");
            }
            println!("Name: {}", user.name);
            if !user.mail_address.is_empty() {
                println!("Email: {}", user.mail_address);
            }
            if let Some(lang) = &user.lang {
                println!("Language: {lang}");
            }
            if let Some(last_login) = &user.last_login_time {
                println!("Last Login: {last_login}");
            }
        }
        Err(e) => {
            eprintln!("Error getting user info: {e}");
        }
    }

    Ok(())
}

/// Show user info by user ID
pub(crate) async fn show(client: &BacklogApiClient, user_id: u32) -> CliResult<()> {
    println!("Getting user info for user ID: {user_id}");

    let user = client.user().get_user(GetUserParams::new(user_id)).await?;
    println!("✅ User found");
    println!("ID: {}", user.id);
    if let Some(login_id) = &user.user_id {
        println!("Login ID: {login_id}");
    }
    println!("Name: {}", user.name);
    println!("Role: {}", user.role_type);
    if !user.mail_address.is_empty() {
        println!("Email: {}", user.mail_address);
    }
    if let Some(lang) = &user.lang {
        println!("Language: {lang}");
    }
    if let Some(last_login) = &user.last_login_time {
        println!("Last Login: {last_login}");
    }

    Ok(())
}

/// Download user icon
pub(crate) async fn icon(
    client: &BacklogApiClient,
    user_id: u32,
    output: PathBuf,
) -> CliResult<()> {
    println!("Downloading user icon to {}", output.display());

    match client
        .user()
        .get_user_icon(GetUserIconParams::new(user_id))
        .await
    {
        Ok(file) => {
            let icon_bytes = file.bytes;
            if let Err(e) = fs::write(&output, &icon_bytes).await {
                eprintln!("Error writing icon to {}: {}", output.display(), e);
            } else {
                println!("User icon downloaded successfully to: {}", output.display());
            }
        }
        Err(e) => {
            eprintln!("Error downloading user icon: {e}");
        }
    }

    Ok(())
}
