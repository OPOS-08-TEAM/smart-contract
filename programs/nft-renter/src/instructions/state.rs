use anchor_lang::prelude::*;
// use anchor_spl::token::{ Mint, Token, TokenAccount };


// #[account]
// pub struct NftOwner {
//     pub nft_owner: Pubkey,
//     pub nft_renter: Pubkey,
//     // pub token_program: Program<'info, Token>
// }

// impl NftOwner {
//     pub const STORAGE_SIZE: usize = 8 + std::mem::size_of::<NftOwner>();
// }

#[account]
pub struct UserListInfo {
    pub lister: Pubkey,
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub amount: u64
}

#[account]
pub struct UserRentInfo {
    pub renter: Pubkey,
    pub mint: Pubkey
}