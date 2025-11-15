# Example: JsonStore

This example shows how to use `JsonStore` to store metadata as a JSON file.

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
