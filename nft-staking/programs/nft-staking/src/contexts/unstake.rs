use anchor_lang::prelude::*;
use anchor_spl::{metadata::{MasterEditionAccount, Metadata}, token::{Mint, Token, TokenAccount}};

use crate::{error::StakeError, states::{StakeAccount, StakeConfig, UserAccount}};

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut)]
    user: Signer<'info>,
    mint: Account<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = user,
    )]
    mint_ata: Account<'info, TokenAccount>,
    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
            b"edition"
        ],
        seeds::program = metadata_program.key(),
        bump
    )]
    edition: Account<'info, MasterEditionAccount>,
    #[account(
        seeds = [b"config".as_ref()],
        bump = config.bump
    )]
    config: Account<'info, StakeConfig>,
    #[account(
        mut,
        close = user,
        seeds = [b"stake".as_ref(), mint.key().as_ref(), config.key().as_ref()],
        bump
    )]
    stake_account: Account<'info, StakeAccount>,
    #[account(
        mut,
        seeds = [b"user".as_ref(), user.key().as_ref()],
        bump = user_account.bump
    )]
    user_account: Account<'info, UserAccount>,
    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
    metadata_program: Program<'info, Metadata>
}

impl<'info> Unstake<'info> {
    pub fn unstake(&mut self) -> Result<()> {
        let time_elapsed = ((Clock::get()?.unix_timestamp - self.stake_account.staked_at)/86400) as u32;

        require!(time_elapsed >= self.config.freeze_period, StakeError::FreezePeriodNotPassed);

        self.user_account.points += time_elapsed as u32 * self.config.points_per_stake as u32;

        let seeds = &[
            b"stake",
            self.mint.to_account_info().key.as_ref(),
            self.config.to_account_info().key.as_ref(),
            &[self.stake_account.bump]
        ];
        let signer_seeds = &[&seeds[..]];

        let delegate = &self.stake_account.to_account_info();
        let token_account = &self.mint_ata.to_account_info();
        let edition = &self.edition.to_account_info();
        let mint = &self.mint.to_account_info();
        let token_program = &self.token_program.to_account_info();
        let metadata_program = &self.metadata_program.to_account_info();
        Ok(())
    }
}