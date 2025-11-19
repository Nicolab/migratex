// This file is part of "Migratex - A Migrations Toolkit".
//
// This source code is licensed under the MIT license, please view the LICENSE
// file distributed with this source code. For the full
// information and documentation: https://github.com/nicolab/migratex
// -----------------------------------------------------------------------------

// -- Common test utilities and helpers for Migratex tests.

use async_trait::async_trait;
use migratex::Migration;
use okerr::Result;
use std::path::{Path, PathBuf};

/// Test metadata - using JsonMetadata directly
#[cfg(feature = "json")]
pub type TestMetadata = migratex::JsonMetadata;

/// Test migration context that tracks applied migrations
#[derive(Debug, Default, Clone)]
pub struct TestContext {
    pub applied_migrations: Vec<i32>,
    pub should_fail_at_version: Option<i32>,
}

impl TestContext {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }

    #[allow(dead_code)]
    pub fn with_fail_at(version: i32) -> Self {
        Self {
            applied_migrations: Vec::new(),
            should_fail_at_version: Some(version),
        }
    }

    pub fn record_up(&mut self, version: i32) {
        self.applied_migrations.push(version);
    }

    pub fn record_down(&mut self, version: i32) {
        self.applied_migrations.retain(|&v| v != version);
    }

    #[allow(dead_code)]
    pub fn is_applied(&self, version: i32) -> bool {
        self.applied_migrations.contains(&version)
    }
}

/// Test migration that records execution
pub struct TestMigration {
    pub version: i32,
    #[allow(dead_code)]
    name: String,
}

#[allow(dead_code)]
impl TestMigration {
    pub fn new(version: i32, name: impl Into<String>) -> Self {
        Self {
            version,
            name: name.into(),
        }
    }
}

#[async_trait]
impl Migration<TestContext> for TestMigration {
    fn version(&self) -> i32 {
        self.version
    }

    async fn up(&self, ctx: &mut TestContext) -> Result<()> {
        if ctx.should_fail_at_version == Some(self.version) {
            okerr::fail!("Intentional failure at version {}", self.version);
        }
        ctx.record_up(self.version);
        Ok(())
    }

    async fn down(&self, ctx: &mut TestContext) -> Result<()> {
        if ctx.should_fail_at_version == Some(self.version) {
            okerr::fail!("Intentional failure at version {}", self.version);
        }
        ctx.record_down(self.version);
        Ok(())
    }
}

/// Helper to create a temporary directory for tests
pub struct TempDir {
    path: PathBuf,
}

impl TempDir {
    pub fn new() -> Result<Self> {
        let path = std::env::temp_dir().join(format!("migratex_test_{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&path)?;
        Ok(Self { path })
    }

    #[allow(dead_code)]
    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn metadata_path(&self) -> PathBuf {
        self.path.join("metadata.json")
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.path);
    }
}

/// Create a list of test migrations
#[allow(dead_code)]
pub fn create_test_migrations(count: i32) -> Vec<migratex::BoxMigration<TestContext>> {
    (1..=count)
        .map(|i| {
            Box::new(TestMigration::new(i, format!("Migration_{}", i)))
                as migratex::BoxMigration<TestContext>
        })
        .collect()
}

mod uuid {
    use std::sync::atomic::{AtomicU64, Ordering};

    static COUNTER: AtomicU64 = AtomicU64::new(0);

    pub struct Uuid(u64);

    impl Uuid {
        pub fn new_v4() -> Self {
            Self(COUNTER.fetch_add(1, Ordering::SeqCst))
        }
    }

    impl std::fmt::Display for Uuid {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:016x}", self.0)
        }
    }
}
