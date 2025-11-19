// This file is part of "Migratex - A Migrations Toolkit".
//
// This source code is licensed under the MIT license, please view the LICENSE
// file distributed with this source code. For the full
// information and documentation: https://github.com/nicolab/migratex
// -----------------------------------------------------------------------------

use std::fs;
use std::path::Path;

use okerr::Result;
use serde::{Deserialize, Serialize};

use crate::{MetaStatus, Metadata, init_meta_datetimes_if_empty, meta_loaded};

/// JsonMetadata provides JSON file-based storage for migration metadata.
/// Metadata is stored in a JSON file on the file system.
///
/// # Example
///
/// ```rust
/// use migratex::{JsonMetadata, Metadata};
/// use okerr::Result;
///
/// fn main() -> Result<()> {
///     // Load or initialize metadata
///     let mut meta = JsonMetadata::load_or_init("metadata.json")?;
///
///     // Modify metadata
///     meta.set_version(1);
///
///     // Save explicitly
///     meta.save("metadata.json")?;
///
///     Ok(())
/// }
/// ```
#[cfg(feature = "json")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonMetadata {
    pub version: i32,
    pub app_version: String,
    pub status: MetaStatus,
    pub created_at: String,
    pub updated_at: String,
}

#[cfg(feature = "json")]
impl Default for JsonMetadata {
    fn default() -> Self {
        Self {
            version: 0,
            app_version: String::new(),
            status: MetaStatus::Clean,
            created_at: String::new(),
            updated_at: String::new(),
        }
    }
}

#[cfg(feature = "json")]
impl JsonMetadata {
    /// Load metadata from a JSON file, or initialize a new one if it doesn't exist.
    pub fn load_or_init(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        if path.exists() {
            let txt = fs::read_to_string(path)?;
            let meta: Self = serde_json::from_str(&txt)?;
            meta_loaded(meta)
        } else {
            Self::init_new(path)
        }
    }

    /// Save metadata to a JSON file.
    pub fn save(&self, path: impl AsRef<Path>) -> Result<()> {
        let txt = serde_json::to_string_pretty(self)?;
        fs::write(path, txt)?;
        Ok(())
    }

    /// Initialize a new metadata instance and save it.
    fn init_new(path: impl AsRef<Path>) -> Result<Self> {
        let mut meta = Self::default();
        meta.set_version(0);
        meta.set_status(MetaStatus::Clean);
        meta.set_app_version(env!("CARGO_PKG_VERSION").to_string());
        init_meta_datetimes_if_empty(&mut meta);
        meta.save(path)?;
        Ok(meta)
    }
}

#[cfg(feature = "json")]
impl Metadata for JsonMetadata {
    crate::metadata_accessors!();
}
