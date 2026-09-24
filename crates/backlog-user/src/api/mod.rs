mod get_notification_count;
mod get_notifications;
mod get_own_user;
mod get_user;
mod get_user_icon;
mod get_user_list;
mod get_user_recent_updates;
mod get_user_star_count;
mod get_user_stars;
mod get_watching_count;
mod get_watching_list;
#[cfg(feature = "writable")]
mod mark_notification_as_read;
#[cfg(feature = "writable")]
mod reset_unread_notification_count;
mod user_api;

pub use user_api::UserApi;

pub use get_notification_count::{GetNotificationCountParams, GetNotificationCountResponse};
pub use get_notifications::{GetNotificationsParams, GetNotificationsResponse, NotificationOrder};
pub use get_own_user::{GetOwnUserParams, GetOwnUserResponse};
pub use get_user::{GetUserParams, GetUserParamsBuilder, GetUserResponse};
pub use get_user_icon::{GetUserIconParams, GetUserIconParamsBuilder, GetUserIconResponse};
pub use get_user_list::{GetUserListParams, GetUserListResponse};
pub use get_user_recent_updates::{GetUserRecentUpdatesParams, GetUserRecentUpdatesResponse};
pub use get_user_star_count::{
    GetUserStarCountParams, GetUserStarCountParamsBuilder, GetUserStarCountResponse, StarCount,
};
pub use get_user_stars::{GetUserStarsParams, GetUserStarsResponse, StarOrder};
pub use get_watching_count::GetWatchingCountParams;
pub use get_watching_list::{
    GetWatchingListParams, GetWatchingListParamsBuilder, GetWatchingListRequest, Order,
    WatchingSort,
};
#[cfg(feature = "writable")]
pub use mark_notification_as_read::MarkNotificationAsReadParams;
#[cfg(feature = "writable")]
pub use reset_unread_notification_count::ResetUnreadNotificationCountParams;
