// This file is part of "Migratex - A Migrations Toolkit".
//
// This source code is licensed under the MIT license, please view the LICENSE
// file distributed with this source code. For the full
// information and documentation: https://github.com/nicolab/migratex
// -----------------------------------------------------------------------------

use okerr::Result;

use crate::BoxMigration;
use crate::Metadata;

/// Migratex manages the migrations, this is the main struct.
/// Think of it as a "migration manager", "migrator", "runner").
/// It can be used to migrate database / data / files / binaries, etc from one version to another.
pub struct Migratex<'m, 'c, MigContext, M: Metadata> {
    /// The migration context, passed to each migration.
    ctx: &'c mut MigContext,
    /// The metadata, passed to each migration.
    meta: &'m mut M,
    /// The migrations list.
    migrations: Vec<BoxMigration<MigContext>>,
}

impl<'m, 'c, MigContext, M: Metadata> Migratex<'m, 'c, MigContext, M> {
    /// Create a new Migratex.
    pub fn new(
        ctx: &'c mut MigContext,
        meta: &'m mut M,
        migrations: Vec<BoxMigration<MigContext>>,
    ) -> Self {
        Self {
            ctx,
            meta,
            migrations,
        }
    }

    /// Get the current metadata.
    pub fn metadata(&self) -> &M {
        self.meta
    }

    /// Get the migration context.
    pub fn context(&self) -> &MigContext {
        self.ctx
    }

    /// Get the most recent migration version.
    pub fn latest_version(&self) -> i32 {
        self.migrations.last().map(|m| m.version()).unwrap_or(0)
    }

    /// Migrate from current metadata.version up to the latest migration version
    pub async fn migrate_to_latest(&mut self) -> Result<()> {
        let target = self.latest_version();
        self.migrate_to(target).await
    }

    /// Rollback everything (migrate to version 0, before the first migration).
    pub async fn migrate_to_zero(&mut self) -> Result<()> {
        self.migrate_to(0).await
    }

    /// Migrate to the next version (up)
    pub async fn migrate_next(&mut self) -> Result<()> {
        let current = self.meta.version();

        if current >= self.latest_version() {
            return Ok(());
        }

        self.migrate_to(current + 1).await
    }

    /// Migrate to the previous version (down)
    pub async fn migrate_prev(&mut self) -> Result<()> {
        let current = self.meta.version();

        if current <= 0 {
            return Ok(());
        }

        self.migrate_to(current - 1).await
    }

    /// Migrate to a specific target version (up or down)
    pub async fn migrate_to(&mut self, target: i32) -> Result<()> {
        let current = self.meta.version();

        if target == current {
            return Ok(());
        }

        self.meta.mark_migrating();

        let result = if target > current {
            // UP:  current+1 ..= target
            self.migrate_up(current, target).await
        } else {
            // DOWN: current ..> target
            self.migrate_down(current, target).await
        };

        match result {
            Ok(()) => {
                self.meta.mark_clean();
                Ok(())
            }
            Err(e) => {
                self.meta.mark_failed();
                Err(e)
            }
        }
    }

    /// Migrate up to a specific target version.
    async fn migrate_up(&mut self, current: i32, target: i32) -> Result<()> {
        for m in &self.migrations {
            let v = m.version();
            if v > current && v <= target {
                m.up(self.ctx).await?;
                self.meta.set_version(v);
            }
        }
        Ok(())
    }

    /// Migrate down to a specific target version.
    async fn migrate_down(&mut self, current: i32, target: i32) -> Result<()> {
        let mut sorted: Vec<_> = self.migrations.iter().collect();
        sorted.sort_by_key(|m| m.version());
        sorted.reverse(); // highest → lowest

        for m in sorted {
            let v = m.version();
            if v <= current && v > target {
                m.down(self.ctx).await?;
                // convention: after down of v, highest applied = v - 1
                self.meta.set_version(v - 1);
            }
        }
        Ok(())
    }
}
