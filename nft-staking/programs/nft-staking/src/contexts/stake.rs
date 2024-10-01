use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, metadata::{mpl_token_metadata::instructions::{FreezeDelegatedAccountCpi, FreezeDelegatedAccountCpiAccounts}, MasterEditionAccount, Metadata, MetadataAccount}, token::{approve, Approve, Mint, Token, TokenAccount}};

use crate::{states::{StakeAccount, StakeConfig, UserAccount}, error::StakeError};

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    signer: Signer<'info>,
    mint: Account<'info, Mint>,
    collection: Account<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = signer,
    )]
    mint_ata: Account<'info, TokenAccount>,
    // check if that NFT is part of the collection
    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            mint.key().as_ref()
        ],
        // this is saying that the seeds are not derived from our program ID
        // it is derived from metadata_program program ID
        // since PDAs are derived from seeds and program ID
        seeds::program = metadata_program.key(),
        bump,
        constraint = metadata.collection.as_ref().unwrap().key.as_ref() == collection.key().as_ref(),
        constraint = metadata.collection.as_ref().unwrap().verified == true
    )]
    metadata: Account<'info, MetadataAccount>,
    // this account is what makes it non fungible
    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
            b"edition"
        ],
        // this is saying that the seeds are not derived from our program ID
        // it is derived from metadata_program program ID
        // since PDAs are derived from seeds and program ID
        seeds::program = metadata_program.key(),
        bump
    )]
    edition: Account<'info, MasterEditionAccount>,
    config: Account<'info, StakeConfig>,
    #[account(
        init,
        payer = signer,
        seeds = [b"stake", mint.key().as_ref(), config.key().as_ref()],
        bump,
        space = 8 + StakeAccount::INIT_SPACE
    )]
    stake_account: Account<'info, StakeAccount>,
    #[account(
        mut,
        seeds = [b"user", signer.key().as_ref()],
        bump = user_account.bump
    )]
    user_account: Account<'info, UserAccount>,
    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
    metadata_program: Program<'info, Metadata>,
    associated_token_program: Program<'info, AssociatedToken>
}

impl<'info> Stake<'info> {
    pub fn stake(&mut self, bumps: &StakeBumps) -> Result<()> {

        require!(self.user_account.amount_staked < self.config.max_stake, StakeError::MaxStakeReached);

        let cpi_program = self.token_program.to_account_info();

        // delegating authority of mint_ata to stake_account
        let cpi_accounts = Approve {
            to: self.mint_ata.to_account_info(),
            delegate: self.stake_account.to_account_info(),
            authority: self.signer.to_account_info()
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        approve(cpi_ctx, 1)?;

        let delegate = &self.stake_account.to_account_info();
        let token_account = &self.mint_ata.to_account_info();
        let edition = &self.edition.to_account_info();
        let mint = &self.mint.to_account_info();
        let token_program = &self.token_program.to_account_info();
        let metadata_program = &self.metadata_program.to_account_info();

        FreezeDelegatedAccountCpi::new(
            metadata_program, 
            FreezeDelegatedAccountCpiAccounts {
                delegate,
                token_account,
                edition,
                mint,
                token_program
            }
        ).invoke()?;
        
        self.stake_account.set_inner(StakeAccount { 
            owner: self.signer.key(), 
            mint: self.mint.key(), 
            staked_at: Clock::get()?.unix_timestamp, 
            bump: bumps.stake_account 
        });

        self.user_account.amount_staked += 1;
        
        Ok(())
    }
}