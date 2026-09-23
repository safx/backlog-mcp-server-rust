pub mod api;

pub use api::StarApi;

#[cfg(feature = "writable")]
pub use api::{AddStarParams, DeleteStarParams, StarTarget};
