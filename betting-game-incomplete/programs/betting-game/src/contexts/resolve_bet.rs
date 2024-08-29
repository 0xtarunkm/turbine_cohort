use anchor_instruction_sysvar::Ed25519InstructionSignatures;
use anchor_lang::prelude::*;
use solana_program::{
    ed25519_program, hash::hash, sysvar::instructions::load_instruction_at_checked,
};

use crate::{error::CustomError, states::Bet};

#[derive(Accounts)]
pub struct ResolveBet<'info> {
    #[account(mut)]
    player: Signer<'info>,
    house: SystemAccount<'info>,
    // we don't need to close the vault because it is a system account we only need to transfer all
    // the SOL from it to close it
    #[account(
        mut,
        seeds = [b"vault", house.key().as_ref()],
        bump
    )]
    vault: SystemAccount<'info>,
    #[account(
        mut,
        close = player,
        seeds = [b"bet", vault.key().as_ref(), bet.seed.to_le_bytes().as_ref()],
        bump = bet.bump
    )]
    bet: Account<'info, Bet>,
    #[account(
        address = solana_program::sysvar::instructions::ID
    )]
    instructions_sysvar: UncheckedAccount<'info>,
    system_program: Program<'info, System>,
}

impl<'info> ResolveBet<'info> {
    pub fn verify_ed25519_signature(&mut self, sig: &[u8], bumps: &ResolveBetBumps) -> Result<()> {
        let ix = load_instruction_at_checked(0, &self.instructions_sysvar.to_account_info())?;

        require_keys_eq!(ix.program_id, ed25519_program::ID, CustomError::CustomError);

        require_eq!(ix.accounts.len(), 0, CustomError::CustomError);

        let signatures = Ed25519InstructionSignatures::unpack(&ix.data)?.0;

        require_eq!(signatures.len(), 1, CustomError::CustomError);
        let signature = &signatures[0];

        require!(signature.is_verifiable, CustomError::CustomError);

        require_keys_eq!(
            signature.public_key.unwrap(),
            self.house.key(),
            CustomError::CustomError
        );

        require!(
            signature.signature.unwrap().eq(sig),
            CustomError::CustomError
        );

        require!(
            signature.message.as_ref().unwrap().eq(&self.bet.to_slice()),
            CustomError::CustomError
        );

        Ok(())
    }

    pub fn resolve_bet(&mut self, sig: &[u8], bumps: &ResolveBetBumps) -> Result<()> {
        let hash = hash(sig).to_bytes();

        let mut hash_16: [u8; 16] = [0; 16];
        hash_16.copy_from_slice(&hash[0..16]);
        let lower = u128::from_le_bytes(hash_16);

        hash_16.copy_from_slice(&hash[16..32]);
        let upper = u128::from_le_bytes(hash_16);

        Ok(())
    }
}
