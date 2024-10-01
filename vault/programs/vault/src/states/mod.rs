use anchor_lang::prelude::*;

// used for creating an account
#[account]
#[derive(InitSpace)]
pub struct VaultState {
    pub value: u8,
    pub vault_bump: u8,
    pub state_bump: u8,
}

