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

  // CHECK: This is safe account 
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

    pub nft_collection_metadata: UncheckedAccount<'info>,

pub metadata_program: Program<'info, Metadata>,
    #[account(
        mut,
        seeds = [
            b"collection_metadata".as_ref(),
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

   pub miner_nft_metadata: UncheckedAccount<'info>,


//    pub associated_token_program: Program<'info, AssociatedToken>,
//    pub rent: Sysvar<'info, Rent>,
//    pub system_program: Program<'info, System>,
//    pub token_program: Interface<'info, TokenInterface>,
   
//    #[account(
//         mut,
//         seeds = [
//             b"metadata".as_ref(),
//             metadata_program.key().as_ref(),
//             mint.key().as_ref(),
//             b"edition".as_ref(),
//         ],
//         bump,
//         seeds::program = metadata_program.key()
//       )]

//    /// CHECK:
//    pub master_edition_account: UncheckedAccount<'info>,
//    #[account(
//         mut,
//         seeds = [
//             b"metadata".as_ref(),
//             metadata_program.key().as_ref(),
//             mint.key().as_ref(),
//         ],
//         bump,
//         seeds::program = metadata_program.key()
//     )]
//    /// CHECK:
//    pub miner_nft_metadata: UncheckedAccount<'info>,



// ---- Miner Info ----
    #[account( 
    init,
    payer = miner, 
    seeds = ["miner".as_bytes(), miner.key().as_ref()], 
    space = 8 + MinerInfo::INIT_SPACE,
    bump,
    )]
    pub miner_info: Account<'info, MinerInfo>,


// ---- BHRT Mint ----
        #[account(
    mut,
    seeds=[b"BHRT"],
    bump = program_state.bhrt_mint_bump,
)]
    pub bhrt_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
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


// #[derive(Accounts)]
// #[instruction(seed: u64)]
// pub struct AMMInitialization<'info> {
//     #[account(mut)]
//     pub initializer: Signer<'info>,
//     pub mint_x: Account<'info, Mint>,
//     pub mint_y: Account<'info, Mint>,
//     #[account(
//     init,
//     payer= initializer,
//     seeds=[b"lp", config.key().as_ref() ],
//     bump,
//     mint::decimals= 6,
//     mint::authority=config,
// )]
//     pub mint_lp: Account<'info, Mint>,

//     #[account(
//     init,
//     payer= initializer,
//     seeds=[b"config",  seed.to_le_bytes().as_ref()],
//     space= 8+ Config::INIT_SPACE,
//     bump
// )]
//     pub config: Account<'info, Config>,

//     #[account(
//         init,
//         payer=initializer,
//         associated_token::mint=mint_x,
//         associated_token::authority=config
//     )]
//     pub vault_x: Account<'info, TokenAccount>,

//     #[account(
//         init,
//         payer=initializer,
//         associated_token::mint=mint_x,
//         associated_token::authority=config
//     )]
//     pub vault_y: Account<'info, TokenAccount>,
//     pub token_program: Program<'info, Token>,
//     pub associated_token_program: Program<'info, AssociatedToken>,
//     pub system_program: Program<'info, System>,
// }

// #[derive(Accounts)]
// // #[instruction(seed: u64)]
// pub struct depositAMM<'info> {
//     #[account(mut)]
//     pub user: Signer<'info>,
//     pub mint_x: Account<'info, Mint>,
//     pub mint_y: Account<'info, Mint>,
//  #[account(
//     // mut,
//     has_one = mint_x,
//     has_one = mint_y,
//     seeds=[b"config",  config.seed.to_le_bytes().as_ref()],
//     bump= config.config_bump
// )]
//     pub config: Account<'info, Config>,

//      #[account(
//         mut,
//         associated_token::mint=mint_x,
//         associated_token::authority=config
//     )]
//     pub vault_x: Account<'info, TokenAccount>,

//     #[account(
//         mut,
//         associated_token::mint=mint_x,
//         associated_token::authority=config
//     )]
//     pub vault_y: Account<'info, TokenAccount>,

//     #[account(
//     mut,
//     seeds=[b"lp", config.key().as_ref() ],
//     bump =  config.lp_bump,
   
// )]
//     pub mint_lp: Account<'info, Mint>,
//  #[account(
//         mut,
//         associated_token::mint=mint_x,
//         associated_token::authority=user
//     )]
//     pub user_x: Account<'info, TokenAccount>,

//     #[account(
//         mut,
//         associated_token::mint=mint_x,
//         associated_token::authority=user
//     )]
//     pub user_y: Account<'info, TokenAccount>,

//     #[account(
//         init_if_needed,
//         payer= user,
//         associated_token::mint=mint_lp,
//         associated_token::authority=user
//     )]
//     pub user_lp: Account<'info, TokenAccount>,
//     pub token_program: Program<'info, Token>,
//     pub associated_token_program: Program<'info, AssociatedToken>,
//     pub system_program: Program<'info, System>,
// }




// #[derive(Accounts)]
// // #[instruction(seed: u64)]
// pub struct AMMSwap<'info> {
//     #[account(mut)]
//     pub user: Signer<'info>,
//     pub mint_x: Account<'info, Mint>,
//     pub mint_y: Account<'info, Mint>,
//     #[account(
//     // mut,
//     has_one = mint_x,
//     has_one = mint_y,
//     seeds=[b"config",  config.seed.to_le_bytes().as_ref()],
//     bump= config.config_bump
// )]
//     pub config: Account<'info, Config>,

//     #[account(
//         mut,
//         associated_token::mint=mint_x,
//         associated_token::authority=config
//     )]
//     pub vault_x: Account<'info, TokenAccount>,

//     #[account(
//         mut,
//         associated_token::mint=mint_x,
//         associated_token::authority=config
//     )]
//     pub vault_y: Account<'info, TokenAccount>,

//      #[account(
//         init_if_needed,
//         payer = user,
//         associated_token::mint=mint_x,
//         associated_token::authority=user
//     )]
//     pub user_x: Account<'info, TokenAccount>,

//     #[account(
//         init_if_needed,
//         payer = user,
//         associated_token::mint=mint_x,
//         associated_token::authority=user
//     )]
//     pub user_y: Account<'info, TokenAccount>,
//      pub token_program: Program<'info, Token>,
//     pub associated_token_program: Program<'info, AssociatedToken>,
//     pub system_program: Program<'info, System>,

// }


// #[derive(Accounts)]
// pub struct AMMWithdraw<'info> {
//     #[account(mut)]
//     pub user: Signer<'info>,

//     #[account(mint::token_program = token_program)]
//     pub mint_x: Account<'info, Mint>,

//     #[account(mint::token_program = token_program)]
//     pub mint_y: Account<'info, Mint>,

//     #[account(
//         seeds = [b"config"],
//         bump = config.config_bump,
//         has_one = mint_x,
//         has_one = mint_y,
//     )]
//     pub config: Account<'info, Config>,

//     #[account(
//         mut,
//         seeds = [b"lp", config.key().as_ref()],
//         bump = config.lp_bump
//     )]
//     pub mint_lp: Account<'info, Mint>,

//     #[account(
//         mut,
//         associated_token::mint = mint_x,
//         associated_token::authority = config,
//         associated_token::token_program = token_program,
//     )]
//     pub vault_x: Account<'info, TokenAccount>,

//     #[account(
//         mut,
//         associated_token::mint = mint_y,
//         associated_token::authority = config,
//         associated_token::token_program = token_program,
//     )]
//     pub vault_y: Account<'info, TokenAccount>,

//     #[account(
//         mut,
//         associated_token::mint = mint_x,
//         associated_token::authority = user,
//         associated_token::token_program = token_program
//     )]
//     pub user_token_account_x: Account<'info, TokenAccount>,

//     #[account(
//         mut,
//         associated_token::mint = mint_y,
//         associated_token::authority = user,
//         associated_token::token_program = token_program
//     )]
//     pub user_token_account_y: Account<'info, TokenAccount>,

//     #[account(
//         mut,
//         associated_token::mint = mint_lp,
//         associated_token::authority = user,
//         associated_token::token_program = token_program
//     )]
//     pub user_token_account_lp: Account<'info, TokenAccount>,

//     pub token_program: Program<'info, Token>,
//     pub associated_token_program: Program<'info, AssociatedToken>,
//     pub system_program: Program<'info, System>,
// }


// #[derive(Accounts)]
// pub struct MinerRevokeMinngPower<'info> {
//   pub miner: Signer<'info>,

//     // CHECK: This is safe account 
//   pub authority: UncheckedAccount<'info>,

//     #[account(
//         mut,
//         seeds = [b"nft_program_info"],
//         bump = nft_program_info.bump,
//         has_one = authority
//     )]
//     pub nft_program_info: Account<'info, ProgramState>,

//   #[account( 
//     mut,
//     mint::authority = nft_program_info,
//     mint::freeze_authority = nft_program_info,
//     mint::token_program = token_program,
//     seeds = ["mint".as_bytes(), miner.key().as_ref(), nft_program_info.nft_id_counter.to_le_bytes().as_ref()], 
//     bump,
//     )]
//   pub mint: InterfaceAccount<'info, Mint>,


//   #[account(
//         init_if_needed,
//         payer = miner,
//         associated_token::mint = mint,
//         associated_token::authority = miner,
//         associated_token::token_program = token_program
//     )]
//    pub nft_account: InterfaceAccount<'info, TokenAccount>,


//    pub associated_token_program: Program<'info, AssociatedToken>,
//    pub rent: Sysvar<'info, Rent>,
//    pub system_program: Program<'info, System>,
//    pub token_program: Interface<'info, TokenInterface>,
//    pub metadata_program: Program<'info, Metadata>,

//     #[account(
//         mut,
//         seeds = [
//             b"metadata".as_ref(),
//             metadata_program.key().as_ref(),
//             mint.key().as_ref(),
//             b"edition".as_ref(),
//         ],
//         bump,
//         seeds::program = metadata_program.key()
//       )]

//    /// CHECK:
//    pub master_edition_account: UncheckedAccount<'info>,
//    #[account(
//         mut,
//         seeds = [
//             b"metadata".as_ref(),
//             metadata_program.key().as_ref(),
//             mint.key().as_ref(),
//         ],
//         bump,
//         seeds::program = metadata_program.key()
//     )]
//    /// CHECK:
//    pub nft_metadata: UncheckedAccount<'info>,

//  #[account(
//         mut,
//         seeds = [b"miner", miners.seed.to_le_bytes().as_ref()],
//         bump = miners.bump,
//     )]
//     pub miners: Account<'info, Miner>,


//      #[account(
//     mut,
//     seeds=[b"BHRT", miners.key().as_ref() ],
//     bump,
// )]
//     pub mint_bhrt: Account<'info, Mint>,
//  #[account(
//         mut,
//         associated_token::mint = mint_bhrt,
//         associated_token::authority = miner,
//         associated_token::token_program = token_program
//     )]
//     pub miner_token_account_bhrt: Account<'info, TokenAccount>,
// }


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

        msg!("Creating seeds");
        // let id_bytes = self.program_state.nft_id_counter.to_le_bytes();
        // let binding = self.miner.key();
        // let seeds = &["mint".as_bytes(), binding.as_ref(), id_bytes.as_ref(), &[bump.mint]];
        // let miner_nft_seeds = &["nft_mint".as_bytes(),self.miner.key().as_ref(), nft_id.to_le_bytes().as_ref(), &[self.program_state.bhrt_mint_bump]];
        let miner_nft_seeds: &[&[&[u8]]; 1] = &[&[
            b"nft_mint", 
            self.miner.key.as_ref(), 
            &nft_id.to_le_bytes()[..],
            &[self.program_state.bhrt_mint_bump]
        ]];
        
        let mut nft_name: String = "Bitcoin Standard Hashrate Token Agreement: ".to_string(); 
        nft_name.push_str(&name);

        let symbol: String = "BHRT".to_string(); 

        msg!("Run mint_to");

        mint_to(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                MintTo {
                    authority: self.authority.to_account_info(),
                    to: self.miner_nft_token_account.to_account_info(),
                    mint: self.miner_nft_mint.to_account_info(),
                },
                // &[&miner_nft_seeds[..]],
                miner_nft_seeds
            ),
            1, // 1 token
        )?;

        msg!("Run create metadata accounts v3");

        let collection_details = Collection {
            verified: false, // Start as unverified
            key: self.collection_mint.key(),
        };

        create_metadata_accounts_v3(
            CpiContext::new_with_signer(
                self.metadata_program.to_account_info(),
                CreateMetadataAccountsV3 {
                    payer: self.miner.to_account_info(),
                    mint: self.miner_nft_mint.to_account_info(),
                    metadata: self.miner_nft_metadata.to_account_info(),
                    mint_authority: self.authority.to_account_info(),
                    update_authority: self.authority.to_account_info(),
                    system_program: self.system_program.to_account_info(),
                    rent: self.rent.to_account_info(),
                },
                // &[&miner_nft_seeds[..]],
                miner_nft_seeds,
            ),
            DataV2 {
                name:"Bitcoin Hashrate Miner Contract NFT".to_string(), 
                symbol: "MINENFT".to_string(),
                uri,
                seller_fee_basis_points: 0,
                creators: None,
                collection: Some(collection_details),
                uses: None,
            },
            true,
            true,
            None,
        )?;

        // msg!("Run create master edition v3");

        // create_master_edition_v3(
        //     CpiContext::new_with_signer(
        //         self.metadata_program.to_account_info(),
        //         CreateMasterEditionV3 {
        //             edition: self.master_edition_account.to_account_info(),
        //             payer: self.miner.to_account_info(),
        //             mint: self.mint.to_account_info(),
        //             metadata: self.nft_metadata.to_account_info(),
        //             mint_authority: self.program_state.to_account_info(),
        //             update_authority: self.program_state.to_account_info(),
        //             system_program: self.system_program.to_account_info(),
        //             token_program: self.token_program.to_account_info(),
        //             rent: self.rent.to_account_info(),
        //         },
        //         &[&seeds[..]],
        //     ),
        //     Some(1),
        // )?;

        verify_sized_collection_item(CpiContext::new(
            self.metadata_program.to_account_info(),
            VerifySizedCollectionItem {
                payer: self.authority.to_account_info(),
                metadata: self.miner_nft_metadata.to_account_info(),
                collection_mint: self.collection_mint.to_account_info(),
                collection_metadata: self.nft_collection_metadata.to_account_info(),
                collection_master_edition: self.collection_master_edition_account.to_account_info(),
                collection_authority: self.authority.to_account_info(),
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
    collection_authority: self.authority.key(),
});



mint_to(
    CpiContext::new(
        self.token_program.to_account_info(),
        MintTo {
            mint: self.bhrt_mint.to_account_info(),
            to: self.miner_bhrt.to_account_info(),
            authority: self.authority.to_account_info(),
        },
    ),
    mining_power * 10, // Convert Th/s to tokens: 0.1 Th/s = 1 token, so 1 Th/s = 10 tokens
)?;


        Ok(())
    }


    



}

