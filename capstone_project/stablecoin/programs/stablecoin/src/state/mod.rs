
use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct StablecoinConfig {
    pub authority: Pubkey,
    pub mint: Pubkey,
    // pub supply: u64, 
    // pub decimals: u8,
    #[max_len(20)]
    pub name: String,
    #[max_len(10)]
    pub symbol: String,
    pub bhrt_collateral_mint: Pubkey,
    pub bhrt_collateral_vault: Pubkey,
    pub total_bhrt_collateral_staked: u64,
    pub number_of_investors: u64,
    pub stablecoin_mint: Pubkey,
    pub total_stablecoin_minted: u64,
    pub stablecoin_config_bump: u8,
    pub stablecoin_mint_bump: u8,
    // pub bhrt_collateral_vault_bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct StablecoinMinter {
    pub user: Pubkey,
    pub number_of_bhrt_collateral: u64,
    pub bhrt_usd_priced: u64,
    pub debt_amount: u64,
    pub bhrt_collateral_mint: Pubkey,
    pub stablecoin_minter_bump: u8,
}


#[account]
#[derive(InitSpace)]
pub struct PriceFeed {
    pub feed: u64,
    pub bhrt_price_oracle_bump: u8,
}