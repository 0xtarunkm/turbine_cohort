use anchor_lang::prelude::*;

mod states;
mod contexts;
mod error;

use contexts::*;

declare_id!("8uWdqibpEv7ooTPTzXHUncCbY6JGSff9kMK26YGo26GQ");

#[program]
pub mod nft_marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, name: String, fee: u16) -> Result<()> {
        ctx.accounts.init(name, fee, &ctx.bumps)
    }
}