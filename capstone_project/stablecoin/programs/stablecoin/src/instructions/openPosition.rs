use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        mint_to, transfer_checked, Mint, MintTo, TokenAccount, TokenInterface, TransferChecked,
    },
};

use crate::constants::{BASIS_POINTS, COLLATERAL_RATIO};
use crate::{
    error::ErrorCode,
    state::{StablecoinConfig, StablecoinMinter},
    PriceFeed,
};

#[derive(Accounts)]
pub struct OpenPosition<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    pub bhrt_collateral_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        // associated_token::mint = bhrt_collateral_mint,
        // associated_token::authority = user,
        // associated_token::token_program = token_program,
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
        init,
        payer = user,
        seeds = [b"stablecoin_minter", user.key().as_ref()],
        space = 8 + StablecoinMinter::INIT_SPACE,
        bump
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
        init,
        payer = user,
        associated_token::mint = stabelcoin_mint,
        associated_token::authority = user,
        associated_token::token_program = token_program,
    )]
    pub stablecoin_user_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = bhrt_collateral_mint,
        associated_token::authority = stablecoin_config,
        associated_token::token_program = token_program,
    )]
    pub bhrt_collateral_vault: InterfaceAccount<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

// Constants for collateralization
// const COLLATERAL_RATIO: u16 = 15000; // 150% in basis points (15000/10000 = 1.5)
// const BASIS_POINTS: u16 = 10000;

impl<'info> OpenPosition<'info> {
    pub fn open_position(
        &mut self,
        collateral_amount: u64,
        mut stablecoin_amount: u64,
        bump: OpenPositionBumps,
    ) -> Result<()> {
        // Step 1: Get BTCST price from oracle (simplified - you'll need actual oracle integration)
        let btcst_price = self.bhrt_price_oracle.feed; // Returns price in USD with 8 decimals

        // Step 2: Calculate collateral value in USD
        let collateral_value_usd = (collateral_amount as u128)
            .checked_mul(btcst_price as u128)
            .ok_or(ProgramError::ArithmeticOverflow)?;
            // .checked_div(10_u128.pow(self.bhrt_collateral_mint.decimals as u32))
            // .ok_or(ProgramError::ArithmeticOverflow)?;
        // .checked_div(10_u128.pow(8)) // Oracle price decimals
        // .ok_or(ProgramError::ArithmeticOverflow)?;

        // Step 3: Calculate maximum stablecoin that can be minted (collateral_value / 1.5)
        let max_stablecoin_mintable = collateral_value_usd
            .checked_mul(BASIS_POINTS as u128)
            .ok_or(ProgramError::ArithmeticOverflow)?
            .checked_div(COLLATERAL_RATIO as u128)
            .ok_or(ProgramError::ArithmeticOverflow)?;

        // Step 4: Check if requested stablecoin amount is within limits
        // ******************************************************
        // require!(
        //     stablecoin_amount as u128 <= max_stablecoin_mintable,
        //     ErrorCode::InsufficientCollateral
        // );
        // ******************************************************
        stablecoin_amount = max_stablecoin_mintable as u64;

        // Step 5: Transfer BTCST collateral from user to vault
        let transfer_accounts = TransferChecked {
            from: self.bhrt_user_token_account.to_account_info(),
            mint: self.bhrt_collateral_mint.to_account_info(),
            to: self.bhrt_collateral_vault.to_account_info(),
            authority: self.user.to_account_info(),
        };

        let transfer_ctx = CpiContext::new(self.token_program.to_account_info(), transfer_accounts);

        transfer_checked(
            transfer_ctx,
            collateral_amount,
            self.bhrt_collateral_mint.decimals,
        )?;

        // Step 6: Update user's position in StablecoinMinter account
        // let minter = &mut self.stablecoin_minter;
        // if minter.user == Pubkey::default() {
        //     // First time user - initialize
        //     minter.user = self.user.key();
        //     minter.collateral_deposited = collateral_amount;
        //     minter.stablecoin_minted = stablecoin_amount;
        // } else {
        //     // Update existing position
        //     minter.collateral_deposited = minter.collateral_deposited
        //         .checked_add(collateral_amount)
        //         .ok_or(ProgramError::ArithmeticOverflow)?;
        //     minter.stablecoin_minted = minter.stablecoin_minted
        //         .checked_add(stablecoin_amount)
        //         .ok_or(ProgramError::ArithmeticOverflow)?;
        // }

        self.stablecoin_minter.set_inner(StablecoinMinter {
            user: self.user.key(),
            number_of_bhrt_collateral: collateral_amount,
            bhrt_usd_priced: btcst_price,
            debt_amount: stablecoin_amount,
            bhrt_collateral_mint: self.bhrt_collateral_mint.key(),
            stablecoin_minter_bump: bump.stablecoin_minter,
        });

        // Step 7: Update global collateral and debt in StablecoinConfig
        let config = &mut self.stablecoin_config;
        config.total_bhrt_collateral_staked = config
            .total_bhrt_collateral_staked
            .checked_add(collateral_amount)
            .ok_or(ProgramError::ArithmeticOverflow)?;
        config.total_stablecoin_minted = config
            .total_stablecoin_minted
            .checked_add(stablecoin_amount)
            .ok_or(ProgramError::ArithmeticOverflow)?;
        config.number_of_investors = config
            .number_of_investors
            .checked_add(1)
            .ok_or(ProgramError::ArithmeticOverflow)?;

        // Step 8: Mint stablecoin to user
        let mint_accounts = MintTo {
            mint: self.stabelcoin_mint.to_account_info(),
            to: self.stablecoin_user_token_account.to_account_info(),
            authority: self.stablecoin_config.to_account_info(),
        };

       
        let collateral_key = self.bhrt_collateral_mint.key();
        let seeds = &[
            b"stablecoin_config".as_ref(),
            collateral_key.as_ref(),
            &[self.stablecoin_config.stablecoin_config_bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let mint_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            mint_accounts,
            signer_seeds,
        );

        mint_to(mint_ctx, stablecoin_amount)?;

        msg!(
            "Minted {} stablecoin against {} BTCST collateral. Collateral ratio: {}%",
            stablecoin_amount,
            collateral_amount,
            COLLATERAL_RATIO / 100
        );

        Ok(())
    }

    // Helper function to get BTCST price from oracle
    // fn get_btcst_price(&self) -> Result<u64> {
    //     // TODO: Implement actual oracle price fetching
    //     // For now, return a mock price (e.g., $50 per BTCST with 8 decimals)
    //     Ok(5000000000u64) // $50.00000000
    // }
}
