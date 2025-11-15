use sqlx::SqlitePool;

/// Migration context that holds the database connection pool.
/// This is passed to each migration and used to execute SQL queries.
#[derive(Debug, Clone)]
pub struct MigContext {
    pub pool: SqlitePool,
}

impl MigContext {
    /// Create a new migration context with the given connection pool.
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}
