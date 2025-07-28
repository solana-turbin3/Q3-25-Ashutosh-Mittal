use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, 
    token_interface::{ Mint, TokenAccount, TransferChecked, TokenInterface, transfer_checked}
};

use crate::error::NftMintError;
use crate::state::NFTProgramInfo;

#[derive(Accounts)]
pub struct ApproveMiner<'info> {
    pub authority: Signer<'info>,
    #[account(
        mut,
        seeds = [b"nft_program_info"],
        bump = nft_program_info.bump,
        has_one = authority
    )]
    pub nft_program_info: Account<'info, NFTProgramInfo>,
    pub system_program: Program<'info, System>,
}

impl<'info> ApproveMiner<'info> {

    pub fn approve_miner(&mut self, miner_to_add: Pubkey) -> Result<()> {

        require!(!self.nft_program_info.miners.contains(&miner_to_add), NftMintError::MinerAlreadyApproved);
        self.nft_program_info.miners.push(miner_to_add);
        
        Ok(())
    }
}