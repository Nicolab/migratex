// This is just an example of a custom metadata implementation.
// It could be anything, like a database table, a file, a cache, etc.
// You can implement your own load_or_init and save methods outside the trait.

use migratex::{MetaStatus, Metadata, init_meta_datetimes_if_empty, meta_loaded};

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct CustomMetadata {
    pub version: i32,
    pub status: MetaStatus,
    pub app_version: String,
    pub created_at: String,
    pub updated_at: String,
}

impl CustomMetadata {
    /// Custom implementation of load_or_init
    pub fn load_or_init(path: impl AsRef<std::path::Path>) -> okerr::Result<Self> {
        use std::fs;
        let path = path.as_ref();

        if path.exists() {
            let txt = fs::read_to_string(path)?;
            let meta: Self = serde_json::from_str(&txt)?;
            meta_loaded(meta)
        } else {
            let mut meta = Self::default();
            meta.set_version(0);
            meta.set_status(MetaStatus::Clean);
            meta.set_app_version(env!("CARGO_PKG_VERSION").to_string());
            init_meta_datetimes_if_empty(&mut meta);
            meta.save(path)?;
            Ok(meta)
        }
    }

    /// Custom implementation of save
    pub fn save(&self, path: impl AsRef<std::path::Path>) -> okerr::Result<()> {
        use std::fs;
        let txt = serde_json::to_string_pretty(self)?;
        fs::write(path, txt)?;
        Ok(())
    }
}

impl Metadata for CustomMetadata {
    // ⬇⬇⬇ generate all the metadata accessors
    migratex::metadata_accessors!();
}
