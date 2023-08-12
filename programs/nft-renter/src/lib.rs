pub mod helpers;
pub mod instructions;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("UdJoMb9zUSkAysoSnuxJB2x5k8bn8AqgsS1BaEqTroE");

#[program]
pub mod nft_renter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn list_nft (ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn delist_nft (ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn rent_nft(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn charge_fee_and_send_nft_back(ctx: Context<Initialize>) -> Result<()> { 
        Ok(())
    }
}
