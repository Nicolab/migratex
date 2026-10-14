// This file is part of "Migratex - A Migrations Toolkit".
//
// This source code is licensed under the MIT license, please view the LICENSE
// file distributed with this source code. For the full
// information and documentation: https://github.com/nicolab/migratex
// -----------------------------------------------------------------------------

use okerr::{Result, ensure};

/// Check that a metadata table name is a plain SQL identifier.
///
/// The table name is interpolated into SQL statements (it cannot be a bind
/// parameter), so it is restricted to `[A-Za-z_][A-Za-z0-9_]*` to rule out
/// any injection through `with_table_name`.
pub(crate) fn checked_table_name(name: &str) -> Result<&str> {
    let mut chars = name.chars();
    let valid = chars
        .next()
        .is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_');

    ensure!(
        valid,
        "invalid metadata table name {name:?}: expected [A-Za-z_][A-Za-z0-9_]*"
    );

    Ok(name)
}
