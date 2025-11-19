mod context;
mod metadata;
mod migrations;

use migratex::Migratex;
use okerr::Result;

use context::MigContext;
use metadata::CustomMetadata;
use migrations::migrations;

#[tokio::main]
async fn main() -> Result<()> {
    let file_path = std::path::Path::new("metadata.json");

    // Create migration context
    let mut ctx = MigContext {
        foo: "foo from custom_store example".to_string(),
        bar: "bar from custom_store example".to_string(),
    };

    // Load or init metadata file
    let mut meta = CustomMetadata::load_or_init(file_path)?;

    println!("Initial context: {:?}\n", &ctx);
    println!("Initial metadata: {:?}\n", meta);

    // Load migrations and create Migratex (migrator / migration manager)
    let migs = migrations();
    let mut mx = Migratex::new(&mut ctx, &mut meta, migs);

    // Run migrations to latest version
    mx.migrate_to_latest().await?;

    // -- OR --

    // Run migrations to the next version (up)
    // mx.migrate_next().await?;

    // Run migrations to version 1
    // mx.migrate_to(1).await?;

    // Run migrations from version 1 to version 8
    // mx.migrate_up(1, 8).await?;

    // Run migrations from version43 to version 2
    // mx.migrate_up(4, 2).await?;

    // Rollback migrations (reset to version 0)
    // mx.migrate_to_zero().await?;

    // ...

    println!("Final context: {:?}\n", ctx);
    println!("Final metadata: {:?}\n", meta);

    // Save metadata file
    meta.save(file_path)?;

    println!("Final metadata saved to {:?}", file_path);
    println!("Done!");

    Ok(())
}
