use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer_checked, Mint, Token, TokenAccount, TransferChecked},
};

use crate::state::Config;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub mint_x: Account<'info, Mint>,
    pub mint_y: Account<'info, Mint>,
    #[account(
    init,
    payer= initializer,
    seeds=[b"lp", config.key().as_ref() ],
    bump,
    mint::decimals= 6,
    mint::authority=config,
)]
    pub mint_lp: Account<'info, Mint>,

    #[account(
    init,
    payer= initializer,
    seeds=[b"config",  seed.to_le_bytes().as_ref()],
    space= 8+ Config::INIT_SPACE,
    bump
)]
    pub config: Account<'info, Config>,

    #[account(
        init,
        payer=initializer,
        associated_token::mint=mint_x,
        associated_token::authority=config
    )]
    pub vault_x: Account<'info, TokenAccount>,

    #[account(
        init,
        payer=initializer,
        associated_token::mint=mint_x,
        associated_token::authority=config
    )]
    pub vault_y: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, seed: u64, fee: u16, authority: Option<Pubkey>, bumps: InitializeBumps) -> Result<()> {
        self.config.set_inner(Config { seed, authority, mint_x: self.mint_x.key(), mint_y: self.mint_y.key(), fee, locked: false, config_bump: bumps.config, lp_bump: bumps.mint_lp });
        Ok(())
    }
}
