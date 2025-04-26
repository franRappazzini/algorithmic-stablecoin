use anchor_lang::prelude::*;

pub const ANCHOR_DISCRIMINATOR: usize = 8;

#[constant]
pub const SEED_CONFIG_ACCOUNT: &[u8] = b"config";

#[constant]
pub const SEED_MINT_ACCOUNT: &[u8] = b"mint";

#[constant]
pub const SEED_COLLATERAL_ACCOUNT: &[u8] = b"collateral";

#[constant]
pub const SEED_SOL_ACCOUNT: &[u8] = b"sol";

#[constant]
pub const LIQUIDATION_THREASHOLD: u64 = 50;

#[constant]
pub const LIQUIDATION_BONUS: u64 = 10;

#[constant]
pub const HEALTH_FACTOR: u64 = 1;

// get_price_no_older_than will fail if the price update is more than 30 seconds old
#[constant]
pub const MAX_AGE: u64 = 30;

#[constant]
pub const SOL_USD_FEED_ID: &str =
    "ef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d";

#[constant]
pub const PRICE_FEED_DECIMAL_ADJUSTMENT: u128 = 10;
