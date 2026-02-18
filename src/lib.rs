//! Common utilities shared across Pushkind services.
//!
//! The crate exposes Actix Web middleware, reusable models, pagination
//! and route helpers. When compiled with the `db` feature it also
//! includes Diesel-based database helpers.

#[cfg(feature = "actix")]
pub mod middleware;
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

pub mod domain;
pub mod models;
pub mod services;
