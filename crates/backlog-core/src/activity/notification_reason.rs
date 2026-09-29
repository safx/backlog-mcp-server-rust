use serde::{Deserialize, Deserializer};
use serde_repr::Serialize_repr;

#[cfg(feature = "schemars")]
use schemars::JsonSchema;

/// Reason codes from `GET /api/v2/notifications`.
///
/// Codes this crate does not know yet deserialize to [`NotificationReason::Unknown`]
/// so that a newly added reason does not break the whole notification list.
#[repr(i8)]
#[derive(Eq, PartialEq, Debug, Clone, Copy, Serialize_repr)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[non_exhaustive]
pub enum NotificationReason {
    NoReason = 0,
    AssignedToIssue = 1,
    IssueCommented = 2,
    IssueCreated = 3,
    IssueUpdated = 4,
    FileAdded = 5,
    ProjectUserAdded = 6,
    // 7, 8 are reserved for future use
    Other = 9,
    AssignedToPullRequest = 10,
    CommentAddedOnPullRequest = 11,
    PullRequestAdded = 12,
    PullRequestUpdated = 13,
    DocumentCommented = 14,
    DocumentCommentReplied = 15,
    /// Bulk issue creation (notification to the assignee)
    IssueMultiCreated = 16,
    DocumentMentioned = 17,
    /// Bulk issue creation (notification to other users)
    IssueMultiCreatedNotified = 18,
    // ponytail: the original code is dropped and serializes back as -1; switch to a
    // custom Unknown(i64) variant if callers ever need the raw value.
    Unknown = -1,
}

impl From<i64> for NotificationReason {
    fn from(code: i64) -> Self {
        match code {
            0 => Self::NoReason,
            1 => Self::AssignedToIssue,
            2 => Self::IssueCommented,
            3 => Self::IssueCreated,
            4 => Self::IssueUpdated,
            5 => Self::FileAdded,
            6 => Self::ProjectUserAdded,
            9 => Self::Other,
            10 => Self::AssignedToPullRequest,
            11 => Self::CommentAddedOnPullRequest,
            12 => Self::PullRequestAdded,
            13 => Self::PullRequestUpdated,
            14 => Self::DocumentCommented,
            15 => Self::DocumentCommentReplied,
            16 => Self::IssueMultiCreated,
            17 => Self::DocumentMentioned,
            18 => Self::IssueMultiCreatedNotified,
            _ => Self::Unknown,
        }
    }
}

impl<'de> Deserialize<'de> for NotificationReason {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        i64::deserialize(deserializer).map(Self::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notification_reason_serialization() {
        let reason = NotificationReason::IssueCommented;
        let serialized = serde_json::to_string(&reason).unwrap();
        assert_eq!(serialized, "2");
    }

    #[test]
    fn test_notification_reason_deserialization() {
        let json = "3";
        let reason: NotificationReason = serde_json::from_str(json).unwrap();
        assert_eq!(reason, NotificationReason::IssueCreated);
    }

    #[test]
    fn test_all_notification_reasons() {
        let test_cases = vec![
            (NotificationReason::NoReason, 0),
            (NotificationReason::AssignedToIssue, 1),
            (NotificationReason::IssueCommented, 2),
            (NotificationReason::IssueCreated, 3),
            (NotificationReason::IssueUpdated, 4),
            (NotificationReason::FileAdded, 5),
            (NotificationReason::ProjectUserAdded, 6),
            (NotificationReason::Other, 9),
            (NotificationReason::AssignedToPullRequest, 10),
            (NotificationReason::CommentAddedOnPullRequest, 11),
            (NotificationReason::PullRequestAdded, 12),
            (NotificationReason::PullRequestUpdated, 13),
            (NotificationReason::DocumentCommented, 14),
            (NotificationReason::DocumentCommentReplied, 15),
            (NotificationReason::IssueMultiCreated, 16),
            (NotificationReason::DocumentMentioned, 17),
            (NotificationReason::IssueMultiCreatedNotified, 18),
        ];

        for (reason, expected_value) in test_cases {
            let serialized = serde_json::to_string(&reason).unwrap();
            assert_eq!(serialized, expected_value.to_string());

            let deserialized: NotificationReason = serde_json::from_str(&serialized).unwrap();
            assert_eq!(deserialized, reason);
        }
    }

    #[test]
    fn test_unknown_notification_reason() {
        for json in ["7", "19", "1000", "-5"] {
            let reason: NotificationReason = serde_json::from_str(json).unwrap();
            assert_eq!(reason, NotificationReason::Unknown, "code {json}");
        }
    }
}
