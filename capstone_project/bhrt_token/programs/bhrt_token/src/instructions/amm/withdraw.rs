use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{burn, transfer_checked, Burn, Mint, Token, TokenAccount, TransferChecked}, token_interface::TokenInterface,
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
    pub fn withdraw(&mut self, amount: u64, min_x: u64, min_y: u64) -> Result<()> {
        require!(amount != 0, AmmError::InvalidAmount);

        let (x, y) = match self.mint_lp.supply == 0
            && self.vault_x.amount == 0
            && self.vault_y.amount == 0
        {
            true => (min_x, min_y),
            false => {
                let amounts = ConstantProduct::xy_withdraw_amounts_from_l(
                    self.vault_x.amount,
                    self.vault_y.amount,
                    self.mint_lp.supply,
                    amount,
                    6,
                ).unwrap();
                (amounts.x, amounts.y)
            }
        };

        require!(x >= min_x && y >= min_y, AmmError::SlippageExceeded);

        self.burn_lp_tokens(amount)?;
        self.withdraw_tokens(x, true)?;
        self.withdraw_tokens(y, false)
    }

    pub fn burn_lp_tokens(&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Burn {
            mint: self.mint_lp.to_account_info(),
            from: self.user_token_account_lp.to_account_info(),
            authority: self.user.to_account_info(),
        };

        let signer_seeds: &[&[&[u8]]] = &[&[
            b"config",
            &[self.config.config_bump],
        ]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        burn(cpi_ctx, amount)
    }

    pub fn withdraw_tokens(&mut self, amount: u64, is_x: bool) -> Result<()> {
        let (from, to, mint, decimals) = match is_x {
            true => (
                self.vault_x.to_account_info(),
                self.user_token_account_x.to_account_info(),
                self.mint_x.to_account_info(),
                self.mint_x.decimals,
            ),
            false => (
                self.vault_y.to_account_info(),
                self.user_token_account_y.to_account_info(),
                self.mint_y.to_account_info(),
                self.mint_y.decimals,
            ),
        };

        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = TransferChecked {
            from,
            to,
            mint,
            authority: self.config.to_account_info(),
        };

        let signer_seeds: &[&[&[u8]]] = &[&[
            b"config",
            &[self.config.config_bump],
        ]];

        let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        transfer_checked(cpi_context, amount, decimals)
    }
}