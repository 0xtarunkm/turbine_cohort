use anchor_lang::prelude::*;

mod contexts;
mod states;

declare_id!("CZUg2ghfih89Put1WYEdH4CXQEzvamrt63xcmGjzRt8j");

#[program]
pub mod amm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        // save_config?
        // deposit_token_x
        // deposit_token_y
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
