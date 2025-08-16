use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::Token, // legacy SPL Token for BHRT only
    token_interface::{
        token_metadata_initialize, Mint, TokenAccount, TokenInterface, TokenMetadataInitialize,
    },
};

use crate::{state::StablecoinConfig, PriceFeed};
// use crate::top_up_to_rent_exempt;

#[derive(Accounts)]
pub struct InitializeVault<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    // Legacy SPL Token mint for BHRT (if BHRT is legacy)
    pub bhrt_collateral_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = admin,
        space = 8 + StablecoinConfig::INIT_SPACE,
        seeds = [b"stablecoin_config", bhrt_collateral_mint.key().as_ref()],
        bump
    )]
    pub stablecoin_config: Account<'info, StablecoinConfig>,

    // HST mint on Token-2022 with metadata pointer to itself
    #[account(
        init,
        payer = admin,
        seeds = [b"HST"],
        bump,
        mint::decimals = 6,
        mint::authority = stablecoin_config,
        mint::freeze_authority = stablecoin_config,
        mint::token_program = token_program,
        extensions::metadata_pointer::authority = stablecoin_config,
        extensions::metadata_pointer::metadata_address = stablecoin_mint,
    )]
    pub stablecoin_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = admin,
        associated_token::mint = stablecoin_mint,
        associated_token::authority = stablecoin_config,
        associated_token::token_program = token_program,
    )]
    pub stablecoin_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = admin,
        // seeds = [b"bhrt_collateral_vault", bhrt_collateral_mint.key().as_ref()],
        // bump,
        // token::mint = bhrt_collateral_mint,
        // token::authority = stablecoin_config,
        // token::token_program = token_program,
        associated_token::mint = bhrt_collateral_mint,
        associated_token::authority = stablecoin_config,
        associated_token::token_program = token_program,
    )]
    pub bhrt_collateral_vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = admin,
        seeds = [b"bhrt_price_oracle"],
        space = 8 + PriceFeed::INIT_SPACE,
        bump
    )]
    pub bhrt_price_oracle: Account<'info, PriceFeed>,

    // Programs
    pub token_program: Interface<'info, TokenInterface>,
    // pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeVault<'info> {


    pub fn top_up_to_rent_exempt(
        target: AccountInfo<'info>,
        payer: AccountInfo<'info>,
        system_program: AccountInfo<'info>,
    ) -> Result<()> {
        let curr = target.lamports();
        let len = target.data_len();
        let needed = Rent::get()?.minimum_balance(len);
        if curr < needed {
            let diff = needed - curr;
            anchor_lang::solana_program::program::invoke(
                &anchor_lang::solana_program::system_instruction::transfer(
                    payer.key,
                    target.key,
                    diff,
                ),
                &[payer, target, system_program],
            )?;
        }
        Ok(())
    }


    pub fn initialize_config_and_vault(
        &mut self,
        bumps: InitializeVaultBumps,
        uri: String,
    ) -> Result<()> {
        self.stablecoin_config.set_inner(StablecoinConfig {
            authority: self.admin.key(),
            mint: self.bhrt_collateral_mint.key(),
            name: "Hashrate Stablecoin Token".to_string(),
            symbol: "HST".to_string(),
            bhrt_collateral_mint: self.bhrt_collateral_mint.key(),
            bhrt_collateral_vault: self.bhrt_collateral_vault.key(),
            total_bhrt_collateral_staked: 0,
            number_of_investors: 0,
            stablecoin_mint: self.stablecoin_mint.key(),
            total_stablecoin_minted: 0,
            stablecoin_config_bump: bumps.stablecoin_config,
            stablecoin_mint_bump: bumps.stablecoin_mint,
            // bhrt_collateral_vault_bump: bumps.bhrt_collateral_vault,
        });

        self.bhrt_price_oracle.set_inner(PriceFeed {
            feed: 50,
            bhrt_price_oracle_bump: bumps.bhrt_price_oracle,
        });

        // Token-2022 metadata init
        let cpi_accounts = TokenMetadataInitialize {
            program_id: self.token_program.to_account_info(),
            mint: self.stablecoin_mint.to_account_info(),
            metadata: self.stablecoin_mint.to_account_info(), // metadata stored in mint
            mint_authority: self.stablecoin_config.to_account_info(),
            update_authority: self.stablecoin_config.to_account_info(),
        };

        let collateral_key = self.bhrt_collateral_mint.key();
        let seeds = &[
            b"stablecoin_config".as_ref(),
            collateral_key.as_ref(),
            &[bumps.stablecoin_config],
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_ctx: CpiContext<'_, '_, '_, '_, TokenMetadataInitialize<'_>> = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            cpi_accounts,
            signer_seeds,
        );

        token_metadata_initialize(
            cpi_ctx,
            "Hashrate Stablecoin Token".to_string(),
            "HST".to_string(),
            uri,
        )?;

        // Top-up mint after realloc so it remains rent-exempt
        Self::top_up_to_rent_exempt(
            self.stablecoin_mint.to_account_info(),
            self.admin.to_account_info(),
            self.system_program.to_account_info(),
        )?;

        Ok(())
    }

}


#[derive(Accounts)]
pub struct changePriceOracle<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [b"bhrt_price_oracle"],
        bump = bhrt_price_oracle.bhrt_price_oracle_bump
    )]
    pub bhrt_price_oracle: Account<'info, PriceFeed>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> changePriceOracle<'info> {
    pub fn change_price_oracle(&mut self, new_price: u64) -> Result<()> {
        self.bhrt_price_oracle.feed = new_price;
        Ok(())
    }
}