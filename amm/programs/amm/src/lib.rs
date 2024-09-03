use anchor_lang::prelude::*;

mod contexts;
mod states;

use contexts::*;

declare_id!("CZUg2ghfih89Put1WYEdH4CXQEzvamrt63xcmGjzRt8j");

#[program]
pub mod amm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, seed: u64, fee: u16, authority: Option<Pubkey>) -> Result<()> {
        ctx.accounts.save_config(seed, fee, authority, &ctx.bumps)
    }
}