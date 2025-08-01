use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, 
    token_interface::{ Mint, TokenAccount, TransferChecked, TokenInterface, transfer_checked},
    metadata::{
    create_master_edition_v3, create_metadata_accounts_v3, CreateMasterEditionV3,
    CreateMetadataAccountsV3, Metadata,
},
token_2022::{MintTo, mint_to}
};

use mpl_token_metadata::types::{Collection, Creator, DataV2};

use crate::error::NftMintError;
use crate::state::NFTProgramInfo;

#[derive(Accounts)]
pub struct CreateNFT<'info> {
  #[account(mut)]
  pub miner: Signer<'info>,

  // CHECK: This is safe account 
  pub authority: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [b"nft_program_info"],
        bump = nft_program_info.bump,
        has_one = authority
    )]
    pub nft_program_info: Account<'info, NFTProgramInfo>,

  #[account( 
    init,
    payer = miner, 
    mint::decimals = 0,
    mint::authority = nft_program_info,
    mint::freeze_authority = nft_program_info,
    mint::token_program = token_program,
    seeds = ["mint".as_bytes(), miner.key().as_ref(), nft_program_info.nft_id_counter.to_le_bytes().as_ref()], 
    bump,
    )]
  pub mint: InterfaceAccount<'info, Mint>,

  #[account(
        init_if_needed,
        payer = miner,
        associated_token::mint = mint,
        associated_token::authority = miner,
        associated_token::token_program = token_program
    )]
   pub token_account: InterfaceAccount<'info, TokenAccount>,

   pub associated_token_program: Program<'info, AssociatedToken>,
   pub rent: Sysvar<'info, Rent>,
   pub system_program: Program<'info, System>,
   pub token_program: Interface<'info, TokenInterface>,
   pub metadata_program: Program<'info, Metadata>,
   #[account(
        mut,
        seeds = [
            b"metadata".as_ref(),
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
            b"edition".as_ref(),
        ],
        bump,
        seeds::program = metadata_program.key()
      )]

   /// CHECK:
   pub master_edition_account: UncheckedAccount<'info>,
   #[account(
        mut,
        seeds = [
            b"metadata".as_ref(),
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
        ],
        bump,
        seeds::program = metadata_program.key()
    )]
   /// CHECK:
   pub nft_metadata: UncheckedAccount<'info>,
}

#[event]
pub struct NftMinted {
    pub nft_id: u64,
    pub mint: Pubkey,
    pub owner: Pubkey,
}


impl<'info> CreateNFT<'info> {


     pub fn create_nft( &mut self, bump: &CreateNFTBumps,name: String, uri: String,) -> Result<()> {

        require!(self.nft_program_info.miners.contains(&self.miner.key()), NftMintError::MinerNotApproved);

        self.nft_program_info.nft_id_counter += 1;

        msg!("Creating seeds");
        // let id_bytes = self.nft_program_info.nft_id_counter.to_le_bytes();
        // let binding = self.miner.key();
        // let seeds = &["mint".as_bytes(), binding.as_ref(), id_bytes.as_ref(), &[bump.mint]];
        let seeds = &["nft_program_info".as_bytes(),&[self.nft_program_info.bump]];
    
        let mut nft_name: String = "Bitcoin Standard Hashrate Token Agreement: ".to_string(); 
        nft_name.push_str(&name);

        let symbol = "BHRA".to_string(); 

        msg!("Run mint_to");

        mint_to(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                MintTo {
                    authority: self.nft_program_info.to_account_info(),
                    to: self.token_account.to_account_info(),
                    mint: self.mint.to_account_info(),
                },
                &[&seeds[..]],
            ),
            1, // 1 token
        )?;

        msg!("Run create metadata accounts v3");

        create_metadata_accounts_v3(
            CpiContext::new_with_signer(
                self.metadata_program.to_account_info(),
                CreateMetadataAccountsV3 {
                    payer: self.miner.to_account_info(),
                    mint: self.mint.to_account_info(),
                    metadata: self.nft_metadata.to_account_info(),
                    mint_authority: self.nft_program_info.to_account_info(),
                    update_authority: self.nft_program_info.to_account_info(),
                    system_program: self.system_program.to_account_info(),
                    rent: self.rent.to_account_info(),
                },
                &[&seeds[..]],
            ),
            DataV2 {
                name:nft_name,
                symbol,
                uri,
                seller_fee_basis_points: 0,
                creators: None,
                collection: None,
                uses: None,
            },
            true,
            true,
            None,
        )?;

        msg!("Run create master edition v3");

        create_master_edition_v3(
            CpiContext::new_with_signer(
                self.metadata_program.to_account_info(),
                CreateMasterEditionV3 {
                    edition: self.master_edition_account.to_account_info(),
                    payer: self.miner.to_account_info(),
                    mint: self.mint.to_account_info(),
                    metadata: self.nft_metadata.to_account_info(),
                    mint_authority: self.nft_program_info.to_account_info(),
                    update_authority: self.nft_program_info.to_account_info(),
                    system_program: self.system_program.to_account_info(),
                    token_program: self.token_program.to_account_info(),
                    rent: self.rent.to_account_info(),
                },
                &[&seeds[..]],
            ),
            Some(1),
        )?;

        msg!("Minted NFT successfully");

        emit!(NftMinted {
    nft_id: self.nft_program_info.nft_id_counter,
    mint: self.mint.key(),
    owner: self.miner.key(),
});


        Ok(())
    }


    



}

