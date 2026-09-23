mod star_api;
pub use star_api::StarApi;

#[cfg(feature = "writable")]
mod add_star;
#[cfg(feature = "writable")]
pub use add_star::{AddStarParams, StarTarget};

#[cfg(feature = "writable")]
mod delete_star;
#[cfg(feature = "writable")]
pub use delete_star::DeleteStarParams;
