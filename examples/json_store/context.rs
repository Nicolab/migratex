// This is just an example of a migration context.
// It could be anything, like a database connection, a file handle, etc.
// It is passed to each migration.

#[derive(Debug, Default, Clone)]
pub struct MigContext {
    pub foo: String,
    pub bar: String,
}
