use std::u64;

use anchor_lang::prelude::*;
use pyth_solana_receiver_sdk::price_update::{get_feed_id_from_hex, PriceUpdateV2};
use solana_program::native_token::LAMPORTS_PER_SOL;

use crate::{
    Collateral, Config, DappError, MAX_AGE, PRICE_FEED_DECIMAL_ADJUSTMENT, SOL_USD_FEED_ID,
};

pub fn check_health_factor(
    price_update: &Account<PriceUpdateV2>,
    collateral: &Account<Collateral>,
    config: &Account<Config>,
) -> Result<()> {
    let health_factor = calculate_health_factor(price_update, collateral, config)?;
    require!(
        health_factor >= config.health_factor,
        DappError::HealthFactorIsTooLow
    );

    Ok(())
}

pub fn calculate_health_factor(
    price_update: &Account<PriceUpdateV2>,
    collateral: &Account<Collateral>,
    config: &Account<Config>,
) -> Result<u64> {
    let sol_value_in_usd = sol_to_usd(collateral.lamport_balance, &price_update)?;

    let sol_adjusted_for_liquidation_threshold =
        (sol_value_in_usd * config.liquidation_threshold) / 100;

    if collateral.total_minted == 0 {
        msg!("Max Health Factor. The total minted is 0");
        return Ok(u64::MAX);
    }

    let health_factor = sol_adjusted_for_liquidation_threshold / collateral.total_minted;

    Ok(health_factor)
}

pub fn sol_to_usd(lamports_amount: u64, price_update: &Account<PriceUpdateV2>) -> Result<u64> {
    // get_price_no_older_than will fail if the price update is for a different price feed.
    // See https://pyth.network/developers/price-feed-ids for all available IDs.
    let feed_id: [u8; 32] = get_feed_id_from_hex(SOL_USD_FEED_ID)?;
    let sol_price = price_update.get_price_no_older_than(&Clock::get()?, MAX_AGE, &feed_id)?;
    // Sample output:
    // The SOL price is (7160106530699 ± 5129162301) * 10^-8
    msg!(
        "The SOL price is ({} ± {}) * 10^{}",
        sol_price.price,
        sol_price.conf,
        sol_price.exponent
    );

    require!(sol_price.price > 0, DappError::InvalidPrice);

    let price_in_usd = sol_price.price as u128 * PRICE_FEED_DECIMAL_ADJUSTMENT;
    let amount_in_usd =
        ((lamports_amount as u128 * price_in_usd) / LAMPORTS_PER_SOL as u128) as u64;

    Ok(amount_in_usd)
}

pub fn usd_to_lamports(usd_amount: u64, price_update: &Account<PriceUpdateV2>) -> Result<u64> {
    let feed_id = get_feed_id_from_hex(SOL_USD_FEED_ID)?;
    let sol_price = price_update.get_price_no_older_than(&Clock::get()?, MAX_AGE, &feed_id)?;

    require!(sol_price.price > 0, DappError::InvalidPrice);

    let price_in_usd = sol_price.price as u128 * PRICE_FEED_DECIMAL_ADJUSTMENT;

    let amount_in_lamports = (usd_amount as u128 * LAMPORTS_PER_SOL as u128) / price_in_usd;

    Ok(amount_in_lamports as u64)
}
