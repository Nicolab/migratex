// This file is part of "Migratex - A Migrations Toolkit".
//
// This source code is licensed under the MIT license, please view the LICENSE
// file distributed with this source code. For the full
// information and documentation: https://github.com/nicolab/migratex
// -----------------------------------------------------------------------------

// -- Tests for Metadata trait functionality.

#![cfg(feature = "json")]

mod common;

use anyhow::Result;
use common::{TempDir, TestMetadata};
use migratex::{MetaStatus, Metadata};

#[test]
fn test_metadata_default_values() -> Result<()> {
    let temp = TempDir::new()?;
    let path = temp.metadata_path();

    let meta = TestMetadata::load_or_init(&path)?;

    assert_eq!(meta.version(), 0);
    assert_eq!(meta.status(), MetaStatus::Clean);
    assert!(!meta.created_at().is_empty());
    assert!(!meta.updated_at().is_empty());
    assert_eq!(meta.created_at(), meta.updated_at());

    Ok(())
}

#[test]
fn test_metadata_set_version() -> Result<()> {
    let temp = TempDir::new()?;
    let path = temp.metadata_path();

    let mut meta = TestMetadata::load_or_init(&path)?;
    let initial_updated = meta.updated_at().to_string();

    std::thread::sleep(std::time::Duration::from_millis(10));

    meta.set_version(42);

    assert_eq!(meta.version(), 42);
    assert_ne!(meta.updated_at(), initial_updated);

    Ok(())
}

#[test]
fn test_metadata_set_app_version() -> Result<()> {
    let temp = TempDir::new()?;
    let path = temp.metadata_path();

    let mut meta = TestMetadata::load_or_init(&path)?;
    let initial_updated = meta.updated_at().to_string();

    std::thread::sleep(std::time::Duration::from_millis(10));

    meta.set_app_version("2.0.0".to_string());

    assert_eq!(meta.app_version(), "2.0.0");
    assert_ne!(meta.updated_at(), initial_updated);

    Ok(())
}

#[test]
fn test_metadata_set_status() -> Result<()> {
    let temp = TempDir::new()?;
    let path = temp.metadata_path();

    let mut meta = TestMetadata::load_or_init(&path)?;
    let initial_updated = meta.updated_at().to_string();

    std::thread::sleep(std::time::Duration::from_millis(10));

    meta.set_status(MetaStatus::Migrating);

    assert_eq!(meta.status(), MetaStatus::Migrating);
    assert_ne!(meta.updated_at(), initial_updated);

    Ok(())
}

#[test]
fn test_metadata_mark_clean() -> Result<()> {
    let temp = TempDir::new()?;
    let path = temp.metadata_path();

    let mut meta = TestMetadata::load_or_init(&path)?;
    meta.set_status(MetaStatus::Failed);

    meta.mark_clean();

    assert_eq!(meta.status(), MetaStatus::Clean);

    Ok(())
}

#[test]
fn test_metadata_mark_migrating() -> Result<()> {
    let temp = TempDir::new()?;
    let path = temp.metadata_path();

    let mut meta = TestMetadata::load_or_init(&path)?;

    meta.mark_migrating();

    assert_eq!(meta.status(), MetaStatus::Migrating);

    Ok(())
}

#[test]
fn test_metadata_mark_failed() -> Result<()> {
    let temp = TempDir::new()?;
    let path = temp.metadata_path();

    let mut meta = TestMetadata::load_or_init(&path)?;

    meta.mark_failed();

    assert_eq!(meta.status(), MetaStatus::Failed);

    Ok(())
}

#[test]
fn test_metadata_touch_updated() -> Result<()> {
    let temp = TempDir::new()?;
    let path = temp.metadata_path();

    let mut meta = TestMetadata::load_or_init(&path)?;
    let initial_updated = meta.updated_at().to_string();

    std::thread::sleep(std::time::Duration::from_millis(10));

    meta.touch_updated();

    assert_ne!(meta.updated_at(), initial_updated);

    Ok(())
}

#[test]
fn test_metadata_created_at_immutable() -> Result<()> {
    let temp = TempDir::new()?;
    let path = temp.metadata_path();

    let mut meta = TestMetadata::load_or_init(&path)?;
    let created_at = meta.created_at().to_string();

    // Multiple operations should not change created_at
    meta.set_version(1);
    assert_eq!(meta.created_at(), created_at);

    meta.set_app_version("1.0.0".to_string());
    assert_eq!(meta.created_at(), created_at);

    meta.mark_migrating();
    assert_eq!(meta.created_at(), created_at);

    meta.mark_clean();
    assert_eq!(meta.created_at(), created_at);

    Ok(())
}

#[test]
fn test_metadata_updated_at_changes() -> Result<()> {
    let temp = TempDir::new()?;
    let path = temp.metadata_path();

    let mut meta = TestMetadata::load_or_init(&path)?;
    let mut last_updated = meta.updated_at().to_string();

    let operations = vec![
        |m: &mut TestMetadata| m.set_version(1),
        |m: &mut TestMetadata| m.set_app_version("1.0.0".to_string()),
        |m: &mut TestMetadata| m.mark_migrating(),
        |m: &mut TestMetadata| m.mark_clean(),
    ];

    for op in operations {
        std::thread::sleep(std::time::Duration::from_millis(10));
        op(&mut meta);
        assert_ne!(meta.updated_at(), last_updated);
        last_updated = meta.updated_at().to_string();
    }

    Ok(())
}

#[test]
fn test_metadata_status_enum_values() {
    assert_eq!(MetaStatus::default(), MetaStatus::Clean);

    let clean = MetaStatus::Clean;
    let migrating = MetaStatus::Migrating;
    let failed = MetaStatus::Failed;

    assert_ne!(clean, migrating);
    assert_ne!(clean, failed);
    assert_ne!(migrating, failed);
}

#[test]
fn test_metadata_version_mut() -> Result<()> {
    let temp = TempDir::new()?;
    let path = temp.metadata_path();

    let mut meta = TestMetadata::load_or_init(&path)?;

    *meta.version_mut() = 999;

    assert_eq!(meta.version(), 999);

    Ok(())
}

#[test]
fn test_metadata_app_version_mut() -> Result<()> {
    let temp = TempDir::new()?;
    let path = temp.metadata_path();

    let mut meta = TestMetadata::load_or_init(&path)?;

    *meta.app_version_mut() = "custom-version".to_string();

    assert_eq!(meta.app_version(), "custom-version");

    Ok(())
}

#[test]
fn test_metadata_status_mut() -> Result<()> {
    let temp = TempDir::new()?;
    let path = temp.metadata_path();

    let mut meta = TestMetadata::load_or_init(&path)?;

    *meta.status_mut() = MetaStatus::Migrating;

    assert_eq!(meta.status(), MetaStatus::Migrating);

    Ok(())
}

#[test]
fn test_metadata_persistence() -> Result<()> {
    let temp = TempDir::new()?;
    let path = temp.metadata_path();

    // Create and modify metadata
    let mut meta = TestMetadata::load_or_init(&path)?;
    meta.set_version(100);
    meta.set_app_version("5.0.0".to_string());
    meta.mark_failed();

    let created = meta.created_at().to_string();
    let updated = meta.updated_at().to_string();

    meta.save(&path)?;

    // Load in a new instance
    let loaded = TestMetadata::load_or_init(&path)?;

    assert_eq!(loaded.version(), 100);
    assert_eq!(loaded.app_version(), "5.0.0");
    assert_eq!(loaded.status(), MetaStatus::Failed);
    assert_eq!(loaded.created_at(), created);
    assert_eq!(loaded.updated_at(), updated);

    Ok(())
}

#[test]
fn test_metadata_multiple_status_transitions() -> Result<()> {
    let temp = TempDir::new()?;
    let path = temp.metadata_path();

    let mut meta = TestMetadata::load_or_init(&path)?;

    // Clean -> Migrating -> Failed -> Clean
    assert_eq!(meta.status(), MetaStatus::Clean);

    meta.mark_migrating();
    assert_eq!(meta.status(), MetaStatus::Migrating);

    meta.mark_failed();
    assert_eq!(meta.status(), MetaStatus::Failed);

    meta.mark_clean();
    assert_eq!(meta.status(), MetaStatus::Clean);

    Ok(())
}

#[test]
fn test_metadata_zero_version() -> Result<()> {
    let temp = TempDir::new()?;
    let path = temp.metadata_path();

    let meta = TestMetadata::load_or_init(&path)?;

    assert_eq!(meta.version(), 0);

    Ok(())
}

#[test]
fn test_metadata_negative_version() -> Result<()> {
    let temp = TempDir::new()?;
    let path = temp.metadata_path();

    let mut meta = TestMetadata::load_or_init(&path)?;
    meta.set_version(-5);

    assert_eq!(meta.version(), -5);

    Ok(())
}

#[test]
fn test_metadata_extreme_version_values() -> Result<()> {
    let temp = TempDir::new()?;
    let path = temp.metadata_path();

    let mut meta = TestMetadata::load_or_init(&path)?;

    // Test i32::MAX
    meta.set_version(i32::MAX);
    assert_eq!(meta.version(), i32::MAX);

    // Test i32::MIN
    meta.set_version(i32::MIN);
    assert_eq!(meta.version(), i32::MIN);

    Ok(())
}

#[test]
fn test_metadata_empty_strings() -> Result<()> {
    let temp = TempDir::new()?;
    let path = temp.metadata_path();

    let mut meta = TestMetadata::load_or_init(&path)?;
    meta.set_app_version("".to_string());

    assert_eq!(meta.app_version(), "");

    Ok(())
}

#[test]
fn test_metadata_special_characters_in_app_version() -> Result<()> {
    let temp = TempDir::new()?;
    let path = temp.metadata_path();

    let mut meta = TestMetadata::load_or_init(&path)?;
    let special_version = "1.0.0-beta+build.123-🚀";
    meta.set_app_version(special_version.to_string());

    assert_eq!(meta.app_version(), special_version);

    Ok(())
}
