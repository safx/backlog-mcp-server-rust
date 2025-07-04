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
