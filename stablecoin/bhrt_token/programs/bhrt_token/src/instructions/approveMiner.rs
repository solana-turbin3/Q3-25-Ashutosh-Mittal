use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, 
    token_interface::{ Mint, TokenAccount, TransferChecked, TokenInterface, transfer_checked}
};

use crate::error::NftMintError;
use crate::state::{ProgramState, MinerInfo};

#[derive(Accounts)]
#[instruction(miner_to_add: Pubkey)]
pub struct ApproveMiner<'info> {
      #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        mut,
        seeds = [b"program_state"],
        bump = program_state.program_state_bump,
        has_one = authority
    )]
    pub program_state: Account<'info, ProgramState>,
    pub system_program: Program<'info, System>,
}

impl<'info> ApproveMiner<'info> {

    pub fn approve_miner(&mut self, miner_to_add: Pubkey) -> Result<()> {

        require!(!self.program_state.approved_miners.contains(&miner_to_add), NftMintError::MinerAlreadyApproved);
        self.program_state.approved_miners.push(miner_to_add);
        
        Ok(())
    }
}