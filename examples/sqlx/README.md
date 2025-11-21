# SQLx Example

This example demonstrates how to use Migratex with SQLx and SQLite, storing migration metadata directly in the database.

## Overview

This example shows:

- **SqliteMetadata**: Ready-to-use SQLite database metadata storage
- **SqliteStorage**: Storage configuration for metadata
- **connect_to_sqlite()**: Helper function to connect to SQLite database (optional)
- **Database migrations**: Creating and managing tables with SQLx
- **Efficient connection reuse**: Single connection pool for both metadata and migrations

## Key Components

### SqliteStorage

The `SqliteStorage` struct provides metadata storage configuration:

```rust
use std::sync::Arc;
use std::path::PathBuf;

use migratex::{SqliteStorage, connect_to_sqlite};

// Connect to database
let pool = connect_to_sqlite(PathBuf::from("app.db")).await?;

// Create storage with default table name
let storage = SqliteStorage::new(Arc::new(pool));

// Or with custom table name
let storage = SqliteStorage::new(Arc::new(pool))
    .with_table_name("_migratex_metadata");
```

### SqliteMetadata

Ready-to-use metadata storage that:

- Stores metadata in a `_migratex_metadata` table (configurable)
- Supports both pooled connections
- Uses UPSERT for atomic metadata updates

```rust
use migratex::{SqliteMetadata, Migratex};

// Load or initialize metadata
let mut meta = SqliteMetadata::load_or_init(&storage).await?;

// Run migrations
let mut mx = Migratex::new(&mut ctx, &mut meta, migrations);
mx.migrate_to_latest().await?;

// Save metadata
meta.save(&storage).await?;
```

### Migrations

- **M1Initial** (version 1): Creates `users` and `subscriptions` tables
- **M2Products** (version 2): Creates `products` table

## Database Schema

### Metadata Table

```sql
CREATE TABLE _migratex_metadata (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    version INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'Clean',
    app_version TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
)
```

### Application Tables

**users** (M1):

```sql
CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE
)
```

**subscriptions** (M1):

```sql
CREATE TABLE subscriptions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    plan TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id)
)
```

**products** (M2):

```sql
CREATE TABLE products (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    price REAL NOT NULL
)
```

## Running the Example

```bash
# Run from the migratex directory
cargo run --example sqlx --features sqlx
```

## Inspecting the Database

After running the example, you can inspect the created database:

```bash
sqlite3 app.db

# View all tables
.tables

# Check metadata
SELECT * FROM _migratex_metadata;

# View table schemas
.schema users
.schema subscriptions
.schema products

# Check if foreign keys are enabled
PRAGMA foreign_keys;
```

## How It Works

1. **Initialize**: Create SQLite connection pool using `connect_to_sqlite()`
2. **Create Storage**: Create `SqliteStorage` with the pool
3. **Load Metadata**: `SqliteMetadata::load_or_init()` checks `_migratex_metadata` table or creates it
4. **Run Migrations**: Migratex executes pending migrations using the pool
5. **Save Metadata**: Updated metadata is stored back to the database
6. **Cleanup**: Connection pool is dropped automatically

## Migration Context

The migration context contains the database connection pool:

```rust
pub struct MigContext {
    pub db: Arc<SqlitePool>,
}
```

This allows migrations to access the database for creating tables and executing SQL queries.

## Notes

- The example uses SQLite, but the pattern can be adapted for other databases
- Foreign key constraints are enforced (see migrations order in `down()`)
- Metadata table uses `CHECK (id = 1)` to ensure single-row constraint
- Connection pools are wrapped in `Arc<>` for efficient cloning
- `connect_to_sqlite()` automatically configures WAL mode and foreign keys

## Features

Enable the `sqlx` feature to use SQLite metadata storage:

```toml
[dependencies]
migratex = { version = "*", features = ["sqlx"] }
```

## See Also

- [json example](../json/) - JSON file-based metadata storage
- [custom example](../custom/) - Custom metadata implementation
