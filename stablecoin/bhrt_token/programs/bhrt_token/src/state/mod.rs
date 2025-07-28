use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct NFTProgramInfo{
    pub nft_id_counter: u64,
    pub authority: Pubkey,
    #[max_len(100)] 
    pub miners: Vec<Pubkey>,
    pub bump: u8
}