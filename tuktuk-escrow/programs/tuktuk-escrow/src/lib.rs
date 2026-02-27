pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use instructions::*;
pub use state::*;

declare_id!("Cg88obve3squf9ai5S6chPkfKFw4qLA5DvoLix7W4kq6");

#[program]
pub mod tuktuk_escrow {
    use super::*;

    pub fn make(ctx: Context<Make>, seed: u64, deposit: u64, receive: u64) -> Result<()> {
        ctx.accounts.init_escrow(seed, receive, &ctx.bumps)?;
        ctx.accounts.deposit(deposit)
    }

    pub fn take(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.validate()?;
        ctx.accounts.deposit()?;
        ctx.accounts.withdraw_and_close_vault()
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        ctx.accounts.refund_and_close_vault()
    }

    pub fn auto_refund(ctx: Context<AutoRefund>) -> Result<()> {
        ctx.accounts.auto_refund()
    }

    pub fn schedule(ctx: Context<Schedule>, task_id: u16) -> Result<()> {
        ctx.accounts.schedule(task_id, &ctx.bumps)
    }
}