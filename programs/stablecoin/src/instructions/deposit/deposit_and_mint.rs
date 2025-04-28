use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, Token2022},
};
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

use crate::{
       check_health_factor,
       Collateral,
       Config,
       ANCHOR_DISCRIMINATOR,
       SEED_COLLATERAL_ACCOUNT,
       SEED_CONFIG_ACCOUNT,
       SEED_MINT_ACCOUNT,
       SEED_SOL_ACCOUNT
};

use super::{deposit_sol, mint_token};

#[derive(Accounts)]
pub struct DepositAndMint<'info> {
    #[account(mut)]
    pub depositor: Signer<'info>,

    #[account(
        mut, // not init_if_needed because already has a sol token account if call this method
        // associated_token::mint = sol_account,
        // associated_token::authority = depositor,
        // associated_token::token_program = token_program
        seeds = [SEED_SOL_ACCOUNT, depositor.key().as_ref()],
        bump
    )]
    pub depositor_sol_account: SystemAccount<'info>, // SystemAccount beacause is real sol

    #[account(
        init_if_needed,
        payer = depositor,
        associated_token::mint = mint_account,
        associated_token::authority = depositor,
        associated_token::token_program = token_program
    )]
    pub depositor_token_account: InterfaceAccount<'info, TokenAccount>, // stablecoin

    #[account(
        init_if_needed,
        payer = depositor,
        space = Collateral::INIT_SPACE + ANCHOR_DISCRIMINATOR,
        seeds = [SEED_COLLATERAL_ACCOUNT, depositor.key().as_ref()],
        bump,
        // has_one = depositor
    )]
    pub collateral: Account<'info, Collateral>,

    // #[account(
    //     init_if_needed,
    //     payer = depositor,
    //     associated_token::mint = sol_account,
    //     associated_token::authority = collateral,
    //     associated_token::token_program = token_program
    // )]
    // pub collateral_sol_account: InterfaceAccount<'info, TokenAccount>,

    #[account( 
        seeds = [SEED_CONFIG_ACCOUNT],
        bump = config.bump,
        has_one = mint_account
    )]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        seeds = [SEED_MINT_ACCOUNT],
        bump 
    )]
    pub mint_account: InterfaceAccount<'info, Mint>,
 
    // pub sol_account: InterfaceAccount<'info, Mint>, // not necesary because we use the system_program to transfer sol

    pub price_update: Account<'info, PriceUpdateV2>,

    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn process_deposit_and_mint(ctx: Context<DepositAndMint>, deposit_amount: u64, mint_amount: u64) -> Result<()> {
    let collateral = &mut ctx.accounts.collateral;
    
    collateral.lamport_balance = ctx.accounts.depositor_sol_account.lamports();
    collateral.total_minted = mint_amount;

    if !collateral.is_initialized{
        collateral.is_initialized = true;
        collateral.depositor = ctx.accounts.depositor.key();
        collateral.sol_account = ctx.accounts.depositor_sol_account.key();
        collateral.token_account = ctx.accounts.depositor_token_account.key();
        collateral.bump = ctx.bumps.collateral;
        collateral.bump_sol_account = ctx.bumps.depositor_sol_account;
    }
    
    let acc = &ctx.accounts;
    
    deposit_sol(&acc.system_program , &acc.depositor, &acc.depositor_sol_account, deposit_amount)?;
    
    check_health_factor(&acc.price_update, &acc.collateral, &acc.config)?;

    mint_token(&acc.token_program, &acc.mint_account, &acc.depositor_token_account, mint_amount , acc.config.bump_mint_account)
}
