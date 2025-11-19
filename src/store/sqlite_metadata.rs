// This file is part of "Migratex - A Migrations Toolkit".
//
// This source code is licensed under the MIT license, please view the LICENSE
// file distributed with this source code. For the full
// information and documentation: https://github.com/nicolab/migratex
// -----------------------------------------------------------------------------

use std::{path::PathBuf, sync::Arc};

use okerr::{Context, Result, ensure};
use sqlx::{
    Row, SqlitePool,
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions},
};

use crate::{MetaStatus, Metadata, init_meta_datetimes_if_empty, meta_loaded};

/// Connect to SQLite database.
#[cfg(feature = "sqlx")]
pub async fn connect_to_sqlite(db_path: PathBuf) -> Result<SqlitePool> {
    let db_path_name = db_path.display();
    let options = SqliteConnectOptions::new()
        .filename(&db_path)
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal)
        .pragma("foreign_keys", "ON");

    let pool = SqlitePoolOptions::new()
        .max_connections(10)
        .min_connections(1)
        //  .acquire_timeout(Duration::from_secs(30)) // uncomment if the app freeze because too much slow query
        .connect_with(options)
        .await
        .with_context(|| format!("failed to connect to database at {}", db_path_name))?;

    // sqlx::query(
    //     r#"
    //     PRAGMA foreign_keys = ON;
    //     PRAGMA journal_mode = WAL;
    //     "#,
    // )
    // .execute(&pool)
    // .await
    // .context("failed to initialize DB pragmas")?;

    let row: (i64,) = sqlx::query_as("PRAGMA foreign_keys;")
        .fetch_one(&pool)
        .await
        .context("find foreign_keys pragma failed")?;

    ensure!(row.0 == 1, "foreign_keys pragma must be set to 1");

    let row: (String,) = sqlx::query_as("PRAGMA journal_mode;")
        .fetch_one(&pool)
        .await
        .context("find journal_mode pragma failed")?;

    ensure!(row.0 == "wal", "journal_mode pragma must be set to WAL");

    Ok(pool)
}

/// Storage configuration for SQLite metadata.
/// Can be extended with additional fields as needed.
#[cfg(feature = "sqlx")]
#[derive(Debug, Clone)]
pub struct SqliteStorage {
    pub pool: Arc<SqlitePool>,
    pub table_name: String,
}

#[cfg(feature = "sqlx")]
impl SqliteStorage {
    /// Create a new SqliteStorage with default table name.
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self {
            pool,
            table_name: "_migratex_metadata".to_string(),
        }
    }

    /// Set a custom table name for metadata storage.
    pub fn with_table_name(mut self, name: impl Into<String>) -> Self {
        self.table_name = name.into();
        self
    }
}

/// SqliteMetadata provides SQLite-based storage for migration metadata.
/// Metadata is stored in a table within the SQLite database.
///
/// # Example
///
/// ```rust
/// use migratex::{SqliteMetadata, SqliteStorage, Metadata, connect_to_sqlite};
/// use std::sync::Arc;
/// use std::path::PathBuf;
/// use okerr::Result;
///
/// #[tokio::main]
/// async fn main() -> Result<()> {
///     // Connect to SQLite database
///     let pool = connect_to_sqlite(PathBuf::from("app.db")).await?;
///     let storage = SqliteStorage::new(Arc::new(pool));
///
///     // Load or initialize metadata
///     let mut meta = SqliteMetadata::load_or_init(&storage).await?;
///
///     // Modify metadata
///     meta.set_version(1);
///
///     // Save explicitly
///     meta.save(&storage).await?;
///
///     Ok(())
/// }
/// ```
#[cfg(feature = "sqlx")]
#[derive(Debug, Clone)]
pub struct SqliteMetadata {
    pub version: i32,
    pub app_version: String,
    pub status: MetaStatus,
    pub created_at: String,
    pub updated_at: String,
}

#[cfg(feature = "sqlx")]
impl Default for SqliteMetadata {
    fn default() -> Self {
        Self {
            version: 0,
            app_version: String::new(),
            status: MetaStatus::Clean,
            created_at: String::new(),
            updated_at: String::new(),
        }
    }
}

#[cfg(feature = "sqlx")]
impl SqliteMetadata {
    /// Load metadata from the database, or initialize a new one if it doesn't exist.
    pub async fn load_or_init(storage: &SqliteStorage) -> Result<Self> {
        Self::ensure_table(storage).await?;

        if let Some(meta) = Self::load_from_db(storage).await? {
            meta_loaded(meta)
        } else {
            Self::init_new(storage).await
        }
    }

    /// Save metadata to the database using UPSERT.
    pub async fn save(&self, storage: &SqliteStorage) -> Result<()> {
        Self::ensure_table(storage).await?;

        sqlx::query(&format!(
            "INSERT INTO {} (id, version, status, app_version, created_at, updated_at)
             VALUES (1, ?, ?, ?, ?, ?)
             ON CONFLICT(id) DO UPDATE SET
                version = excluded.version,
                status = excluded.status,
                app_version = excluded.app_version,
                updated_at = excluded.updated_at",
            storage.table_name
        ))
        .bind(self.version)
        .bind(self.to_status_str())
        .bind(&self.app_version)
        .bind(&self.created_at)
        .bind(&self.updated_at)
        .execute(&*storage.pool)
        .await?;

        Ok(())
    }

    /// Ensure the metadata table exists.
    async fn ensure_table(storage: &SqliteStorage) -> Result<()> {
        sqlx::query(&format!(
            "CREATE TABLE IF NOT EXISTS {} (
                id INTEGER PRIMARY KEY CHECK (id = 1),
                version INTEGER NOT NULL DEFAULT 0,
                status TEXT NOT NULL DEFAULT 'Clean',
                app_version TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )",
            storage.table_name
        ))
        .execute(&*storage.pool)
        .await?;

        Ok(())
    }

    /// Load metadata from the database table.
    async fn load_from_db(storage: &SqliteStorage) -> Result<Option<Self>> {
        let row = sqlx::query(&format!(
            "SELECT version, status, app_version, created_at, updated_at
             FROM {} WHERE id = 1",
            storage.table_name
        ))
        .fetch_optional(&*storage.pool)
        .await?;

        if let Some(row) = row {
            let status_str: String = row.try_get("status")?;
            let status = match status_str.as_str() {
                "Migrating" => MetaStatus::Migrating,
                "Failed" => MetaStatus::Failed,
                _ => MetaStatus::Clean,
            };

            Ok(Some(Self {
                version: row.try_get("version")?,
                app_version: row.try_get("app_version")?,
                status,
                created_at: row.try_get("created_at")?,
                updated_at: row.try_get("updated_at")?,
            }))
        } else {
            Ok(None)
        }
    }

    /// Initialize a new metadata instance and save it.
    async fn init_new(storage: &SqliteStorage) -> Result<Self> {
        let mut meta = Self::default();
        meta.set_version(0);
        meta.set_status(MetaStatus::Clean);
        meta.set_app_version(env!("CARGO_PKG_VERSION").to_string());
        init_meta_datetimes_if_empty(&mut meta);
        meta.save(storage).await?;
        Ok(meta)
    }
}

#[cfg(feature = "sqlx")]
impl Metadata for SqliteMetadata {
    crate::metadata_accessors!();
}
