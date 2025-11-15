// This file is part of "Migratex - A Migrations Toolkit".
//
// This source code is licensed under the MIT license, please view the LICENSE
// file distributed with this source code. For the full
// information and documentation: https://github.com/nicolab/migratex
// ----------------------------------------------------------------------------

use std::path::Path;
use std::sync::Arc;

use sqlx::SqlitePool;

/// A wrapper type that holds both a database file path and an optional SQLite connection pool.
/// It is an advanced pattern to pass both path and connection pool,
/// that is not required for basic usage. But the example is here if needed. ;)
///
/// This allows passing both the path (required by MetadataStore trait) and a reusable
/// connection pool to avoid creating multiple connections.
///
/// Implements `AsRef<Path>` to be compatible with `MetadataStore` trait methods.
#[derive(Clone)]
pub struct DbPath {
    path: String,
    pool: Option<Arc<SqlitePool>>,
}

impl DbPath {
    /// Create a new DbPath with just the file path.
    /// The SqlxStore will create temporary connections as needed.
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            pool: None,
        }
    }

    /// Create a DbPath with both path and a shared connection pool.
    /// The SqlxStore will use this pool instead of creating new connections.
    pub fn with_pool(path: impl Into<String>, pool: SqlitePool) -> Self {
        Self {
            path: path.into(),
            pool: Some(Arc::new(pool)),
        }
    }

    /// Get the database file path as a string.
    pub fn path(&self) -> &str {
        &self.path
    }

    /// Get a reference to the connection pool if available.
    pub fn pool(&self) -> Option<&SqlitePool> {
        self.pool.as_ref().map(|arc| arc.as_ref())
    }
}

impl AsRef<Path> for DbPath {
    fn as_ref(&self) -> &Path {
        Path::new(&self.path)
    }
}

impl From<String> for DbPath {
    fn from(path: String) -> Self {
        Self::new(path)
    }
}

impl From<&str> for DbPath {
    fn from(path: &str) -> Self {
        Self::new(path)
    }
}
