use crate::state::Bet;
use anchor_instruction_sysvar::Ed25519InstructionSignatures;
use anchor_lang::{prelude::*, solana_program::{self, ed25519_program, sysvar::instructions::load_instruction_at_checked}, system_program::{transfer, Transfer}};
use solana_program::example_mocks::solana_sdk::signature;


#[derive(Accounts)]
pub struct ResolveBet <'info>{
    #[account(mut)]
    pub house:Signer<'info>,

    // CHECK: This is safe
pub player: UncheckedAccount<'info>,

#[account(
    mut,
    close= player,
    has_one= player,
    seeds = [b"bet", vault.key().as_ref(), bet.seed.to_le_bytes().as_ref()],
    bump = bet.bump
)]
pub bet: Account<'info, Bet>,

    #[account(
        mut, 
        seeds=[b"vault",house.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,

#[account(
    address= solana_program::sysvar::instructions::ID
)]
    pub instruction_sysvar: AccountInfo<'info>,
    pub system_program:Program<'info, System>
}

impl <'info> ResolveBet<'info> {
    pub fn verify_ef25519_signature(&mut self, sig:&[u8])-> Result<()>{
        let ix= load_instruction_at_checked(0, &self.instruction_sysvar.to_account_info())?;

        require_keys_eq!(ix.program_id, ed25519_program::ID);

        require_eq!(ix.accounts.len(), 0);

        let signatures = Ed25519InstructionSignatures::unpack(&ix.data)?.0;

        require_eq!(signatures.len(), 1);

        let signature = &signatures[0];

        require!(signature.is_verifiable);

        require_keys_eq!(signature.public_key.ok_or(err)?, self.house.key(), err);

        require!(&signature.signature.ok_or(err)?.eq(sig), err);

        require!(&signature.message.as_ref().ok_or(err).eq(&self.bet.to_slice()), err);

        Ok(())



    }

    pub 
}
