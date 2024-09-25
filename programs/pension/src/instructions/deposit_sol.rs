use crate::state::Pension;
use anchor_lang::prelude::*;

use anchor_lang::system_program;

#[derive(Accounts)]
pub struct DepositSol<'info> {
    #[account(
        init,
        payer = user,
        seeds = [b"doposit".as_ref(), user.key().as_ref()],
        space = 8 + 4 + 2 + 1,
        bump
    )]
    pub pension_account: Account<'info, Pension>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}


pub fn deposit_sol(
    ctx: Context<DepositSol>,
    amount: u64,
    expected_lamports: u16,
    expected_year: u8,
) -> Result<()> {
    let pension_account = &mut ctx.accounts.pension_account;

    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.user.to_account_info(),
                to: pension_account.to_account_info(),
            },
        ),
        amount,
    )?;

    pension_account.expected_lamports = expected_lamports;
    pension_account.expected_year = expected_year;

    Ok(())
}
