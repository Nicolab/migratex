// This file is part of "Migratex - A Migrations Toolkit".
//
// This source code is licensed under the MIT license, please view the LICENSE
// file distributed with this source code. For the full
// information and documentation: https://github.com/nicolab/migratex
// -----------------------------------------------------------------------------

use anyhow::Result;

use crate::Metadata;

pub fn meta_loaded<M: Metadata>(mut meta: M) -> Result<M> {
    init_meta_datetimes_if_empty(&mut meta);
    Ok(meta)
}

/// Initialize created_at / updated_at if created_at is empty.
pub fn init_meta_datetimes_if_empty(meta: &mut impl Metadata) {
    if meta.created_at().is_empty() {
        let now = chrono::Utc::now().to_rfc3339();
        *meta.created_at_mut() = now.clone();
        *meta.updated_at_mut() = now;
    }
}

// A convenient macro to generate all the accessors for concrete Metadata.
#[macro_export]
macro_rules! metadata_accessors {
    () => {
        fn version(&self) -> i32 {
            self.version
        }

        fn version_mut(&mut self) -> &mut i32 {
            &mut self.version
        }

        fn status(&self) -> $crate::MetaStatus {
            self.status
        }

        fn status_mut(&mut self) -> &mut $crate::MetaStatus {
            &mut self.status
        }

        fn app_version(&self) -> &str {
            &self.app_version
        }

        fn app_version_mut(&mut self) -> &mut String {
            &mut self.app_version
        }

        fn created_at(&self) -> &str {
            &self.created_at
        }

        fn created_at_mut(&mut self) -> &mut String {
            &mut self.created_at
        }

        fn updated_at(&self) -> &str {
            &self.updated_at
        }

        fn updated_at_mut(&mut self) -> &mut String {
            &mut self.updated_at
        }
    };
}
