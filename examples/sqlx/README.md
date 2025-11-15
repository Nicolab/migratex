# SQLx Example

This example demonstrates how to use Migratex with SQLx and SQLite, storing migration metadata directly in the database.

## Overview

This example shows:

- **SqlxStore**: Custom metadata storage using SQLite database tables
- **DbPath wrapper**: Advanced pattern to pass both path and connection pool
- **Database migrations**: Creating and managing tables with SQLx
- **Efficient connection reuse**: Single connection pool for both metadata and migrations

## Key Components

### DbPath Wrapper

The `DbPath` type is a clever wrapper that:

- Implements `AsRef<Path>` for compatibility with `MetadataStore` trait
- Optionally holds a `SqlitePool` for connection reuse
- Allows efficient resource management

```rust
let db_path = DbPath::with_pool("app.db", pool.clone());
```

### SqlxStore

Custom implementation of `MetadataStore` that:

- Stores metadata in a `_migratex_metadata` table
- Supports both pooled and temporary connections
- Uses UPSERT for atomic metadata updates

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
cargo run --example sqlx
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

1. **Initialize**: Create SQLite connection pool
2. **Load Metadata**: SqlxStore checks `_migratex_metadata` table or creates it
3. **Run Migrations**: Migratex executes pending migrations using the pool
4. **Save Metadata**: Updated metadata is stored back to the database
5. **Cleanup**: Connection pool is dropped automatically

## Advanced Pattern: DbPath

The DbPath wrapper demonstrates an advanced Rust pattern:

```rust
pub struct DbPath {
    path: String,
    pool: Option<Arc<SqlitePool>>,
}

impl AsRef<Path> for DbPath {
    fn as_ref(&self) -> &Path {
        Path::new(&self.path)
    }
}
```

This allows:

- Type-safe path conversion
- Optional connection pool injection
- Compatibility with trait-based APIs
- Zero-cost abstraction

## Notes

- The example uses SQLite, but the pattern works with any SQLx-supported database
- Foreign key constraints are enforced (see migrations order in `down()`)
- Metadata table uses `CHECK (id = 1)` to ensure single-row constraint
- Connection pools are wrapped in `Arc<>` for efficient cloning

## See Also

- [json_store example](../json_store/) - File-based metadata storage
- [custom_store example](../custom_store/) - Custom metadata implementation
