use anchor_lang::{prelude::*, system_program};
use anchor_spl::{
    token_2022,
    token_interface::{Mint, Token2022, TokenAccount},
};

use crate::SEED_SOL_ACCOUNT;

pub fn burn_token<'info>(
    token_program: &Program<'info, Token2022>,
    mint: &InterfaceAccount<'info, Mint>,
    from: &InterfaceAccount<'info, TokenAccount>,
    authority: &Signer<'info>,
    burn_amount: u64,
) -> Result<()> {
    token_2022::burn(
        CpiContext::new(
            token_program.to_account_info(),
            token_2022::Burn {
                mint: mint.to_account_info(),
                from: from.to_account_info(),
                authority: authority.to_account_info(),
            },
        ),
        burn_amount,
    )
}

pub fn redeem_sol<'info>(
    system_program: &Program<'info, System>,
    from: &SystemAccount<'info>,
    to: &Signer<'info>,
    amount: u64,
    bump: u8,
) -> Result<()> {
    let to_key = to.key();
    let signer_seeds: &[&[&[u8]]] = &[&[SEED_SOL_ACCOUNT, to_key.as_ref(), &[bump]]]; // [?] check this

    system_program::transfer(
        CpiContext::new_with_signer(
            system_program.to_account_info(),
            system_program::Transfer {
                from: from.to_account_info(),
                to: to.to_account_info(),
            },
            signer_seeds,
        ),
        amount,
    )
}
