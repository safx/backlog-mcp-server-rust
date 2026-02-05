use anyhow::Result;
use backlog_api_client::client::BacklogApiClient;
use chrono::{Local, TimeZone, Utc};
use clap::Subcommand;
use std::env;

#[derive(Subcommand)]
pub enum RateLimitCommand {
    /// Get current rate limit information
    Get,
}

pub async fn handle_rate_limit_command(cmd: RateLimitCommand) -> Result<()> {
    match cmd {
        RateLimitCommand::Get => get_rate_limit().await,
    }
}

async fn get_rate_limit() -> Result<()> {
    let base_url = env::var("BACKLOG_BASE_URL")?;
    let api_key = env::var("BACKLOG_API_KEY")?;
    let client = BacklogApiClient::new(&base_url)?.with_api_key(api_key);
    let response = client.rate_limit().get_rate_limit().await?;

    println!("Rate Limit Information:");
    println!("======================");

    let rate_limit = &response.rate_limit;

    // Read operations
    println!("\nRead Operations:");
    println!("  Limit:     {}", rate_limit.read.limit);
    println!("  Remaining: {}", rate_limit.read.remaining);
    println!("  Reset:     {}", format_timestamp(rate_limit.read.reset));

    // Update operations
    println!("\nUpdate Operations:");
    println!("  Limit:     {}", rate_limit.update.limit);
    println!("  Remaining: {}", rate_limit.update.remaining);
    println!("  Reset:     {}", format_timestamp(rate_limit.update.reset));

    // Search operations
    println!("\nSearch Operations:");
    println!("  Limit:     {}", rate_limit.search.limit);
    println!("  Remaining: {}", rate_limit.search.remaining);
    println!("  Reset:     {}", format_timestamp(rate_limit.search.reset));

    // Icon operations
    println!("\nIcon Operations:");
    println!("  Limit:     {}", rate_limit.icon.limit);
    println!("  Remaining: {}", rate_limit.icon.remaining);
    println!("  Reset:     {}", format_timestamp(rate_limit.icon.reset));

    Ok(())
}

fn format_timestamp(timestamp: i32) -> String {
    let dt = Utc
        .timestamp_opt(timestamp as i64, 0)
        .single()
        .map(|utc| utc.with_timezone(&Local))
        .unwrap_or_else(Local::now);

    dt.format("%Y-%m-%d %H:%M:%S %Z").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_timestamp_valid() {
        // 2024-01-15 00:00:00 UTC
        let timestamp = 1705276800;
        let result = format_timestamp(timestamp);

        // Should contain date components (timezone-independent check)
        assert!(result.contains("2024-01-"));
        assert!(result.contains(":"));
    }

    #[test]
    fn test_format_timestamp_epoch() {
        // Unix epoch: 1970-01-01 00:00:00 UTC
        let timestamp = 0;
        let result = format_timestamp(timestamp);

        // Should contain 1970 (timezone-independent check)
        assert!(result.contains("1970-"));
    }

    #[test]
    fn test_format_timestamp_format() {
        let timestamp = 1705276800;
        let result = format_timestamp(timestamp);

        // Should match format: YYYY-MM-DD HH:MM:SS TZ
        let parts: Vec<&str> = result.split_whitespace().collect();
        assert_eq!(parts.len(), 3, "Expected 3 parts: date, time, timezone");

        // Date format check
        let date_parts: Vec<&str> = parts[0].split('-').collect();
        assert_eq!(date_parts.len(), 3, "Date should have 3 parts");

        // Time format check
        let time_parts: Vec<&str> = parts[1].split(':').collect();
        assert_eq!(time_parts.len(), 3, "Time should have 3 parts");
    }
}
