use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, metadata::{
    Metadata
}, token_interface::{ burn, Burn, Mint, TokenAccount, TokenInterface}
};

use mpl_token_metadata::{instructions::{BurnV1CpiBuilder, UnverifyCollectionV1CpiBuilder}};

use crate::error::{NftMintError, RevokeMinerParticipationError};
use crate::state::{ProgramState, MinerInfo};

#[derive(Accounts)]
#[instruction(nft_id: u64)]
pub struct RevokeMinerParticipation <'info> {
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

    // ---- Collection Mint ----
     #[account(
        mut,
        seeds = [b"collection_mint"],
        bump = program_state.collection_mint_bump,
        mint::token_program = token_program
    )]
    pub collection_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        seeds = [
            b"metadata".as_ref(),
            metadata_program.key().as_ref(),
            collection_mint.key().as_ref(),
        ],
        bump,
        seeds::program = metadata_program.key()
    )]
/// CHECK:
    pub nft_collection_metadata: UncheckedAccount<'info>,

pub metadata_program: Program<'info, Metadata>,
    #[account(
        mut,
        seeds = [
            b"metadata".as_ref(),
            metadata_program.key().as_ref(),
            collection_mint.key().as_ref(),
            b"edition".as_ref(),
        ],
        bump,
        seeds::program = metadata_program.key()
      )]

    /// CHECK:
    pub collection_master_edition_account: UncheckedAccount<'info>,



// ---- Miner NFT Mint ----
  #[account( 
    mut, 
    seeds = ["nft_mint".as_bytes(), miner.key().as_ref(), nft_id.to_le_bytes().as_ref()], 
    mint::token_program = token_program,
    bump = miner_info.miner_nft_bump
    )]
  pub miner_nft_mint: InterfaceAccount<'info, Mint>,

  #[account(
        mut,
        associated_token::mint = miner_nft_mint,
        associated_token::authority = miner,
        associated_token::token_program = token_program
    )]

   pub miner_nft_token_account: InterfaceAccount<'info, TokenAccount>,

   #[account(
    mut,
    seeds = [
        b"metadata".as_ref(),
        metadata_program.key().as_ref(),
        miner_nft_mint.key().as_ref(),
        b"edition".as_ref(),
    ],  
    bump,
    seeds::program = metadata_program.key()
  )]

/// CHECK:
    pub miner_nft_master_edition_account: UncheckedAccount<'info>,

   #[account(
    mut,
    seeds = [
        b"metadata".as_ref(),
        metadata_program.key().as_ref(),
        miner_nft_mint.key().as_ref(),
    ],
    bump,
    seeds::program = metadata_program.key()
)]

/// CHECK:
   pub miner_nft_metadata: UncheckedAccount<'info>,


// ---- Miner Info ----
    #[account( 
    mut, 
    seeds = ["miner".as_bytes(), miner.key().as_ref()], 
    bump = miner_info.miner_bump,
    close = miner
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
        mut,
        associated_token::mint= bhrt_mint,
        associated_token::authority=miner,
        associated_token::token_program = token_program
    )]
    pub miner_bhrt: InterfaceAccount<'info, TokenAccount>,

    #[account(
        address= solana_program::sysvar::instructions::ID
    )]
    /// CHECK: This is the instructions sysvar
    pub instruction_sysvar: AccountInfo<'info>,

    // ---- Required Programs ----
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}



impl<'info> RevokeMinerParticipation <'info> {

pub fn revoke_miner_participation( &mut self, bump: &RevokeMinerParticipationBumps,amount: u64) -> Result<()> {

        require!(self.program_state.approved_miners.contains(&self.miner.key()), NftMintError::MinerNotApproved);

     require!(self.miner_info.mint_amount == amount, RevokeMinerParticipationError::InsufficientBhrAmount);
    
    
let state_seeds =&[
    b"program_state".as_ref(),
    &[self.program_state.program_state_bump]
];

let signer_seeds = &[&state_seeds[..]];

UnverifyCollectionV1CpiBuilder::new(&self.metadata_program.to_account_info())
.authority(&self.program_state.to_account_info())
.metadata(&self.miner_nft_metadata.to_account_info())
.collection_mint(&self.collection_mint.to_account_info())
.collection_metadata(Some(&self.nft_collection_metadata.to_account_info()))
.system_program(&self.system_program.to_account_info())
.sysvar_instructions(&self.instruction_sysvar.to_account_info())
.invoke_signed(signer_seeds)?;

// RevokeCollectionItemV1CpiBuilder::new(&self.metadata_program.to_account_info())
// .authority(&self.program_state.to_account_info())
// .payer(&self.miner.to_account_info())
// .token(Some(&self.miner_nft_token_account.to_account_info()))
// .metadata(&self.miner_nft_metadata.to_account_info())
// .master_edition(Some(&self.miner_nft_master_edition_account.to_account_info()))
// .mint(&self.miner_nft_mint.to_account_info())
// .system_program(&self.system_program.to_account_info())
// .spl_token_program(Some(&self.token_program.to_account_info()))
// .sysvar_instructions(&self.instruction_sysvar.to_account_info())
// .invoke_signed(signer_seeds)?;

BurnV1CpiBuilder::new(&self.metadata_program.to_account_info())
.authority(&self.program_state.to_account_info())
.token(&self.miner_nft_token_account.to_account_info())
.metadata(&self.miner_nft_metadata.to_account_info())
.master_edition(Some(&self.miner_nft_master_edition_account.to_account_info()))
.mint(&self.miner_nft_mint.to_account_info())
// .payer(&self.miner.to_account_info())

.system_program(&self.system_program.to_account_info())
.spl_token_program(&self.token_program.to_account_info())
// .spl_ata_program(&self.associated_token_program.to_account_info())
.sysvar_instructions(&self.instruction_sysvar.to_account_info())
.amount(1).invoke_signed(signer_seeds)?;
            


    



let cpi_program = self.token_program.to_account_info();
let cpi_accounts = Burn {
    mint: self.bhrt_mint.to_account_info(),
    from: self.miner_bhrt.to_account_info(),
    authority: self.miner.to_account_info(),
};

let signer_seeds: &[&[&[u8]]] = &[&[
    b"program_state",
    &[self.program_state.program_state_bump],
]];

let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
burn(cpi_ctx, amount)?;



self.miner_info.mint_amount -= amount;

        Ok(())
    }





}

