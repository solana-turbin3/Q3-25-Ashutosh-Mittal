use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{mint_to, transfer_checked, Mint, MintTo, TokenAccount, TokenInterface, Transfer, TransferChecked},
};
use constant_product_curve::ConstantProduct;

use crate::state::{AmmConfig, ProgramState};
use crate::error::AmmError;

#[derive(Accounts)]
// #[instruction(seed: u64)]
pub struct Deposit<'info> {
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
        init_if_needed,
        payer= user,
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

impl<'info> Deposit<'info> {

        pub fn deposit(&mut self, amount: u64, max_bhrt: u64, max_usdt:u64) -> Result<()> {
            require!(self.amm_config.locked == false, AmmError::PoolLocked);
            require!(amount != 0, AmmError::InvalidAmount);

            let(bhrt,usdt)= match self.lp_mint.supply == 0 && self.vault_bhrt.amount == 0 && self.vault_usdt.amount == 0 {
                true => (max_bhrt, max_usdt),
                false => {
                    let amount = ConstantProduct::xy_deposit_amounts_from_l( self.vault_bhrt.amount, self.vault_usdt.amount,  self.lp_mint.supply,  amount,  6).unwrap();
                    (amount.x, amount.y)
                }
            };

            require!(bhrt<=max_bhrt && usdt <= max_usdt, AmmError::SlippageExceeded);

            self.deposit_tokens(true, bhrt);
            self.deposit_tokens(false, usdt);
            self.mint_lp_token(amount)
        }

    pub fn deposit_tokens(&mut self, is_bhrt:bool, amount: u64) -> Result<()> {
        let (from, to) = match is_bhrt{                 // from =user x   , to = vault x     (and vice versa)
            true => (self.user_bhrt.to_account_info(), self.vault_bhrt.to_account_info()),
                        false => (self.user_usdt.to_account_info(), self.vault_usdt.to_account_info()),
        };

        let decimals: u8;
        let mint: AccountInfo<'_>;
        if is_bhrt{
             decimals = self.bhrt_mint.decimals;
             mint = self.bhrt_mint.to_account_info();
        } else {
             decimals = self.udst_mint.decimals;
             mint = self.udst_mint.to_account_info();
        }

        let cpi_program: AccountInfo<'_> = self.token_program.to_account_info();

        let cpi_accounts = TransferChecked {
            from,
            mint , 
            to,
            authority: self.user.to_account_info()
        };

        let ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer_checked(ctx, amount, decimals)
    }

    pub fn mint_lp_token(&mut self,amount: u64) -> Result<()> {

        let cpi_program: AccountInfo<'_> = self.token_program.to_account_info();
         let cpi_accounts = MintTo {
            mint: self.lp_mint.to_account_info(),
            to: self.user.to_account_info(),
            authority: self.amm_config.to_account_info()
        };

        let seeds = &[&b"amm_config"[..], &self.amm_config.key().to_bytes(), &[self.amm_config.amm_config_bump]];
        let signer_seed = &[&seeds[..]];
        let ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seed);

        mint_to(ctx, amount)

    }


}
