use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::metadata::{
    create_master_edition_v3, create_metadata_accounts_v3, CreateMasterEditionV3,
    CreateMetadataAccountsV3, Metadata,
};
use anchor_spl::token::{mint_to, Mint, MintTo, Token, TokenAccount};
use mpl_token_metadata::types::{Collection, Creator, DataV2};

declare_id!("En3mGHz3rK9bMp7trVvNrUZrgfsQwd934td2CJcAjTCr");

#[program]
pub mod nft_mint_miner {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.program_info.authority = ctx.accounts.authority.key();
        ctx.accounts.program_info.nft_ids = 0;
        Ok(())
    }

        pub fn approve_miner(ctx: Context<ApproveMiner>, miner_to_add: Pubkey) -> Result<()> {
        let program_info = &mut ctx.accounts.program_info;

        require!(!program_info.miners.contains(&miner_to_add), NftMintError::MinerAlreadyApproved);
        program_info.miners.push(miner_to_add);
        
        Ok(())
    }


    pub fn create_single_nft( ctx: Context<CreateNFT>,name: String, id: u64, uri: String, price: f32) -> Result<()> {
         require!(ctx.accounts.program_info.miners.contains(&ctx.accounts.miner.key()), NftMintError::MinerNotApproved);

        ctx.accounts.program_info.nft_ids += 1;

        msg!("Creating seeds");
        let id_bytes = ctx.accounts.program_info.nft_ids.to_le_bytes();
        let seeds = &["mint_authority".as_bytes(), id_bytes.as_ref(), &[ctx.bumps.mint_authority]];
        let mut nft_name: String = "Bitcoin Standard Hashrate Token Agreement: ".to_string(); 
        nft_name.push_str(&name);

        let symbol = "BSHA".to_string(); 

        msg!("Run mint_to");

        mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    authority: ctx.accounts.mint_authority.to_account_info(),
                    to: ctx.accounts.token_account.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                },
                &[&seeds[..]],
            ),
            1, // 1 token
        )?;

        msg!("Run create metadata accounts v3");

        create_metadata_accounts_v3(
            CpiContext::new_with_signer(
                ctx.accounts.metadata_program.to_account_info(),
                CreateMetadataAccountsV3 {
                    payer: ctx.accounts.miner.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                    metadata: ctx.accounts.nft_metadata.to_account_info(),
                    mint_authority: ctx.accounts.mint_authority.to_account_info(),
                    update_authority: ctx.accounts.mint_authority.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
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
                ctx.accounts.metadata_program.to_account_info(),
                CreateMasterEditionV3 {
                    edition: ctx.accounts.master_edition_account.to_account_info(),
                    payer: ctx.accounts.miner.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                    metadata: ctx.accounts.nft_metadata.to_account_info(),
                    mint_authority: ctx.accounts.mint_authority.to_account_info(),
                    update_authority: ctx.accounts.mint_authority.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    token_program: ctx.accounts.token_program.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                },
                &[&seeds[..]],
            ),
            Some(1),
        )?;

        msg!("Minted NFT successfully");

        Ok(())
    }

}


#[derive(Accounts)]
// #[instruction(id: u64)]
pub struct CreateNFT<'info> {
 #[account(mut)]
    pub miner: Signer<'info>, // Renamed from 'payer' for clarity

    #[account(
        mut,
        seeds = [b"program_info"], 
        bump
    )]
    pub program_info: Account<'info, ProgramInfo>,

    // Improvement: The Mint Authority is now a PDA, not a user.
    /// CHECK: PDA, not a user account
    #[account(
        seeds = ["mint_authority".as_bytes(), program_info.nft_ids.to_le_bytes().as_ref()],
        bump
    )]
    pub mint_authority: UncheckedAccount<'info>,

    // The NFT mint PDA. Its seeds now use the on-chain counter.
    #[account(
        init,
        payer = miner,
        mint::decimals = 0,
        mint::authority = miner,
        mint::freeze_authority = miner,
        seeds = ["nft_mint".as_bytes(), program_info.nft_ids.to_le_bytes().as_ref()],
        bump
    )]
    pub mint: Account<'info, Mint>,
  #[account(
        init_if_needed,
        payer = miner,
        associated_token::mint = mint,
        associated_token::authority = miner,
    )]
    pub token_account: Account<'info, TokenAccount>,
   pub associated_token_program: Program<'info, AssociatedToken>,
   pub rent: Sysvar<'info, Rent>,
   pub system_program: Program<'info, System>,
   pub token_program: Program<'info, Token>,
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


#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        space = 8 + ProgramInfo::INIT_SPACE,
        seeds = [b"program_info"],
        bump
    )]
    pub program_info: Account<'info, ProgramInfo>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ApproveMiner<'info> {
  pub authority: Signer<'info>,
       #[account(
        mut,
        seeds = [b"program_info"],
        bump,
        has_one = authority 
    )]
    pub program_info: Account<'info, ProgramInfo>,
    pub system_program: Program<'info, System>,
}


#[account]
#[derive(InitSpace)]
pub struct ProgramInfo{
    pub nft_ids: u64,
    pub authority: Pubkey,
    #[max_len(100)] 
    pub miners: Vec<Pubkey>,
    pub bump: u8
}

// #[account]
// #[derive(InitSpace)]
// pub struct ApprovedMiners {
//     #[max_len(100)] // Can store up to 100 miner pubkeys
//     pub miners: Vec<Pubkey>,
// }

#[error_code]
pub enum NftMintError {
    #[msg("The signer is not an approved miner.")]
    MinerNotApproved,
    #[msg("This miner has already been approved.")]
    MinerAlreadyApproved,
}