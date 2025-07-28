use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, 
    token_interface::{ Mint, TokenAccount, TransferChecked, TokenInterface, transfer_checked}
};

// use crate::error::NftMintError;
use crate::state::NFTProgramInfo;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        space = 8 + NFTProgramInfo::INIT_SPACE,
        seeds = [b"nft_program_info"],
        bump
    )]
    pub nft_program_info: Account<'info, NFTProgramInfo>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {

    pub fn init_authority(&mut self, bump: &InitializeBumps) -> Result<()>{
        self.nft_program_info.set_inner(NFTProgramInfo { nft_id_counter: 0, authority: self.authority.key(), miners:Vec::new(), bump: bump.nft_program_info });
        Ok(())
    } 
}