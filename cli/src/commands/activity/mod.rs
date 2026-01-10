#[cfg(feature = "project")]
pub mod args;
#[cfg(feature = "project")]
mod handler;
#[cfg(feature = "project")]
mod subcommands;

#[cfg(feature = "project")]
pub use args::ActivityArgs;
#[cfg(feature = "project")]
pub use handler::execute;
