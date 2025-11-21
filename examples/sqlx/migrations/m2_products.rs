use async_trait::async_trait;
use migratex::Migration;
use okerr::Result;

use crate::context::MigContext;

/// Second migration: creates products table.
pub struct M2Products;

#[async_trait]
impl Migration<MigContext> for M2Products {
    fn version(&self) -> i32 {
        2
    }

    async fn up(&self, ctx: &mut MigContext) -> Result<()> {
        println!(
            "UP: M2Products (version {}). Creating products table...\n",
            self.version()
        );

        // Use a transaction to ensure atomicity
        let mut tx = ctx.db.begin().await?;

        // Create products table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS products (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                price REAL NOT NULL
            )
            "#,
        )
        .execute(&mut *tx)
        .await?;

        // Commit the transaction
        tx.commit().await?;

        println!("✓ Table 'products' created successfully\n");

        Ok(())
    }

    async fn down(&self, ctx: &mut MigContext) -> Result<()> {
        println!(
            "DOWN: M2Products (version {}). Dropping products table...\n",
            self.version()
        );

        // Use a transaction to ensure atomicity
        let mut tx = ctx.db.begin().await?;

        sqlx::query("DROP TABLE IF EXISTS products")
            .execute(&mut *tx)
            .await?;

        // Commit the transaction
        tx.commit().await?;

        println!("✓ Table 'products' dropped successfully\n");

        Ok(())
    }
}
