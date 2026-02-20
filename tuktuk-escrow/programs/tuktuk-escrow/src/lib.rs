use anchor_lang::prelude::*;

declare_id!("Cg88obve3squf9ai5S6chPkfKFw4qLA5DvoLix7W4kq6");

#[program]
pub mod tuktuk_escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
