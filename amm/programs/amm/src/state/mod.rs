use anchor_lang::prelude::*;

#[account]

pub struct Config{
    pub seed: u64,
    pub authority: Option<Pubkey>,
    pub mint_x: Pubkey,
        pub mint_y: Pubkey,
        

}