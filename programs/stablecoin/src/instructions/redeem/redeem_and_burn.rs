use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, Token2022, TokenAccount},
};
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

use crate::{
    check_health_factor, Collateral, Config, SEED_COLLATERAL_ACCOUNT, SEED_CONFIG_ACCOUNT,
    SEED_SOL_ACCOUNT,
};

use super::{burn_token, redeem_sol};

#[derive(Accounts)]
pub struct RedeemAndBurn<'info> {
    #[account(mut)]
    pub redeemer: Signer<'info>,

    #[account(
        mut,
        associated_token::mint = mint_account,
        associated_token::authority = redeemer,
        associated_token::token_program = token_program
    )]
    pub redeemer_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [SEED_SOL_ACCOUNT, redeemer.key().as_ref()],
        bump
    )]
    pub redeemer_sol_account: SystemAccount<'info>,

    #[account(
        mut,
        seeds = [SEED_COLLATERAL_ACCOUNT, redeemer.key().as_ref()],
        bump,
        constraint = collateral.sol_account == redeemer_sol_account.key(),
        constraint = collateral.token_account == mint_account.key()
    )]
    pub collateral: Account<'info, Collateral>,

    #[account(
        seeds = [SEED_CONFIG_ACCOUNT],
        bump = config.bump,
        has_one = mint_account,
    )]
    pub config: Account<'info, Config>,

    pub mint_account: InterfaceAccount<'info, Mint>,

    pub price_update: Account<'info, PriceUpdateV2>,

    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn process_redeem_and_burn(
    ctx: Context<RedeemAndBurn>,
    redeemable_amount: u64,
    burn_amount: u64,
) -> Result<()> {
    let collateral = &mut ctx.accounts.collateral;
    collateral.total_minted -= burn_amount;
    collateral.lamport_balance = ctx.accounts.redeemer_sol_account.lamports() - burn_amount;

    let acc = &ctx.accounts;

    check_health_factor(&acc.price_update, &acc.collateral, &acc.config)?;

    burn_token(
        &acc.token_program,
        &acc.mint_account,
        &acc.redeemer_token_account,
        &acc.redeemer,
        burn_amount,
    )?;

    redeem_sol(
        &acc.system_program,
        &acc.redeemer_sol_account,
        &acc.redeemer,
        acc.redeemer.key(),
        redeemable_amount,
        acc.collateral.bump_sol_account,
    )
}
