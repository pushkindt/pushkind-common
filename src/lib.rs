//! Common utilities shared across Pushkind services.
//!
//! The crate exposes Actix Web middleware, reusable models, pagination
//! and route helpers. When compiled with the `db` feature it also
//! includes Diesel-based database helpers.

#[cfg(feature = "actix")]
pub mod middleware;
#[cfg(feature = "actix")]
pub mod models;
#[cfg(feature = "actix")]
pub mod pagination;
#[cfg(feature = "actix")]
pub mod routes;

#[cfg(feature = "db")]
pub mod db;
#[cfg(feature = "db")]
pub mod repository;

#[cfg(feature = "zeromq")]
pub mod zmq;
