// This file is part of "Migratex - A Migrations Toolkit".
//
// This source code is licensed under the MIT license, please view the LICENSE
// file distributed with this source code. For the full
// information and documentation: https://github.com/nicolab/migratex
// -----------------------------------------------------------------------------

use std::path::Path;

use anyhow::Result;

use crate::Metadata;

/// A MetadataStore is a place where metadata is stored.
/// It can be a file, a database, a cache, a remote server, etc.
/// 2 simple methods to implement anything that can store metadata.
/// See `JsonStore` for examples.
pub trait MetadataStore<M: Metadata> {
    /// Load or init from a path.
    /// This is the most flexible way to load or init metadata.
    ///
    /// `path` can also be a string or &str. So a file name, table name, collection name, identifier...
    /// `path` accepts any type that implements `AsRef<Path>`:
    ///   - &str
    ///   - String
    ///   - Path
    ///   - &Path
    ///   - PathBuf
    ///   - &PathBuf
    ///   - OsString
    ///   - OsStr
    ///   - Cow<str>, Cow<Path>, Cow<_>...
    ///   - Smart pointers: Arc<_>, Rc<_>, Box<_>...
    ///   - Into<Path>...
    ///   - and many others...
    #[cfg(not(feature = "json"))]
    fn load_or_init(path: impl AsRef<Path>) -> Result<M>
    where
        M: Default;

    #[cfg(feature = "json")]
    fn load_or_init(path: impl AsRef<Path>) -> Result<M>
    where
        M: Default + serde::de::DeserializeOwned;

    /// Save metadata to a path.
    /// This is the most flexible way to save metadata.
    ///
    /// `path` can also be a string or &str. So a file name, table name, collection name, identifier...
    /// `path` accepts any type that implements `AsRef<Path>`:
    ///   - &str
    ///   - String
    ///   - Path
    ///   - &Path
    ///   - PathBuf
    ///   - &PathBuf
    ///   - OsString
    ///   - OsStr
    ///   - Cow<str>, Cow<Path>, Cow<_>...
    ///   - Smart pointers: Arc<_>, Rc<_>, Box<_>...
    ///   - Into<Path>...
    ///   - and many others...
    #[cfg(not(feature = "json"))]
    fn save(metadata: &M, path: impl AsRef<Path>) -> Result<()>;

    #[cfg(feature = "json")]
    fn save(metadata: &M, path: impl AsRef<Path>) -> Result<()>
    where
        M: serde::Serialize;
}
