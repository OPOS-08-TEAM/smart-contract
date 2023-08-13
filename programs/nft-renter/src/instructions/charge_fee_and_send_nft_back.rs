use crate::helpers::consts::*;
use crate::helpers::errors::*;
use crate::instructions::state::*;

use anchor_lang::prelude::*;
use anchor_spl::token::{ Token, TokenAccount };

#[derive(Accounts)]
#[instruction()]
pub struct ChargeFeeAndSendNftBack {
    
}