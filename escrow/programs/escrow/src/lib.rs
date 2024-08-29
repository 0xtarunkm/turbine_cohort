use anchor_lang::prelude::*;

mod contexts;
mod states;

use contexts::*;
declare_id!("HLXsWsuCgWJrSSCgjQa9TF88d8GfAhtFXqXUP29qN7FB");

#[program]
pub mod escrow {
    use super::*;

    pub fn make(ctx: Context<Make>, seed: u64, amount: u64, receive: u64) -> Result<()> {       
        ctx.accounts.save_escrow(seed, receive, &ctx.bumps)?;
        ctx.accounts.deposit_to_vault(amount)
    }

    pub fn take(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.transfer_to_maker()?;
        ctx.accounts.withdraw_and_close()
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        ctx.accounts.withdraw_and_close()
    }
}