use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{mint_to, transfer_checked, Mint, MintTo, TokenAccount, TokenInterface, Transfer, TransferChecked},
};
use constant_product_curve::{ConstantProduct, LiquidityPair};

use crate::{state::{AmmConfig, ProgramState}, error::AmmError};

#[derive(Accounts)]
// #[instruction(seed: u64)]
pub struct Swap<'info> {

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
        // seeds=[b"BHRT"],
        // bump = program_state.bhrt_mint_bump,
        // mint::token_program = token_program
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
        // seeds=[b"lp", amm_config.key().as_ref() ],
        // bump =  amm_config.lp_bump,
        // mint::token_program = token_program
        )]
        pub lp_mint: InterfaceAccount<'info, Mint>,

 // ----------------------------- Program Vaults ----------------------------
    // ---- Vault BHRT ----
    #[account(
        mut,
        // associated_token::mint=bhrt_mint,
        // associated_token::authority=amm_config,
        // associated_token::token_program = token_program
    )]
    pub vault_bhrt: InterfaceAccount<'info, TokenAccount>,

    // ---- Vault USDT ----
    #[account(
        mut,
        // associated_token::mint=udst_mint,
        // associated_token::authority=amm_config,
        // associated_token::token_program = token_program
    )]
    pub vault_usdt: InterfaceAccount<'info, TokenAccount>,


    // ----------------------------- User Vaults ---------------------------------
    // ---- User BHRT ----
    #[account(
        init_if_needed,
        payer= user,
        associated_token::mint=bhrt_mint,
        associated_token::authority=user,
        associated_token::token_program = token_program
    )]
    pub user_bhrt: InterfaceAccount<'info, TokenAccount>,

    // ---- User USDT ----
    #[account(
        init_if_needed,
        payer= user,
        associated_token::mint=udst_mint,
        associated_token::authority=user,
        associated_token::token_program = token_program
    )]
    pub user_usdt: InterfaceAccount<'info, TokenAccount>,

   // ---- Required Programs ----
   pub token_program: Interface<'info, TokenInterface>,
   pub associated_token_program: Program<'info, AssociatedToken>,
   pub system_program: Program<'info, System>,

}


impl<'info> Swap<'info> {

    pub fn swap(&mut self, is_bhrt: bool, amount_in: u64, min_amount_out: u64) -> Result<()> {
        require!(amount_in > 0, AmmError::InvalidAmount);

        let mut curve = ConstantProduct::init(self.vault_bhrt.amount, self.vault_usdt.amount, self.lp_mint.supply, self.amm_config.fee, None,)
        .map_err(AmmError::from)?;

        let p: LiquidityPair = if is_bhrt {
            LiquidityPair::X
        } else {
            LiquidityPair::Y
        };

        let swap_result = curve.swap(p, amount_in, min_amount_out)
            .map_err(AmmError::from)?;

        require!(swap_result.deposit != 0, AmmError::InvalidAmount);
        require!(swap_result.withdraw != 0, AmmError::InvalidAmount);

        self.deposit_token(is_bhrt, swap_result.deposit)?;
        self.withdraw_token(!is_bhrt, swap_result.withdraw)?;

        Ok(())
    }

    pub fn deposit_token(&mut self, is_bhrt: bool, amount: u64) -> Result<()> {

        let (from, to, mint, decimals) = if is_bhrt {
            (
                self.user_bhrt.to_account_info(),
                self.vault_bhrt.to_account_info(),
                self.bhrt_mint.to_account_info(),
                self.bhrt_mint.decimals,
            )
        } else {
            (
                self.user_usdt.to_account_info(),
                self.vault_usdt.to_account_info(),
                self.udst_mint.to_account_info(),
                self.udst_mint.decimals,
            )
        };

        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = TransferChecked {
            from,
            mint,
            to,
            authority: self.user.to_account_info()
        };

        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
        transfer_checked(cpi_context, amount, decimals)
    }

    pub fn withdraw_token(&mut self, is_bhrt: bool, amount: u64) -> Result<()> {
        let (from, to, mint, decimals) = if is_bhrt {
            (
                self.vault_bhrt.to_account_info(),
                self.user_bhrt.to_account_info(),
                self.bhrt_mint.to_account_info(),
                self.bhrt_mint.decimals,
            )
        } else {
            (
                self.vault_usdt.to_account_info(),
                self.user_usdt.to_account_info(),
                self.udst_mint.to_account_info(),
                self.udst_mint.decimals,
            )
        };

        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = TransferChecked {
            from,
            to,
            mint,
            authority: self.amm_config.to_account_info(),
        };

        let program_state_key = self.program_state.key();
        let seeds: &[&[u8]; 3] = &[&b"amm_config"[..], program_state_key.as_ref(), &[self.amm_config.amm_config_bump]];
        let signer_seed = &[&seeds[..]];

        let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seed);
        transfer_checked(cpi_context, amount, decimals)
    }
}