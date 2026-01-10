use serde_repr::{Deserialize_repr, Serialize_repr};
#[repr(i64)]
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize_repr, Deserialize_repr,
)]
pub enum ActiveTypeId {
    IssueCreated = 1,
    IssueUpdated = 2,
    IssueCommented = 3,
    IssueDeleted = 4,
    WikiCreated = 5,
    WikiUpdated = 6,
    WikiDeleted = 7,
    FileAdded = 8,
    FileUpdated = 9,
    FileDeleted = 10,
    SVNCommitted = 11,
    GitPushed = 12,
    GitRepositoryCreated = 13,
    IssueMultiUpdated = 14,
    ProjectUserAdded = 15,
    ProjectUserDeleted = 16,
    CommentNotificationAdded = 17,
    PullRequestAdded = 18,
    PullRequestUpdated = 19,
    CommentAddedOnPullRequest = 20,
    PullRequestDeleted = 21,
    MilestoneCreated = 22,
    MilestoneUpdated = 23,
    MilestoneDeleted = 24,
    ProjectGroupAdded = 25,
    ProjectGroupDeleted = 26,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_variants_serialize_deserialize() {
        let test_cases = vec![
            (ActiveTypeId::IssueCreated, 1),
            (ActiveTypeId::IssueUpdated, 2),
            (ActiveTypeId::IssueCommented, 3),
            (ActiveTypeId::IssueDeleted, 4),
            (ActiveTypeId::WikiCreated, 5),
            (ActiveTypeId::WikiUpdated, 6),
            (ActiveTypeId::WikiDeleted, 7),
            (ActiveTypeId::FileAdded, 8),
            (ActiveTypeId::FileUpdated, 9),
            (ActiveTypeId::FileDeleted, 10),
            (ActiveTypeId::SVNCommitted, 11),
            (ActiveTypeId::GitPushed, 12),
            (ActiveTypeId::GitRepositoryCreated, 13),
            (ActiveTypeId::IssueMultiUpdated, 14),
            (ActiveTypeId::ProjectUserAdded, 15),
            (ActiveTypeId::ProjectUserDeleted, 16),
            (ActiveTypeId::CommentNotificationAdded, 17),
            (ActiveTypeId::PullRequestAdded, 18),
            (ActiveTypeId::PullRequestUpdated, 19),
            (ActiveTypeId::CommentAddedOnPullRequest, 20),
            (ActiveTypeId::PullRequestDeleted, 21),
            (ActiveTypeId::MilestoneCreated, 22),
            (ActiveTypeId::MilestoneUpdated, 23),
            (ActiveTypeId::MilestoneDeleted, 24),
            (ActiveTypeId::ProjectGroupAdded, 25),
            (ActiveTypeId::ProjectGroupDeleted, 26),
        ];

        for (variant, expected_value) in test_cases {
            // Serialize
            let serialized = serde_json::to_string(&variant).unwrap();
            assert_eq!(serialized, expected_value.to_string());

            // Deserialize
            let deserialized: ActiveTypeId = serde_json::from_str(&serialized).unwrap();
            assert_eq!(deserialized, variant);
        }
    }

    #[test]
    fn test_ordering() {
        assert!(ActiveTypeId::IssueCreated < ActiveTypeId::IssueUpdated);
        assert!(ActiveTypeId::ProjectGroupDeleted > ActiveTypeId::ProjectGroupAdded);
    }
}
