use crate::{instruction, state::UserAccount, ID};
use anchor_lang::prelude::*;
use ephemeral_vrf_sdk::{
    instructions::{create_request_randomness_ix, RequestRandomnessParams},
    types::SerializableAccountMeta,
};

// Define VrfProgram type that implements Id trait for Anchor
#[derive(Clone)]
pub struct VrfProgram;

impl anchor_lang::Id for VrfProgram {
    fn id() -> Pubkey {
        ephemeral_vrf_sdk::consts::VRF_PROGRAM_IDENTITY
    }
}

#[derive(Accounts)]
pub struct GenerateDataDelegated<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"user", user.key().as_ref()],
        bump = user_account.bump,
    )]
    pub user_account: Account<'info, UserAccount>,
    /// CHECK: The oracle queue
    #[account(mut)]
    pub oracle_queue: AccountInfo<'info>,
    /// The VRF program
    pub vrf_program: Program<'info, VrfProgram>,
}

impl<'info> GenerateDataDelegated<'info> {
    pub fn generate(&mut self, client_seed: u8) -> Result<()> {
        let ix = create_request_randomness_ix(RequestRandomnessParams {
            payer: self.user.key(),
            oracle_queue: self.oracle_queue.key(),
            callback_program_id: ID,
            callback_discriminator: instruction::VrfCallback::DISCRIMINATOR.to_vec(),
            caller_seed: [client_seed; 32],
            accounts_metas: Some(vec![SerializableAccountMeta {
                pubkey: self.user_account.key(),
                is_signer: false,
                is_writable: true,
            }]),
            ..Default::default()
        });
        
        // CPI to VRF program
        anchor_lang::solana_program::program::invoke_signed(
            &ix,
            &[
                self.user.to_account_info(),
                self.oracle_queue.to_account_info(),
                self.vrf_program.to_account_info(),
            ],
            &[],
        )?;
        
        Ok(())
    }
}