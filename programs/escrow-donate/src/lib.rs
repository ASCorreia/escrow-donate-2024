use anchor_lang::prelude::*;

declare_id!("2xTyo53CgbFQHt9ibVxqKgA1JPS1WYTYkqiDQA2SXsUm");

pub mod state;
pub mod contexts;

pub use contexts::*;

#[program]
pub mod escrow_donate {
    use super::*;

    pub fn make(ctx: Context<Make>, amount: u64) -> Result<()> {
        ctx.accounts.make(amount, &ctx.bumps)

    }
}
