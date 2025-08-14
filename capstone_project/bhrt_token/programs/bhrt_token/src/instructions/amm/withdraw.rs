use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{burn, transfer_checked, Burn, Mint, TokenAccount, TokenInterface, TransferChecked},
};
use constant_product_curve::ConstantProduct;

use crate::{state::{AmmConfig, ProgramState}, error::AmmError};

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

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

   // ---- AmmConfig ----
    #[account(
        mut,
        seeds=[b"amm_config", program_state.key().as_ref()],
        bump, 
        has_one = bhrt_mint,
        has_one = udst_mint
    )]
    pub amm_config: Account<'info, AmmConfig>,

    // ----------------------------- Mints -----------------------------------
    // ---- BHRT Mint ----
    #[account(
        mut,
        seeds=[b"BHRT"],
        bump = program_state.bhrt_mint_bump,
        mint::token_program = token_program
    )]
    pub bhrt_mint: InterfaceAccount<'info, Mint>,

    // ---- USDT Mint ----
    #[account(
        mint::token_program = token_program
    )]
    pub udst_mint: InterfaceAccount<'info, Mint>,


    // ---- LP Mint ----
    #[account(
        mut,
        seeds=[b"lp", amm_config.key().as_ref() ],
        bump =  amm_config.lp_bump,
        mint::token_program = token_program
        )]
        pub lp_mint: InterfaceAccount<'info, Mint>,

    // ----------------------------- Program Vaults ----------------------------
    // ---- Vault BHRT ----
    #[account(
        mut,
        associated_token::mint=bhrt_mint,
        associated_token::authority=amm_config,
        associated_token::token_program = token_program
    )]
    pub vault_bhrt: InterfaceAccount<'info, TokenAccount>,

    // ---- Vault USDT ----
    #[account(
        mut,
        associated_token::mint=udst_mint,
        associated_token::authority=amm_config,
        associated_token::token_program = token_program
    )]
    pub vault_usdt: InterfaceAccount<'info, TokenAccount>,

    // ----------------------------- User Vaults ---------------------------------
    // ---- User BHRT ----
    #[account(
        mut,
        associated_token::mint=bhrt_mint,
        associated_token::authority=user,
        associated_token::token_program = token_program
    )]
    pub user_bhrt: InterfaceAccount<'info, TokenAccount>,

    // ---- User USDT ----
    #[account(
        mut,
        associated_token::mint=udst_mint,
        associated_token::authority=user,
        associated_token::token_program = token_program
    )]
    pub user_usdt: InterfaceAccount<'info, TokenAccount>,

    // ---- User LP ----
    #[account(
        mut,
        associated_token::mint=lp_mint,
        associated_token::authority=user,
        associated_token::token_program = token_program
    )]
    pub user_lp: InterfaceAccount<'info, TokenAccount>,

    // ---- Required Programs ----
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Withdraw<'info> {
    pub fn withdraw(&mut self, amount: u64, min_bhrt: u64, min_usdt: u64) -> Result<()> {
        require!(amount != 0, AmmError::InvalidAmount);

        let (bhrt, usdt) = match self.lp_mint.supply == 0 && self.vault_bhrt.amount == 0 && self.vault_usdt.amount == 0 {

            true => (min_bhrt, min_usdt),
            false => {
                let amounts = ConstantProduct::xy_withdraw_amounts_from_l( self.vault_bhrt.amount, self.vault_usdt.amount, self.lp_mint.supply, amount, 6,).unwrap();
                (amounts.x, amounts.y)
            }

        };

        require!(bhrt >= min_bhrt && usdt >= min_usdt, AmmError::SlippageExceeded);

        self.burn_lp_tokens(amount)?;
        self.withdraw_tokens(bhrt, true)?;
        self.withdraw_tokens(usdt, false)
    }

    pub fn burn_lp_tokens(&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts: Burn<'_> = Burn {
            mint: self.lp_mint.to_account_info(),
            from: self.user_lp.to_account_info(),
            authority: self.user.to_account_info(),
        };

        let signer_seeds: &[&[&[u8]]] = &[&[
            b"amm_config",
            &[self.amm_config.amm_config_bump],
        ]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        burn(cpi_ctx, amount)
    }

    pub fn withdraw_tokens(&mut self, amount: u64, is_x: bool) -> Result<()> {
        let (from, to, mint, decimals) = match is_x {
            true => (
                self.vault_bhrt.to_account_info(),
                self.user_bhrt.to_account_info(),
                self.bhrt_mint.to_account_info(),
                self.bhrt_mint.decimals,
            ),
            false => (
                self.vault_usdt.to_account_info(),
                self.user_usdt.to_account_info(),
                self.udst_mint.to_account_info(),
                self.udst_mint.decimals,
            ),
        };

        let cpi_program = self.token_program.to_account_info();
        
        let cpi_accounts = TransferChecked {
            from,
            to,
            mint,
            authority: self.amm_config.to_account_info(),
        };

        let signer_seeds: &[&[&[u8]]] = &[&[
            b"amm_config",
            &[self.amm_config.amm_config_bump],
        ]];

        let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        transfer_checked(cpi_context, amount, decimals)
    }
}