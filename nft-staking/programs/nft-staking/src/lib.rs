use anchor_lang::prelude::*;

mod states;
mod contexts;
mod error;

use contexts::*;
declare_id!("3mcDuBzTMCrLUe6wJQenKKq9KdvMJ2CiFpyFFJWmXHkj");

#[program]
pub mod nft_staking {
    use super::*;

    pub fn initialize_config(ctx: Context<InitializeConfig>, points_per_stake: u8, max_stake: u8, freeze_period: u32, seed: u64) -> Result<()> {
        ctx.accounts.initialize_config(points_per_stake, max_stake, freeze_period, seed, &ctx.bumps)
    }

    pub fn initialize_user(ctx: Context<InitializeUser>) -> Result<()> {
        ctx.accounts.initialize_user(&ctx.bumps)
    }

    pub fn stake(ctx: Context<Stake>) -> Result<()> {
        ctx.accounts.stake(&ctx.bumps)
    }
}