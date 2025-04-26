use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, Token2022};

use crate::{
    Config, ANCHOR_DISCRIMINATOR, HEALTH_FACTOR, LIQUIDATION_BONUS, LIQUIDATION_THREASHOLD,
    SEED_CONFIG_ACCOUNT, SEED_MINT_ACCOUNT,
};

#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = Config::INIT_SPACE + ANCHOR_DISCRIMINATOR,
        seeds = [SEED_CONFIG_ACCOUNT],
        bump
    )]
    pub config: Account<'info, Config>,

    #[account(
        init,
        payer = authority,
        seeds = [SEED_MINT_ACCOUNT],
        bump,
        mint::decimals = 9,
        mint::authority = mint_account,
        mint::freeze_authority = mint_account,
        mint::token_program = token_program
    )]
    pub mint_account: InterfaceAccount<'info, Mint>, // stablecoin

    pub token_program: Program<'info, Token2022>, // Program -> beacause is Token2022
    pub system_program: Program<'info, System>,
}

pub fn process_initialize_config(ctx: Context<InitializeConfig>) -> Result<()> {
    ctx.accounts.config.set_inner(Config {
        authority: ctx.accounts.authority.key(),
        mint_account: ctx.accounts.mint_account.key(),
        liquidation_threshold: LIQUIDATION_THREASHOLD,
        liquidation_bonus: LIQUIDATION_BONUS,
        health_factor: HEALTH_FACTOR,
        bump: ctx.bumps.config,
        bump_mint_account: ctx.bumps.mint_account,
    });

    Ok(())
}
