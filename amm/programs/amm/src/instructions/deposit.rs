use anchor_lang::prelude::*;
use anchor_spl::{aasociated_token::AssociatedToken, token::{Mint, Token, TokenAccount}};
use constant_product_curve:: ConstantProduct;


use crate::{state::Config};

#[derive(Accounts)]
pub struct Deposit<'info>{
    #[account(munt)]
    pub user: Signer<'info>,
     pub mint_x: Account<'info, Mint>,
    pub mint_y:  Account<'info, Mint>,
    #[account(
        hash_one = mint_x,
        has_one = mint_y,
        seeds = [b"config", config. seed.to_le_bytes().as_ref()],
        bump = 
    )]


        #[account(
        mut,
        aasociated_token::mint = mint_x,
        aasociated_token::authority = config
    )]
    pub vault_x : Account<'info, TokenAccount>,
    #[account(
        mut,
        aasociated_token::mint = mint_y,
        aasociated_token::authority = config
    )]
    pub vault_y : Account<'info, TokenAccount>,

}