# Example: custom store

This example shows how to use your own custom store to store metadata.

To put it simply, let's imagine that you want metadata to be persisted in a JSON file. It works just as easily with any type of format (YAML, TOML, GraphQL, etc.).

It could just as easily be a database (Postgres, SQLite, MySQL, Mongo, Redis, etc.).

## Usage

Run the example:

```rust
cargo run --example custom_store --features json
```

The example will create a `metadata.json` file.

You can edit it and run the example again to see the changes.

### Features

#### JSON

Enable the `json` feature to serialization in JSON format. Also, to use  `JsonStore` implementation to store metadata in a JSON file.

Cargo.toml:

```toml
[dependencies]
migratex = { version = "*", features = ["json"] }
```

> Put the latest version of `migratex` in your `Cargo.toml`!
