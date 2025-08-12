use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        burn, transfer_checked, Burn, Mint, TokenAccount, TokenInterface, TransferChecked,
    },
};

use crate::{
    error::PositionDebtSettlementError,
    state::{StablecoinConfig, StablecoinMinter},
    PriceFeed,
};

#[derive(Accounts)]
pub struct PositionDebtSettlement<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    pub bhrt_collateral_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = bhrt_collateral_mint,
        associated_token::authority = user,
        associated_token::token_program = token_program,
    )]
    pub bhrt_user_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"stablecoin_config", bhrt_collateral_mint.key().as_ref()],
        bump = stablecoin_config.stablecoin_config_bump
    )]
    pub stablecoin_config: Account<'info, StablecoinConfig>,

    #[account(
        mut,
        seeds = [b"bhrt_price_oracle"],
        bump = bhrt_price_oracle.bhrt_price_oracle_bump
    )]
    pub bhrt_price_oracle: Account<'info, PriceFeed>,

    #[account(
        mut,
        seeds = [b"stablecoin_minter", user.key().as_ref()],
        bump = stablecoin_minter.stablecoin_minter_bump
    )]
    pub stablecoin_minter: Account<'info, StablecoinMinter>,

    #[account(
        mut,
        seeds=[b"HST"],
        bump,
        mint::token_program = token_program,
    )]
    pub stabelcoin_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = stabelcoin_mint,
        associated_token::authority = user,
        associated_token::token_program = token_program,
    )]
    pub stablecoin_user_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [b"bhrt_collateral_vault", stablecoin_config.key().as_ref(), stabelcoin_mint.key().as_ref()],
        token::mint = bhrt_collateral_mint,
        token::authority = stablecoin_config,
        bump = stablecoin_config.bhrt_collateral_vault_bump
    )]
    pub bhrt_collateral_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> PositionDebtSettlement<'info> {
    pub fn position_debt_settlement(&mut self, debt_amount: u64) -> Result<()> {
        require!(
            debt_amount > 0 && debt_amount <= self.stablecoin_minter.debt_amount,
            PositionDebtSettlementError::DebtAmountInvalid
        );

        let current_bhrt_usd_price = self.bhrt_price_oracle.feed;

        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Burn {
            mint: self.stabelcoin_mint.to_account_info(),
            from: self.stablecoin_user_token_account.to_account_info(),
            authority: self.user.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        burn(cpi_ctx, debt_amount)?;

        let get_back_collateral_amount: u64 = (self.stablecoin_minter.number_of_bhrt_collateral)
            * (debt_amount / self.stablecoin_minter.debt_amount);

        let transfer_accounts = TransferChecked {
            from: self.bhrt_collateral_vault.to_account_info(),
            mint: self.bhrt_collateral_mint.to_account_info(),
            to: self.bhrt_user_token_account.to_account_info(),
            authority: self.stablecoin_config.to_account_info(),
        };

        let seeds = &[
            b"stablecoin_config",
            self.bhrt_collateral_mint.to_account_info().key.as_ref(),
            &[self.stablecoin_config.stablecoin_config_bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let transfer_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            transfer_accounts,
            signer_seeds,
        );

        transfer_checked(
            transfer_ctx,
            get_back_collateral_amount,
            self.bhrt_collateral_mint.decimals,
        )?;

        self.stablecoin_minter.debt_amount -= debt_amount;
        self.stablecoin_minter.number_of_bhrt_collateral -= get_back_collateral_amount;

        let is_all_debt_paid = self.stablecoin_minter.debt_amount == 0;

        if is_all_debt_paid {
            // Send rent lamports to liquidator
            let lamports = self.stablecoin_minter.to_account_info().lamports();
            **self.user.lamports.borrow_mut() += lamports;
            **self
                .stablecoin_minter
                .to_account_info()
                .lamports
                .borrow_mut() = 0;

            // Set account owner to System Program to fully close
            self.stablecoin_minter
                .to_account_info()
                .assign(&self.system_program.to_account_info().key());
        }

        Ok(())
    }
}
