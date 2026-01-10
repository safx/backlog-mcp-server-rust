use backlog_api_core::IntoRequest;
use backlog_core::{IssueIdOrKey, User};

/// Response type for getting a list of participants in an issue
pub type GetParticipantListResponse = Vec<User>;

/// Parameters for getting participant list for a specific issue.
/// Corresponds to `GET /api/v2/issues/:issueIdOrKey/participants`.
#[derive(Debug, Clone, PartialEq)]
pub struct GetParticipantListParams {
    pub issue_id_or_key: IssueIdOrKey,
}

impl GetParticipantListParams {
    pub fn new(issue_id_or_key: impl Into<IssueIdOrKey>) -> Self {
        Self {
            issue_id_or_key: issue_id_or_key.into(),
        }
    }
}

impl IntoRequest for GetParticipantListParams {
    fn path(&self) -> String {
        format!("/api/v2/issues/{}/participants", self.issue_id_or_key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use backlog_core::identifier::{Identifier, IssueId};
    use backlog_core::{IssueKey, Role};
    use std::str::FromStr;

    #[test]
    fn test_get_participant_list_params_new_with_issue_key() {
        let issue_key = IssueKey::from_str("TEST-123").unwrap();
        let params = GetParticipantListParams::new(issue_key.clone());
        assert_eq!(params.issue_id_or_key, IssueIdOrKey::Key(issue_key));
    }

    #[test]
    fn test_get_participant_list_params_new_with_issue_id() {
        let issue_id = IssueId::new(12345);
        let params = GetParticipantListParams::new(issue_id);
        assert_eq!(params.issue_id_or_key, IssueIdOrKey::Id(issue_id));
    }

    #[test]
    fn test_get_participant_list_params_path_with_issue_key() {
        let params = GetParticipantListParams::new(IssueKey::from_str("TEST-123").unwrap());
        assert_eq!(params.path(), "/api/v2/issues/TEST-123/participants");
    }

    #[test]
    fn test_get_participant_list_params_path_with_issue_id() {
        let params = GetParticipantListParams::new(IssueId::new(12345));
        assert_eq!(params.path(), "/api/v2/issues/12345/participants");
    }

    #[test]
    fn test_get_participant_list_params_debug() {
        let params = GetParticipantListParams::new(IssueKey::from_str("TEST-123").unwrap());
        let debug_str = format!("{params:?}");
        assert!(debug_str.contains("GetParticipantListParams"));
        assert!(debug_str.contains("TEST"));
    }

    #[test]
    fn test_get_participant_list_params_clone() {
        let params = GetParticipantListParams::new(IssueKey::from_str("TEST-123").unwrap());
        let cloned = params.clone();
        assert_eq!(params, cloned);
    }

    #[test]
    fn test_get_participant_list_params_partial_eq() {
        let params1 = GetParticipantListParams::new(IssueKey::from_str("TEST-123").unwrap());
        let params2 = GetParticipantListParams::new(IssueKey::from_str("TEST-123").unwrap());
        let params3 = GetParticipantListParams::new(IssueKey::from_str("TEST-456").unwrap());

        assert_eq!(params1, params2);
        assert_ne!(params1, params3);
    }

    #[test]
    fn test_get_participant_list_response_deserialization() {
        let json_str = r#"[
            {
                "id": 1,
                "userId": "admin",
                "name": "Administrator",
                "roleType": 1,
                "lang": "ja",
                "mailAddress": "admin@example.com",
                "lastLoginTime": "2022-09-01T06:35:39Z"
            },
            {
                "id": 2,
                "userId": "user1",
                "name": "User One",
                "roleType": 2,
                "lang": "en",
                "mailAddress": "user1@example.com",
                "lastLoginTime": null
            }
        ]"#;

        let participants: GetParticipantListResponse = serde_json::from_str(json_str).unwrap();
        assert_eq!(participants.len(), 2);

        let first_participant = &participants[0];
        assert_eq!(first_participant.id.value(), 1);
        assert_eq!(first_participant.user_id, Some("admin".to_string()));
        assert_eq!(first_participant.name, "Administrator");
        assert_eq!(first_participant.role_type, Role::Admin);
        assert_eq!(first_participant.mail_address, "admin@example.com");

        let second_participant = &participants[1];
        assert_eq!(second_participant.id.value(), 2);
        assert_eq!(second_participant.user_id, Some("user1".to_string()));
        assert_eq!(second_participant.name, "User One");
        assert_eq!(second_participant.role_type, Role::User);
        assert_eq!(second_participant.mail_address, "user1@example.com");
        assert!(second_participant.last_login_time.is_none());
    }

    #[test]
    fn test_get_participant_list_response_empty_list() {
        let json_str = "[]";
        let participants: GetParticipantListResponse = serde_json::from_str(json_str).unwrap();
        assert!(participants.is_empty());
    }
}
