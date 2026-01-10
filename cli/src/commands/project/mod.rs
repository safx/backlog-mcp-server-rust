//! Project command module
//!
//! This module provides CLI commands for project management including:
//! - CRUD operations (List, Show, Add, Update, Delete)
//! - User and administrator management
//! - Custom fields management
//! - Categories, issue types, versions/milestones, and statuses management
//! - Team management

pub mod args;
mod handler;
mod subcommands;

pub use args::ProjectArgs;
pub use handler::execute;
