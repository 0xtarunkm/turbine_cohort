use anchor_lang::prelude::*;

mod contexts;
mod error;
mod states;

use contexts::*;
use error::*;

declare_id!("Au2hTusvyqhxAHEEJxMh8LV48ke7d6sFGxd7cNtMh9qX");

#[program]
pub mod betting_game {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
