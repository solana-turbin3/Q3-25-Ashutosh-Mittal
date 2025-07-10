#![allow(unexpected_cfgs)]
#![allow(deprecated)]
use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};

declare_id!("Aww3qCodkpqeVb2akSBLumb8SVYkSSGbPrV14NJZ9KnJ");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        // msg!("Greetings from: {:?}", ctx.program_id);
        ctx.accounts.initialize(&ctx.bumps)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        // msg!("Greetings from: {:?}", ctx.program_id);
        ctx.accounts.deposit(amount)
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw(amount)
    }

    pub fn close(ctx: Context<Close>) -> Result<()> {
        ctx.accounts.close()
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



#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub signer: Signer <'info>,
    #[account(
mut, 
seeds = [b"vault",vault_state.key().as_ref()],
bump = vault_state.vault_bump,

    )]  
        pub vault: SystemAccount <'info>,
 #[account(
seeds = [b"state",signer.key().as_ref()],
bump =  vault_state.state_bump,

    )]  

    pub vault_state: Account <'info, VaultState>,
        pub system_program: Program<'info, System>,

}


impl <'info> Deposit <'info>{
    pub fn deposit (&mut self, amount: u64)-> Result<()>{

       let cpi_program = self.system_program.to_account_info();

       let cpi_account = Transfer{
        from: self.signer.to_account_info(),
        to: self.vault.to_account_info()
       };

       let cpi_ctx = CpiContext::new(cpi_program, cpi_account);

       transfer(cpi_ctx, amount)?;

        Ok(())
    }
}


#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub signer: Signer <'info>,
    #[account(
mut, 
seeds = [b"vault",vault_state.key().as_ref()],
bump = vault_state.vault_bump,

    )]  
        pub vault: SystemAccount <'info>,
 #[account(
seeds = [b"state",signer.key().as_ref()],
bump =  vault_state.state_bump,

    )]  

    pub vault_state: Account <'info, VaultState>,
        pub system_program: Program<'info, System>,

}


impl <'info> Withdraw <'info>{
    pub fn withdraw (&mut self, amount: u64)-> Result<()>{

       let cpi_program = self.system_program.to_account_info();

       let cpi_account = Transfer{
        from: self.vault.to_account_info(),
        to: self.signer.to_account_info()
       };

       let pda_signinga_seed = [b"vault",self.vault_state.to_account_info().key.as_ref(), &[self.vault_state.vault_bump] ];
       let seed = [&pda_signinga_seed[..]];

       let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_account, &seed);

       transfer(cpi_ctx, amount)?;

        Ok(())
    }
}



#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut)]
    pub signer: Signer <'info>,
    #[account(
mut, 
seeds = [b"vault",vault_state.key().as_ref()],
bump = vault_state.vault_bump,

    )]  
        pub vault: SystemAccount <'info>,
 #[account(
    mut,
seeds = [b"state",signer.key().as_ref()],
bump =  vault_state.state_bump,
close = signer
    )]  

    pub vault_state: Account <'info, VaultState>,
        pub system_program: Program<'info, System>,

}


impl <'info> Close <'info>{
    pub fn close (&mut self)-> Result<()>{

       let cpi_program = self.system_program.to_account_info();

       let cpi_account = Transfer{
        from: self.vault.to_account_info(),
        to: self.signer.to_account_info()
       };

       let pda_signinga_seed = [b"vault",self.vault_state.to_account_info().key.as_ref(), &[self.vault_state.vault_bump] ];
       let seed = [&pda_signinga_seed[..]];

       let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_account, &seed);

       transfer(cpi_ctx, self.vault.lamports())?;

        Ok(())
    }
}



#[account]
#[derive(InitSpace)]
pub struct VaultState{
    pub vault_bump: u8,
    pub state_bump: u8
}


