// This is just an example of a metadata store.
// It could be anything, like a database table, a file, a cache, etc.
// It is passed to each migration.
// It is also used to store the metadata.

use migratex::{MetaStatus, Metadata, meta_loaded};

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

    // and you can implement your own load_or_init method
    fn load_or_init(path: impl AsRef<std::path::Path>) -> anyhow::Result<Self> {
        // Your own implementation
        // or JsonStore implementation: JsonStore::load_or_init(path)
        // or community implementation
        // ...
        // custom JSON implementation here:

        use std::fs;
        let path = path.as_ref();

        if path.exists() {
            let txt = fs::read_to_string(path)?;
            let meta: Self = serde_json::from_str(&txt)?;
            meta_loaded(meta)
        } else {
            Self::new_with_path(path)
        }
    }

    /// Save metadata to a path.
    fn save(&self, path: impl AsRef<std::path::Path>) -> anyhow::Result<()> {
        use std::fs;
        let txt = serde_json::to_string_pretty(self)?;
        fs::write(path, txt)?;
        Ok(())
    }
}
