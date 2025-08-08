use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, metadata::{
    create_master_edition_v3, create_metadata_accounts_v3, verify_sized_collection_item, CreateMasterEditionV3, CreateMetadataAccountsV3, Metadata, VerifySizedCollectionItem
}, token_2022::{mint_to, MintTo}, token_interface::{ transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked}
};

use mpl_token_metadata::types::{DataV2, Collection};

use crate::{error::NftMintError};
use crate::state::{ProgramState, MinerInfo, AmmConfig};

#[derive(Accounts)]
#[instruction(nft_id: u64)]
pub struct OnboardMiner<'info> {
  #[account(mut)]
  pub miner: Signer<'info>,

  /// CHECK: This is safe account 
    pub authority: UncheckedAccount<'info>,


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
        bump = program_state.collection_mint_bump
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
    init,
    payer = miner, 
    seeds = ["nft_mint".as_bytes(), miner.key().as_ref(), nft_id.to_le_bytes().as_ref()], 
    mint::decimals = 0,
    mint::authority = program_state,
    mint::freeze_authority = program_state,
    mint::token_program = token_program,
    bump,
    )]
  pub miner_nft_mint: InterfaceAccount<'info, Mint>,

  #[account(
        init_if_needed,
        payer = miner,
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
    ],
    bump,
    seeds::program = metadata_program.key()
)]

/// CHECK:
   pub miner_nft_metadata: UncheckedAccount<'info>,


// ---- Miner Info ----
    #[account( 
    init,
    payer = miner, 
    seeds = ["miner".as_bytes(), miner.key().as_ref()], 
    space = 8 + MinerInfo::INIT_SPACE,
    bump,
    )]
    pub miner_info: Box<Account<'info, MinerInfo>>,


// ---- BHRT Mint ----
        #[account(
    mut,
    seeds=[b"BHRT"],
    bump = program_state.bhrt_mint_bump,
)]
    pub bhrt_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer= miner,
        associated_token::mint= bhrt_mint,
        associated_token::authority=miner
    )]
    pub miner_bhrt: InterfaceAccount<'info, TokenAccount>,


    // ---- Required Programs ----
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}




#[event]
pub struct NftMinted {
    pub nft_id: u64,
    pub mint: Pubkey,
    pub owner: Pubkey,
    pub collection_mint: Pubkey,
    pub collection_metadata: Pubkey,
    pub collection_master_edition: Pubkey,
    pub collection_authority: Pubkey,
}


impl<'info> OnboardMiner<'info> {


     pub fn onboard_miner( &mut self, bump: &OnboardMinerBumps,name: String, uri: String,nft_id: u64, mining_power: u64) -> Result<()> {

        require!(self.program_state.approved_miners.contains(&self.miner.key()), NftMintError::MinerNotApproved);

        self.program_state.nft_id_counter += 1;

        msg!("Creating miner nft mint seeds");
        // let id_bytes = self.program_state.nft_id_counter.to_le_bytes();
        // let binding = self.miner.key();
        // let seeds = &["mint".as_bytes(), binding.as_ref(), id_bytes.as_ref(), &[bump.mint]];
        // let miner_nft_seeds = &["nft_mint".as_bytes(),self.miner.key().as_ref(), nft_id.to_le_bytes().as_ref(), &[self.program_state.bhrt_mint_bump]];
        // let miner_nft_seeds: &[&[&[u8]]; 1] = &[&[
        //     b"nft_mint", 
        //     self.miner.key.as_ref(), 
        //     &nft_id.to_le_bytes()[..],
        //     &[self.program_state.bhrt_mint_bump]
        // ]];

    
let state_seeds =&[
    &b"program_state"[..],
    &[self.program_state.program_state_bump]
];

let signer_seeds = &[&state_seeds[..]];
        
    
        msg!("Run mint_to for nft minting");

        mint_to(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                MintTo {
                    authority: self.program_state.to_account_info(),
                    to: self.miner_nft_token_account.to_account_info(),
                    mint: self.miner_nft_mint.to_account_info(),
                },
                // &[&miner_nft_seeds[..]],
                signer_seeds
            ),
            1, // 1 token
        )?;

        msg!("Run create metadata accounts v3");

        let collection_details = Collection {
            verified: false, 
            key: self.collection_mint.key(),
        };

        create_metadata_accounts_v3(
            CpiContext::new_with_signer(
                self.metadata_program.to_account_info(),
                CreateMetadataAccountsV3 {
                    payer: self.miner.to_account_info(),
                    mint: self.miner_nft_mint.to_account_info(),
                    metadata: self.miner_nft_metadata.to_account_info(),
                    mint_authority: self.program_state.to_account_info(),
                    update_authority: self.program_state.to_account_info(),
                    system_program: self.system_program.to_account_info(),
                    rent: self.rent.to_account_info(),
                },
                // &[&miner_nft_seeds[..]],
                signer_seeds,
            ),
            DataV2 {
                name:"Bitcoin Hashrate Miner Contract NFT".to_string(), 
                symbol: "MINENFT".to_string(),
                uri: uri.clone(),
                seller_fee_basis_points: 0,
                creators: None,
                collection: Some(collection_details),
                uses: None,
            },
            true,
            true,
            None,
        )?;

        verify_sized_collection_item(CpiContext::new(
            self.metadata_program.to_account_info(),
            VerifySizedCollectionItem {
                payer: self.miner.to_account_info(),
                metadata: self.miner_nft_metadata.to_account_info(),
                collection_mint: self.collection_mint.to_account_info(),
                collection_metadata: self.nft_collection_metadata.to_account_info(),
                collection_master_edition: self.collection_master_edition_account.to_account_info(),
                collection_authority: self.program_state.to_account_info(),
            },
        ), None)?;


        msg!("Minted NFT successfully");

        emit!(NftMinted {
    nft_id: nft_id,
    mint: self.miner_nft_mint.key(),
    owner: self.miner.key(),
    collection_mint: self.collection_mint.key(),
    collection_metadata: self.nft_collection_metadata.key(),
    collection_master_edition: self.collection_master_edition_account.key(),
    collection_authority: self.program_state.key(),
});



mint_to(
    CpiContext::new(
        self.token_program.to_account_info(),
        MintTo {
            mint: self.bhrt_mint.to_account_info(),
            to: self.miner_bhrt.to_account_info(),
            authority: self.program_state.to_account_info(),
        },
    ),
    mining_power * 10, 
)?;

self.miner_info.set_inner(MinerInfo {
    hashrate_power: mining_power,
    legal_document_uri: uri,
    hashrate_token_mint: self.bhrt_mint.key(),
    mint_amount: mining_power * 10,
    miner_bump: bump.miner_info,
    miner_nft_bump: bump.miner_nft_mint,
});

        Ok(())
    }





}

