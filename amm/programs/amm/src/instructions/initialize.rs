use anchor_lang::prelude::*;
use anchor_spl::{aasociated_token::AssociatedToken, token:: {Mint, Token, TokenAccount}};

#[derive(Accounts)]
pub struct Initialize {
    pub Initializer : Signer<'info>,
    pub mint_x: Account<'info, Mint>,
    pub mint_y:  Account<'info, Mint>,
    #[account(
        init, 
        payer= Initializer,
        seed=[b"lp", config.key().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority =config
    )]
    pub mint_lp: Account<'info,Mint>,
    #[account(
        init, 
        payer= Initializer,
        seed=[b"config", seed.to_le_bytes().as_ref()],
        bump,
       space = 8 + Config::INIT_SPACE,
    )]
    pub config: Account<'info,Config>,


    #[account(
        init,
        payer=Initializer,
        aasociated_token::mint = mint_x,
        aasociated_token::authority = config
    )]
    pub vault_x : Account<'info, TokenAccount>,
    #[account(
        init,
        payer=Initializer,
        aasociated_token::mint = mint_y,
        aasociated_token::authority = config
    )]
    pub vault_y : Account<'info, TokenAccount>,

    pub token_program : Program<'info, Token>,
    pub aasociated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>


}

// pub fn handler(ctx: Context<Initialize>) -> Result<()> {
//     msg!("Greetings from: {:?}", ctx.program_id);
//     Ok(())
// }

impl <'info> Initialize<'info>{
    pub fn init (&mut self, seed: u64, fee: u16, authority: Option<Pubkey>, bumps: InitializeBumps)-> Result<()>{
        self.config.set_inner(Config{seed, authority, mint_x:self.mint_x.key(), mint_y: self.mint_y.key(), fee, locked: false, config_bump: bumps.config, lp_bump:bumps.mint_lp});
    }
}