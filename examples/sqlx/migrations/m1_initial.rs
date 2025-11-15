use anyhow::Result;
use async_trait::async_trait;

use crate::context::MigContext;
use migratex::Migration;

/// Initial migration: creates users and subscriptions tables.
pub struct M1Initial;

#[async_trait]
impl Migration<MigContext> for M1Initial {
    fn version(&self) -> i32 {
        1
    }

    async fn up(&self, ctx: &mut MigContext) -> Result<()> {
        println!(
            "UP: M1Initial (version {}). Creating users and subscriptions tables...\n",
            self.version()
        );

        // Use a transaction to ensure atomicity
        let mut tx = ctx.pool.begin().await?;

        // Create users table
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                email TEXT NOT NULL UNIQUE
            )",
        )
        .execute(&mut *tx)
        .await?;

        // Create subscriptions table with foreign key to users
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS subscriptions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                user_id INTEGER NOT NULL,
                plan TEXT NOT NULL,
                FOREIGN KEY (user_id) REFERENCES users(id)
            )",
        )
        .execute(&mut *tx)
        .await?;

        // Commit the transaction
        tx.commit().await?;

        println!("✓ Tables 'users' and 'subscriptions' created successfully\n");

        Ok(())
    }

    async fn down(&self, ctx: &mut MigContext) -> Result<()> {
        println!(
            "DOWN: M1Initial (version {}). Dropping users and subscriptions tables...\n",
            self.version()
        );

        // Use a transaction to ensure atomicity
        let mut tx = ctx.pool.begin().await?;

        // Drop tables in reverse order due to foreign key constraints
        sqlx::query("DROP TABLE IF EXISTS subscriptions")
            .execute(&mut *tx)
            .await?;

        sqlx::query("DROP TABLE IF EXISTS users")
            .execute(&mut *tx)
            .await?;

        // Commit the transaction
        tx.commit().await?;

        println!("✓ Tables 'subscriptions' and 'users' dropped successfully\n");

        Ok(())
    }
}
