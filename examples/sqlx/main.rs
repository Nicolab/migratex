mod context;
mod db_path;
mod metadata;
mod migrations;
mod sqlx_store;

use anyhow::Result;
use migratex::{Metadata, Migratex};
use sqlx::SqlitePool;

use context::MigContext;
use db_path::DbPath;
use metadata::AppMetadata;
use migrations::migrations;

#[tokio::main]
async fn main() -> Result<()> {
    let db_file = "app.db";

    println!("=== Migratex SQLx Example ===\n");
    println!("Database file: {}\n", db_file);

    // Create SQLite connection pool (create file if it doesn't exist)
    let pool = SqlitePool::connect(&format!("sqlite:{}?mode=rwc", db_file)).await?;
    println!("✓ Connected to database\n");

    // Create migration context with the connection pool
    let mut ctx = MigContext::new(pool.clone());

    // Create DbPath wrapper that includes both path and pool
    let db_path = DbPath::with_pool(db_file, pool.clone());

    // Load or initialize metadata using SqlxStore
    let mut meta = AppMetadata::load_or_init(&db_path)?;

    println!("Initial metadata:");
    println!("  Version: {}", meta.version());
    println!("  Status: {:?}", meta.status());
    println!("  App version: {}", meta.app_version());
    println!("  Created at: {}", meta.created_at());
    println!("  Updated at: {}\n", meta.updated_at());

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
    meta.save(&db_path)?;
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
