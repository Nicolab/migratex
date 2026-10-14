use std::sync::Arc;

use sqlx::SqlitePool;

/// Migration context that holds the database connection.
/// This is passed to each migration and used to execute SQL queries.
#[derive(Debug, Clone)]
pub struct MigContext {
    pub db: Arc<SqlitePool>,
}

impl MigContext {
    /// Create a new migration context with the given connection pool.
    pub fn new(db: Arc<SqlitePool>) -> Self {
        Self { db }
    }
}
