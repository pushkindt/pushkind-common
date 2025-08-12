//! Domain models shared between services.
//!
//! These data structures intentionally avoid any database or
//! framework-specific details so they can be reused across crates.

#[cfg(feature = "dantes")]
pub mod benchmark;
#[cfg(feature = "dantes")]
pub mod crawler;
#[cfg(feature = "dantes")]
pub mod product;
