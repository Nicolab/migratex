mod context;
mod migrations;

use migratex::{JsonMetadata, Migratex};
use okerr::Result;

use context::MigContext;
use migrations::migrations;

#[tokio::main]
async fn main() -> Result<()> {
    let file_path = std::path::PathBuf::from("metadata.json");

    // Create migration context
    let mut ctx = MigContext {
        foo: "foo from JsonStore example".to_string(),
        bar: "bar from JsonStore example".to_string(),
    };

    // Load or init metadata file using JsonMetadata
    let mut meta = JsonMetadata::load_or_init(&file_path)?;

    println!("Initial context: {:?}\n", ctx);
    println!("Initial metadata: {:?}\n", meta);

    // Load migrations and create Migratex (migrator / migration manager)
    let migs = migrations();
    let mut mx = Migratex::new(&mut ctx, &mut meta, migs);

    // Run migrations to latest version
    mx.migrate_to_latest().await?;

    println!("Final context: {:?}\n", ctx);
    println!("Final metadata: {:?}\n", meta);

    // Save metadata file
    meta.save(&file_path)?;

    println!("Final metadata saved to {:?}", file_path);
    println!("Done!");

    Ok(())
}
