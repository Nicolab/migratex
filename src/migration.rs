// This file is part of "Migratex - A Migrations Toolkit".
//
// This source code is licensed under the MIT license, please view the LICENSE
// file distributed with this source code. For the full
// information and documentation: https://github.com/nicolab/migratex
// -----------------------------------------------------------------------------

use okerr::Result;
use async_trait::async_trait;

/// A migration is a version of a change in the data.
/// It can be a database migration, a file migration, a binary migration, etc.
/// It can be up (upgrade) or down (downgrade / rollback).
#[async_trait]
pub trait Migration<MigContext>: Send + Sync {
    /// The version of the migration.
    fn version(&self) -> i32;

    /// Upgrade the data to the `version` of the migration.
    async fn up(&self, ctx: &mut MigContext) -> Result<()>;

    /// Downgrade (rollback) the data. Think of it as a rollback / cancel of the current migration.
    async fn down(&self, ctx: &mut MigContext) -> Result<()>;
}

/// BoxMigration is the type of a migration.
pub type BoxMigration<MigContext> = Box<dyn Migration<MigContext> + Send + Sync>;
