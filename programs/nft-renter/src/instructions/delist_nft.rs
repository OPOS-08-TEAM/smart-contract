use crate::helpers::consts::*;
use crate::helpers::errors::*;
use crate::instructions::state::*;

use anchor_lang::prelude::*;
use anchor_spl::token::{ self, Token, TokenAccount, Mint, Transfer };


#[derive(Accounts)]
#[instruction()]

pub struct DelistNft<'info> {
    // Check account seed and init if required
    #[account(
        mut, seeds=[b"stake_info", initializer.key().as_ref(), mint.key().as_ref()], bump,
        constraint = initializer.key() == list_info.lister,
        close = initializer
    )]
    pub list_info: Account<'info, UserListInfo>,
    // Check if initializer is signer, mut is required to reduce lamports (fees)
    #[account(mut)]
    pub initializer: Signer<'info>,
    // Check if token account owner is correct owner, mint and has amount of 0
    #[account(
        mut,
        constraint = user_nft_account.owner.key() == initializer.key(),
        constraint = user_nft_account.mint == mint.key(),
        constraint = user_nft_account.amount == 0
    )]
    pub user_nft_account: Account<'info, TokenAccount>,
    // Check if accounts has correct owner, mint and has amount of 1
    #[account(
        mut,
        constraint = pda_nft_account.owner == list_info.key(),
        constraint = pda_nft_account.mint == mint.key(),
        constraint = pda_nft_account.amount == 1,
    )]
    pub pda_nft_account: Account<'info, TokenAccount>,
    // mint is required to check staking_info, user_nft_account, and pda_nft_account
    #[account(constraint = list_info.mint == mint.key())]
    pub mint: Account<'info, Mint>,
    // Token Program required to call transfer instruction
    pub token_program: Program<'info, Token>,
    // System Program requred for deduction of lamports (fees)
    pub system_program: Program<'info, System>,
}

pub fn delist_nft(ctx: Context<DelistNft>) -> Result<()> {
    // Proceed to transfer
    let auth_bump = *ctx.bumps.get("list_info").unwrap();
    let seeds = &[
        b"stake_info".as_ref(),
        &ctx.accounts.initializer.key().to_bytes(),
        &ctx.accounts.mint.key().to_bytes(),
        &[auth_bump],
    ];
    let signer = &[&seeds[..]];
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_accounts = Transfer {
        from: ctx.accounts.pda_nft_account.to_account_info(),
        to: ctx.accounts.user_nft_account.to_account_info(),
        authority: ctx.accounts.list_info.to_account_info(),
    };
    let token_transfer_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
    token::transfer(token_transfer_context, 1)?;

    Ok(())
}