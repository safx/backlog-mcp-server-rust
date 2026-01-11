//! Common utilities and helpers for CLI commands
//!
//! This module provides reusable functions for:
//! - ID parsing (ProjectIdOrKey, IssueIdOrKey, comma-separated IDs)
//! - Date parsing and conversion
//! - Display helpers (truncate text, format bytes)
//! - File operations (download files)
//! - Error handling

use backlog_core::identifier::{IssueId, ProjectId};
use backlog_core::{IssueIdOrKey, IssueKey, ProjectIdOrKey, ProjectKey};
use chrono::{DateTime, NaiveDate, Utc};
use std::error::Error;
use std::path::Path;
use tokio::fs;

/// Type alias for CLI results
pub type CliResult<T = ()> = Result<T, Box<dyn Error>>;

/// Parse a string into ProjectIdOrKey
///
/// Tries to parse as u32 first (numeric ID), falls back to ProjectKey
pub fn parse_project_id_or_key(input: &str) -> CliResult<ProjectIdOrKey> {
    if let Ok(id) = input.parse::<u32>() {
        Ok(ProjectIdOrKey::from(ProjectId::new(id)))
    } else {
        let key = input
            .parse::<ProjectKey>()
            .map_err(|e| format!("Invalid project key '{}': {}", input, e))?;
        Ok(ProjectIdOrKey::from(key))
    }
}

/// Parse a string into IssueIdOrKey
///
/// Tries to parse as u32 first (numeric ID), falls back to IssueKey
pub fn parse_issue_id_or_key(input: &str) -> CliResult<IssueIdOrKey> {
    if let Ok(id) = input.parse::<u32>() {
        Ok(IssueIdOrKey::Id(IssueId::from(id)))
    } else {
        let key = input
            .parse::<IssueKey>()
            .map_err(|e| format!("Invalid issue key '{}': {}", input, e))?;
        Ok(IssueIdOrKey::Key(key))
    }
}

/// Parse comma-separated IDs into a vector
///
/// # Example
/// ```no_run
/// let ids = parse_comma_separated_ids("1,2,3", UserId::new)?;
/// ```
pub fn parse_comma_separated_ids<T, F>(input: &str, constructor: F) -> CliResult<Vec<T>>
where
    F: Fn(u32) -> T,
{
    input
        .split(',')
        .map(|s| {
            s.trim()
                .parse::<u32>()
                .map(&constructor)
                .map_err(|e| format!("Invalid ID '{}': {}", s.trim(), e).into())
        })
        .collect()
}

/// Parse date string in YYYY-MM-DD format
pub fn parse_date(input: &str) -> CliResult<NaiveDate> {
    NaiveDate::parse_from_str(input, "%Y-%m-%d").map_err(|e| {
        format!(
            "Invalid date format '{}': {}. Expected YYYY-MM-DD",
            input, e
        )
        .into()
    })
}

/// Convert NaiveDate to start of day DateTime<Utc> (00:00:00)
pub fn date_to_start_of_day(date: NaiveDate) -> DateTime<Utc> {
    date.and_hms_opt(0, 0, 0)
        .expect("00:00:00 is always valid")
        .and_utc()
}

/// Convert NaiveDate to end of day DateTime<Utc> (23:59:59)
pub fn date_to_end_of_day(date: NaiveDate) -> DateTime<Utc> {
    date.and_hms_opt(23, 59, 59)
        .expect("23:59:59 is always valid")
        .and_utc()
}

/// Truncate text safely at UTF-8 boundary
///
/// If the text is longer than max_length, truncates it and adds "..."
pub fn truncate_text(text: &str, max_length: usize) -> String {
    if text.len() <= max_length {
        text.to_string()
    } else {
        let mut end = max_length;
        while !text.is_char_boundary(end) && end > 0 {
            end -= 1;
        }
        format!("{}...", &text[..end])
    }
}

/// Format bytes in human-readable form (B, KB, MB, GB, TB)
pub fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", size as u64, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

/// Print success message with checkmark emoji
pub fn print_success(msg: &str) {
    println!("✅ {}", msg);
}

/// Download file helper
///
/// Writes bytes to the specified path and prints a success message
pub async fn download_file(bytes: &[u8], output_path: &Path, resource_name: &str) -> CliResult<()> {
    fs::write(output_path, bytes).await.map_err(|e| {
        format!(
            "Failed to save {} to {}: {}",
            resource_name,
            output_path.display(),
            e
        )
    })?;
    println!(
        "{} downloaded successfully to: {}",
        resource_name,
        output_path.display()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_truncate_text() {
        assert_eq!(truncate_text("Hello", 10), "Hello");
        assert_eq!(truncate_text("Hello World", 5), "Hello...");
        assert_eq!(truncate_text("こんにちは世界", 9), "こんに...");
    }

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(0), "0 B");
        assert_eq!(format_bytes(1023), "1023 B");
        assert_eq!(format_bytes(1024), "1.0 KB");
        assert_eq!(format_bytes(1536), "1.5 KB");
        assert_eq!(format_bytes(1048576), "1.0 MB");
        assert_eq!(format_bytes(1073741824), "1.0 GB");
    }

    #[test]
    fn test_parse_date() {
        assert!(parse_date("2024-01-10").is_ok());
        assert!(parse_date("invalid").is_err());
        assert!(parse_date("2024/01/10").is_err());
    }

    #[test]
    fn test_date_conversions() {
        let date = NaiveDate::from_ymd_opt(2024, 1, 10).unwrap();
        let start = date_to_start_of_day(date);
        let end = date_to_end_of_day(date);

        assert_eq!(start.format("%H:%M:%S").to_string(), "00:00:00");
        assert_eq!(end.format("%H:%M:%S").to_string(), "23:59:59");
    }
}
