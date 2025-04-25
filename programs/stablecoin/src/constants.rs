use anchor_lang::prelude::*;

pub const ANCHOR_DISCRIMINATOR: usize = 8;

#[constant]
pub const SEED_CONFIG_ACCOUNT: &[&str] = b"config".as_ref();

#[constant]
pub const SEED_MINT_ACCOUNT: &[&str] = b"mint".as_ref();

#[constant]
pub const LIQUIDATION_THREASHOLD: u64 = 50;

#[constant]
pub const LIQUIDATION_BONUS: u64 = 10;

#[constant]
pub const HEALTH_FACTOR: u64 = 1;
