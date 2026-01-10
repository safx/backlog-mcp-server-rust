use crate::commands::common::{CliResult, date_to_end_of_day, date_to_start_of_day};
use backlog_api_client::client::BacklogApiClient;
use backlog_core::ApiDate;
use backlog_user::api::StarOrder;
use backlog_user::{GetUserStarCountParams, GetUserStarsParams};
use chrono::NaiveDate;

/// Get star count for a user
pub(crate) async fn star_count(
    client: &BacklogApiClient,
    user_id: u32,
    since: Option<String>,
    until: Option<String>,
) -> CliResult<()> {
    println!("Getting star count for user ID: {user_id}");

    let mut params = GetUserStarCountParams::new(user_id);

    if let Some(since_str) = since {
        match NaiveDate::parse_from_str(&since_str, "%Y-%m-%d") {
            Ok(date) => {
                let datetime = date_to_start_of_day(date);
                params = params.with_since(ApiDate::from(datetime));
                println!("Counting stars from: {since_str}");
            }
            Err(_) => {
                eprintln!(
                    "Invalid date format for 'since': {since_str}. Expected format: YYYY-MM-DD"
                );
                return Ok(());
            }
        }
    }

    if let Some(until_str) = until {
        match NaiveDate::parse_from_str(&until_str, "%Y-%m-%d") {
            Ok(date) => {
                let datetime = date_to_end_of_day(date);
                params = params.with_until(ApiDate::from(datetime));
                println!("Counting stars until: {until_str}");
            }
            Err(_) => {
                eprintln!(
                    "Invalid date format for 'until': {until_str}. Expected format: YYYY-MM-DD"
                );
                return Ok(());
            }
        }
    }

    match client.user().get_user_star_count(params).await {
        Ok(star_count) => {
            println!("User has received {} star(s)", star_count.count);
        }
        Err(e) => {
            eprintln!("Error getting star count: {e}");
        }
    }

    Ok(())
}

/// Get stars for a user
pub(crate) async fn stars(
    client: &BacklogApiClient,
    user_id: u32,
    min_id: Option<u64>,
    max_id: Option<u64>,
    count: Option<u32>,
    order: Option<String>,
) -> CliResult<()> {
    println!("Getting stars for user ID: {user_id}");

    let mut params = GetUserStarsParams::new(user_id);

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
        let order_enum = match order_str.to_lowercase().as_str() {
            "asc" => StarOrder::Asc,
            "desc" => StarOrder::Desc,
            _ => {
                eprintln!("Invalid order: '{order_str}'. Must be 'asc' or 'desc'");
                return Ok(());
            }
        };
        params = params.with_order(order_enum);
    }

    match client.user().get_user_stars(params).await {
        Ok(stars) => {
            if stars.is_empty() {
                println!("No stars found for this user");
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
            eprintln!("Error getting stars: {e}");
        }
    }

    Ok(())
}
