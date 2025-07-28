//! Common utilities shared across Pushkind services.
//!
//! The crate exposes Actix Web middleware, reusable models, pagination
//! and route helpers. When compiled with the `db` feature it also
//! includes Diesel-based database helpers.

pub mod middleware;
pub mod models;
pub mod pagination;
pub mod routes;

#[cfg(feature = "db")]
pub mod db;
#[cfg(feature = "db")]
pub mod repository;

#[cfg(feature = "zeromq")]
pub mod zmq;
