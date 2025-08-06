use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct ProgramState {
    pub nft_id_counter: u64,
    pub authority: Pubkey,
    #[max_len(100)]
    pub approved_miners: Vec<Pubkey>,
    pub program_state_bump: u8,
    pub bhrt_mint_bump: u8,
    pub collection_mint_bump: u8,
    pub collection_metadata_bump: u8
}

#[account]
#[derive(InitSpace)]
pub struct MinerInfo {
    pub seed: u64,
    pub hashrate_power: u64,
    #[max_len(150)]
    pub legal_document_uri: String,
    pub hashrate_token_mint: Pubkey,
    pub mint_amount: u64,
    // pub locked: bool,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct AmmConfig {
    pub seed: u64,
    pub authority: Option<Pubkey>,
    pub mint_x: Pubkey,
    pub mint_y: Pubkey,
    pub fee_basis_points: u16,
    pub locked: bool,
    pub config_bump: u8,
    pub lp_bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct BhrtMetadata {
    pub collection: Pubkey, // This field points to the Collection NFT's Mint address
    #[max_len(50)]
    pub description: String,
    #[max_len(10)]
    pub symbol: String,
}
