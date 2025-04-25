mod constants;
mod instructions;
mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("GwrhJR6FF8YNdnofmTR8E2jRxj3wMcnSrPr1rwx3TJ3Q");

#[program]
pub mod stablecoin {
    use super::*;

    pub fn initialize_config(ctx: Context<InitializeConfig>) -> Result<()> {
        initialize_config::process_initialize_config(ctx)
    }
}
