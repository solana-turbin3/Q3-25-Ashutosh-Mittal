use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        token_metadata_initialize, transfer_checked, Mint, TokenAccount, TokenInterface,
        TokenMetadataInitialize, TransferChecked,
    },
};

use crate::state::StablecoinConfig;

#[derive(Accounts)]
pub struct InitializeVault<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    pub bhrt_collateral_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        seeds = [b"stablecoin_config", bhrt_collateral_mint.key().as_ref()],
        bump
    )]
    pub stablecoin_config: Account<'info, StablecoinConfig>,

    #[account(
        init,
        payer= admin,
        seeds=[b"BHRT"],
        bump,
        mint::decimals= 6,
        mint::authority = stablecoin_config,
        mint::freeze_authority = stablecoin_config,
        mint::token_program = token_program,
        extensions::metadata_pointer::authority = stablecoin_config,
        extensions::metadata_pointer::metadata_address = stabelcoin_mint
    )]
    pub stabelcoin_mint: InterfaceAccount<'info, Mint>,

    #[account(
            init,
            payer = admin,
            associated_token::mint = stabelcoin_mint,
            associated_token::authority = stablecoin_config,
            associated_token::token_program = token_program,
        )]
    pub stablecoin_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
            init,
            payer = admin,
            seeds = [b"bhrt_collateral_vault", stablecoin_config.key().as_ref(), stabelcoin_mint.key().as_ref()],
            token::mint = bhrt_collateral_mint,
            token::authority = stablecoin_config,
            bump
        )]
    bhrt_collateral_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeVault<'info> {
    pub fn initialize_config_and_vault( &mut self, bump: InitializeVaultBumps, uri: String) -> Result<()> {
        self.stablecoin_config.set_inner(StablecoinConfig {
            authority: self.admin.key(),
            mint: self.bhrt_collateral_mint.key(),
            name: "Hashrate Stablecoin Token".to_string(),
            symbol: "HST".to_string(),
            bhrt_collateral_mint: self.bhrt_collateral_mint.key(),
            bhrt_collateral_vault: self.bhrt_collateral_vault.key(),
            total_bhrt_collateral_staked: 0,
            number_of_investors: 0,
            stablecoin_mint: self.stabelcoin_mint.key(),
            total_stablecoin_minted: 0,
            stablecoin_config_bump: bump.stablecoin_config,
            stablecoin_mint_bump: bump.stabelcoin_mint,
            bhrt_collateral_vault_bump: bump.bhrt_collateral_vault,
        });

        let cpi_accounts = TokenMetadataInitialize {
            program_id: self.token_program.to_account_info(),
            mint: self.stabelcoin_mint.to_account_info(),
            metadata: self.stabelcoin_mint.to_account_info(), // **metadata stored in the mint** (metadata_pointer -> mint)
            mint_authority: self.stablecoin_config.to_account_info(),
            update_authority: self.stablecoin_config.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);

        token_metadata_initialize(
            cpi_ctx,
            "Hashrate Stablecoin Token".to_string(),
            "HST".to_string(),
            uri,
        )?;

        // 2) Ensure mint is rent-exempt for its *new* size (metadata init may realloc)
        //    helper (below) - pass payer so it can top-up lamports if needed.
        // update_account_lamports_to_minimum_balance(
        //     self.stabelcoin_mint.to_account_info(),
        //     self.admin.to_account_info(),
        //     self.system_program.to_account_info(),
        // )?;

        Ok(())
    }
}
