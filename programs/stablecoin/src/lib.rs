mod constants;
mod errors;
mod events;
mod instructions;
mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use errors::*;
pub use events::*;
pub use instructions::*;
pub use state::*;

declare_id!("GwrhJR6FF8YNdnofmTR8E2jRxj3wMcnSrPr1rwx3TJ3Q");

#[program]
pub mod stablecoin {
    use super::*;

    pub fn initialize_config(ctx: Context<InitializeConfig>) -> Result<()> {
        process_initialize_config(ctx)
    }

    pub fn update_config(
        ctx: Context<UpdateConfig>,
        health_factor: u64,
        liquidation_threshold: u64,
        liquidation_bonus: u64,
    ) -> Result<()> {
        process_update_config(ctx, health_factor, liquidation_threshold, liquidation_bonus)
    }

    pub fn deposit_and_mint(ctx: Context<DepositAndMint>, deposit_amount: u64) -> Result<()> {
        process_deposit_and_mint(ctx, deposit_amount)
    }
}
