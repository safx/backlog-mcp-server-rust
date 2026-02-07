use serde::{Deserialize, Serialize};

#[cfg(feature = "schemars")]
use schemars::JsonSchema;

pub trait Identifier {
    type Id;
    fn value(&self) -> Self::Id;
}

macro_rules! impl_identifier {
    ($(($type_name:ident,$ty:ty)),*) => {
        $(
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
            #[cfg_attr(feature = "schemars", derive(JsonSchema))]
            pub struct $type_name(pub $ty);

            impl $type_name {
                pub fn new(value: $ty) -> Self {
                    Self(value)
                }
            }

            impl Identifier for $type_name {
                type Id = $ty;
                fn value(&self) -> Self::Id {
                    self.0
                }
            }

            impl From<$ty> for $type_name {
                fn from(value: $ty) -> Self {
                    $type_name(value)
                }
            }

            impl std::str::FromStr for $type_name {
                type Err = crate::error::Error;

                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    match s.parse::<$ty>() {
                        Ok(val) => Ok($type_name(val)),
                        Err(_) => Err(crate::error::Error::InvalidParameter(format!(
                            "Failed to parse {} from string '{}': expected a {} integer.",
                            stringify!($type_name),
                            s,
                            stringify!($ty)
                        ))),
                    }
                }
            }

            impl std::fmt::Display for $type_name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{}", self.0)
                }
            }

        )*
    };
}

impl_identifier!(
    // u32 identifiers
    (ProjectId, u32),
    (UserId, u32),
    (IssueId, u32),
    (SpaceId, u32),
    (MilestoneId, u32),
    (CategoryId, u32),
    (IssueTypeId, u32),
    (StatusId, u32),
    (PriorityId, u32),
    (ResolutionId, u32),
    (CommentId, u32),
    (AttachmentId, u32),            // Issue attachment ID
    (WikiAttachmentId, u32),        // Wiki attachment ID
    (PullRequestAttachmentId, u32), // PR attachment ID
    (DocumentAttachmentId, u32),    // Document attachment ID
    (ExternalFileLinkId, u32),
    (NotificationId, u32),
    (StarId, u32),
    (SharedFileId, u32),
    (CustomFieldId, u32),
    (CustomFieldItemId, u32),
    (RepositoryId, u32),
    (PullRequestId, u32),
    (PullRequestCommentId, u32),
    (CustomListItemId, u32),
    (WikiId, u32),
    (WikiTagId, u32),
    (ActivityTypeId, u32),
    (ActivityId, u32),
    (TeamId, u32),
    (WatchingId, u32),
    (WebhookId, u32),
    // u64 identifiers
    (SvnRevision, u64),
    (PullRequestNumber, u64)
);

mod document_id;
pub use document_id::DocumentId;

#[cfg(test)]
mod activity_id_test;

#[cfg(test)]
mod u64_identifier_test;
