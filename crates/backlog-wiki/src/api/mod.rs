mod download_wiki_attachment;
mod get_recently_viewed_wikis;
mod get_wiki_attachment_list;
mod get_wiki_count;
mod get_wiki_detail;
mod get_wiki_history;
mod get_wiki_list;
mod get_wiki_shared_file_list;
mod get_wiki_stars;
mod get_wiki_tag_list;
mod wiki_api;

#[cfg(feature = "writable")]
mod add_recently_viewed_wiki;
#[cfg(feature = "writable")]
mod add_wiki;
#[cfg(feature = "writable")]
mod attach_files_to_wiki;
#[cfg(feature = "writable")]
mod delete_wiki;
#[cfg(feature = "writable")]
mod delete_wiki_attachment;
#[cfg(feature = "writable")]
mod link_shared_files_to_wiki;
#[cfg(feature = "writable")]
mod unlink_shared_file_from_wiki;
#[cfg(feature = "writable")]
mod update_wiki;

// Export response types (always available)
pub use download_wiki_attachment::DownloadWikiAttachmentParams;
pub use get_recently_viewed_wikis::{
    GetRecentlyViewedWikisParams, GetRecentlyViewedWikisParamsBuilder,
    GetRecentlyViewedWikisResponse,
};
pub use get_wiki_attachment_list::{GetWikiAttachmentListParams, GetWikiAttachmentListResponse};
pub use get_wiki_count::{GetWikiCountParams, GetWikiCountResponse};
pub use get_wiki_detail::{GetWikiDetailParams, GetWikiDetailResponse};
pub use get_wiki_history::{GetWikiHistoryParams, GetWikiHistoryResponse};
pub use get_wiki_list::{GetWikiListParams, GetWikiListResponse};
pub use get_wiki_shared_file_list::{GetWikiSharedFileListParams, GetWikiSharedFileListResponse};
pub use get_wiki_stars::{GetWikiStarsParams, GetWikiStarsResponse};
pub use get_wiki_tag_list::{GetWikiTagListParams, GetWikiTagListResponse};

// Export writable types with feature gates
#[cfg(feature = "writable")]
pub use add_recently_viewed_wiki::{AddRecentlyViewedWikiParams, AddRecentlyViewedWikiResponse};
#[cfg(feature = "writable")]
pub use add_wiki::{AddWikiParams, AddWikiResponse};
#[cfg(feature = "writable")]
pub use attach_files_to_wiki::{AttachFilesToWikiParams, AttachFilesToWikiResponse};
#[cfg(feature = "writable")]
pub use delete_wiki::{DeleteWikiParams, DeleteWikiResponse};
#[cfg(feature = "writable")]
pub use delete_wiki_attachment::{DeleteWikiAttachmentParams, DeleteWikiAttachmentResponse};
#[cfg(feature = "writable")]
pub use link_shared_files_to_wiki::{
    LinkSharedFilesToWikiParams, LinkSharedFilesToWikiParamsBuilder, LinkSharedFilesToWikiResponse,
};
#[cfg(feature = "writable")]
pub use unlink_shared_file_from_wiki::{
    UnlinkSharedFileFromWikiParams, UnlinkSharedFileFromWikiResponse,
};
#[cfg(feature = "writable")]
pub use update_wiki::{UpdateWikiParams, UpdateWikiResponse};

pub use wiki_api::WikiApi;
