use crate::helpers::consts::*;
use crate::helpers::errors::*;
use crate::helpers::transfer_native_to_account;
use crate::instructions::state::*;

use anchor_lang::prelude::*;
use anchor_spl::token::{ self, Mint, Token, TokenAccount, Transfer };
use anchor_spl::associated_token::AssociatedToken;

#[derive(Accounts)]
#[instruction()]
pub struct RentNft<'info> {
    // Chek account seed and init if required for lister and renter
    #[account(
        seeds = [
            LIST_INFO_SEED,
            lister.key().as_ref(),
            mint.key().as_ref()
        ],
        bump,
    )]
    pub list_info: Account<'info, UserListInfo>,

    /// CHECK: This account is used to verify escrow_token_account_lister
    pub lister: AccountInfo<'info>,

    #[account(
        associated_token::mint = mint, // If init required, mint will be set to Mint
        associated_token::authority = list_info // If init required, authority set to PDA
    )]
    pub pda_nft_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub renter: Signer<'info>,

    // #[account(
    //     init_if_needed,
    //     seeds = [
    //         RENT_INFO_SEED,
    //         renter.key().as_ref(),
    //         mint.key().as_ref()
    //     ],
    //     bump,
    //     payer = renter,
    //     space = std::mem::size_of::<UserRentInfo>() + 8,
    // )]
    // pub rent_info: Account<'info, UserRentInfo>,

    // #[account(
    //     init_if_needed,
    //     payer = renter,
    //     associated_token::mint = mint,
    //     associated_token::authority = rent_info,
    // )]
    // pub pda_rent_account: Account<'info, TokenAccount>,

    // mint is required to create new account for PDA and for checking
    pub mint: Account<'info, Mint>,

    // // Token Program required to call transfer instruction
    // pub token_program: Program<'info, Token>,

    // // ATA Program required to create ATA for pda_nft_account
    // pub associated_token_program: Program<'info, AssociatedToken>,

    // System Program requred since a new account may be created and there's a deduction of lamports (fees/rent)
    pub system_program: Program<'info, System>,
    
    // // Rent required to get Rent
    // pub rent: Sysvar<'info, Rent>,
}

pub fn rent_nft(ctx: Context<RentNft>) -> Result<()> {
    // Proceed to renting nft
    let amount = ctx.accounts.list_info.amount;
    let seeds = None;
    transfer_native_to_account(
        ctx.accounts.renter.to_account_info(), 
        ctx.accounts.lister.to_account_info(), 
        amount, 
        ctx.accounts.system_program.to_account_info(), 
        seeds
    )?;

    // Assign rent info 
    ctx.accounts.list_info.owner = ctx.accounts.renter.key();

    Ok(())
}