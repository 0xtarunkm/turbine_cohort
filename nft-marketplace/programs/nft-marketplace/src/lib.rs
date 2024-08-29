use anchor_lang::prelude::*;

mod states;
mod contexts;

use contexts::*;

declare_id!("9FdZSLixE2hinhcCnWjQ8nWWv6sr1e37AHRpb1qgE7ns");

#[program]
pub mod nft_marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
