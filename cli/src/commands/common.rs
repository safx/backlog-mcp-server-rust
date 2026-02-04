//! Common utilities and helpers for CLI commands
//!
//! This module provides reusable functions for:
//! - Date parsing and conversion
//! - Display helpers (truncate text, format bytes)
//! - File operations (download files)
//! - Error handling

use chrono::{DateTime, NaiveDate, Utc};

/// Type alias for CLI results
pub type CliResult<T = ()> = anyhow::Result<T>;

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
    fn test_date_conversions() {
        let date = NaiveDate::from_ymd_opt(2024, 1, 10).unwrap();
        let start = date_to_start_of_day(date);
        let end = date_to_end_of_day(date);

        assert_eq!(start.format("%H:%M:%S").to_string(), "00:00:00");
        assert_eq!(end.format("%H:%M:%S").to_string(), "23:59:59");
    }
}
