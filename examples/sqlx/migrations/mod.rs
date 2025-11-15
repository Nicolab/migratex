use crate::context::MigContext;
use migratex::BoxMigration;

mod m1_initial;
mod m2_products;

/// Returns the list of migrations in order.
/// IMPORTANT: Migrations must be sorted by version (ascending).
pub fn migrations() -> Vec<BoxMigration<MigContext>> {
    vec![
        Box::new(m1_initial::M1Initial),
        Box::new(m2_products::M2Products),
    ]
}
