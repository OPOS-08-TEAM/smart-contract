use anchor_lang::prelude::*;

declare_id!("UdJoMb9zUSkAysoSnuxJB2x5k8bn8AqgsS1BaEqTroE");

#[program]
pub mod nft_renter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
