use anchor_lang::prelude::*;

pub mod contexts;
use contexts::*;

pub mod state;

declare_id!("GZaT6yDCxRqypRfY5Ndxfz7kCcZdeGCk1tUfic6vxiHu");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)
    }

    pub fn payments(ctx: Context<Payments>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)?;
        ctx.accounts.withdraw(amount)
    }

    pub fn close(ctx: Context<Close>) -> Result<()> {
        ctx.accounts.close()
    }
} 

