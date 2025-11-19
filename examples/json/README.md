# Example: JSON

This example shows how to use `JsonMetadata` to store metadata as a JSON file.

## Overview

This example demonstrates:

- **JsonMetadata**: Ready-to-use JSON file-based metadata storage
- **Simple API**: Just `load_or_init()` and `save()`

## Usage

Run the example:

```bash
cargo run --example json --features json
```

The example will create a `metadata.json` file in the current directory.

You can edit it and run the example again to see the changes.

## Code

```rust
use migratex::{JsonMetadata, Migratex};

// Load or initialize metadata
let mut meta = JsonMetadata::load_or_init("metadata.json")?;

// Create migrations and context
let migrations = vec![/* your migrations */];
let mut ctx = YourContext::new();

// Run migrations
let mut mx = Migratex::new(&mut ctx, &mut meta, migrations);
mx.migrate_to_latest().await?;

// Save metadata
meta.save("metadata.json")?;
```

## Features

### JSON

Enable the `json` feature to use `JsonMetadata`:

Cargo.toml:

```toml
[dependencies]
migratex = { version = "*", features = ["json"] }
```

> Put the latest version of `migratex` in your `Cargo.toml`!

## See Also

- [sqlx example](../sqlx/) - Sqlx (SQLite) database metadata storage
- [custom example](../custom/) - Custom metadata implementation
