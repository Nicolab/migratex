// This file is part of "Migratex - A Migrations Toolkit".
//
// This source code is licensed under the MIT license, please view the LICENSE
// file distributed with this source code. For the full
// information and documentation: https://github.com/nicolab/migratex
// ----------------------------------------------------------------------------

use std::path::Path;

use anyhow::Result;
use sqlx::{Row, SqlitePool};

use migratex::{MetaStatus, Metadata, MetadataStore, meta_loaded};

use crate::db_path::DbPath;
use crate::metadata::AppMetadata;

/// SqlxStore provides SQLite-based storage (with Sqlx) for migration metadata.
/// Metadata is stored in a `_migratex_metadata` table within the database.
///
/// This implementation can either:
/// - Use a provided connection pool (via DbPath::with_pool) for efficiency
/// - Create temporary connections as needed (via DbPath::new)
pub struct SqlxStore;

impl SqlxStore {
    const TABLE_NAME: &'static str = "_migratex_metadata";

    /// Get or create a connection pool from the DbPath.
    /// If DbPath contains a pool, use it. Otherwise create a temporary connection.
    async fn get_pool(path: &DbPath) -> Result<SqlitePool> {
        if let Some(pool) = path.pool() {
            Ok(pool.clone())
        } else {
            let db_url = format!("sqlite:{}?mode=rwc", path.path());
            Ok(SqlitePool::connect(&db_url).await?)
        }
    }

    /// Ensure the metadata table exists.
    async fn ensure_table(pool: &SqlitePool) -> Result<()> {
        sqlx::query(&format!(
            "CREATE TABLE IF NOT EXISTS {} (
                id INTEGER PRIMARY KEY CHECK (id = 1),
                version INTEGER NOT NULL DEFAULT 0,
                status TEXT NOT NULL DEFAULT 'Clean',
                app_version TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )",
            Self::TABLE_NAME
        ))
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Load metadata from the database table.
    async fn load_from_db(pool: &SqlitePool) -> Result<Option<AppMetadata>> {
        Self::ensure_table(pool).await?;

        let row = sqlx::query(&format!(
            "SELECT version, status, app_version, created_at, updated_at FROM {} WHERE id = 1",
            Self::TABLE_NAME
        ))
        .fetch_optional(pool)
        .await?;

        if let Some(row) = row {
            let status_str: String = row.try_get("status")?;
            let status = match status_str.as_str() {
                "Migrating" => MetaStatus::Migrating,
                "Failed" => MetaStatus::Failed,
                _ => MetaStatus::Clean,
            };

            Ok(Some(AppMetadata {
                version: row.try_get("version")?,
                status,
                app_version: row.try_get("app_version")?,
                created_at: row.try_get("created_at")?,
                updated_at: row.try_get("updated_at")?,
            }))
        } else {
            Ok(None)
        }
    }

    /// Save metadata to the database table using UPSERT.
    async fn save_to_db(pool: &SqlitePool, metadata: &AppMetadata) -> Result<()> {
        Self::ensure_table(pool).await?;

        let status_str = match metadata.status {
            MetaStatus::Clean => "Clean",
            MetaStatus::Migrating => "Migrating",
            MetaStatus::Failed => "Failed",
        };

        sqlx::query(&format!(
            "INSERT INTO {} (id, version, status, app_version, created_at, updated_at)
             VALUES (1, ?, ?, ?, ?, ?)
             ON CONFLICT(id) DO UPDATE SET
                version = excluded.version,
                status = excluded.status,
                app_version = excluded.app_version,
                created_at = excluded.created_at,
                updated_at = excluded.updated_at",
            Self::TABLE_NAME
        ))
        .bind(metadata.version)
        .bind(status_str)
        .bind(&metadata.app_version)
        .bind(&metadata.created_at)
        .bind(&metadata.updated_at)
        .execute(pool)
        .await?;

        Ok(())
    }
}

impl MetadataStore<AppMetadata> for SqlxStore {
    fn load_or_init(path: impl AsRef<Path>) -> Result<AppMetadata>
    where
        AppMetadata: Default,
    {
        // Use block_in_place to handle async operations in sync context
        // This allows async work without creating a nested runtime
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                // Create DbPath from the path string
                let db_path = DbPath::new(path.as_ref().to_string_lossy().to_string());

                let pool = Self::get_pool(&db_path).await?;

                if let Some(meta) = Self::load_from_db(&pool).await? {
                    meta_loaded(meta)
                } else {
                    // Initialize new metadata
                    let meta = AppMetadata::new_with_path(&db_path)?;
                    Ok(meta)
                }
            })
        })
    }

    fn save(metadata: &AppMetadata, path: impl AsRef<Path>) -> Result<()> {
        // Use block_in_place to handle async operations in sync context
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                // Create DbPath from the path string
                let db_path = DbPath::new(path.as_ref().to_string_lossy().to_string());

                let pool = Self::get_pool(&db_path).await?;
                Self::save_to_db(&pool, metadata).await?;

                Ok(())
            })
        })
    }
}
