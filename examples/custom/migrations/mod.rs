mod m1_initial;
mod m2_something;
// mod m3_other;

use migratex::BoxMigration;

use crate::MigContext;

pub fn migrations() -> Vec<BoxMigration<MigContext>> {
    // IMPORTANT: sort by version (ascending)
    vec![
        Box::new(m1_initial::M1Initial),
        Box::new(m2_something::M2Something),
        // Box::new(m2_other::M2Other),
        // ...
    ]
}
