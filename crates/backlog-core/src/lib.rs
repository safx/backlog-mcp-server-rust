mod active_type_id;
pub mod activity;
mod api_date;
pub mod date;
mod error;
mod file_type;
pub mod identifier;
mod issue_id_or_key;
mod issue_key;
mod language;
mod nulab_account;
mod project_id_or_key;
mod project_key;
mod repository_id_or_name;
mod repository_name;
mod role;
mod space_key;
mod star;
mod text_formatting_rule;
mod user;

pub use active_type_id::ActiveTypeId;
pub use api_date::ApiDate;
pub use date::Date;
pub use error::{Error, Result};
pub use file_type::FileType;
pub use issue_id_or_key::IssueIdOrKey;
pub use issue_key::IssueKey;
pub use language::Language;
pub use nulab_account::NulabAccount;
pub use project_id_or_key::ProjectIdOrKey;
pub use project_key::ProjectKey;
pub use repository_id_or_name::RepositoryIdOrName;
pub use repository_name::RepositoryName;
pub use role::Role;
pub use space_key::SpaceKey;
pub use star::Star;
pub use text_formatting_rule::TextFormattingRule;
pub use user::User;

// Re-export identifiers under `id` namespace
pub mod id {
    pub use crate::identifier::{
        ActivityId, ActivityTypeId, AttachmentId, CategoryId, CommentId, CustomFieldId,
        CustomFieldItemId, CustomListItemId, DocumentAttachmentId, DocumentId, ExternalFileLinkId,
        IssueId, IssueTypeId, MilestoneId, NotificationId, PriorityId, ProjectId,
        PullRequestAttachmentId, PullRequestCommentId, PullRequestId, PullRequestNumber,
        RepositoryId, ResolutionId, SharedFileId, SpaceId, StarId, StatusId, SvnRevision, TeamId,
        UserId, WatchingId, WebhookId, WikiAttachmentId, WikiId, WikiTagId,
    };
}
