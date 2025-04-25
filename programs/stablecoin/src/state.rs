use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Collateral {
    pub depositor: Pubkey,
    pub sol_account: Pubkey,
    pub stablecoin_account: Pubkey,
    pub lamport_balance: u64,
    pub total_minted: u64,
    pub bump: u8,
    pub bump_sol_account: u8,
    pub is_initialized: bool,
}

#[account]
#[derive(InitSpace)]
pub struct Config {
    pub auhority: Pubkey,
    pub mint_account: Pubkey,
    pub liquidation_threshold: u64, // determina cuanto collateral es necesario
    pub liquidation_bonus: u64,     // bonus al liquidador de una posicion (porcentaje en lamports)
    pub health_factor: u64,         // numero minimo por el cual una posicion puede ser liquidada
    pub bump: u8,
    pub bump_mint_account: u8,
}
