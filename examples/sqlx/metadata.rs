use migratex::{MetaStatus, Metadata, MetadataStore};

use crate::sqlx_store::SqlxStore;

/// Application metadata for SQLx example.
/// Stores migration state in a SQLite database table via SqlxStore.
#[derive(Debug, Default, Clone)]
pub struct AppMetadata {
    pub version: i32,
    pub status: MetaStatus,
    pub app_version: String,
    pub created_at: String,
    pub updated_at: String,
}

impl Metadata for AppMetadata {
    // ⬇⬇⬇ generate all the metadata accessors
    migratex::metadata_accessors!();

    /// Load or initialize metadata from the database using SqlxStore.
    fn load_or_init(path: impl AsRef<std::path::Path>) -> anyhow::Result<Self> {
        SqlxStore::load_or_init(path)
    }

    /// Save metadata to the database using SqlxStore.
    fn save(&self, path: impl AsRef<std::path::Path>) -> anyhow::Result<()> {
        SqlxStore::save(self, path)
    }
}
