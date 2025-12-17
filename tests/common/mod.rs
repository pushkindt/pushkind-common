//! Helpers for integration tests.

use pushkind_common::db::{DbPool, establish_connection_pool};
use tempfile::NamedTempFile;

/// Temporary database used in integration tests.
pub struct TestDb {
    _tempfile: NamedTempFile,
    pool: DbPool,
}

impl TestDb {
    pub fn new() -> Self {
        let tempfile = NamedTempFile::new().expect("Failed to create temp file");
        let pool = establish_connection_pool(tempfile.path().to_str().unwrap())
            .expect("Failed to establish SQLite connection.");
        TestDb {
            _tempfile: tempfile,
            pool,
        }
    }

    pub fn pool(&self) -> DbPool {
        self.pool.clone()
    }
}
