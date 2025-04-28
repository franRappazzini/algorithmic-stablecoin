use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, Token2022, TokenAccount},
};
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

use crate::{
    burn_token, redeem_sol, usd_to_lamports, Collateral, Config, DappError, SEED_CONFIG_ACCOUNT,
};

use super::calculate_health_factor;

#[derive(Accounts)]
pub struct Liquidate<'info> {
    #[account(mut)]
    pub liquidator: Signer<'info>,

    #[account(
        mut,
        associated_token::mint = mint_account,
        associated_token::authority = liquidator,
        associated_token::token_program = token_program
    )]
    pub liquidator_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)] // es "encontrado" por el constraint de collateral
    pub liquidator_sol_account: SystemAccount<'info>,

    #[account(
        mut, // es pasado como account
        constraint = collateral.sol_account == liquidator_sol_account.key()
    )]
    pub collateral: Account<'info, Collateral>,

    #[account(
        seeds = [SEED_CONFIG_ACCOUNT],
        bump = config.bump,
        has_one = mint_account,
    )]
    pub config: Account<'info, Config>,

    #[account(mut)] // es "encontrado" por el has_one de config
    pub mint_account: InterfaceAccount<'info, Mint>,

    pub price_update: Account<'info, PriceUpdateV2>,

    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn procees_liquidate(ctx: Context<Liquidate>, liquidate_amount: u64) -> Result<()> {
    let acc = &ctx.accounts;

    let health_factor = calculate_health_factor(&acc.price_update, &acc.collateral, &acc.config)?;
    require!(
        health_factor < acc.config.health_factor,
        DappError::HealthFactorIsTooBig
    );

    let lamports = usd_to_lamports(liquidate_amount, &acc.price_update)?;
    let liquidation_bonus = (lamports * acc.config.liquidation_bonus) / 100;
    let liquidable_lamports = lamports + liquidation_bonus;

    burn_token(
        &acc.token_program,
        &acc.mint_account,
        &acc.liquidator_token_account,
        &acc.liquidator,
        liquidate_amount,
    )?;

    redeem_sol(
        &acc.system_program,
        &acc.liquidator_sol_account,
        &acc.liquidator,
        acc.collateral.depositor,
        liquidable_lamports,
        acc.collateral.bump_sol_account,
    )?;

    let collateral = &mut ctx.accounts.collateral;
    collateral.lamport_balance = ctx.accounts.liquidator_sol_account.lamports();
    collateral.total_minted -= liquidate_amount;

    Ok(())
}
