// This file is part of "Migratex - A Migrations Toolkit".
//
// This source code is licensed under the MIT license, please view the LICENSE
// file distributed with this source code. For the full
// information and documentation: https://github.com/nicolab/migratex
// -----------------------------------------------------------------------------

use std::sync::Arc;

use okerr::{Context, Result};
use sqlx::{
    AssertSqlSafe, PgPool, Row,
    postgres::{PgConnectOptions, PgPoolOptions},
};

use crate::{MetaStatus, Metadata, init_meta_datetimes_if_empty, meta_loaded};

use super::sql_identifier::checked_table_name;

/// Connect to a PostgreSQL database.
///
/// `url` is a standard connection string, e.g. `postgres://user:pass@host:5432/db`.
/// The URL is never included in error messages (it may contain a password).
///
/// To store the metadata table in a specific schema, set the `search_path`,
/// e.g. `postgres://…/db?options=-c%20search_path%3Dapp`, or build the pool
/// yourself with [`PgConnectOptions::options`].
#[cfg(feature = "postgres")]
pub async fn connect_to_postgres(url: &str) -> Result<PgPool> {
    let options: PgConnectOptions = url.parse().context("invalid PostgreSQL connection URL")?;

    PgPoolOptions::new()
        .max_connections(10)
        .connect_with(options)
        .await
        .context("failed to connect to PostgreSQL database")
}

/// Storage configuration for PostgreSQL metadata.
/// Can be extended with additional fields as needed.
#[cfg(feature = "postgres")]
#[derive(Debug, Clone)]
pub struct PgStorage {
    pub pool: Arc<PgPool>,
    pub table_name: String,
}

#[cfg(feature = "postgres")]
impl PgStorage {
    /// Create a new PgStorage with default table name.
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self {
            pool,
            table_name: "_migratex_metadata".to_string(),
        }
    }

    /// Set a custom table name for metadata storage.
    ///
    /// The name must be a plain SQL identifier (`[A-Za-z_][A-Za-z0-9_]*`),
    /// otherwise every metadata operation returns an error before running any
    /// query. The name is not quoted: PostgreSQL folds it to lowercase, and
    /// reserved keywords (e.g. `user`) fail at query time with the PostgreSQL error.
    /// The table is created in the first schema of the connection `search_path`.
    pub fn with_table_name(mut self, name: impl Into<String>) -> Self {
        self.table_name = name.into();
        self
    }

    /// Return the table name after checking it is a plain SQL identifier.
    fn checked_table_name(&self) -> Result<&str> {
        checked_table_name(&self.table_name)
    }
}

/// PgMetadata provides PostgreSQL-based storage for migration metadata.
/// Metadata is stored in a single-row table within the database.
///
/// # Example
///
/// ```rust,no_run
/// use migratex::{PgMetadata, PgStorage, Metadata, connect_to_postgres};
/// use std::sync::Arc;
/// use okerr::Result;
///
/// #[tokio::main]
/// async fn main() -> Result<()> {
///     // Connect to PostgreSQL database
///     let pool = connect_to_postgres("postgres://user:pass@localhost:5432/app").await?;
///     let storage = PgStorage::new(Arc::new(pool));
///
///     // Load or initialize metadata
///     let mut meta = PgMetadata::load_or_init(&storage).await?;
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
#[cfg(feature = "postgres")]
#[derive(Debug, Clone)]
pub struct PgMetadata {
    pub version: i32,
    pub app_version: String,
    pub status: MetaStatus,
    pub created_at: String,
    pub updated_at: String,
}

#[cfg(feature = "postgres")]
impl Default for PgMetadata {
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

#[cfg(feature = "postgres")]
impl PgMetadata {
    /// Load metadata from the database, or initialize a new one if it doesn't exist.
    pub async fn load_or_init(storage: &PgStorage) -> Result<Self> {
        Self::ensure_table(storage).await?;

        if let Some(meta) = Self::load_from_db(storage).await? {
            meta_loaded(meta)
        } else {
            Self::init_new(storage).await
        }
    }

    /// Save metadata to the database using UPSERT.
    pub async fn save(&self, storage: &PgStorage) -> Result<()> {
        Self::ensure_table(storage).await?;
        let table_name = storage.checked_table_name()?;

        sqlx::query(AssertSqlSafe(format!(
            "INSERT INTO {} (id, version, status, app_version, created_at, updated_at)
             VALUES (1, $1, $2, $3, $4, $5)
             ON CONFLICT (id) DO UPDATE SET
                version = excluded.version,
                status = excluded.status,
                app_version = excluded.app_version,
                updated_at = excluded.updated_at",
            table_name
        )))
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
    async fn ensure_table(storage: &PgStorage) -> Result<()> {
        let table_name = storage.checked_table_name()?;

        sqlx::query(AssertSqlSafe(format!(
            "CREATE TABLE IF NOT EXISTS {} (
                id INTEGER PRIMARY KEY CHECK (id = 1),
                version INTEGER NOT NULL DEFAULT 0,
                status TEXT NOT NULL DEFAULT 'Clean',
                app_version TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )",
            table_name
        )))
        .execute(&*storage.pool)
        .await?;

        Ok(())
    }

    /// Load metadata from the database table.
    async fn load_from_db(storage: &PgStorage) -> Result<Option<Self>> {
        let table_name = storage.checked_table_name()?;

        let row = sqlx::query(AssertSqlSafe(format!(
            "SELECT version, status, app_version, created_at, updated_at
             FROM {} WHERE id = 1",
            table_name
        )))
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
    async fn init_new(storage: &PgStorage) -> Result<Self> {
        let mut meta = Self::default();
        meta.set_version(0);
        meta.set_status(MetaStatus::Clean);
        meta.set_app_version(env!("CARGO_PKG_VERSION").to_string());
        init_meta_datetimes_if_empty(&mut meta);
        meta.save(storage).await?;
        Ok(meta)
    }
}

#[cfg(feature = "postgres")]
impl Metadata for PgMetadata {
    crate::metadata_accessors!();
}
