use anchor_lang::prelude::*;

#[error_code]
pub enum NftMintError {
    #[msg("The signer is not an approved miner.")]
    MinerNotApproved,
    #[msg("This miner has already been approved.")]
    MinerAlreadyApproved,
}
