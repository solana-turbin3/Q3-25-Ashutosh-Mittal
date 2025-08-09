use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, transfer, transfer_checked, Mint, MintTo, Token, TokenAccount, Transfer, TransferChecked},
};
use constant_product_curve::ConstantProduct;

use crate::{state::{AmmConfig}, error::AmmError};

#[derive(Accounts)]
// #[instruction(seed: u64)]
pub struct Swap<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub mint_x: Account<'info, Mint>,
    pub mint_y: Account<'info, Mint>,
    #[account(
    // mut,
    has_one = mint_x,
    has_one = mint_y,
    seeds=[b"config",  config.seed.to_le_bytes().as_ref()],
    bump= config.config_bump
)]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        associated_token::mint=mint_x,
        associated_token::authority=config
    )]
    pub vault_x: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint=mint_x,
        associated_token::authority=config
    )]
    pub vault_y: Account<'info, TokenAccount>,

     #[account(
        init_if_needed,
        payer = user,
        associated_token::mint=mint_x,
        associated_token::authority=user
    )]
    pub user_x: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint=mint_x,
        associated_token::authority=user
    )]
    pub user_y: Account<'info, TokenAccount>,
     pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,

}

impl <'info> Swap <'info>{

    pub fn swap (&mut self, )
}

