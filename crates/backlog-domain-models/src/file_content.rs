#[cfg(feature = "schemars")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Represents the content type of a shared file.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum FileContent {
    /// A regular file with size information
    File { size: u64 },
    /// A directory (no size information)
    Directory,
}
