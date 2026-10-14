# SQLx PostgreSQL Example

This example demonstrates how to use Migratex with SQLx and PostgreSQL, storing migration metadata directly in the database.

## Overview

This example shows:

- **PgMetadata**: Ready-to-use PostgreSQL database metadata storage
- **PgStorage**: Storage configuration for metadata
- **connect_to_postgres()**: Helper function to connect to PostgreSQL database (optional)
- **Database migrations**: Creating and managing tables with SQLx, in transactions
- **Efficient connection reuse**: Single connection pool for both metadata and migrations

## Key Components

### PgStorage

The `PgStorage` struct provides metadata storage configuration:

```rust
use std::sync::Arc;

use migratex::{PgStorage, connect_to_postgres};

// Connect to database
let pool = connect_to_postgres("postgres://user:pass@localhost:5432/app").await?;

// Create storage with default table name
let storage = PgStorage::new(Arc::new(pool));

// Or with custom table name
let storage = PgStorage::new(Arc::new(pool))
    .with_table_name("_migratex_metadata");
```

The table name must be a plain SQL identifier (`[A-Za-z_][A-Za-z0-9_]*`), otherwise metadata operations fail before running any query. It is not quoted, so PostgreSQL folds it to lowercase and may reject reserved keywords (e.g. `user`).

### Schema

The metadata table is created in the first schema of the connection `search_path` (`public` by default). To use another schema, set it in the connection URL:

```text
postgres://user:pass@localhost:5432/app?options=-c%20search_path%3Dapp
```

or build the pool yourself with `PgConnectOptions::options([("search_path", "app")])`.

### PgMetadata

Ready-to-use metadata storage that:

- Stores metadata in a `_migratex_metadata` table (configurable)
- Supports pooled connections
- Uses UPSERT (`ON CONFLICT`) for atomic metadata updates

```rust
use migratex::{PgMetadata, Migratex};

// Load or initialize metadata
let mut meta = PgMetadata::load_or_init(&storage).await?;

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
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE
)
```

**subscriptions** (M1):

```sql
CREATE TABLE subscriptions (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id),
    plan TEXT NOT NULL
)
```

**products** (M2):

```sql
CREATE TABLE products (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    name TEXT NOT NULL,
    price NUMERIC(12, 2) NOT NULL
)
```

## Running the Example

```bash
# Start a PostgreSQL server
docker run --rm -d --name migratex-pg -p 5432:5432 -e POSTGRES_PASSWORD=pg postgres:18

# Run from the migratex directory
DATABASE_URL=postgres://postgres:pg@localhost:5432/postgres \
  cargo run --example sqlx-postgres --features postgres
```

## Inspecting the Database

```bash
psql postgres://postgres:pg@localhost:5432/postgres

# View all tables
\dt

# Check metadata
SELECT * FROM _migratex_metadata;

# View table schemas
\d users
\d subscriptions
\d products
```

## How It Works

1. **Initialize**: Create PostgreSQL connection pool using `connect_to_postgres()`
2. **Create Storage**: Create `PgStorage` with the pool
3. **Load Metadata**: `PgMetadata::load_or_init()` checks `_migratex_metadata` table or creates it
4. **Run Migrations**: Migratex executes pending migrations using the pool
5. **Save Metadata**: Updated metadata is stored back to the database
6. **Cleanup**: Connection pool is dropped automatically

## Migration Context

The migration context contains the database connection pool:

```rust
pub struct MigContext {
    pub db: Arc<PgPool>,
}
```

This allows migrations to access the database for creating tables and executing SQL queries.

## Notes

- PostgreSQL DDL is transactional: a failing migration rolls back entirely
- Foreign key constraints are enforced (see migrations order in `down()`)
- Metadata table uses `CHECK (id = 1)` to ensure single-row constraint
- Connection pools are wrapped in `Arc<>` for efficient cloning
- `connect_to_postgres()` never includes the connection URL (and its password) in errors

## Features

Enable the `postgres` feature to use PostgreSQL metadata storage:

```toml
[dependencies]
migratex = { version = "*", features = ["postgres"] }
```

## See Also

- [sqlx-sqlite example](../sqlx-sqlite/) - SQLite database metadata storage
- [json example](../json/) - JSON file-based metadata storage
- [custom example](../custom/) - Custom metadata implementation
