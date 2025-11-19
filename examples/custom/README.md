# Example: Custom

This example demonstrates how to implement custom metadata storage.

## Overview

This example shows:

- **CustomMetadata**: Custom metadata struct with personalized fields
- **Custom implementation**: Implementing your own `load_or_init()` and `save()` methods
- **Metadata trait**: Using the `Metadata` trait for compatibility with Migratex
- **Flexibility**: Full control over storage format and location

## When to Use Custom Metadata

You should implement custom metadata when:

- You need additional metadata fields
- You want to use a different storage format (TOML, YAML, GraphQL, etc.)
- You need custom serialization logic
- You want to integrate with your own storage system (other database, ORM, remote storage, etc.)

## Code

### 1. Define Your Metadata Struct

```rust
use migratex::{Metadata, MetaStatus};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomMetadata {
    pub version: i32,
    pub status: MetaStatus,
    pub app_version: String,
    pub created_at: String,
    pub updated_at: String,
}
```

### 2. Implement Load and Save

```rust
impl CustomMetadata {
    pub fn load_or_init(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();

        if path.exists() {
            let txt = fs::read_to_string(path)?;
            let meta: Self = serde_json::from_str(&txt)?;
            meta_loaded(meta)
        } else {
            let mut meta = Self::default();
            meta.set_version(0);
            meta.set_status(MetaStatus::Clean);
            meta.set_app_version(env!("CARGO_PKG_VERSION").to_string());
            init_meta_datetimes_if_empty(&mut meta);
            meta.save(path)?;
            Ok(meta)
        }
    }

    pub fn save(&self, path: impl AsRef<Path>) -> Result<()> {
        let txt = serde_json::to_string_pretty(self)?;
        fs::write(path, txt)?;
        Ok(())
    }
}
```

### 3. Implement Metadata Trait

```rust
impl Metadata for CustomMetadata {
    migratex::metadata_accessors!();
}
```

The `metadata_accessors!()` macro provides default implementations for:

- `version()`, `version_mut()`, `set_version()`
- `status()`, `status_mut()`, `set_status()`
- `app_version()`, `app_version_mut()`, `set_app_version()`
- `created_at()`, `created_at_mut()`
- `updated_at()`, `updated_at_mut()`

It's ready to use!

### 4. Use It

```rust
use migratex::{Migratex, CustomMetadata};

// Load or initialize metadata
let mut meta = CustomMetadata::load_or_init("metadata.json")?;

// Run migrations
let mut mx = Migratex::new(&mut ctx, &mut meta, migrations);
mx.migrate_to_latest().await?;

// Save metadata
meta.save("metadata.json")?;
```

## Running the Example

```bash
cargo run --example custom --features json
```

The example will create a `metadata.json` file.

You can edit it and run the example again to see the changes.

## Advantages

- **Full control**: Complete control over storage format and logic
- **Extensibility**: Easy to add custom fields or behavior
- **Compatibility**: Works seamlessly with Migratex API
- **Type safety**: Strong typing for your metadata

## Notes

- This example uses JSON, but you can use any format (TOML, YAML, GraphQL, binary, etc.)
- The `meta_loaded()` and `init_meta_datetimes_if_empty()` helpers are provided by Migratex
- You can implement async `load_or_init()` and `save()` if needed (see SQLite example)

## See Also

- [json example](../json/) - JSON file-based metadata storage (ready-to-use)
- [sqlx example](../sqlx/) - Sqlx (SQLite) database metadata storage (ready-to-use)
