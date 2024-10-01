use anchor_lang::prelude::*;

mod contexts;
mod states;

declare_id!("Dk9Gr3w1VFMhkiMLWLU7PPStourMr6UbwNVAbS96dmvG");
#[program]
pub mod amm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
