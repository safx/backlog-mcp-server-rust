//! Participant list operations for issues
//!
//! This module provides handlers for listing participants in an issue.

use crate::commands::common::CliResult;
use backlog_api_client::IssueIdOrKey;
use backlog_api_client::client::BacklogApiClient;
use backlog_core::identifier::Identifier;
use backlog_issue::GetParticipantListParams;
use std::str::FromStr;

/// List participants in an issue
///
/// Corresponds to `GET /api/v2/issues/:issueIdOrKey/participants`
pub async fn list_participants(
    client: &BacklogApiClient,
    issue_id_or_key: String,
) -> CliResult<()> {
    println!("Listing participants for issue: {issue_id_or_key}");

    let parsed_issue_id_or_key = IssueIdOrKey::from_str(&issue_id_or_key)
        .map_err(|e| format!("Failed to parse issue_id_or_key '{issue_id_or_key}': {e}"))?;

    match client
        .issue()
        .get_participant_list(GetParticipantListParams::new(parsed_issue_id_or_key))
        .await
    {
        Ok(participants) => {
            if participants.is_empty() {
                println!("No participants found for this issue.");
            } else {
                println!("Found {} participant(s):", participants.len());
                for participant in participants {
                    println!("- {} (ID: {})", participant.name, participant.id.value());
                    if let Some(user_id) = &participant.user_id {
                        println!("  User ID: {user_id}");
                    }
                    println!("  Email: {}", participant.mail_address);
                    println!("  Role: {:?}", participant.role_type);
                    if let Some(last_login) = &participant.last_login_time {
                        println!("  Last Login: {}", last_login.format("%Y-%m-%d %H:%M:%S"));
                    }
                    println!();
                }
            }
        }
        Err(e) => {
            eprintln!("Error listing participants: {e}");
        }
    }
    Ok(())
}
