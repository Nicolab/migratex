// This file is part of "Migratex - A Migrations Toolkit".
//
// This source code is licensed under the MIT license, please view the LICENSE
// file distributed with this source code. For the full
// information and documentation: https://github.com/nicolab/migratex
// -----------------------------------------------------------------------------

#[cfg(feature = "json")]
use serde;

/// The status of a migration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
pub enum MetaStatus {
    Clean,
    Migrating,
    Failed,
}

impl Default for MetaStatus {
    fn default() -> Self {
        Self::Clean
    }
}

pub trait Metadata {
    //
    // -- CORE: logical fields
    //

    /// Migration version.
    fn version(&self) -> i32;
    fn version_mut(&mut self) -> &mut i32;

    /// Application version.
    /// By default, it is the project `package.version` field (Cargo).
    fn app_version(&self) -> &str;
    fn app_version_mut(&mut self) -> &mut String;

    /// Migration status.
    fn status(&self) -> MetaStatus;
    fn status_mut(&mut self) -> &mut MetaStatus;

    /// Creation date (first usage).
    fn created_at(&self) -> &str;
    fn created_at_mut(&mut self) -> &mut String;

    /// Last update date.
    fn updated_at(&self) -> &str;
    fn updated_at_mut(&mut self) -> &mut String;

    //
    // -- Helpers (with default implementations)
    //

    /// Get the status as a string.
    fn to_status_str(&self) -> &str {
        match self.status() {
            MetaStatus::Clean => "Clean",
            MetaStatus::Migrating => "Migrating",
            MetaStatus::Failed => "Failed",
        }
    }

    /// Update `updated_at` with current time.
    fn touch_updated(&mut self) {
        *self.updated_at_mut() = chrono::Utc::now().to_rfc3339();
    }

    /// Set the version and update `updated_at`.
    fn set_version(&mut self, v: i32) {
        *self.version_mut() = v;
        self.touch_updated();
    }

    /// Set the app_version and update `updated_at`.
    fn set_app_version(&mut self, v: String) {
        *self.app_version_mut() = v;
        self.touch_updated();
    }

    /// Set the status and update `updated_at`.
    fn set_status(&mut self, s: MetaStatus) {
        *self.status_mut() = s;
        self.touch_updated();
    }

    /// Set the status to `Clean` and update `updated_at`.
    fn mark_clean(&mut self) {
        self.set_status(MetaStatus::Clean);
    }

    /// Set the status to `Migrating` and update `updated_at`.
    fn mark_migrating(&mut self) {
        self.set_status(MetaStatus::Migrating);
    }

    /// Set the status to `Failed` and update `updated_at`.
    fn mark_failed(&mut self) {
        self.set_status(MetaStatus::Failed);
    }
}
