use anchor_lang::prelude::*;
use anchor_lang::InstructionData;
use anchor_lang::solana_program::instruction::Instruction;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};
use tuktuk_program::{
    compile_transaction,
    tuktuk::{
        cpi::{accounts::QueueTaskV0, queue_task_v0},
        program::Tuktuk,
        types::TriggerV0,
    },
    types::QueueTaskArgsV0,
    TransactionSourceV0,
};

use crate::state::Escrow;
use crate::instructions::make::REFUND_DELAY_SECONDS;

/// Schedules the auto-refund via TukTuk.
/// Called by the maker after creating the escrow.
/// Compiles the `auto_refund` instruction and queues it as a TukTuk task
/// with a timestamp trigger set to the refund deadline.
#[derive(Accounts)]
#[instruction(task_id: u16)]
pub struct ScheduleRefund<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(
        has_one = maker,
        seeds = [b"escrow", maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump,
    )]
    pub escrow: Account<'info, Escrow>,

    pub mint_a: InterfaceAccount<'info, Mint>,

    #[account(
        associated_token::mint = mint_a,
        associated_token::authority = maker,
    )]
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,

    #[account(
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    // --- TukTuk accounts ---
    /// CHECK: The task queue account
    #[account(mut)]
    pub task_queue: UncheckedAccount<'info>,
    /// CHECK: The task queue authority PDA that has permission to queue
    pub task_queue_authority: UncheckedAccount<'info>,
    /// CHECK: Initialized by TukTuk CPI
    #[account(mut)]
    pub task: UncheckedAccount<'info>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub tuktuk_program: Program<'info, Tuktuk>,
}

impl<'info> ScheduleRefund<'info> {
    pub fn handler(&self, task_id: u16) -> Result<()> {
        msg!("Scheduling auto-refund for escrow at deadline: {}", self.escrow.refund_deadline);

        // Build the auto_refund instruction that TukTuk will execute
        let auto_refund_ix = Instruction {
            program_id: crate::ID,
            accounts: vec![
                // maker (mut, not signer — permissionless call)
                anchor_lang::solana_program::instruction::AccountMeta::new(self.maker.key(), false),
                // mint_a
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(self.mint_a.key(), false),
                // maker_ata_a (mut)
                anchor_lang::solana_program::instruction::AccountMeta::new(self.maker_ata_a.key(), false),
                // escrow (mut)
                anchor_lang::solana_program::instruction::AccountMeta::new(self.escrow.key(), false),
                // vault (mut)
                anchor_lang::solana_program::instruction::AccountMeta::new(self.vault.key(), false),
                // token_program
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(self.token_program.key(), false),
                // system_program
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(self.system_program.key(), false),
            ],
            data: crate::instruction::AutoRefund.data(),
        };

        // Compile the instruction into a TukTuk transaction
        let (compiled_tx, _) = compile_transaction(
            vec![auto_refund_ix],
            vec![],
        )
        .map_err(|_| error!(crate::EscrowError::ScheduleFailed))?;

        // Queue the task with a timestamp trigger = refund_deadline
        queue_task_v0(
            CpiContext::new(
                self.tuktuk_program.to_account_info(),
                QueueTaskV0 {
                    payer: self.maker.to_account_info(),
                    queue_authority: self.maker.to_account_info(),
                    task_queue: self.task_queue.to_account_info(),
                    task_queue_authority: self.task_queue_authority.to_account_info(),
                    task: self.task.to_account_info(),
                    system_program: self.system_program.to_account_info(),
                },
            ),
            QueueTaskArgsV0 {
                trigger: TriggerV0::Timestamp(self.escrow.refund_deadline),
                transaction: TransactionSourceV0::CompiledV0(compiled_tx),
                crank_reward: None,
                free_tasks: 0,
                id: task_id,
                description: format!(
                    "auto-refund escrow {} seed {}",
                    self.escrow.key(),
                    self.escrow.seed
                ),
            },
        )?;

        msg!("Auto-refund task scheduled with TukTuk (task_id: {})", task_id);

        Ok(())
    }
}