mod context;
mod migrations;

use std::sync::Arc;

use migratex::{Metadata, Migratex, SqliteMetadata, SqliteStorage, connect_to_sqlite};
use okerr::Result;

use context::MigContext;
use migrations::migrations;

#[tokio::main]
async fn main() -> Result<()> {
    let db_file = "app.db";

    println!("=== Migratex SQLx Example ===\n");
    println!("Database file: {}\n", db_file);

    // Connect to SQLite database
    let pool = connect_to_sqlite(db_file.into()).await?;
    let storage = SqliteStorage::new(Arc::new(pool.clone()));

    println!("✓ Connected to database {}\n", db_file);

    // Load or initialize metadata using SqliteMetadata
    let mut meta = SqliteMetadata::load_or_init(&storage).await?;

    println!("Initial metadata:");
    println!("  Version: {}", meta.version());
    println!("  Status: {:?}", meta.status());
    println!("  App version: {}", meta.app_version());
    println!("  Created at: {}", meta.created_at());
    println!("  Updated at: {}\n", meta.updated_at());

    // Create migration context with the pool
    let mut ctx = MigContext::new(storage.pool.clone());

    // Load migrations and create Migratex (migration manager)
    let migs = migrations();
    let mut mx = Migratex::new(&mut ctx, &mut meta, migs);

    println!("Latest migration version: {}\n", mx.latest_version());

    // Run migrations to latest version
    println!("Running migrations...\n");
    mx.migrate_to_latest().await?;

    println!("\nFinal metadata:");
    println!("  Version: {}", meta.version());
    println!("  Status: {:?}", meta.status());
    println!("  App version: {}", meta.app_version());
    println!("  Updated at: {}\n", meta.updated_at());

    // Save metadata to database
    meta.save(&storage).await?;
    println!("✓ Metadata saved to database");

    println!("\n=== Migration Complete ===");
    println!("\nYou can now inspect the database:");
    println!("  sqlite3 {}", db_file);
    println!("  .tables");
    println!("  SELECT * FROM _migratex_metadata;");
    println!("  .schema users");
    println!("  .schema subscriptions");
    println!("  .schema products");

    Ok(())
}
