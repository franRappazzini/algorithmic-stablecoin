use anchor_lang::{prelude::*, system_program};
use anchor_spl::{
    token_2022,
    token_interface::{Mint, Token2022, TokenAccount},
};

use crate::SEED_MINT_ACCOUNT;

pub fn deposit_sol<'info>(
    system_program: &Program<'info, System>,
    from: &Signer<'info>,
    to: &SystemAccount<'info>,
    lamports: u64,
) -> Result<()> {
    system_program::transfer(
        CpiContext::new(
            system_program.to_account_info(),
            system_program::Transfer {
                from: from.to_account_info(),
                to: to.to_account_info(),
            },
        ),
        lamports,
    )
}

pub fn mint_token<'info>(
    token_program: &Program<'info, Token2022>,
    mint: &InterfaceAccount<'info, Mint>,
    to: &InterfaceAccount<'info, TokenAccount>,
    amount: u64,
    bump: u8,
) -> Result<()> {
    let signer_seeds: &[&[&[u8]]] = &[&[SEED_MINT_ACCOUNT, &[bump]]];

    token_2022::mint_to(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            token_2022::MintTo {
                mint: mint.to_account_info(),
                to: to.to_account_info(),
                authority: mint.to_account_info(),
            },
            signer_seeds,
        ),
        amount,
    )
}
