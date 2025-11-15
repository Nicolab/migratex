// This example shows how to use JsonStore to store metadata as a JSON file.
// JsonStore is a built-in implementation that provides simple JSON file storage.

use migratex::{JsonStore, MetaStatus, Metadata, MetadataStore};

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
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

    // Use JsonStore's implementation directly
    fn load_or_init(path: impl AsRef<std::path::Path>) -> anyhow::Result<Self> {
        JsonStore::load_or_init(path)
    }

    /// Save metadata to a path using JsonStore.
    fn save(&self, path: impl AsRef<std::path::Path>) -> anyhow::Result<()> {
        JsonStore::save(self, path)
    }
}
