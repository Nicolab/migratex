// This file is part of "Migratex - A Migrations Toolkit".
//
// This source code is licensed under the MIT license, please view the LICENSE
// file distributed with this source code. For the full
// information and documentation: https://github.com/nicolab/migratex
// -----------------------------------------------------------------------------

#[cfg(feature = "json")]
mod json_metadata;

#[cfg(feature = "sqlx")]
mod sqlite_metadata;

#[cfg(feature = "json")]
pub use json_metadata::*;

#[cfg(feature = "sqlx")]
pub use sqlite_metadata::*;
