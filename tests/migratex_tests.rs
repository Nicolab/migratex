// This file is part of "Migratex - A Migrations Toolkit".
//
// This source code is licensed under the MIT license, please view the LICENSE
// file distributed with this source code. For the full
// information and documentation: https://github.com/nicolab/migratex
// -----------------------------------------------------------------------------

// -- Tests for Migratex core functionality.

#![cfg(feature = "json")]

mod common;

use okerr::Result;
use common::{TempDir, TestContext, TestMetadata, create_test_migrations};
use migratex::{MetaStatus, Metadata, Migratex};

#[tokio::test]
async fn test_migratex_new() -> Result<()> {
    let temp = TempDir::new()?;
    let path = temp.metadata_path();

    let mut ctx = TestContext::new();
    let mut meta = TestMetadata::load_or_init(&path)?;
    let migrations = create_test_migrations(3);

    let mx = Migratex::new(&mut ctx, &mut meta, migrations);

    assert_eq!(mx.metadata().version(), 0);
    assert_eq!(mx.latest_version(), 3);

    Ok(())
}

#[tokio::test]
async fn test_migrate_to_latest() -> Result<()> {
    let temp = TempDir::new()?;
    let path = temp.metadata_path();

    let mut ctx = TestContext::new();
    let mut meta = TestMetadata::load_or_init(&path)?;
    let migrations = create_test_migrations(5);

    let mut mx = Migratex::new(&mut ctx, &mut meta, migrations);

    // Migrate to latest
    mx.migrate_to_latest().await?;

    assert_eq!(meta.version(), 5);
    assert_eq!(meta.status(), MetaStatus::Clean);
    assert_eq!(ctx.applied_migrations, vec![1, 2, 3, 4, 5]);

    Ok(())
}

#[tokio::test]
async fn test_migrate_to_specific_version() -> Result<()> {
    let temp = TempDir::new()?;
    let path = temp.metadata_path();

    let mut ctx = TestContext::new();
    let mut meta = TestMetadata::load_or_init(&path)?;
    let migrations = create_test_migrations(10);

    let mut mx = Migratex::new(&mut ctx, &mut meta, migrations);

    // Migrate to version 3
    mx.migrate_to(3).await?;

    assert_eq!(meta.version(), 3);
    assert_eq!(meta.status(), MetaStatus::Clean);
    assert_eq!(ctx.applied_migrations, vec![1, 2, 3]);

    Ok(())
}

#[tokio::test]
async fn test_migrate_to_same_version_is_noop() -> Result<()> {
    let temp = TempDir::new()?;
    let path = temp.metadata_path();

    let mut ctx = TestContext::new();
    let mut meta = TestMetadata::load_or_init(&path)?;
    let migrations = create_test_migrations(5);

    let mut mx = Migratex::new(&mut ctx, &mut meta, migrations);

    // Migrate to version 3
    mx.migrate_to(3).await?;

    // Drop mx to release borrows
    drop(mx);

    let initial_updated = meta.updated_at().to_string();
    assert_eq!(meta.version(), 3);
    assert_eq!(ctx.applied_migrations, vec![1, 2, 3]);

    // Wait a bit
    std::thread::sleep(std::time::Duration::from_millis(10));

    // Create new Migratex instance
    let migrations2 = create_test_migrations(5);
    let mut mx = Migratex::new(&mut ctx, &mut meta, migrations2);

    // Migrate to same version should be no-op
    mx.migrate_to(3).await?;

    drop(mx);

    assert_eq!(meta.version(), 3);
    assert_eq!(ctx.applied_migrations, vec![1, 2, 3]);
    // updated_at should not have changed
    assert_eq!(meta.updated_at(), initial_updated);

    Ok(())
}

#[tokio::test]
async fn test_migrate_next() -> Result<()> {
    let temp = TempDir::new()?;
    let path = temp.metadata_path();

    let mut ctx = TestContext::new();
    let mut meta = TestMetadata::load_or_init(&path)?;
    let migrations = create_test_migrations(5);

    let mut mx = Migratex::new(&mut ctx, &mut meta, migrations);

    // Start at version 0, migrate next
    mx.migrate_next().await?;
    drop(mx);
    assert_eq!(meta.version(), 1);
    assert_eq!(ctx.applied_migrations, vec![1]);

    let migrations2 = create_test_migrations(5);
    let mut mx = Migratex::new(&mut ctx, &mut meta, migrations2);
    mx.migrate_next().await?;
    drop(mx);
    assert_eq!(meta.version(), 2);
    assert_eq!(ctx.applied_migrations, vec![1, 2]);

    let migrations3 = create_test_migrations(5);
    let mut mx = Migratex::new(&mut ctx, &mut meta, migrations3);
    mx.migrate_next().await?;
    drop(mx);
    assert_eq!(meta.version(), 3);
    assert_eq!(ctx.applied_migrations, vec![1, 2, 3]);

    Ok(())
}

#[tokio::test]
async fn test_migrate_next_at_latest_is_noop() -> Result<()> {
    let temp = TempDir::new()?;
    let path = temp.metadata_path();

    let mut ctx = TestContext::new();
    let mut meta = TestMetadata::load_or_init(&path)?;
    let migrations = create_test_migrations(3);

    let mut mx = Migratex::new(&mut ctx, &mut meta, migrations);

    // Migrate to latest
    mx.migrate_to_latest().await?;
    drop(mx);
    assert_eq!(meta.version(), 3);

    // Migrate next should be no-op
    let migrations2 = create_test_migrations(3);
    let mut mx = Migratex::new(&mut ctx, &mut meta, migrations2);
    mx.migrate_next().await?;
    drop(mx);
    assert_eq!(meta.version(), 3);
    assert_eq!(ctx.applied_migrations, vec![1, 2, 3]);

    Ok(())
}

#[tokio::test]
async fn test_migrate_prev() -> Result<()> {
    let temp = TempDir::new()?;
    let path = temp.metadata_path();

    let mut ctx = TestContext::new();
    let mut meta = TestMetadata::load_or_init(&path)?;
    let migrations = create_test_migrations(5);

    let mut mx = Migratex::new(&mut ctx, &mut meta, migrations);

    // Migrate to latest
    mx.migrate_to_latest().await?;
    drop(mx);
    assert_eq!(meta.version(), 5);

    // Migrate prev
    let migrations2 = create_test_migrations(5);
    let mut mx = Migratex::new(&mut ctx, &mut meta, migrations2);
    mx.migrate_prev().await?;
    drop(mx);
    assert_eq!(meta.version(), 4);
    assert_eq!(ctx.applied_migrations, vec![1, 2, 3, 4]);

    let migrations3 = create_test_migrations(5);
    let mut mx = Migratex::new(&mut ctx, &mut meta, migrations3);
    mx.migrate_prev().await?;
    drop(mx);
    assert_eq!(meta.version(), 3);
    assert_eq!(ctx.applied_migrations, vec![1, 2, 3]);

    Ok(())
}

#[tokio::test]
async fn test_migrate_prev_at_zero_is_noop() -> Result<()> {
    let temp = TempDir::new()?;
    let path = temp.metadata_path();

    let mut ctx = TestContext::new();
    let mut meta = TestMetadata::load_or_init(&path)?;
    let migrations = create_test_migrations(3);

    let mut mx = Migratex::new(&mut ctx, &mut meta, migrations);

    // Migrate prev should be no-op
    mx.migrate_prev().await?;
    drop(mx);
    assert_eq!(meta.version(), 0);
    assert!(ctx.applied_migrations.is_empty());

    Ok(())
}

#[tokio::test]
async fn test_migrate_to_zero() -> Result<()> {
    let temp = TempDir::new()?;
    let path = temp.metadata_path();

    let mut ctx = TestContext::new();
    let mut meta = TestMetadata::load_or_init(&path)?;
    let migrations = create_test_migrations(5);

    let mut mx = Migratex::new(&mut ctx, &mut meta, migrations);

    // Migrate to latest
    mx.migrate_to_latest().await?;
    drop(mx);
    assert_eq!(meta.version(), 5);
    assert_eq!(ctx.applied_migrations, vec![1, 2, 3, 4, 5]);

    // Rollback everything
    let migrations2 = create_test_migrations(5);
    let mut mx = Migratex::new(&mut ctx, &mut meta, migrations2);
    mx.migrate_to_zero().await?;
    drop(mx);
    assert_eq!(meta.version(), 0);
    assert_eq!(meta.status(), MetaStatus::Clean);
    assert!(ctx.applied_migrations.is_empty());

    Ok(())
}

#[tokio::test]
async fn test_migrate_down() -> Result<()> {
    let temp = TempDir::new()?;
    let path = temp.metadata_path();

    let mut ctx = TestContext::new();
    let mut meta = TestMetadata::load_or_init(&path)?;
    let migrations = create_test_migrations(5);

    let mut mx = Migratex::new(&mut ctx, &mut meta, migrations);

    // Migrate to latest
    mx.migrate_to_latest().await?;
    drop(mx);
    assert_eq!(meta.version(), 5);

    // Migrate down to version 2
    let migrations2 = create_test_migrations(5);
    let mut mx = Migratex::new(&mut ctx, &mut meta, migrations2);
    mx.migrate_to(2).await?;
    drop(mx);
    assert_eq!(meta.version(), 2);
    assert_eq!(meta.status(), MetaStatus::Clean);
    assert_eq!(ctx.applied_migrations, vec![1, 2]);

    Ok(())
}

#[tokio::test]
async fn test_migrate_up_then_down_then_up() -> Result<()> {
    let temp = TempDir::new()?;
    let path = temp.metadata_path();

    let mut ctx = TestContext::new();
    let mut meta = TestMetadata::load_or_init(&path)?;
    let migrations = create_test_migrations(5);

    let mut mx = Migratex::new(&mut ctx, &mut meta, migrations);

    // Up to 3
    mx.migrate_to(3).await?;
    drop(mx);
    assert_eq!(meta.version(), 3);
    assert_eq!(ctx.applied_migrations, vec![1, 2, 3]);

    // Down to 1
    let migrations2 = create_test_migrations(5);
    let mut mx = Migratex::new(&mut ctx, &mut meta, migrations2);
    mx.migrate_to(1).await?;
    drop(mx);
    assert_eq!(meta.version(), 1);
    assert_eq!(ctx.applied_migrations, vec![1]);

    // Up to 5
    let migrations3 = create_test_migrations(5);
    let mut mx = Migratex::new(&mut ctx, &mut meta, migrations3);
    mx.migrate_to(5).await?;
    drop(mx);
    assert_eq!(meta.version(), 5);
    assert_eq!(ctx.applied_migrations, vec![1, 2, 3, 4, 5]);

    Ok(())
}

#[tokio::test]
async fn test_migration_failure_marks_failed() -> Result<()> {
    let temp = TempDir::new()?;
    let path = temp.metadata_path();

    // Create context that will fail at version 3
    let mut ctx = TestContext::with_fail_at(3);
    let mut meta = TestMetadata::load_or_init(&path)?;
    let migrations = create_test_migrations(5);

    let mut mx = Migratex::new(&mut ctx, &mut meta, migrations);

    // Try to migrate to latest, should fail at version 3
    let result = mx.migrate_to_latest().await;
    assert!(result.is_err());

    // Metadata should be marked as failed
    assert_eq!(meta.status(), MetaStatus::Failed);

    // Version should be at 2 (last successful migration)
    assert_eq!(meta.version(), 2);
    assert_eq!(ctx.applied_migrations, vec![1, 2]);

    Ok(())
}

#[tokio::test]
async fn test_migration_failure_during_down() -> Result<()> {
    let temp = TempDir::new()?;
    let path = temp.metadata_path();

    let mut ctx = TestContext::new();
    let mut meta = TestMetadata::load_or_init(&path)?;
    let migrations = create_test_migrations(5);

    let mut mx = Migratex::new(&mut ctx, &mut meta, migrations);

    // Migrate up successfully
    mx.migrate_to_latest().await?;
    drop(mx);
    assert_eq!(meta.version(), 5);

    // Now create new context with failure
    let mut ctx2 = TestContext::with_fail_at(3);
    ctx2.applied_migrations = ctx.applied_migrations.clone();

    // Try to migrate down, should fail at version 3
    let migrations2 = create_test_migrations(5);
    let mut mx = Migratex::new(&mut ctx2, &mut meta, migrations2);
    let result = mx.migrate_to(1).await;
    assert!(result.is_err());
    drop(mx);

    // Status should be failed
    assert_eq!(meta.status(), MetaStatus::Failed);

    Ok(())
}

#[tokio::test]
async fn test_latest_version_with_empty_migrations() -> Result<()> {
    let temp = TempDir::new()?;
    let path = temp.metadata_path();

    let mut ctx = TestContext::new();
    let mut meta = TestMetadata::load_or_init(&path)?;
    let migrations: Vec<migratex::BoxMigration<TestContext>> = vec![];

    let mx = Migratex::new(&mut ctx, &mut meta, migrations);

    assert_eq!(mx.latest_version(), 0);

    Ok(())
}

#[tokio::test]
async fn test_migrate_to_latest_with_no_migrations() -> Result<()> {
    let temp = TempDir::new()?;
    let path = temp.metadata_path();

    let mut ctx = TestContext::new();
    let mut meta = TestMetadata::load_or_init(&path)?;
    let migrations: Vec<migratex::BoxMigration<TestContext>> = vec![];

    let mut mx = Migratex::new(&mut ctx, &mut meta, migrations);

    mx.migrate_to_latest().await?;

    assert_eq!(meta.version(), 0);
    assert_eq!(meta.status(), MetaStatus::Clean);

    Ok(())
}

#[tokio::test]
async fn test_migrate_to_future_version_beyond_latest() -> Result<()> {
    let temp = TempDir::new()?;
    let path = temp.metadata_path();

    let mut ctx = TestContext::new();
    let mut meta = TestMetadata::load_or_init(&path)?;
    let migrations = create_test_migrations(3);

    let mut mx = Migratex::new(&mut ctx, &mut meta, migrations);

    // Try to migrate to version 10 (beyond latest which is 3)
    mx.migrate_to(10).await?;

    // Should only migrate to latest (3)
    assert_eq!(meta.version(), 3);
    assert_eq!(ctx.applied_migrations, vec![1, 2, 3]);

    Ok(())
}

#[tokio::test]
async fn test_migrate_to_negative_version() -> Result<()> {
    let temp = TempDir::new()?;
    let path = temp.metadata_path();

    let mut ctx = TestContext::new();
    let mut meta = TestMetadata::load_or_init(&path)?;
    let migrations = create_test_migrations(5);

    let mut mx = Migratex::new(&mut ctx, &mut meta, migrations);

    // Migrate up first
    mx.migrate_to(3).await?;
    drop(mx);
    assert_eq!(meta.version(), 3);

    // Try to migrate to negative version
    let migrations2 = create_test_migrations(5);
    let mut mx = Migratex::new(&mut ctx, &mut meta, migrations2);
    mx.migrate_to(-1).await?;
    drop(mx);

    // Should migrate to 0
    assert_eq!(meta.version(), 0);
    assert!(ctx.applied_migrations.is_empty());

    Ok(())
}

#[tokio::test]
async fn test_status_transitions() -> Result<()> {
    let temp = TempDir::new()?;
    let path = temp.metadata_path();

    let mut ctx = TestContext::new();
    let mut meta = TestMetadata::load_or_init(&path)?;
    let migrations = create_test_migrations(3);

    let mut mx = Migratex::new(&mut ctx, &mut meta, migrations);

    // During migration, status should transition
    mx.migrate_to(2).await?;
    drop(mx);

    // After successful migration, should be clean
    assert_eq!(meta.status(), MetaStatus::Clean);

    Ok(())
}

#[tokio::test]
async fn test_context_getter() -> Result<()> {
    let temp = TempDir::new()?;
    let path = temp.metadata_path();

    let mut ctx = TestContext::new();
    ctx.applied_migrations.push(99);

    let mut meta = TestMetadata::load_or_init(&path)?;
    let migrations = create_test_migrations(3);

    let mx = Migratex::new(&mut ctx, &mut meta, migrations);

    // Test context getter
    assert_eq!(mx.context().applied_migrations, vec![99]);

    Ok(())
}

#[tokio::test]
async fn test_metadata_getter() -> Result<()> {
    let temp = TempDir::new()?;
    let path = temp.metadata_path();

    let mut ctx = TestContext::new();
    let mut meta = TestMetadata::load_or_init(&path)?;
    meta.set_version(42);

    let migrations = create_test_migrations(3);

    let mx = Migratex::new(&mut ctx, &mut meta, migrations);

    // Test metadata getter
    assert_eq!(mx.metadata().version(), 42);

    Ok(())
}

#[tokio::test]
async fn test_multiple_sequential_migrations() -> Result<()> {
    let temp = TempDir::new()?;
    let path = temp.metadata_path();

    let mut ctx = TestContext::new();
    let mut meta = TestMetadata::load_or_init(&path)?;
    let migrations = create_test_migrations(10);

    let mut mx = Migratex::new(&mut ctx, &mut meta, migrations);

    // Migrate all at once
    mx.migrate_to(10).await?;
    drop(mx);

    assert_eq!(meta.version(), 10);
    assert_eq!(meta.status(), MetaStatus::Clean);
    assert_eq!(ctx.applied_migrations, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

    Ok(())
}

#[tokio::test]
async fn test_rollback_and_reapply() -> Result<()> {
    let temp = TempDir::new()?;
    let path = temp.metadata_path();

    let mut ctx = TestContext::new();
    let mut meta = TestMetadata::load_or_init(&path)?;
    let migrations = create_test_migrations(5);

    let mut mx = Migratex::new(&mut ctx, &mut meta, migrations);

    // Apply all
    mx.migrate_to_latest().await?;
    drop(mx);
    assert_eq!(meta.version(), 5);

    // Rollback to 2
    let migrations2 = create_test_migrations(5);
    let mut mx = Migratex::new(&mut ctx, &mut meta, migrations2);
    mx.migrate_to(2).await?;
    drop(mx);
    assert_eq!(meta.version(), 2);
    assert_eq!(ctx.applied_migrations, vec![1, 2]);

    // Reapply to 5
    let migrations3 = create_test_migrations(5);
    let mut mx = Migratex::new(&mut ctx, &mut meta, migrations3);
    mx.migrate_to_latest().await?;
    drop(mx);
    assert_eq!(meta.version(), 5);
    assert_eq!(ctx.applied_migrations, vec![1, 2, 3, 4, 5]);

    Ok(())
}
