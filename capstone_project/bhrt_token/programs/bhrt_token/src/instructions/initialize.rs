use anchor_lang::prelude::*;
use anchor_spl::metadata::{
    Metadata,
};
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{ Mint, TokenAccount, TokenInterface},
};
use mpl_token_metadata::types::{ PrintSupply};
use mpl_token_metadata::instructions::{ MintV1CpiBuilder};
use mpl_token_metadata::instructions::CreateV1CpiBuilder;
use crate::state::{BhrtMetadata, ProgramState};
use mpl_token_metadata::types::TokenStandard;
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    // ---- ProgramState ----
    #[account(
        init,
        payer = authority,
        space = 8 + ProgramState::INIT_SPACE,
        seeds = [b"program_state"],
        bump
    )]
    pub program_state: Account<'info, ProgramState>,

    // ---- BHRT ----
    #[account(
    init,
    payer= authority,
    seeds=[b"BHRT"],
    bump,
    mint::decimals= 9,
    mint::authority = program_state,
    mint::token_program = token_program,
    extensions::metadata_pointer::authority = program_state,
    extensions::metadata_pointer::metadata_address = bhrt_metadata.key()
    )]
    pub bhrt_mint: InterfaceAccount<'info, Mint>,

    // #[account(
    //     init,
    //     payer= authority,
    //     associated_token::mint=bhrt_mint,
    //     associated_token::authority=program_state,
    //     associated_token::token_program = token_program
    // )]
    // pub bhrt_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = authority,
        seeds = [b"bhrt_metadata", program_state.key().as_ref()],
        space = 8 + BhrtMetadata::INIT_SPACE,
        bump
    )]
    pub bhrt_metadata: Account<'info, BhrtMetadata>,

    // ---- Collection NFT ----
    #[account(
        init,
        payer = authority,
        seeds = [b"collection_mint"],
        mint::decimals = 0,
        mint::authority = program_state,
        mint::freeze_authority = program_state,
        mint::token_program = token_program,
        bump
    )]
    pub collection_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = authority,
        associated_token::mint = collection_mint,
        associated_token::authority = program_state,
        associated_token::token_program = token_program
    )]
    pub collection_token_account: InterfaceAccount<'info, TokenAccount>,

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
    
    #[account(
        address= anchor_lang::solana_program::sysvar::instructions::ID
    )]
    /// CHECK: This is the instructions sysvar
        pub instruction_sysvar: AccountInfo<'info>,


    // --- Required Programs ---
    pub associated_token_program: Program<'info, AssociatedToken>,
    // pub rent: Sysvar<'info, Rent>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
    
}

impl<'info> Initialize<'info> {
    pub fn init_authority(&mut self, bump: &InitializeBumps) -> Result<()> {
        self.program_state.set_inner(ProgramState {
            nft_id_counter: 0,
            authority: self.authority.key(),
            approved_miners: Vec::new(),
            program_state_bump: bump.program_state,
            bhrt_mint_bump : bump.bhrt_mint,
            collection_mint_bump: bump.collection_mint,
            collection_metadata_bump : bump.nft_collection_metadata,
        });

        // let state_seeds = &[
        //     b"program_state".as_ref(),
        //     &[bump.program_state]
        // ];
        // let signer_seeds = &[&state_seeds[..]];

        let signers_seeds: &[&[&[u8]]] = &[&[
            b"program_state",
            &[bump.program_state],
        ]]; 

        // // Create metadata for the collection NFT
        // create_metadata_accounts_v3(
        //     CpiContext::new_with_signer( 
        //         self.metadata_program.to_account_info(),
        //         CreateMetadataAccountsV3 {
        //             metadata: self.nft_collection_metadata.to_account_info(),
        //             mint: self.collection_mint.to_account_info(),
        //             mint_authority: self.program_state.to_account_info(),
        //             payer: self.authority.to_account_info(),
        //             update_authority: self.program_state.to_account_info(),
        //             system_program: self.system_program.to_account_info(),
        //             rent: self.rent.to_account_info(),
        //         }, signer_seeds,
        //     ),
        //     DataV2 {
        //         name: "Miner Contract NFT Collection".to_string(),
        //         symbol: "MINERSNFT".to_string(),
        //         uri: "URL_TO_COLLECTION_JSON".to_string(),
        //         seller_fee_basis_points: 0,
        //         creators: None,
        //         collection: None,
        //         uses: None,
        //     },
        //     false,
        //     true,
        //     Some(mpl_token_metadata::types::CollectionDetails::V1 { size: 0 }),
            
        // )?;

        // // Create the master edition, officially making it a collection
        // create_master_edition_v3(
        //     CpiContext::new_with_signer( 
        //         self.metadata_program.to_account_info(),
        //         CreateMasterEditionV3 {
        //             edition: self.collection_master_edition_account.to_account_info(),
        //             mint: self.collection_mint.to_account_info(),
        //             update_authority: self.program_state.to_account_info(),
        //             mint_authority: self.program_state.to_account_info(),
        //             payer: self.authority.to_account_info(),
        //             metadata: self.nft_collection_metadata.to_account_info(),
        //             token_program: self.token_program.to_account_info(),
        //             system_program: self.system_program.to_account_info(),
        //             rent: self.rent.to_account_info(),
        //         }, signer_seeds,
        //     ),
        //     Some(0),
        // )?;

        CreateV1CpiBuilder::new(&self.metadata_program.to_account_info())
            .metadata(&self.nft_collection_metadata.to_account_info())
            .mint(&self.collection_mint.to_account_info(), false)
            .authority(&self.program_state.to_account_info())
            .payer(&self.authority.to_account_info())
            .update_authority(&self.program_state.to_account_info(), true)
            .master_edition(Some(&self.collection_master_edition_account.to_account_info()))
            .system_program(&self.system_program.to_account_info())
            .sysvar_instructions(&self.instruction_sysvar.to_account_info())
            .spl_token_program(Some(&self.token_program.to_account_info()))
            .token_standard(TokenStandard::NonFungible)
            .name("Miner Contract NFT Collection".to_string())
            .symbol("MINERSNFT".to_string())
            .uri(" https://gateway.irys.xyz/3ZUd1gbTBK81MnGjvTARBcyVUS3VzAiaq39DhJ9yyU9i".to_string())
            .seller_fee_basis_points(0)
            .primary_sale_happened(false)
            .is_mutable(true) 
            .print_supply(PrintSupply::Zero)
            // .collection_details(mpl_token_metadata::types::CollectionDetails::V1 { size: 0 })
            .invoke_signed(signers_seeds)?;

            msg!("Minting collection token");
           MintV1CpiBuilder::new(&self.metadata_program.to_account_info())
                .token(&self.collection_token_account.to_account_info())
                .token_owner(Some(&self.program_state.to_account_info()))
                .metadata(&self.nft_collection_metadata.to_account_info())
                // .token_record(Some(&self.collection_mint.to_account_info()))
                .master_edition(Some(&self.collection_master_edition_account.to_account_info()))
                .mint(&self.collection_mint.to_account_info())
                .payer(&self.authority.to_account_info())
                .authority(&self.program_state.to_account_info())
                .system_program(&self.system_program.to_account_info())
                .spl_token_program(&self.token_program.to_account_info())
                .spl_ata_program(&self.associated_token_program.to_account_info())
                .sysvar_instructions(&self.instruction_sysvar.to_account_info())
                .amount(1).invoke_signed(signers_seeds)?;
            
            

        let metadata_account = &mut self.bhrt_metadata;
        metadata_account.mint = self.bhrt_mint.key();
        metadata_account.collection = self.collection_mint.key();
        metadata_account.description =
            "Fungible Hashrate Token BHRT linked to Miner NFT Collection".to_string();
        metadata_account.symbol = "BHRT".to_string();

        Ok(())
    }
}
