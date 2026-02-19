use anchor_lang::prelude::*;

declare_id!("4JU8utpp6Ldg2THKLFA6TVetzenwm4h2evwEBTrEpLJT");

#[program]
pub mod week1_challenge {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
