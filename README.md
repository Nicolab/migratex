# Migratex

[![Crates.io](https://img.shields.io/crates/v/migratex.svg)](https://crates.io/crates/migratex)
[![Docs.rs](https://docs.rs/migratex/badge.svg)](https://docs.rs/migratex)
[![License](https://img.shields.io/crates/l/migratex.svg)](https://github.com/nicolab/migratex/blob/main/LICENSE)

**Migratex** is an agnostic migration toolkit library.

> Migrate anything! Anywhere! 🚀

💪 It can be used to migrate database / data / files / binaries, etc from one version to another.

- ✅ Easy to use
- ✅ Agnostic
- ✅ Standalone
- ✅ Async
- ✅ Easy to extend
- ✅ Easy to use with any storage (DB, file, etc)
- ✅ Easy to use with any migration type
- ✅ Minimal boilerplate - Ready-to-use metadata stores

Simple and intuitive API: `migrate_up`, `migrate_down`, `migrate_to`, `migrate_to_latest`, `migrate_to_zero`, `latest_version`, `metadata`, etc.

## Quick Start

### With JSON file storage

```rust
use migratex::{JsonMetadata, Migratex};

// Load or initialize metadata
let mut meta = JsonMetadata::load_or_init("metadata.json")?;

// Run migrations
let mut mx = Migratex::new(&mut ctx, &mut meta, migrations);
mx.migrate_to_latest().await?;

// Save metadata
meta.save("metadata.json")?;
```

### With SQLite storage

```rust
use migratex::{SqliteMetadata, SqliteStorage, connect_to_sqlite, Migratex};
use std::sync::Arc;
use std::path::PathBuf;

// Connect to database
let pool = connect_to_sqlite(PathBuf::from("app.db")).await?;
let storage = SqliteStorage::new(Arc::new(pool));

// Load or initialize metadata
let mut meta = SqliteMetadata::load_or_init(&storage).await?;

// Run migrations
let mut mx = Migratex::new(&mut ctx, &mut meta, migrations);
mx.migrate_to_latest().await?;

// Save metadata
meta.save(&storage).await?;
```

## Examples

Look at the [examples](https://github.com/nicolab/migratex/tree/main/examples):

- [json example](https://github.com/nicolab/migratex/tree/main/examples/json) - JSON file-based metadata storage
- [sqlx example](https://github.com/nicolab/migratex/tree/main/examples/sqlx) - SQLite database metadata storage
- [custom example](https://github.com/nicolab/migratex/tree/main/examples/custom) - Custom metadata implementation

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
migratex = "*"
```

> Put the latest version of `migratex` in your `Cargo.toml`!

### Features

#### JSON

Enable the `json` feature for JSON file-based metadata storage:

```toml
[dependencies]
migratex = { version = "*", features = ["json"] }
```

This provides the `JsonMetadata` struct for storing metadata in a JSON file.

#### SQLx

Enable the `sqlx` feature for SQLite database metadata storage:

```toml
[dependencies]
migratex = { version = "*", features = ["sqlx"] }
```

> Put the latest version of `migratex` in your `Cargo.toml`!

This provides:

- `SqliteMetadata` - Metadata stored in a SQLite table
- `SqliteStorage` - Storage configuration
- `connect_to_sqlite()` - Helper function to connect to SQLite database

> Note: Other database drivers can be implemented by implementing the `Metadata` trait (look at SQLite implementation for inspiration).

## Custom Metadata Storage

You can implement your own metadata storage by implementing the `Metadata` trait:

```rust
use migratex::{Metadata, MetaStatus};

#[derive(Debug, Clone)]
pub struct CustomMetadata {
    pub version: i32,
    pub status: MetaStatus,
    pub app_version: String,
    pub created_at: String,
    pub updated_at: String,
}

impl CustomMetadata {
    pub fn load_or_init(path: impl AsRef<Path>) -> Result<Self> {
        // Your custom implementation
    }

    pub fn save(&self, path: impl AsRef<Path>) -> Result<()> {
        // Your custom implementation
    }
}

impl Metadata for CustomMetadata {
    migratex::metadata_accessors!();
}
```

See the [custom example](https://github.com/nicolab/migratex/tree/main/examples/custom) for a complete implementation.

## Tests

Run all tests:

```sh
cargo test --all-features
```

## Notes

- This library is in its early stages, so expect minor breaking changes.
- `okerr` is used for error handling (100% compatible with `anyhow`), this should work properly with any error-handling library.

## LICENSE

[MIT](https://github.com/nicolab/migratex/blob/main/LICENSE) (c) 2025, Nicolas Talle.

## Author

- [Nicolas Talle](https://ntalle.com)
- <https://www.linkedin.com/in/ntalle/>

> Buy me a coffee ☕ via [PayPal](https://www.paypal.com/cgi-bin/webscr?cmd=_s-xclick&hosted_button_id=PGRH4ZXP36GUC)!
