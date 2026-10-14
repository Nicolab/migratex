// This file is part of "Migratex - A Migrations Toolkit".
//
// This source code is licensed under the MIT license, please view the LICENSE
// file distributed with this source code. For the full
// information and documentation: https://github.com/nicolab/migratex
// -----------------------------------------------------------------------------

// -- Tests for PgMetadata / PgStorage functionality.
//
// Requires a PostgreSQL server, set by `MIGRATEX_TEST_PG_URL`, e.g.:
//
//   docker run --rm -d -p 5432:5432 -e POSTGRES_PASSWORD=pg postgres:18
//   MIGRATEX_TEST_PG_URL=postgres://postgres:pg@localhost:5432/postgres \
//     cargo test --features postgres --test postgres_store_tests
//
// Each test runs in its own schema (dropped at the end), so tests can run
// in parallel against the same database.

#![cfg(feature = "postgres")]

use std::sync::{
    Arc,
    atomic::{AtomicU32, Ordering},
};

use migratex::{MetaStatus, Metadata, PgMetadata, PgStorage, connect_to_postgres};
use okerr::{Context, Result};
use sqlx::{
    AssertSqlSafe, PgPool,
    postgres::{PgConnectOptions, PgPoolOptions},
};

static SCHEMA_COUNTER: AtomicU32 = AtomicU32::new(0);

/// An isolated schema, used as `search_path` by its pool.
struct TestSchema {
    admin: PgPool,
    name: String,
    storage: PgStorage,
}

impl TestSchema {
    async fn new() -> Result<Self> {
        let url = std::env::var("MIGRATEX_TEST_PG_URL")
            .context("MIGRATEX_TEST_PG_URL must be set to run PostgreSQL tests")?;

        let admin = connect_to_postgres(&url).await?;
        let name = format!(
            "migratex_test_{}_{}",
            std::process::id(),
            SCHEMA_COUNTER.fetch_add(1, Ordering::SeqCst)
        );
        sqlx::query(AssertSqlSafe(format!("CREATE SCHEMA {name}")))
            .execute(&admin)
            .await?;

        let options: PgConnectOptions = url.parse()?;
        let pool = PgPoolOptions::new()
            .max_connections(2)
            .connect_with(options.options([("search_path", name.as_str())]))
            .await?;

        Ok(Self {
            admin,
            name,
            storage: PgStorage::new(Arc::new(pool)),
        })
    }

    /// Count the rows of the given metadata table.
    async fn count_rows(&self, table: &str) -> Result<i64> {
        let count: i64 = sqlx::query_scalar(AssertSqlSafe(format!(
            "SELECT COUNT(*) FROM {}.{table}",
            self.name
        )))
        .fetch_one(&self.admin)
        .await?;
        Ok(count)
    }

    /// Count the tables of the schema.
    async fn count_tables(&self) -> Result<i64> {
        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM information_schema.tables WHERE table_schema = $1",
        )
        .bind(&self.name)
        .fetch_one(&self.admin)
        .await?;
        Ok(count)
    }

    async fn drop(self) -> Result<()> {
        self.storage.pool.close().await;
        sqlx::query(AssertSqlSafe(format!("DROP SCHEMA {} CASCADE", self.name)))
            .execute(&self.admin)
            .await?;
        Ok(())
    }
}

#[tokio::test]
async fn test_pg_store_init_creates_new_row() -> Result<()> {
    let schema = TestSchema::new().await?;

    let meta = PgMetadata::load_or_init(&schema.storage).await?;

    // Default values should be set
    assert_eq!(meta.version(), 0);
    assert_eq!(meta.status(), MetaStatus::Clean);
    assert!(!meta.created_at().is_empty());
    assert!(!meta.updated_at().is_empty());

    // Init should persist a single row in the default table
    assert_eq!(schema.count_rows("_migratex_metadata").await?, 1);

    schema.drop().await
}

#[tokio::test]
async fn test_pg_store_load_existing_row() -> Result<()> {
    let schema = TestSchema::new().await?;

    // Create initial metadata
    let mut meta1 = PgMetadata::load_or_init(&schema.storage).await?;
    meta1.set_version(5);
    meta1.set_app_version("1.2.3".to_string());
    meta1.save(&schema.storage).await?;

    // Load existing metadata
    let meta2 = PgMetadata::load_or_init(&schema.storage).await?;

    // Values should match
    assert_eq!(meta2.version(), 5);
    assert_eq!(meta2.app_version(), "1.2.3");
    assert_eq!(meta2.status(), MetaStatus::Clean);
    assert_eq!(meta2.created_at(), meta1.created_at());

    schema.drop().await
}

#[tokio::test]
async fn test_pg_store_save_preserves_data() -> Result<()> {
    let schema = TestSchema::new().await?;

    // Create and save metadata
    let mut meta = PgMetadata::load_or_init(&schema.storage).await?;
    meta.set_version(42);
    meta.set_app_version("2.0.0".to_string());
    meta.mark_migrating();

    let created_at = meta.created_at().to_string();
    meta.save(&schema.storage).await?;

    // Reload and verify
    let loaded = PgMetadata::load_or_init(&schema.storage).await?;
    assert_eq!(loaded.version(), 42);
    assert_eq!(loaded.app_version(), "2.0.0");
    assert_eq!(loaded.status(), MetaStatus::Migrating);
    assert_eq!(loaded.created_at(), created_at);

    schema.drop().await
}

#[tokio::test]
async fn test_pg_store_multiple_saves_keep_single_row() -> Result<()> {
    let schema = TestSchema::new().await?;

    let mut meta = PgMetadata::load_or_init(&schema.storage).await?;

    // Save multiple times with different values (UPSERT on id = 1)
    for i in 1..=10 {
        meta.set_version(i);
        meta.save(&schema.storage).await?;

        let loaded = PgMetadata::load_or_init(&schema.storage).await?;
        assert_eq!(loaded.version(), i);
    }

    assert_eq!(schema.count_rows("_migratex_metadata").await?, 1);

    schema.drop().await
}

#[tokio::test]
async fn test_pg_store_all_status_values() -> Result<()> {
    let schema = TestSchema::new().await?;

    let statuses = vec![MetaStatus::Clean, MetaStatus::Migrating, MetaStatus::Failed];

    for status in statuses {
        let mut meta = PgMetadata::load_or_init(&schema.storage).await?;
        meta.set_status(status);
        meta.save(&schema.storage).await?;

        let loaded = PgMetadata::load_or_init(&schema.storage).await?;
        assert_eq!(loaded.status(), status);
    }

    schema.drop().await
}

#[tokio::test]
async fn test_pg_store_preserves_timestamps() -> Result<()> {
    let schema = TestSchema::new().await?;

    // Create initial metadata
    let mut meta = PgMetadata::load_or_init(&schema.storage).await?;
    let initial_created = meta.created_at().to_string();
    let initial_updated = meta.updated_at().to_string();

    // created_at and updated_at should be the same initially
    assert_eq!(initial_created, initial_updated);

    // Wait a tiny bit to ensure timestamp would change
    std::thread::sleep(std::time::Duration::from_millis(10));

    // Update version (which updates updated_at)
    meta.set_version(1);
    meta.save(&schema.storage).await?;

    // Reload
    let loaded = PgMetadata::load_or_init(&schema.storage).await?;

    // created_at should remain the same, updated_at should be different
    assert_eq!(loaded.created_at(), initial_created);
    assert_ne!(loaded.updated_at(), initial_updated);

    schema.drop().await
}

#[tokio::test]
async fn test_pg_store_with_unicode_content() -> Result<()> {
    let schema = TestSchema::new().await?;

    let mut meta = PgMetadata::load_or_init(&schema.storage).await?;
    meta.set_app_version("1.0.0-café-🚀".to_string());
    meta.save(&schema.storage).await?;

    let loaded = PgMetadata::load_or_init(&schema.storage).await?;
    assert_eq!(loaded.app_version(), "1.0.0-café-🚀");

    schema.drop().await
}

#[tokio::test]
async fn test_pg_store_custom_table_name() -> Result<()> {
    let schema = TestSchema::new().await?;
    let storage = schema.storage.clone().with_table_name("custom_meta_v2");

    let mut meta = PgMetadata::load_or_init(&storage).await?;
    meta.set_version(7);
    meta.save(&storage).await?;

    let loaded = PgMetadata::load_or_init(&storage).await?;
    assert_eq!(loaded.version(), 7);

    // Data lives in the custom table, and the default one was never created
    assert_eq!(schema.count_rows("custom_meta_v2").await?, 1);
    assert!(schema.count_rows("_migratex_metadata").await.is_err());

    schema.drop().await
}

#[tokio::test]
async fn test_pg_store_rejects_invalid_table_name() -> Result<()> {
    let schema = TestSchema::new().await?;

    let invalid_names = [
        "",
        "1meta",
        "my-meta",
        "public.meta",
        "meta table",
        "meta\"",
        "meta; DROP TABLE users",
        "mèta",
    ];

    for name in invalid_names {
        let storage = schema.storage.clone().with_table_name(name);

        let loaded = PgMetadata::load_or_init(&storage).await;
        assert!(loaded.is_err(), "load_or_init should reject {name:?}");

        let saved = PgMetadata::default().save(&storage).await;
        assert!(saved.is_err(), "save should reject {name:?}");
    }

    // Nothing should have been created in the schema
    assert_eq!(schema.count_tables().await?, 0);

    schema.drop().await
}

#[tokio::test]
async fn test_pg_connect_rejects_invalid_url_without_leaking_it() -> Result<()> {
    let err = connect_to_postgres("not a url with secret-password")
        .await
        .expect_err("invalid URL should fail");

    assert!(!format!("{err:#}").contains("secret-password"));

    Ok(())
}
