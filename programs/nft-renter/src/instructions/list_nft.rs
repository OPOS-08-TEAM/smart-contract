use crate::helpers::consts::*;
use crate::helpers::errors::*;
use crate::instructions::state::*;

use anchor_lang::prelude::*;
use anchor_spl::token::{ self, Mint, Token, TokenAccount, Transfer };
use anchor_spl::associated_token::AssociatedToken;

#[derive(Accounts)]
#[instruction()]
pub struct ListNft<'info> {
    // Check account seed and init if required
    #[account(
        init_if_needed, 
        seeds = [
            LIST_INFO_SEED, 
            initializer.key().as_ref(), 
            mint.key().as_ref()
        ], 
        bump, 
        payer = initializer, 
        space = std::mem::size_of::<UserListInfo>() + 8 
    )]
    pub list_info: Account<'info, UserListInfo>,

    // Check if initializer is signer, mut is required to reduce lamports (fees)
    #[account(mut)]
    pub initializer: Signer<'info>,

    // Check if token account owner is the initializer and check if token amount = 1
    #[account(
        mut,
        constraint = user_nft_account.owner.key() == initializer.key(),
        constraint = user_nft_account.amount == 1
    )]
    pub user_nft_account: Account<'info, TokenAccount>,

    // Init if needed
    #[account(
        init_if_needed,
        payer = initializer, // If init required, payer will be initializer
        associated_token::mint = mint, // If init required, mint will be set to Mint
        associated_token::authority = list_info // If init required, authority set to PDA
    )]
    pub pda_nft_account: Account<'info, TokenAccount>,
    // mint is required to create new account for PDA and for checking
    pub mint: Account<'info, Mint>,
    // Token Program required to call transfer instruction
    pub token_program: Program<'info, Token>,
    // ATA Program required to create ATA for pda_nft_account
    pub associated_token_program: Program<'info, AssociatedToken>,
    // System Program requred since a new account may be created and there's a deduction of lamports (fees/rent)
    pub system_program: Program<'info, System>,
    // Rent required to get Rent
    pub rent: Sysvar<'info, Rent>,
}

pub fn list_nft(ctx: Context<ListNft>, amount: u64) -> Result<()> {
    // Proceed to transfer
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_accounts = Transfer {
        from: ctx.accounts.user_nft_account.to_account_info(),
        to: ctx.accounts.pda_nft_account.to_account_info(),
        authority: ctx.accounts.initializer.to_account_info(),
    };
    let token_transfer_context: CpiContext<'_, '_, '_, '_, Transfer<'_>> = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(token_transfer_context, 1)?;

    // Populate list_info info
    ctx.accounts.list_info.mint = ctx.accounts.mint.key();
    ctx.accounts.list_info.lister = ctx.accounts.initializer.key();
    ctx.accounts.list_info.owner = ctx.accounts.initializer.key();
    ctx.accounts.list_info.amount = amount;

    Ok(())
}