use anchor_lang::prelude::*;

use crate::{Config, UpdateConfigEvent, SEED_CONFIG_ACCOUNT};

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [SEED_CONFIG_ACCOUNT],
        bump = config.bump,
        has_one = authority
    )]
    pub config: Account<'info, Config>,
}

pub fn process_update_config(
    ctx: Context<UpdateConfig>,
    liquidation_threshold: u64,
    liquidation_bonus: u64,
    health_factor: u64,
) -> Result<()> {
    let config = &mut ctx.accounts.config;

    config.liquidation_threshold = liquidation_threshold;
    config.liquidation_bonus = liquidation_bonus;
    config.health_factor = health_factor;

    emit!(UpdateConfigEvent {
        message: "The Config account was updated".to_string()
    });

    Ok(())
}
