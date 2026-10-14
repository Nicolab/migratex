// This file is part of "Migratex - A Migrations Toolkit".
//
// This source code is licensed under the MIT license, please view the LICENSE
// file distributed with this source code. For the full
// information and documentation: https://github.com/nicolab/migratex
// -----------------------------------------------------------------------------

#[cfg(feature = "json")]
mod json_metadata;

#[cfg(any(feature = "sqlx", feature = "postgres"))]
mod sql_identifier;

#[cfg(feature = "sqlx")]
mod sqlite_metadata;

#[cfg(feature = "postgres")]
mod postgres_metadata;

#[cfg(feature = "json")]
pub use json_metadata::*;

#[cfg(feature = "sqlx")]
pub use sqlite_metadata::*;

#[cfg(feature = "postgres")]
pub use postgres_metadata::*;
