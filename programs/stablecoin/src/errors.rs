use anchor_lang::prelude::*;

#[error_code]
pub enum DappError {
    #[msg("Invalid price from Pyth oracle")]
    InvalidPrice,

    #[msg("Health factor is too low")]
    HealthFactorIsTooLow,
}
