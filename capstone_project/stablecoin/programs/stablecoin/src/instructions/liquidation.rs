use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        burn, transfer_checked, Burn, Mint, TokenAccount, TokenInterface, TransferChecked,
    },
};

use crate::constants::{BASIS_POINTS, COLLATERAL_RATIO};
use crate::{
    error::LiquidationError,
    state::{StablecoinConfig, StablecoinMinter},
    PriceFeed, PENALTY_REWARD_PERCENTAGE,
};

#[derive(Accounts)]
pub struct Liquidation<'info> {
    #[account(mut)]
    pub liquidator: Signer<'info>,

    #[account(mut)]
    pub target_position: SystemAccount<'info>,

    pub bhrt_collateral_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = bhrt_collateral_mint,
        associated_token::authority = liquidator,
        associated_token::token_program = token_program,
    )]
    pub bhrt_liquidator_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = bhrt_collateral_mint,
        associated_token::authority = target_position,
        associated_token::token_program = token_program,
    )]
    pub bhrt_target_position_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"stablecoin_config", bhrt_collateral_mint.key().as_ref()],
        bump = stablecoin_config.stablecoin_config_bump
    )]
    pub stablecoin_config: Account<'info, StablecoinConfig>,

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
        associated_token::authority = liquidator,
        associated_token::token_program = token_program,
    )]
    pub stablecoin_liquidator_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [b"stablecoin_minter", target_position.key().as_ref()],
        bump = stablecoin_minter.stablecoin_minter_bump
    )]
    pub stablecoin_minter: Account<'info, StablecoinMinter>,

    #[account(
        mut,
        associated_token::mint = bhrt_collateral_mint,
        associated_token::authority = stablecoin_config,
        associated_token::token_program = token_program,
    )]
    pub bhrt_collateral_vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"bhrt_price_oracle"],
        bump = bhrt_price_oracle.bhrt_price_oracle_bump
    )]
    pub bhrt_price_oracle: Account<'info, PriceFeed>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

// Constants for liquidation
// const COLLATERAL_RATIO: u16 = 15000; // 150% in basis points
// const LIQUIDATION_THRESHOLD: u16 = 14999; // Below 150%
// const LIQUIDATION_PENALTY: u16 = 500; // 5% penalty (500 basis points)
// const BASIS_POINTS: u16 = 10000;

impl<'info> Liquidation<'info> {
    pub fn liquidate(&mut self, liquidation_amount: u64) -> Result<()> {
        let current_bhrt_usd_price = 30;

        let collateral_ratio = (self.stablecoin_minter.number_of_bhrt_collateral
            * current_bhrt_usd_price)
            / (self.stablecoin_minter.debt_amount * 1);
        let threshold_collateral_ratio = COLLATERAL_RATIO / BASIS_POINTS;
        msg!("stablecoin_minter.number_of_bhrt_collateral: {}", self.stablecoin_minter.number_of_bhrt_collateral);
        msg!("stablecoin_minter.debt_amount: {}", self.stablecoin_minter.debt_amount);
        msg!("current_bhrt_usd_price: {}", current_bhrt_usd_price);
        msg!("collateral_ratio: {}", collateral_ratio);
        msg!("threshold_collateral_ratio: {}", threshold_collateral_ratio);
        require!(
            collateral_ratio < threshold_collateral_ratio as u64,
            LiquidationError::SufficientCollateral
        );
        require!(
            liquidation_amount <= self.stablecoin_minter.debt_amount && liquidation_amount > 0,
            LiquidationError::LiquidationAmountInvalid
        );

        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Burn {
            mint: self.stabelcoin_mint.to_account_info(),
            from: self.stablecoin_liquidator_token_account.to_account_info(),
            authority: self.liquidator.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        burn(cpi_ctx, liquidation_amount)?;

        let penalty_reward = (liquidation_amount * PENALTY_REWARD_PERCENTAGE as u64) / 100;

        let liquidator_reward = (liquidation_amount + penalty_reward) / current_bhrt_usd_price;

        let transfer_accounts = TransferChecked {
            from: self.bhrt_collateral_vault.to_account_info(),
            mint: self.bhrt_collateral_mint.to_account_info(),
            to: self.bhrt_liquidator_token_account.to_account_info(),
            authority: self.stablecoin_config.to_account_info(),
        };

        let seeds = &[
            b"stablecoin_config",
            self.bhrt_collateral_mint.to_account_info().key.as_ref(),
            &[self.stablecoin_config.stablecoin_config_bump],
        ];
        let signer_seeds: &[&[&[u8]]; 1] = &[&seeds[..]];

        let transfer_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            transfer_accounts,
            signer_seeds,
        );

        transfer_checked(
            transfer_ctx,
            liquidator_reward,
            self.bhrt_collateral_mint.decimals,
        )?;

        let is_all_debt_paid = (self.stablecoin_minter.debt_amount - liquidation_amount) == 0;

        if is_all_debt_paid {
            let transfer_accounts_2 = TransferChecked {
                from: self.bhrt_collateral_vault.to_account_info(),
                mint: self.bhrt_collateral_mint.to_account_info(),
                to: self.bhrt_target_position_token_account.to_account_info(),
                authority: self.stablecoin_config.to_account_info(),
            };

            // let signer_seeds: &[&[&[u8]]] = &[&[
            //     b"stablecoin_config",
            //     self.bhrt_collateral_mint.key().as_ref(),
            //     &[self.stablecoin_config.stablecoin_config_bump],
            // ]];

            let transfer_ctx_2 = CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                transfer_accounts_2,
                signer_seeds,
            );

            transfer_checked(
                transfer_ctx_2,
                self.stablecoin_minter.number_of_bhrt_collateral,
                self.bhrt_collateral_mint.decimals,
            )?;
        }

        self.stablecoin_minter.debt_amount -= liquidation_amount;
        self.stablecoin_minter.number_of_bhrt_collateral -= liquidator_reward;

        if is_all_debt_paid {
            // Send rent lamports to liquidator
            let lamports = self.stablecoin_minter.to_account_info().lamports();
            **self.liquidator.lamports.borrow_mut() += lamports;
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
