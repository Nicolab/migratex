# Migratex

[![Crates.io](https://img.shields.io/crates/v/migratex.svg)](https://crates.io/crates/migratex)
[![Docs.rs](https://docs.rs/migratex/badge.svg)](https://docs.rs/migratex)
[![License](https://img.shields.io/crates/l/migratex.svg)](https://github.com/nicolab/migratex/blob/master/LICENSE)

**Migratex** is an agnostic migration toolkit library.

> Migrate anything! Anywhere! 🚀

💪 It can be used to migrate database / data / files / binaries, etc from one version to another.

- ✅ Easy to use
- ✅ Agnostic
- ✅ Standalone
- ✅ Async
- ✅ Easy to extend
- ✅ Easy to use with any storage
- ✅ Easy to use with any migration type

Simple and intuitive API: migrate_up, migrate_down, migrate_to, migrate_to_latest, migrate_to_zero, latest_version, metadata, etc.

Look at the [examples](https://github.com/nicolab/migratex/tree/master/examples).

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
migratex = "*"
```

> Put the latest version of `migratex` in your `Cargo.toml`!

Look at the [examples](https://github.com/nicolab/migratex/tree/master/examples).

### Features

#### JSON

Enable the `json` feature to serialization in JSON format. Also, to use  `JsonStore` implementation to store metadata in a JSON file.

Cargo.toml:

```toml
[dependencies]
migratex = { version = "*", features = ["json"] }
```

> Put the latest version of `migratex` in your `Cargo.toml`!

## Tests

Run all tests:

```sh
cargo test  --tests --features json
```

## LICENSE

[MIT](https://github.com/nicolab/migratex/blob/master/LICENSE) (c) 2025, Nicolas Talle.

## Author

- [Nicolas Talle](https://ntalle.com)
- <https://www.linkedin.com/in/ntalle/>

> Buy me a coffee ☕ via [PayPal](https://www.paypal.com/cgi-bin/webscr?cmd=_s-xclick&hosted_button_id=PGRH4ZXP36GUC)!
