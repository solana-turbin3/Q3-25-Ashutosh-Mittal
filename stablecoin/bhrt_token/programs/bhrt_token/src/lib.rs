pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("F8MQK7RcrUisW1p9pwuTa6B5XqM9L3ZrzTec8eWAC6Qa");

#[program]
pub mod bhrt_token {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }
}
