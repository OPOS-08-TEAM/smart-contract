pub mod helpers;
pub mod instructions;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("UdJoMb9zUSkAysoSnuxJB2x5k8bn8AqgsS1BaEqTroE");

#[program]
pub mod nft_renter {
    use super::*;

    pub fn initialize_instruction(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn list_nft_instruction (ctx: Context<ListNft>) -> Result<()> {
        list_nft(ctx)?;
        Ok(())
    }

    pub fn delist_nft_instruction (ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn rent_nft_instruction (ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn charge_fee_and_send_nft_back_instruction (ctx: Context<Initialize>) -> Result<()> { 
        Ok(())
    }
}
