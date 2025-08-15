use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, token_2022::{mint_to, MintTo}, token_interface::{  Mint, TokenAccount, TokenInterface}
};

use crate::{error::NftMintError};
use crate::state::{ProgramState, MinerInfo};

#[derive(Accounts)]
#[instruction(nft_id: u64)]
pub struct OnboardMinerMint<'info> {
  #[account(mut)]
  pub miner: Signer<'info>,

  #[account(mut)]
  /// CHECK: This is safe account 
    pub authority: SystemAccount<'info>,


    // ---- ProgramState ----
    #[account(  
        mut,
        seeds = [b"program_state"],
        bump = program_state.program_state_bump,
        has_one = authority
    )]
    pub program_state: Account<'info, ProgramState>,


// ---- Miner NFT Mint ----
  #[account( 
    mut,
    seeds = ["nft_mint".as_bytes(), miner.key().as_ref(), nft_id.to_le_bytes().as_ref()], 
    mint::token_program = token_program,
    bump,
    )]
  pub miner_nft_mint: Box<InterfaceAccount<'info, Mint>>,



// ---- Miner Info ----
    #[account( 
    mut, 
    seeds = ["miner".as_bytes(), miner.key().as_ref()], 
    bump = miner_info.miner_bump,
    )]
    pub miner_info: Box<Account<'info, MinerInfo>>,


// ---- BHRT Mint ----
        #[account(
    mut,
    seeds=[b"BHRT"],
    bump = program_state.bhrt_mint_bump,
    mint::token_program = token_program
)]
    pub bhrt_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer= miner,
        associated_token::mint= bhrt_mint,
        associated_token::authority=miner,
        associated_token::token_program = token_program
    )]
    pub miner_bhrt: InterfaceAccount<'info, TokenAccount>,

    // ---- Required Programs ----
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}



impl<'info> OnboardMinerMint<'info> {


     pub fn onboard_miner_mint( &mut self,nft_id: u64, mining_power: u64) -> Result<()> {

        require!(self.program_state.approved_miners.contains(&self.miner.key()), NftMintError::MinerNotApproved);


mint_to(
    CpiContext::new_with_signer(
        self.token_program.to_account_info(),
        MintTo {
            mint: self.bhrt_mint.to_account_info(),
            to: self.miner_bhrt.to_account_info(),
            authority: self.program_state.to_account_info(),
        },
        &[&[b"program_state".as_ref(), &[self.program_state.program_state_bump]]],
    ),
    mining_power * 10, 
)?;



self.miner_info.hashrate_power = mining_power;
self.miner_info.mint_amount = mining_power * 10;
self.miner_info.hashrate_token_mint = self.bhrt_mint.key();


        Ok(())
    }





}

