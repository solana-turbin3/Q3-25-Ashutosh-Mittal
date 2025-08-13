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

    pub fn onboard_miner(ctx: Context<OnboardMiner>, nft_id: u64,name: String, uri: String, mining_power: u64) -> Result<()> {
        ctx.accounts.onboard_miner(nft_id,  name, uri, mining_power, &ctx.bumps)?;
        Ok(())
    }

    pub fn amm_initialize(ctx: Context<AmmInitialize>, fee: u16) -> Result<()> {
        ctx.accounts.amm_initialize(fee, ctx.bumps)
    }

    pub fn amm_deposit(ctx: Context<Deposit>, amount: u64, max_bhrt: u64, max_usdt: u64) -> Result<()> {
        ctx.accounts.deposit(amount, max_bhrt, max_usdt)
    }

    pub fn amm_withdraw(ctx: Context<Withdraw>, amount: u64, min_bhrt: u64, min_usdt: u64) -> Result<()> {
        ctx.accounts.withdraw(amount, min_bhrt, min_usdt)
    }

    pub fn amm_swap( ctx: Context<Swap>, is_bhrt: bool, amount_in: u64, min_amount_out: u64) -> Result<()> {
        ctx.accounts.swap(is_bhrt, amount_in, min_amount_out)
    }

    pub fn revoke_miner_participation(ctx: Context<RevokeMinerParticipation>, amount: u64) -> Result<()> {
        ctx.accounts.revoke_miner_participation(&ctx.bumps, amount)
    }
}
