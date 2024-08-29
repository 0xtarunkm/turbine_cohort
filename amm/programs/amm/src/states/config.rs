use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Config {
    pub authority: Option<Pubkey>,
    pub mint_x: Pubkey,
    pub mint_y: Pubkey,
    pub seed: u64,
    pub config_bump: u8,
    pub auth_bump: u8,
    pub fee: u16,
}
