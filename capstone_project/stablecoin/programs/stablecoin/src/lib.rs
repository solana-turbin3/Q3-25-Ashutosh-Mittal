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
}
