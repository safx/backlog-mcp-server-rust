//! Subcommand modules for issue operations
//!
//! This module organizes issue command handlers into logical groups:
//! - `list`: Basic listing and viewing operations
//! - `crud`: Create, update, and delete operations (require issue_writable)
//! - `comments`: Comment management operations
//! - `attachments`: Attachment management operations
//! - `shared_files`: Shared file linking operations
//! - `participants`: Participant listing operations

pub mod attachments;
pub mod comments;
pub mod crud;
pub mod list;
pub mod participants;
pub mod shared_files;
