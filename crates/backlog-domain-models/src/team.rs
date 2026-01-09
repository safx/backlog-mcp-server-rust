use backlog_core::{User, id::TeamId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    pub id: TeamId,
    pub name: String,
    pub members: Vec<User>,
    pub created_user: User,
    pub created: DateTime<Utc>,
    pub updated_user: User,
    pub updated: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_team_deserialize() {
        let json = r##"{"id":1,"name":"Development Team","members":[],"createdUser":{"id":1,"userId":"admin","name":"Admin User","roleType":1,"lang":"ja","mailAddress":"admin@example.com"},"created":"2024-01-01T00:00:00Z","updatedUser":{"id":1,"userId":"admin","name":"Admin User","roleType":1,"lang":"ja","mailAddress":"admin@example.com"},"updated":"2024-01-01T00:00:00Z"}"##;
        let team: Team = serde_json::from_str(json).expect("should deserialize Team from JSON");
        assert_eq!(team.name, "Development Team");
        assert!(team.members.is_empty());
    }
}
