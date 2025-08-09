use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::state::{ProgramState, AmmConfig};

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
        init,
        payer= authority,
        seeds=[b"amm_config", program_state.key().as_ref()],
        space= 8+ AmmConfig::INIT_SPACE,
        bump
    )]
    pub amm_config: Account<'info, AmmConfig>,

    #[account(
        mut,
        seeds=[b"BHRT"],
        bump = program_state.bhrt_mint_bump,
        mint::token_program = token_program
    )]
    pub bhrt_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mint::token_program = token_program
    )]
    pub udst_mint: InterfaceAccount<'info, Mint>,

    #[account(
    init,
    payer= authority,
    seeds=[b"lp", amm_config.key().as_ref() ],
    bump,
    mint::decimals= 6,
    mint::authority= amm_config,
    mint::token_program = token_program,
)]
    pub lp_mint: InterfaceAccount<'info, Mint>,


    #[account(
        init,
        payer= authority,
        associated_token::mint= bhrt_mint,
        associated_token::authority=  amm_config,
        associated_token::token_program = token_program
    )]
    pub vault_bhrt: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer= authority,
        associated_token::mint= udst_mint,
        associated_token::authority=  amm_config,
        associated_token::token_program = token_program
    )]
    pub vault_usdt: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> AmmInitialize<'info> {
    pub fn amm_initialize(&mut self, fee: u16, bumps: AmmInitializeBumps) -> Result<()> {
        self.amm_config.set_inner(AmmConfig { authority: Some(self.authority.key()), bhrt_mint: self.bhrt_mint.key(), udst_mint: self.udst_mint.key(), fee, locked: false, amm_config_bump: bumps.amm_config, lp_bump: bumps.lp_mint });
        Ok(())
    }
}
