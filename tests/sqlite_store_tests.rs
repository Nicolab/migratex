// This file is part of "Migratex - A Migrations Toolkit".
//
// This source code is licensed under the MIT license, please view the LICENSE
// file distributed with this source code. For the full
// information and documentation: https://github.com/nicolab/migratex
// -----------------------------------------------------------------------------

// -- Tests for SqliteMetadata / SqliteStorage functionality.

#![cfg(feature = "sqlx")]

mod common;

use std::sync::Arc;

use migratex::{MetaStatus, Metadata, SqliteMetadata, SqliteStorage, connect_to_sqlite};
use okerr::Result;
use sqlx::AssertSqlSafe;

use common::TempDir;

/// Open a fresh SQLite database inside the temp dir and wrap it in a storage.
async fn new_storage(temp: &TempDir) -> Result<SqliteStorage> {
    let pool = connect_to_sqlite(temp.path().join("metadata.db")).await?;
    Ok(SqliteStorage::new(Arc::new(pool)))
}

/// Count the rows of the given metadata table.
async fn count_rows(storage: &SqliteStorage, table: &str) -> Result<i64> {
    let count: i64 = sqlx::query_scalar(AssertSqlSafe(format!("SELECT COUNT(*) FROM {table}")))
        .fetch_one(&*storage.pool)
        .await?;
    Ok(count)
}

#[tokio::test]
async fn test_sqlite_store_init_creates_new_row() -> Result<()> {
    let temp = TempDir::new()?;
    let storage = new_storage(&temp).await?;

    let meta = SqliteMetadata::load_or_init(&storage).await?;

    // Default values should be set
    assert_eq!(meta.version(), 0);
    assert_eq!(meta.status(), MetaStatus::Clean);
    assert!(!meta.created_at().is_empty());
    assert!(!meta.updated_at().is_empty());

    // Init should persist a single row in the default table
    assert_eq!(count_rows(&storage, "_migratex_metadata").await?, 1);

    Ok(())
}

#[tokio::test]
async fn test_sqlite_store_load_existing_row() -> Result<()> {
    let temp = TempDir::new()?;
    let storage = new_storage(&temp).await?;

    // Create initial metadata
    let mut meta1 = SqliteMetadata::load_or_init(&storage).await?;
    meta1.set_version(5);
    meta1.set_app_version("1.2.3".to_string());
    meta1.save(&storage).await?;

    // Load existing metadata
    let meta2 = SqliteMetadata::load_or_init(&storage).await?;

    // Values should match
    assert_eq!(meta2.version(), 5);
    assert_eq!(meta2.app_version(), "1.2.3");
    assert_eq!(meta2.status(), MetaStatus::Clean);
    assert_eq!(meta2.created_at(), meta1.created_at());

    Ok(())
}

#[tokio::test]
async fn test_sqlite_store_save_preserves_data() -> Result<()> {
    let temp = TempDir::new()?;
    let storage = new_storage(&temp).await?;

    // Create and save metadata
    let mut meta = SqliteMetadata::load_or_init(&storage).await?;
    meta.set_version(42);
    meta.set_app_version("2.0.0".to_string());
    meta.mark_migrating();

    let created_at = meta.created_at().to_string();
    meta.save(&storage).await?;

    // Reload and verify
    let loaded = SqliteMetadata::load_or_init(&storage).await?;
    assert_eq!(loaded.version(), 42);
    assert_eq!(loaded.app_version(), "2.0.0");
    assert_eq!(loaded.status(), MetaStatus::Migrating);
    assert_eq!(loaded.created_at(), created_at);

    Ok(())
}

#[tokio::test]
async fn test_sqlite_store_multiple_saves_keep_single_row() -> Result<()> {
    let temp = TempDir::new()?;
    let storage = new_storage(&temp).await?;

    let mut meta = SqliteMetadata::load_or_init(&storage).await?;

    // Save multiple times with different values (UPSERT on id = 1)
    for i in 1..=10 {
        meta.set_version(i);
        meta.save(&storage).await?;

        let loaded = SqliteMetadata::load_or_init(&storage).await?;
        assert_eq!(loaded.version(), i);
    }

    assert_eq!(count_rows(&storage, "_migratex_metadata").await?, 1);

    Ok(())
}

#[tokio::test]
async fn test_sqlite_store_all_status_values() -> Result<()> {
    let temp = TempDir::new()?;
    let storage = new_storage(&temp).await?;

    let statuses = vec![MetaStatus::Clean, MetaStatus::Migrating, MetaStatus::Failed];

    for status in statuses {
        let mut meta = SqliteMetadata::load_or_init(&storage).await?;
        meta.set_status(status);
        meta.save(&storage).await?;

        let loaded = SqliteMetadata::load_or_init(&storage).await?;
        assert_eq!(loaded.status(), status);
    }

    Ok(())
}

#[tokio::test]
async fn test_sqlite_store_preserves_timestamps() -> Result<()> {
    let temp = TempDir::new()?;
    let storage = new_storage(&temp).await?;

    // Create initial metadata
    let mut meta = SqliteMetadata::load_or_init(&storage).await?;
    let initial_created = meta.created_at().to_string();
    let initial_updated = meta.updated_at().to_string();

    // created_at and updated_at should be the same initially
    assert_eq!(initial_created, initial_updated);

    // Wait a tiny bit to ensure timestamp would change
    std::thread::sleep(std::time::Duration::from_millis(10));

    // Update version (which updates updated_at)
    meta.set_version(1);
    meta.save(&storage).await?;

    // Reload
    let loaded = SqliteMetadata::load_or_init(&storage).await?;

    // created_at should remain the same, updated_at should be different
    assert_eq!(loaded.created_at(), initial_created);
    assert_ne!(loaded.updated_at(), initial_updated);

    Ok(())
}

#[tokio::test]
async fn test_sqlite_store_with_unicode_content() -> Result<()> {
    let temp = TempDir::new()?;
    let storage = new_storage(&temp).await?;

    let mut meta = SqliteMetadata::load_or_init(&storage).await?;
    meta.set_app_version("1.0.0-café-🚀".to_string());
    meta.save(&storage).await?;

    let loaded = SqliteMetadata::load_or_init(&storage).await?;
    assert_eq!(loaded.app_version(), "1.0.0-café-🚀");

    Ok(())
}

#[tokio::test]
async fn test_sqlite_store_custom_table_name() -> Result<()> {
    let temp = TempDir::new()?;
    let storage = new_storage(&temp).await?.with_table_name("custom_meta_v2");

    let mut meta = SqliteMetadata::load_or_init(&storage).await?;
    meta.set_version(7);
    meta.save(&storage).await?;

    let loaded = SqliteMetadata::load_or_init(&storage).await?;
    assert_eq!(loaded.version(), 7);

    // Data lives in the custom table, and the default one was never created
    assert_eq!(count_rows(&storage, "custom_meta_v2").await?, 1);
    assert!(count_rows(&storage, "_migratex_metadata").await.is_err());

    Ok(())
}

#[tokio::test]
async fn test_sqlite_store_rejects_invalid_table_name() -> Result<()> {
    let temp = TempDir::new()?;
    let base = new_storage(&temp).await?;

    let invalid_names = [
        "",
        "1meta",
        "my-meta",
        "main.meta",
        "meta table",
        "meta\"",
        "meta; DROP TABLE users",
        "mèta",
    ];

    for name in invalid_names {
        let storage = base.clone().with_table_name(name);

        let loaded = SqliteMetadata::load_or_init(&storage).await;
        assert!(loaded.is_err(), "load_or_init should reject {name:?}");

        let saved = SqliteMetadata::default().save(&storage).await;
        assert!(saved.is_err(), "save should reject {name:?}");
    }

    // Nothing should have been created in the database
    let tables: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM sqlite_master WHERE type = 'table'")
        .fetch_one(&*base.pool)
        .await?;
    assert_eq!(tables, 0);

    Ok(())
}
