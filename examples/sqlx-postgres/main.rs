mod context;
mod migrations;

use std::sync::Arc;

use migratex::{Metadata, Migratex, PgMetadata, PgStorage, connect_to_postgres};
use okerr::{Context, Result};
use sqlx::PgPool;

use context::MigContext;
use migrations::migrations;

#[tokio::main]
async fn main() -> Result<()> {
    // e.g. postgres://postgres:pg@localhost:5432/postgres
    let db_url = std::env::var("DATABASE_URL").context("DATABASE_URL must be set")?;

    // Connect to PostgreSQL database
    let pool = connect_to_postgres(&db_url).await?;
    migrate(&pool).await?;

    Ok(())
}

// Update DB (eventually: + filesystem migrations + other migrations here...)
// Migrations up → latest
async fn migrate(pool: &PgPool) -> Result<()> {
    println!("=== Migratex PostgreSQL Example ===\n");

    let storage = PgStorage::new(Arc::new(pool.clone()));

    println!("✓ Connected to database\n");

    // Load or initialize metadata using PgMetadata
    let mut meta = PgMetadata::load_or_init(&storage).await?;

    let initial_version = meta.version();

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

    println!("\n=== Migration Complete ===");

    if meta.version() == initial_version {
        println!("✓ Nothing to migrate");
    } else {
        println!("✓ Metadata saved to database");
    }

    println!("\nYou can inspect the database:");
    println!(r#"  psql "$DATABASE_URL""#);
    println!("  \\dt");
    println!("  SELECT * FROM _migratex_metadata;");
    println!("  \\d users");
    println!("  \\d subscriptions");
    println!("  \\d products");

    Ok(())
}
