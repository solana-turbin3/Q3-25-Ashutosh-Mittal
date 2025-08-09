use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer_checked, Mint, Token, TokenAccount, TransferChecked},
};

use crate::state::ProgramState;

#[derive(Accounts)]
// #[instruction(seed: u64)]
pub struct AmmInitialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [b"program_state"],
        bump = program_state.program_state_bump,
        has_one = authority
    )]
    pub program_state: Account<'info, ProgramState>,

    #[account(
        mut,
        seeds=[b"BHRT"],
        bump = program_state.bhrt_mint_bump,
    )]
    pub bhrt_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mint::token_program = token_program
    )]
    pub udst_mint: InterfaceAccount<'info, Mint>,

    #[account(
    init,
    payer= authority,
    seeds=[b"lp", program_state.key().as_ref() ],
    bump,
    mint::decimals= 6,
    mint::authority=config,
    mint::token_program = token_program,
)]
    pub mint_lp: InterfaceAccount<'info, Mint>,


    #[account(
        init,
        payer= authority,
        associated_token::mint= bhrt_mint,
        associated_token::authority= program_state,
        associated_token::token_program = token_program
    )]
    pub vault_bhrt: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer= authority,
        associated_token::mint= usdt_mint,
        associated_token::authority= program_state,
        associated_token::token_program = token_program
    )]
    pub vault_usdt: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> AmmInitialize<'info> {
    pub fn initialize(&mut self, seed: u64, fee: u16, authority: Option<Pubkey>, bumps: InitializeBumps) -> Result<()> {
        self.config.set_inner(Config { seed, authority, mint_x: self.mint_x.key(), mint_y: self.mint_y.key(), fee, locked: false, config_bump: bumps.config, lp_bump: bumps.mint_lp });
        Ok(())
    }
}
