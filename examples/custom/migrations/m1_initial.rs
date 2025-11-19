use async_trait::async_trait;
use okerr::Result;

use crate::context::MigContext;
use migratex::Migration;

pub struct M1Initial;

#[async_trait]
impl Migration<MigContext> for M1Initial {
    fn version(&self) -> i32 {
        1
    }

    async fn up(&self, ctx: &mut MigContext) -> Result<()> {
        println!(
            "UP: M1Initial. Version {} / Context: {:?}\n",
            self.version(),
            ctx
        );

        ctx.foo += " + foo from M1Initial";
        ctx.bar += " + bar from M1Initial";

        Ok(())
    }

    async fn down(&self, ctx: &mut MigContext) -> Result<()> {
        println!(
            "DOWN: M1Initial. Version {} / Context: {:?}\n",
            self.version(),
            ctx
        );

        ctx.foo = ctx.foo.replace(" + foo from M1Initial", "").to_string();
        ctx.bar = ctx.bar.replace(" + bar from M1Initial", "").to_string();

        Ok(())
    }
}
