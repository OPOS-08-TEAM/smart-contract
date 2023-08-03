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
}
