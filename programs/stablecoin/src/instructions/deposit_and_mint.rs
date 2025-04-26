use anchor_lang::{prelude::*, system_program};
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, Token2022},
};
use pyth_solana_receiver_sdk::price_update::{get_feed_id_from_hex, PriceUpdateV2};

use crate::{
    Collateral, Config, ANCHOR_DISCRIMINATOR, MAX_AGE, SEED_COLATERAL_ACCOUNT, SEED_CONFIG_ACCOUNT, SEED_MINT_ACCOUNT, SEED_SOL_ACCOUNT, SOL_USD_FEED_ID
};

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
        seeds = [SEED_COLATERAL_ACCOUNT, depositor.key().as_ref()],
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
        bump = config.bump_mint_account // [?]: check if it doesn't fail
    )]
    pub mint_account: InterfaceAccount<'info, Mint>,
 
    // pub sol_account: InterfaceAccount<'info, Mint>, // not necesary because we use the system_program to transfer sol

    pub price_update: Account<'info, PriceUpdateV2>,

    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn process_deposit_and_mint(ctx: Context<DepositAndMint>, deposit_amount: u64) -> Result<()> {
    // transfer from signer to sol collateral account

    // system_program::transfer(
    //     CpiContext::new(
    //         ctx.accounts.system_program.to_account_info(),
    //         system_program::Transfer {
    //             from: ctx.accounts.depositor.to_account_info(),
    //             to: ctx,
    //         }
    //     ),
    //     deposit_amount
    // );

    // calculate how many stablecoin send to signer
    let price_update = &mut ctx.accounts.price_update;

    // get_price_no_older_than will fail if the price update is for a different price feed.
    // See https://pyth.network/developers/price-feed-ids for all available IDs.
    let feed_id: [u8; 32] = get_feed_id_from_hex(SOL_USD_FEED_ID)?;
    let price = price_update.get_price_no_older_than(&Clock::get()?, MAX_AGE, &feed_id)?;
    // Sample output:
    // The price is (7160106530699 ± 5129162301) * 10^-8
    msg!("The price is ({} ± {}) * 10^{}", price.price, price.conf, price.exponent);

    // mint stablecoin to signer

    // update accounts
    let collateral = &mut ctx.accounts.collateral;

    if !collateral.is_initialized{
        // CHECK this
        collateral.is_initialized = true;
        collateral.depositor = ctx.accounts.depositor.key();
        collateral.sol_account = ctx.accounts.depositor_sol_account.key();
        collateral.token_account = ctx.accounts.depositor_token_account.key();
        // collateral.lamport_balance = ctx.accounts.depositor_sol_account.lamports();
        // collateral.total_minted = // TODO: calculate how many mint
        collateral.bump = ctx.bumps.collateral;
        collateral.bump_sol_account = ctx.bumps.depositor_sol_account;
    }

    Ok(())
}
