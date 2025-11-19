use okerr::Result;
use async_trait::async_trait;

use crate::context::MigContext;
use migratex::Migration;

pub struct M2Something;

#[async_trait]
impl Migration<MigContext> for M2Something {
    fn version(&self) -> i32 {
        2
    }

    async fn up(&self, ctx: &mut MigContext) -> Result<()> {
        println!(
            "UP: M2Something. Version {} / Context: {:?}\n",
            self.version(),
            ctx
        );

        ctx.foo += " + foo from M2Something";
        ctx.bar += " + bar from M2Something";

        Ok(())
    }

    async fn down(&self, ctx: &mut MigContext) -> Result<()> {
        println!(
            "DOWN: M2Something. Version {} / Context: {:?}\n",
            self.version(),
            ctx
        );

        ctx.foo = ctx.foo.replace(" + foo from M2Something", "").to_string();
        ctx.bar = ctx.bar.replace(" + bar from M2Something", "").to_string();

        Ok(())
    }
}
