// This file is part of "Migratex - A Migrations Toolkit".
//
// This source code is licensed under the MIT license, please view the LICENSE
// file distributed with this source code. For the full
// information and documentation: https://github.com/nicolab/migratex
// -----------------------------------------------------------------------------

// -- Tests for JsonStore functionality.

#![cfg(feature = "json")]

mod common;

use okerr::Result;
use common::{TempDir, TestMetadata};
use migratex::{MetaStatus, Metadata};
use std::fs;

#[test]
fn test_json_store_init_creates_new_file() -> Result<()> {
    let temp = TempDir::new()?;
    let path = temp.metadata_path();

    // File should not exist initially
    assert!(!path.exists());

    // Load or init should create new metadata
    let meta = TestMetadata::load_or_init(&path)?;

    // File should now exist
    assert!(path.exists());

    // Default values should be set
    assert_eq!(meta.version(), 0);
    assert_eq!(meta.status(), MetaStatus::Clean);
    assert!(!meta.created_at().is_empty());
    assert!(!meta.updated_at().is_empty());

    Ok(())
}

#[test]
fn test_json_store_load_existing_file() -> Result<()> {
    let temp = TempDir::new()?;
    let path = temp.metadata_path();

    // Create initial metadata
    let mut meta1 = TestMetadata::load_or_init(&path)?;
    meta1.set_version(5);
    meta1.set_app_version("1.2.3".to_string());
    meta1.save(&path)?;

    // Load existing metadata
    let meta2 = TestMetadata::load_or_init(&path)?;

    // Values should match
    assert_eq!(meta2.version(), 5);
    assert_eq!(meta2.app_version(), "1.2.3");
    assert_eq!(meta2.status(), MetaStatus::Clean);
    assert_eq!(meta2.created_at(), meta1.created_at());

    Ok(())
}

#[test]
fn test_json_store_save_preserves_data() -> Result<()> {
    let temp = TempDir::new()?;
    let path = temp.metadata_path();

    // Create and save metadata
    let mut meta = TestMetadata::load_or_init(&path)?;
    meta.set_version(42);
    meta.set_app_version("2.0.0".to_string());
    meta.mark_migrating();

    let created_at = meta.created_at().to_string();
    meta.save(&path)?;

    // Reload and verify
    let loaded = TestMetadata::load_or_init(&path)?;
    assert_eq!(loaded.version(), 42);
    assert_eq!(loaded.app_version(), "2.0.0");
    assert_eq!(loaded.status(), MetaStatus::Migrating);
    assert_eq!(loaded.created_at(), created_at);

    Ok(())
}

#[test]
fn test_json_store_data_integrity() -> Result<()> {
    let temp = TempDir::new()?;
    let path = temp.metadata_path();

    // Create metadata with specific values
    let mut meta = TestMetadata::load_or_init(&path)?;
    meta.set_version(100);
    meta.set_app_version("3.14.159".to_string());
    meta.mark_failed();
    meta.save(&path)?;

    // Read raw JSON file
    let json_content = fs::read_to_string(&path)?;
    let parsed: serde_json::Value = serde_json::from_str(&json_content)?;

    // Verify JSON structure
    assert_eq!(parsed["version"], 100);
    assert_eq!(parsed["app_version"], "3.14.159");
    assert_eq!(parsed["status"], "Failed");
    assert!(parsed["created_at"].is_string());
    assert!(parsed["updated_at"].is_string());

    Ok(())
}

#[test]
fn test_json_store_multiple_saves() -> Result<()> {
    let temp = TempDir::new()?;
    let path = temp.metadata_path();

    let mut meta = TestMetadata::load_or_init(&path)?;

    // Save multiple times with different values
    for i in 1..=10 {
        meta.set_version(i);
        meta.save(&path)?;

        let loaded = TestMetadata::load_or_init(&path)?;
        assert_eq!(loaded.version(), i);
    }

    Ok(())
}

#[test]
fn test_json_store_all_status_values() -> Result<()> {
    let temp = TempDir::new()?;
    let path = temp.metadata_path();

    let statuses = vec![MetaStatus::Clean, MetaStatus::Migrating, MetaStatus::Failed];

    for status in statuses {
        let mut meta = TestMetadata::load_or_init(&path)?;
        meta.set_status(status);
        meta.save(&path)?;

        let loaded = TestMetadata::load_or_init(&path)?;
        assert_eq!(loaded.status(), status);
    }

    Ok(())
}

#[test]
fn test_json_store_preserves_timestamps() -> Result<()> {
    let temp = TempDir::new()?;
    let path = temp.metadata_path();

    // Create initial metadata
    let mut meta = TestMetadata::load_or_init(&path)?;
    let initial_created = meta.created_at().to_string();
    let initial_updated = meta.updated_at().to_string();

    // created_at and updated_at should be the same initially
    assert_eq!(initial_created, initial_updated);

    meta.save(&path)?;

    // Wait a tiny bit to ensure timestamp would change
    std::thread::sleep(std::time::Duration::from_millis(10));

    // Update version (which updates updated_at)
    meta.set_version(1);
    meta.save(&path)?;

    // Reload
    let loaded = TestMetadata::load_or_init(&path)?;

    // created_at should remain the same
    assert_eq!(loaded.created_at(), initial_created);

    // updated_at should be different
    assert_ne!(loaded.updated_at(), initial_updated);

    Ok(())
}

#[test]
fn test_json_store_with_unicode_content() -> Result<()> {
    let temp = TempDir::new()?;
    let path = temp.metadata_path();

    let mut meta = TestMetadata::load_or_init(&path)?;
    meta.set_app_version("1.0.0-café-🚀".to_string());
    meta.save(&path)?;

    let loaded = TestMetadata::load_or_init(&path)?;
    assert_eq!(loaded.app_version(), "1.0.0-café-🚀");

    Ok(())
}

#[test]
fn test_json_store_empty_app_version() -> Result<()> {
    let temp = TempDir::new()?;
    let path = temp.metadata_path();

    let mut meta = TestMetadata::load_or_init(&path)?;
    meta.set_app_version("".to_string());
    meta.save(&path)?;

    let loaded = TestMetadata::load_or_init(&path)?;
    assert_eq!(loaded.app_version(), "");

    Ok(())
}

#[test]
fn test_json_store_negative_version() -> Result<()> {
    let temp = TempDir::new()?;
    let path = temp.metadata_path();

    let mut meta = TestMetadata::load_or_init(&path)?;
    meta.set_version(-1);
    meta.save(&path)?;

    let loaded = TestMetadata::load_or_init(&path)?;
    assert_eq!(loaded.version(), -1);

    Ok(())
}

#[test]
fn test_json_store_large_version_number() -> Result<()> {
    let temp = TempDir::new()?;
    let path = temp.metadata_path();

    let mut meta = TestMetadata::load_or_init(&path)?;
    meta.set_version(i32::MAX);
    meta.save(&path)?;

    let loaded = TestMetadata::load_or_init(&path)?;
    assert_eq!(loaded.version(), i32::MAX);

    Ok(())
}

#[test]
fn test_json_store_invalid_json_file() -> Result<()> {
    let temp = TempDir::new()?;
    let path = temp.metadata_path();

    // Write invalid JSON
    fs::write(&path, "{ invalid json")?;

    // Loading should fail
    let result = TestMetadata::load_or_init(&path);
    assert!(result.is_err());

    Ok(())
}

#[test]
fn test_json_store_concurrent_access() -> Result<()> {
    let temp = TempDir::new()?;
    let path = temp.metadata_path();

    // Initialize metadata
    let mut meta1 = TestMetadata::load_or_init(&path)?;
    meta1.set_version(1);
    meta1.save(&path)?;

    // Load again (simulating concurrent access)
    let mut meta2 = TestMetadata::load_or_init(&path)?;
    assert_eq!(meta2.version(), 1);

    // Both modify and save
    meta1.set_version(2);
    meta1.save(&path)?;

    meta2.set_version(3);
    meta2.save(&path)?;

    // Last write wins
    let loaded = TestMetadata::load_or_init(&path)?;
    assert_eq!(loaded.version(), 3);

    Ok(())
}
