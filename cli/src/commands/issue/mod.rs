//! Issue command module
//!
//! Provides commands for managing Backlog issues.

pub mod args;
mod handler;
mod subcommands;

// Re-export the main entry point
pub use handler::execute;

// Re-export args for use in main.rs
pub use args::IssueArgs;
