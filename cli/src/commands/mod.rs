pub mod common;

#[cfg(feature = "issue")]
pub mod issue;

#[cfg(feature = "project")]
pub mod project;

#[cfg(feature = "rate-limit")]
pub mod rate_limit;
#[cfg(feature = "star")]
pub mod star;
#[cfg(feature = "team")]
pub mod team;
#[cfg(feature = "user")]
pub mod user;
#[cfg(feature = "watching")]
pub mod watching;
#[cfg(feature = "webhook")]
pub mod webhook;
#[cfg(feature = "wiki")]
pub mod wiki;
