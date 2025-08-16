pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("CCLDZoaXu8EchMrVdVHokdyhBUGgHWBVguMibawESYJj");

#[program]
pub mod stablecoin {
    use super::*;

    pub fn initialize_config_and_vault(ctx: Context<InitializeVault>, uri: String) -> Result<()> {
        ctx.accounts.initialize_config_and_vault(ctx.bumps, uri)
    }

    pub fn open_position(ctx: Context<OpenPosition>, collateral_amount: u64,stablecoin_amount: u64) -> Result<()> {
        ctx.accounts.open_position(collateral_amount, stablecoin_amount, ctx.bumps)
    }

    pub fn liquidate(ctx: Context<Liquidation>, liquidation_amount: u64) -> Result<()> {
        ctx.accounts.liquidate(liquidation_amount)
    }

    pub fn position_debt_settlement(ctx: Context<PositionDebtSettlement>, debt_amount: u64) -> Result<()> {
        ctx.accounts.position_debt_settlement(debt_amount)
    }

}
