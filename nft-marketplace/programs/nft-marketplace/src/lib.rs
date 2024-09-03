use anchor_lang::prelude::*;

mod states;
mod contexts;
mod error;

use contexts::*;

declare_id!("9FdZSLixE2hinhcCnWjQ8nWWv6sr1e37AHRpb1qgE7ns");

#[program]
pub mod nft_marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, name: String, fee: u16) -> Result<()> {
        ctx.accounts.init(name, fee, &ctx.bumps)
    }
}