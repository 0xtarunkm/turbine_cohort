use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};

use crate::states::StakeConfig;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct InitializeConfig<'info> {
    #[account(mut)]
    admin: Signer<'info>,
    #[account(
        init,
        payer = admin,
        seeds = [b"config", seed.to_le_bytes().as_ref()],
        bump,
        space = StakeConfig::INIT_SPACE
    )]
    config: Account<'info, StakeConfig>,
    #[account(
        init,
        payer = admin,
        seeds = [b"rewards", config.key().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = config,
    )]
    rewards_mint: Account<'info, Mint>,
    system_program: Program<'info, System>,
    token_program: Program<'info, Token>
}

impl<'info> InitializeConfig<'info> {
    pub fn initialize_config(&mut self, points_per_stake: u8, max_stake: u8, freeze_period: u32, seed: u64, bumps: &InitializeConfigBumps) -> Result<()> {
        self.config.set_inner(StakeConfig { 
            points_per_stake, 
            max_stake, 
            freeze_period, 
            rewards_bump: bumps.rewards_mint, 
            seed,
            bump: bumps.config
        });
        
        Ok(())
    }
}