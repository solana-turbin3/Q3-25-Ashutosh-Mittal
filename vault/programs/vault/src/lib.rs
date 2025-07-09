#![allow(unexpected_cfgs)]
#![allow(deprecated)]
use anchor_lang::prelude::*;

declare_id!("Aww3qCodkpqeVb2akSBLumb8SVYkSSGbPrV14NJZ9KnJ");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        // msg!("Greetings from: {:?}", ctx.program_id);
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer <'info>,
    #[account(
mut, 
seeds = [b"vault",vault_state.key().as_ref()],
bump,

    )]  
        pub vault: SystemAccount <'info>,
 #[account(
init, 
payer= signer,
seeds = [b"state",signer.key().as_ref()],
space = 8 + VaultState::INIT_SPACE,
bump,

    )]  

    pub vault_state: Account <'info, VaultState>,
        pub system_program: Program<'info, System>,

}


impl <'info> Initialize <'info>{
    pub fn initialize (&mut self, bumps: &InitializeBumps)-> Result<()>{

        self.vault_state.state_bump = bumps.vault_state;

        self.vault_state.vault_bump = bumps.vault;

        Ok(())
    }
}


#[account]
#[derive(InitSpace)]
pub struct VaultState{
    pub vault_bump: u8,
    pub state_bump: u8
}


