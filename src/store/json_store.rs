// This file is part of "Migratex - A Migrations Toolkit".
//
// This source code is licensed under the MIT license, please view the LICENSE
// file distributed with this source code. For the full
// information and documentation: https://github.com/nicolab/migratex
// -----------------------------------------------------------------------------

use std::fs;
use std::path::Path;

use anyhow::Result;

use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::{Metadata, MetadataStore, meta_loaded};

/// A MetadataStore that stores metadata as a JSON file, on the file system.
///
/// # Example
///
/// ```
/// use migratex::JsonStore;
/// use migratex::{MetaStatus, Metadata, MetadataStore};
///
/// #[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
/// pub struct AppMetadata {
///     pub version: i32,
///     pub status: MetaStatus,
///     pub app_version: String,
///     pub created_at: String,
///     pub updated_at: String,
/// }
///
/// impl MetadataStore<AppMetadata> for AppMetadata {
///     migratex::metadata_accessors!();
///
///     fn load_or_init(path: impl AsRef<std::path::Path>) -> anyhow::Result<Self> {
///         JsonStore::load_or_init(path)
///     }
///
///     fn save(&self, path: impl AsRef<std::path::Path>) -> anyhow::Result<()> {
///         JsonStore::save(self, path)
///     }
/// }
/// ```
pub struct JsonStore;

impl<M: Metadata> MetadataStore<M> for JsonStore {
    /// Load or init from a JSON file.
    /// `path` is the path to the JSON file.
    fn load_or_init(path: impl AsRef<Path>) -> Result<M>
    where
        M: Default + DeserializeOwned,
    {
        let path = path.as_ref();

        if path.exists() {
            let txt = fs::read_to_string(path)?;
            let meta: M = serde_json::from_str(&txt)?;
            meta_loaded(meta)
        } else {
            M::new_with_path(path)
        }
    }

    /// Save metadata to a JSON file.
    /// `path` is the path to the JSON file.
    fn save(metadata: &M, path: impl AsRef<Path>) -> Result<()>
    where
        M: Serialize,
    {
        let txt = serde_json::to_string_pretty(metadata)?;
        fs::write(path, txt)?;
        Ok(())
    }
}
