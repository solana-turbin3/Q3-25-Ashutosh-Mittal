pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

// declare_id!("F8MQK7RcrUisW1p9pwuTa6B5XqM9L3ZrzTec8eWAC6Qa");
declare_id!("AbBt8CJq2PrE9WoDR5iSmJXxuFCGcs7PMKUuZKzVxFDD");
#[program]
pub mod bhrt_token {
    use super::*;

    pub fn authorityinitialization(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.init_authority(&ctx.bumps)?;
        Ok(())
    }

    pub fn approve_miners(ctx: Context<ApproveMiner>, miner_to_add: Pubkey) -> Result<()> {
        ctx.accounts.approve_miner(miner_to_add)?;
        Ok(())
    }

    pub fn onboard_miner(ctx: Context<OnboardMiner>, name: String, uri: String,nft_id: u64, mining_power: u64) -> Result<()> {
        ctx.accounts.onboard_miner(&ctx.bumps, name, uri, nft_id, mining_power)?;
        Ok(())
    }
}
